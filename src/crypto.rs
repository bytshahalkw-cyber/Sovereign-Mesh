use ring::signature::{self, KeyPair, Ed25519KeyPair};
use ring::rand::{SystemRandom, SecureRandom};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("فشل في توليد المفاتيح العشوائية")]
    KeyGenerationFailed,
    #[error("فشل في التوقيع الرقمي للرسالة")]
    SigningFailed,
    #[error("فشل التحقق من صحة التوقيع")]
    VerificationFailed,
    #[error("تنسيق المفتاح غير صالح")]
    InvalidKeyFormat,
}

pub struct NodeIdentity {
    key_pair: Ed25519KeyPair,
    public_key_bytes: Vec<u8>,
}

impl NodeIdentity {
    pub fn generate() -> Result<Self, CryptoError> {
        let rng = SystemRandom::new();
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng)
            .map_err(|_| CryptoError::KeyGenerationFailed)?;
        
        let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())
            .map_err(|_| CryptoError::KeyGenerationFailed)?;
        
        let public_key_bytes = key_pair.public_key().as_ref().to_vec();

        Ok(Self {
            key_pair,
            public_key_bytes,
        })
    }

    pub fn public_key(&self) -> &[u8] {
        &self.public_key_bytes
    }

    pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let signature = self.key_pair.sign(message);
        Ok(signature.as_ref().to_vec())
    }

    pub fn verify(public_key: &[u8], message: &[u8], signature: &[u8]) -> Result<(), CryptoError> {
        let peer_public_key = signature::UnparsedPublicKey::new(
            &signature::ED25519,
            public_key,
        );

        peer_public_key
            .verify(message, signature)
            .map_err(|_| CryptoError::VerificationFailed)
    }
}
