content = """use serde::{Deserialize, Serialize};
use ring::signature::{self, UnparsedPublicKey, ED25519};

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Handshake { pub_key: Vec<u8> },
    Data { sender_pub_key: Vec<u8>, payload: Vec<u8>, signature: Vec<u8> },
}

pub struct MeshProtocol;

impl MeshProtocol {
    pub fn create_handshake(pub_key: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let msg = MessageType::Handshake {
            pub_key: pub_key.to_vec(),
        };
        Ok(bincode::serialize(&msg)?)
    }

    pub fn create_data_packet(
        sender_pub_key: &[u8],
        payload: &[u8],
        keypair: &ring::signature::Ed25519KeyPair,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let signature = keypair.sign(payload).as_ref().to_vec();
        let msg = MessageType::Data {
            sender_pub_key: sender_pub_key.to_vec(),
            payload: payload.to_vec(),
            signature,
        };
        Ok(bincode::serialize(&msg)?)
    }

    pub fn verify_packet(data: &[u8]) -> Result<MessageType, String> {
        let message: MessageType = bincode::deserialize(data)
            .map_err(|e| format!("حزمة تالفة أو غير صالحة: {}", e))?;

        match &message {
            MessageType::Handshake { pub_key } => {
                if pub_key.len() != 32 {
                    return Err(format!("مفتاح عام غير صالح، الطول: {}", pub_key.len()));
                }
            }
            MessageType::Data { sender_pub_key, payload, signature } => {
                let peer_public_key = UnparsedPublicKey::new(&ED25519, sender_pub_key);
                peer_public_key.verify(payload, signature)
                    .map_err(|_| "فشل التحقق من التوقيع الرقمي (حزمة مزيفة أو معدلة)".to_string())?;
            }
        }

        Ok(message)
    }
}
"""

with open("src/protocol.rs", "w", encoding="utf-8") as f:
    f.write(content)
print("تم إنشاء src/protocol.rs بنجاح!")
