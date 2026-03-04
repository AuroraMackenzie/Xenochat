use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit, OsRng, rand_core::RngCore},
};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use sha2::{Digest, Sha256};

const ENCRYPTED_PREFIX: &str = "enc:v1:";
const NONCE_SIZE: usize = 12;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecretCodecError {
    InvalidFormat,
    EmptyMasterKey,
    DecodeFailure,
    EncryptFailure,
    DecryptFailure,
    Utf8Failure,
}

pub fn is_encrypted_secret(value: &str) -> bool {
    value.starts_with(ENCRYPTED_PREFIX)
}

pub fn derive_key_from_passphrase(passphrase: &str) -> Result<[u8; 32], SecretCodecError> {
    if passphrase.trim().is_empty() {
        return Err(SecretCodecError::EmptyMasterKey);
    }

    let digest = Sha256::digest(passphrase.as_bytes());
    let mut key = [0_u8; 32];
    key.copy_from_slice(&digest);
    Ok(key)
}

pub fn seal_secret(plaintext: &str, master_key: &str) -> Result<String, SecretCodecError> {
    let key = derive_key_from_passphrase(master_key)?;

    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|_| SecretCodecError::EncryptFailure)?;

    let mut nonce_bytes = [0_u8; NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce_bytes);

    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .map_err(|_| SecretCodecError::EncryptFailure)?;

    let nonce_encoded = URL_SAFE_NO_PAD.encode(nonce_bytes);
    let payload_encoded = URL_SAFE_NO_PAD.encode(ciphertext);

    Ok(format!(
        "{ENCRYPTED_PREFIX}{nonce_encoded}:{payload_encoded}"
    ))
}

pub fn open_secret(sealed: &str, master_key: &str) -> Result<String, SecretCodecError> {
    let key = derive_key_from_passphrase(master_key)?;

    let Some(raw) = sealed.strip_prefix(ENCRYPTED_PREFIX) else {
        return Err(SecretCodecError::InvalidFormat);
    };

    let mut parts = raw.splitn(2, ':');
    let nonce_part = parts.next().ok_or(SecretCodecError::InvalidFormat)?;
    let payload_part = parts.next().ok_or(SecretCodecError::InvalidFormat)?;

    let nonce_bytes = URL_SAFE_NO_PAD
        .decode(nonce_part)
        .map_err(|_| SecretCodecError::DecodeFailure)?;
    if nonce_bytes.len() != NONCE_SIZE {
        return Err(SecretCodecError::InvalidFormat);
    }

    let payload = URL_SAFE_NO_PAD
        .decode(payload_part)
        .map_err(|_| SecretCodecError::DecodeFailure)?;

    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|_| SecretCodecError::DecryptFailure)?;
    let nonce = Nonce::from_slice(&nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, payload.as_ref())
        .map_err(|_| SecretCodecError::DecryptFailure)?;

    String::from_utf8(plaintext).map_err(|_| SecretCodecError::Utf8Failure)
}

#[cfg(test)]
mod tests {
    use super::{SecretCodecError, is_encrypted_secret, open_secret, seal_secret};

    #[test]
    fn encrypt_and_decrypt_roundtrip() {
        let sealed = seal_secret("demo-key", "master-passphrase").expect("seal success");
        assert!(is_encrypted_secret(&sealed));

        let plain = open_secret(&sealed, "master-passphrase").expect("open success");
        assert_eq!(plain, "demo-key");
    }

    #[test]
    fn reject_empty_master_key() {
        let err = seal_secret("abc", "").expect_err("must fail");
        assert_eq!(err, SecretCodecError::EmptyMasterKey);
    }
}
