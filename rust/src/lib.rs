// A simple C wrapper of tokenzier library
//
// Modified by Ambarella International LP. in 2026.
// Changes: added decoder-aware tokenizers_id_to_token_ext(...) and supporting
// ByteLevel character-to-byte conversion.
// Based on tokenizers-cpp commit
// 0621f84c1aed671d97cfd6eddd127809a20bc94f.
use serde_json::Value;
use std::{collections::HashMap, str::FromStr};
use lazy_static::lazy_static;
use tokenizers::models::bpe::BPE;
use tokenizers::pre_tokenizers::byte_level::ByteLevel;
use tokenizers::tokenizer::Tokenizer;
use tokenizers::tokenizer::Decoder;
use tokenizers::decoders::DecoderWrapper;

pub struct TokenizerWrapper {
    tokenizer: Tokenizer,
    decode_str: String,
    id_to_token_result: String,
}

pub type Vocab = HashMap<String, u32>;
pub type Merges = Vec<(String, String)>;

#[repr(C)]
pub struct TokenizerEncodeResult {
    token_ids: *mut u32,
    len: usize,
}

impl TokenizerWrapper {
    pub fn from_str(json: &str) -> TokenizerWrapper {
        TokenizerWrapper {
            tokenizer: Tokenizer::from_str(json).unwrap().into(),
            decode_str: String::new(),
            id_to_token_result: String::new(),
        }
    }

    pub fn byte_level_bpe_from_str(
        vocab: &str,
        merges: &str,
        added_tokens: &str,
    ) -> TokenizerWrapper {
        let vocab_json: Value = serde_json::from_str(vocab).unwrap();
        let added_tokens_json: Value = serde_json::from_str(added_tokens).unwrap();
        let mut vocab = HashMap::new();
        match vocab_json {
            Value::Object(m) => {
                for (token, id) in m {
                    if let Value::Number(id) = id {
                        let id = id.as_u64().unwrap() as u32;
                        vocab.insert(token, id);
                    }
                }
            }
            _ => panic!("Invalid vocab.json file."),
        };
        match added_tokens_json {
            Value::Object(m) => {
                for (token, id) in m {
                    if let Value::Number(id) = id {
                        let id = id.as_u64().unwrap() as u32;
                        vocab.insert(token, id);
                    }
                }
            }
            _ => panic!("Invalid added_tokens.json file."),
        }

        let merges = merges
            .lines()
            .filter(|line| !line.starts_with("#version"))
            .map(|line| {
                let parts = line.split(' ').collect::<Vec<_>>();
                if parts.len() != 2 {
                    panic!("Invalid merges.txt file.")
                }
                return (parts[0].to_string(), parts[1].to_string()); // Add the `return` keyword here
            })
            .collect::<Vec<(String, String)>>();
        let byte_level = ByteLevel::new(
            /*add_prefix_space=*/ false, /*trim_offsets=*/ false,
            /*use_regex=*/ false,
        );
        let mut tokenizer = Tokenizer::new(BPE::new(vocab, merges));
        tokenizer
            .with_pre_tokenizer(Some(byte_level))
            .with_decoder(Some(byte_level));
        TokenizerWrapper {
            tokenizer: tokenizer,
            decode_str: String::new(),
            id_to_token_result: String::new(),
        }
    }

    pub fn encode(&mut self, text: &str, add_special_tokens: bool) -> Vec<u32> {
        let encoded = self.tokenizer.encode(text, add_special_tokens).unwrap();
        return encoded.get_ids().to_vec();
    }

    pub fn encode_batch(&mut self, texts: Vec<&str>, add_special_tokens: bool) -> Vec<Vec<u32>> {
        let results = self.tokenizer.encode_batch(texts, add_special_tokens).unwrap()
            .into_iter()
            .map(|encoded| encoded.get_ids().to_vec())
            .collect::<Vec<Vec<u32>>>();
        return results;
    }

    pub fn decode(&mut self, ids: &[u32], skip_special_tokens: bool) {
        self.decode_str = self.tokenizer.decode(ids, skip_special_tokens).unwrap();
    }
}

#[no_mangle]
extern "C" fn tokenizers_new_from_str(input_cstr: *const u8, len: usize) -> *mut TokenizerWrapper {
    unsafe {
        let json = std::str::from_utf8(std::slice::from_raw_parts(input_cstr, len)).unwrap();
        return Box::into_raw(Box::new(TokenizerWrapper::from_str(json)));
    }
}

#[no_mangle]
extern "C" fn byte_level_bpe_tokenizers_new_from_str(
    input_vocab_str: *const u8,
    len_vocab: usize,
    input_merges_str: *const u8,
    len_merges: usize,
    input_added_tokens_str: *const u8,
    len_added_tokens: usize,
) -> *mut TokenizerWrapper {
    unsafe {
        let vocab =
            std::str::from_utf8(std::slice::from_raw_parts(input_vocab_str, len_vocab)).unwrap();
        let merges =
            std::str::from_utf8(std::slice::from_raw_parts(input_merges_str, len_merges)).unwrap();
        let added_tokens = std::str::from_utf8(std::slice::from_raw_parts(
            input_added_tokens_str,
            len_added_tokens,
        ))
        .unwrap();
        return Box::into_raw(Box::new(TokenizerWrapper::byte_level_bpe_from_str(
            vocab,
            merges,
            added_tokens,
        )));
    }
}

#[no_mangle]
extern "C" fn tokenizers_encode(
    handle: *mut TokenizerWrapper,
    input_cstr: *const u8,
    len: usize,
    add_special_tokens: i32,
    out_result: *mut TokenizerEncodeResult,
) {
    unsafe {
        let input_data = std::str::from_utf8(std::slice::from_raw_parts(input_cstr, len)).unwrap();
        let encoded = (*handle).encode(input_data, add_special_tokens != 0);
        let len = encoded.len();
        *out_result = TokenizerEncodeResult {
            token_ids: Box::into_raw(encoded.into_boxed_slice()) as *mut u32,
            len: len,
        };
    }
}

#[no_mangle]
extern "C" fn tokenizers_encode_batch(
    handle: *mut TokenizerWrapper,
    input_cstr: *const *const u8,
    input_len: *const usize,
    num_seqs: usize,
    add_special_tokens: i32,
    out_result: *mut TokenizerEncodeResult,
) {
    unsafe {
        let input_data = (0..num_seqs)
            .map(|i| {
                std::str::from_utf8(std::slice::from_raw_parts(*input_cstr.offset(i as isize), *input_len.offset(i as isize))).unwrap()
            })
            .collect::<Vec<&str>>();
        let encoded_batch = (*handle).encode_batch(input_data, add_special_tokens != 0);
        for (i, encoded) in encoded_batch.into_iter().enumerate() {
            let len = encoded.len();
            let result = TokenizerEncodeResult {
                token_ids: Box::into_raw(encoded.into_boxed_slice()) as *mut u32,
                len: len,
            };
            *out_result.offset(i as isize) = result;
        }
    }
}

#[no_mangle]
extern "C" fn tokenizers_free_encode_results(results: *mut TokenizerEncodeResult, num_seqs: usize) {
    unsafe {
        let slice = std::slice::from_raw_parts_mut(results, num_seqs);
        for result in &mut *slice {
            drop(Box::from_raw(std::slice::from_raw_parts_mut(result.token_ids, result.len)));
        }
    }
}

#[no_mangle]
extern "C" fn tokenizers_decode(
    handle: *mut TokenizerWrapper,
    input_ids: *const u32,
    len: usize,
    skip_special_tokens: i32,
) {
    unsafe {
        let input_data = std::slice::from_raw_parts(input_ids, len);
        (*handle).decode(input_data, skip_special_tokens != 0);
    }
}

#[no_mangle]
extern "C" fn tokenizers_get_decode_str(
    handle: *mut TokenizerWrapper,
    out_cstr: *mut *mut u8,
    out_len: *mut usize,
) {
    unsafe {
        *out_cstr = (*handle).decode_str.as_mut_ptr();
        *out_len = (*handle).decode_str.len();
    }
}

#[no_mangle]
extern "C" fn tokenizers_free(wrapper: *mut TokenizerWrapper) {
    unsafe {
        drop(Box::from_raw(wrapper));
    }
}

#[no_mangle]
extern "C" fn tokenizers_get_vocab_size(handle: *mut TokenizerWrapper, size: *mut usize) {
    unsafe {
        *size = (*handle).tokenizer.get_vocab_size(true);
    }
}

#[no_mangle]
extern "C" fn tokenizers_id_to_token(
    handle: *mut TokenizerWrapper,
    id: u32,
    out_cstr: *mut *mut u8,
    out_len: *mut usize,
) {
    unsafe {
        let str = (*handle).tokenizer.id_to_token(id);
        (*handle).id_to_token_result = match str {
            Some(s) => s,
            None => String::from(""),
        };

        *out_cstr = (*handle).id_to_token_result.as_mut_ptr();
        *out_len = (*handle).id_to_token_result.len();
    }
}

#[no_mangle]
extern "C" fn tokenizers_token_to_id(
    handle: *mut TokenizerWrapper,
    token: *const u8,
    len: usize,
    out_id: *mut i32,
) {
    unsafe {
        let token: &str = std::str::from_utf8(std::slice::from_raw_parts(token, len)).unwrap();
        let id = (*handle).tokenizer.token_to_id(token);
        *out_id = match id {
            Some(id) => id as i32,
            None => -1,
        };
    }
}

pub(crate) fn bytes_char() -> HashMap<u8, char> {
    let mut bs: Vec<u8> = vec![];
    bs.extend(b'!'..=b'~');
    bs.extend(b'\xA1'..=b'\xAC');
    bs.extend(b'\xAE'..=b'\xFF');

    let mut cs: Vec<u32> = bs.iter().map(|i| *i as u32).collect();
    let mut n = 0;

    for b in 0..=255u8 {
        if !bs.contains(&b) {
            bs.push(b);
            cs.push(u32::pow(2, 8) + n);
            n += 1;
        }
    }

    bs.into_iter()
        .zip(cs)
        .map(|(f, t)| (f, unsafe { std::char::from_u32_unchecked(t) }))
        .collect()
}

lazy_static! {
    static ref CHAR_BYTES: HashMap<char, u8> =
        bytes_char().into_iter().map(|(c, b)| (b, c)).collect();
}

const INVALID_RAW_WORD : u32 = 0xFFFFFFFF;
#[no_mangle]
extern "C" fn tokenizers_id_to_token_ext(
    handle: *mut TokenizerWrapper,
    id: u32,
    out_cstr: *mut *mut u8,
    out_len: *mut usize,
    is_start: u32,
    raw_word: *mut u32,
) {
    unsafe {
        let str = (*handle).tokenizer.id_to_token(id);
        let raw_token = match str {
            Some(s) => s,
            None => String::from(""),
        };

        *raw_word = INVALID_RAW_WORD;

        let mut token = raw_token.clone();

        if let Some(decoder) = (*handle).tokenizer.get_decoder() {
            match decoder {
                DecoderWrapper::Sequence(sequence) => {
                    let sequence_decoders = sequence.get_decoders();
                    for sequence_decode in sequence_decoders {
                        match sequence_decode {
                            DecoderWrapper::Strip(sequence_decode_strip) => {
                                if is_start != 0 {
                                    token = sequence_decode_strip.decode_chain(vec![token].clone()).unwrap()[0].clone();
                                }
                            },

                            DecoderWrapper::ByteFallback(_) => {
                                if raw_token.len() == 6 && raw_token.starts_with("<0x") && raw_token.ends_with('>') {
                                    if let Ok(byte) = u8::from_str_radix(&raw_token[3..5], 16) {
                                        *raw_word = byte.into();
                                    } else {
                                        println!("Fail in ByteFallback!");
                                    }
                                }

                            },

                            decode => {
                                token = decode.decode_chain(vec![token].clone()).unwrap()[0].clone();
                            }
                        }
                    }
                },

                DecoderWrapper::Strip(decode_strip) => {
                    if is_start != 0 {
                        token = decode_strip.decode_chain(vec![token].clone()).unwrap()[0].clone();
                    }
                },

                DecoderWrapper::ByteFallback(_) => {
                    if raw_token.len() == 6 && raw_token.starts_with("<0x") && raw_token.ends_with('>') {
                        if let Ok(byte) = u8::from_str_radix(&raw_token[3..5], 16) {
                            *raw_word = byte.into();
                        } else {
                            println!("Fail in ByteFallback!");
                        }
                    }

                },

                DecoderWrapper::ByteLevel(_) => {
                    let toks = vec![token].clone()
                        .into_iter()
                        .flat_map(|t| {
                            t.chars()
                                .try_fold(vec![], |mut acc, c| {
                                    CHAR_BYTES.get(&c).map(|b| {
                                        acc.push(*b);
                                        acc
                                    })
                                })
                                .unwrap_or_else(|| t.as_bytes().to_vec())
                        })
                        .collect::<Vec<u8>>();
                    token = String::from_utf8_unchecked(toks).to_string().clone();

                },

                other_decode => {
                    token = other_decode.decode_chain(vec![token].clone()).unwrap()[0].clone();
                },
            }
        } else {
            println!("Fail to get Decoder!");
        }

        (*handle).id_to_token_result = token;
        *out_cstr = (*handle).id_to_token_result.as_mut_ptr();
        *out_len = (*handle).id_to_token_result.len();
    }
}
