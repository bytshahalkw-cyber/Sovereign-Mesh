use serde::{Serialize, Deserialize};
use ring::signature::{self, UnparsedPublicKey, ED25519, Ed25519KeyPair, KeyPair};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageType {
    Handshake { 
        pub_key: Vec<u8> 
    },
    Data { 
        sender_pub_key: Vec<u8>, 
        destination_pub_key: Vec<u8>,
        payload: Vec<u8>, 
        signature: Vec<u8> 
    },
    Heartbeat,
}

pub struct MeshProtocol;

impl MeshProtocol {
    pub fn create_data_packet(
        sender_pub_key: &[u8],
        destination_pub_key: &[u8],
        payload: &[u8],
        keypair: &Ed25519KeyPair,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut signed_data = Vec::new();
        signed_data.extend_from_slice(sender_pub_key);
        signed_data.extend_from_slice(destination_pub_key);
        signed_data.extend_from_slice(payload);

        let signature = keypair.sign(&signed_data);

        let message = MessageType::Data {
            sender_pub_key: sender_pub_key.to_vec(),
            destination_pub_key: destination_pub_key.to_vec(),
            payload: payload.to_vec(),
            signature: signature.as_ref().to_vec(),
        };

        let encoded = bincode::serialize(&message)?;
        Ok(encoded)
    }

    pub fn verify_packet(bytes: &[u8]) -> Result<MessageType, Box<dyn std::error::Error>> {
        let message: MessageType = bincode::deserialize(bytes)?;
        
        match &message {
            MessageType::Data { sender_pub_key, destination_pub_key, payload, signature } => {
                let peer_key = UnparsedPublicKey::new(&ED25519, sender_pub_key);
                
                let mut signed_data = Vec::new();
                signed_data.extend_from_slice(sender_pub_key);
                signed_data.extend_from_slice(destination_pub_key);
                signed_data.extend_from_slice(payload);

                peer_key.verify(&signed_data, signature)
                    .map_err(|_| "فشل التحقق من صحة التوقيع الرقمي")?;
            }
            _ => {}
        }
        
        Ok(message)
    }
}

use x25519_dalek::{EphemeralSecret, PublicKey};

pub struct OnionLayerBuilder {
    path: Vec<String>,
}

impl OnionLayerBuilder {
    pub fn new(path: Vec<String>) -> Self {
        Self { path }
    }

    pub fn build_onion_packet(&self, final_payload: &[u8]) -> Vec<u8> {
        let mut current_packet = final_payload.to_vec();
        for hop in self.path.iter().rev() {
            let _secret = EphemeralSecret::random();
            let _public = PublicKey::from(&_secret);
            
            let mut layered_data = hop.as_bytes().to_vec();
            layered_data.extend_from_slice(&current_packet);
            current_packet = layered_data;
        }
        current_packet
    }
}
