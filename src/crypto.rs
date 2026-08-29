use ring::rand::SystemRandom;
use ring::signature::{self, Ed25519KeyPair, KeyPair, UnparsedPublicKey};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum CryptoError {
    #[error("فشل في توليد المفتاح: {0}")]
    GenerationFailed(String),
    #[error("فشل التحقق من صحة التوقيع الرقمي")]
    VerificationFailed,
}

#[derive(Clone)]
pub struct NodeIdentity {
    pub pub_key_bytes: Vec<u8>,
    kp_bytes: Vec<u8>,
}

impl NodeIdentity {
    pub fn generate() -> Result<Self, CryptoError> {
        let rng = SystemRandom::new();
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng)
            .map_err(|e| CryptoError::GenerationFailed(format!("{:?}", e)))?;
        let keypair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())
            .map_err(|e| CryptoError::GenerationFailed(format!("{:?}", e)))?;
        
        Ok(Self {
            pub_key_bytes: keypair.public_key().as_ref().to_vec(),
            kp_bytes: pkcs8_bytes.as_ref().to_vec(),
        })
    }

    pub fn public_key(&self) -> &[u8] {
        &self.pub_key_bytes
    }

    pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let keypair = Ed25519KeyPair::from_pkcs8(&self.kp_bytes)
            .map_err(|e| CryptoError::GenerationFailed(format!("{:?}", e)))?;
        let signature = keypair.sign(message);
        Ok(signature.as_ref().to_vec())
    }

    pub fn verify(public_key: &[u8], message: &[u8], signature_bytes: &[u8]) -> Result<(), CryptoError> {
        let peer_public_key = UnparsedPublicKey::new(&signature::ED25519, public_key);
        peer_public_key.verify(message, signature_bytes)
            .map_err(|_| CryptoError::VerificationFailed)
    }
}

pub struct CryptoManager {
    rng: SystemRandom,
}

impl CryptoManager {
    pub fn new() -> Self {
        Self {
            rng: SystemRandom::new(),
        }
    }

    pub fn generate_keypair(&self) -> Result<Ed25519KeyPair, String> {
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&self.rng)
            .map_err(|e| format!("فشل في توليد المفتاح: {:?}", e))?;
        
        Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())
            .map_err(|e| format!("فشل في تحليل المفتاح: {:?}", e))
    }

    pub fn sign(keypair: &Ed25519KeyPair, message: &[u8]) -> Vec<u8> {
        let signature = keypair.sign(message);
        signature.as_ref().to_vec()
    }

    pub fn verify(public_key_bytes: &[u8], message: &[u8], signature_bytes: &[u8]) -> Result<(), CryptoError> {
        NodeIdentity::verify(public_key_bytes, message, signature_bytes)
    }
}
