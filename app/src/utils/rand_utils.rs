use anyhow::Context;
use argon2::{password_hash::SaltString, PasswordHash, Argon2};
use rand::Rng;
use crate::Result;
///  生成指定长度的字符串
pub fn rand_s(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    let mut rng = rand::thread_rng();

    let rand_string: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    rand_string
}

 
 
