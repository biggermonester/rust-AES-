use crypto::aes::KeySize::KeySize128;
use crypto::blockmodes::PkcsPadding;
use crypto::buffer::{RefReadBuffer, RefWriteBuffer};
use crypto::digest::Digest;
use crypto::md5::Md5;
use std::error::Error;

/// 加密
/// 16, 24, or 32 字节的 key 对应 KeySize128, KeySize192, or KeySize256
pub fn encrypt(key: &[u8], text: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut encrypt = crypto::aes::ecb_encryptor(KeySize128, key, PkcsPadding);
    let mut read_buffer = RefReadBuffer::new(text);
    let mut result = vec![0; text.len() * 4];
    let mut write_buffer = RefWriteBuffer::new(&mut result);
    encrypt
        .encrypt(&mut read_buffer, &mut write_buffer, true)
        .unwrap();
    Ok(result.into_iter().filter(|v| *v != 0).collect())
}

/// 解密
pub fn decrypt(key: &[u8], text: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut decrypt = crypto::aes::ecb_decryptor(KeySize128, key, PkcsPadding);
    let mut read_buffer = RefReadBuffer::new(text);
    let mut result = vec![0; text.len()];
    let mut write_buffer = RefWriteBuffer::new(&mut result);
    decrypt
        .decrypt(&mut read_buffer, &mut write_buffer, true)
        .unwrap();
    Ok(result)
}
