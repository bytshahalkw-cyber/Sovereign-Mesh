use crate::crypto::{NodeIdentity, CryptoError};
use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RoutingError {
    #[error("خطأ في تشفير أو فك تشفير الحزمة: {0}")]
    CryptoError(#[from] CryptoError),
    #[error("فشل في توجيه الحزمة: الوجهة غير صالحة")]
    InvalidDestination,
    #[error("انتهت صلاحية الحزمة أو تم رفضها")]
    PacketDropped,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MeshPacket {
    pub source: Vec<u8>,
    pub destination: Vec<u8>,
    pub payload: Vec<u8>,
    pub signature: Vec<u8>,
}

impl MeshPacket {
    pub fn create(
        identity: &NodeIdentity,
        destination: &[u8],
        payload: Vec<u8>,
    ) -> Result<Self, RoutingError> {
        let source = identity.public_key().to_vec();
        
        let mut sign_data = source.clone();
        sign_data.extend_from_slice(destination);
        sign_data.extend_from_slice(&payload);

        let signature = identity.sign(&sign_data)?;

        Ok(Self {
            source,
            destination: destination.to_vec(),
            payload,
            signature,
        })
    }

    pub fn verify_packet(&self) -> Result<(), RoutingError> {
        let mut sign_data = self.source.clone();
        sign_data.extend_from_slice(&self.destination);
        sign_data.extend_from_slice(&self.payload);

        NodeIdentity::verify(&self.source, &sign_data, &self.signature)
            .map_err(|_| RoutingError::PacketDropped)?;

        Ok(())
    }
}
