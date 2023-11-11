use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use base64::{engine::general_purpose, Engine as _};
use digest::Digest;
use rand_core::{OsRng, RngCore};
use sha2::Sha256;

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

pub struct AESCipher {
    key: [u8; 32],
}

impl AESCipher {
    /// 生成密钥
    pub fn new(k: &str) -> Self {
        // 通过 sha256 生成 32 字节的密钥
        let mut hasher = Sha256::new();
        hasher.update(k.as_bytes());
        let key = hasher.finalize().into();
        AESCipher { key }
    }

    /// 生成随机 iv
    fn generate_iv() -> [u8; 16] {
        // iv 为 16 字节
        let mut bytes = [0u8; 16];
        let mut rng = OsRng;
        rng.fill_bytes(&mut bytes);

        bytes
    }

    /// 加密
    pub fn encrypt(&self, plain: &str) -> String {
        // 生成随机 iv
        let iv: [u8; 16] = AESCipher::generate_iv();

        // 每 16 字节进行填充
        let buf_len = plain.len() + 16 - plain.len() % 16;
        let mut buf = vec![0u8; buf_len];

        // 指定密钥、iv、PKCS7Padding 的填充方式、明文, 进行加密
        Aes256CbcEnc::new(self.key.as_slice().into(), &iv.into())
            .encrypt_padded_b2b_mut::<Pkcs7>(plain.as_bytes(), &mut buf)
            .unwrap();

        // iv + ct
        let mut ct = iv.to_vec();
        ct.extend_from_slice(&buf);

        // base64 编码
        // general_purpose::STANDARD_NO_PAD.encode(ct)
        base64::encode(ct)
    }

    /// 解密
    pub fn decrypt(&self, encrypt: &str) -> String {
        // base64 解码
        // let bytes = general_purpose::STANDARD_NO_PAD.decode(encrypt).unwrap();
        let bytes = base64::decode(encrypt).unwrap();

        // 前 16 字节为 iv, 后面为密文
        let iv = &bytes[0..16];
        let ciphertext = &bytes[16..];

        print!("iv: {}, clip: {}", iv.len(), ciphertext.len());

        // 每 16 字节进行填充
        let buf_len = ciphertext.len() + 16 - ciphertext.len() % 16;
        let mut buf = vec![0u8; buf_len];

        // 指定密钥、iv、PKCS7Padding 的填充方式、密文, 进行解密
        let ct = Aes256CbcDec::new(self.key.as_slice().into(), iv.into())
            .decrypt_padded_b2b_mut::<Pkcs7>(ciphertext, &mut buf)
            .unwrap();

        // &[u8] -> String
        String::from_utf8_lossy(&ct).to_string()
    }
}
