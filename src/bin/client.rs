use sovereign_mesh_core::protocol::{MeshPacket, MessageType};
use ring::signature::{Ed25519KeyPair, KeyPair};
use ring::rand::SystemRandom;
use std::net::UdpSocket;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📤 عميل Sovereign Mesh - إرسال رسالة");

    let rng = SystemRandom::new();
    let key_pair = Ed25519KeyPair::generate_pkcs8(&rng)?;
    let key_pair = Ed25519KeyPair::from_pkcs8(&ring::signature::ED25519, key_pair.as_ref())?;
    
    let destination = vec![0u8; 32];
    let payload = b"رسالة سرية تقفز عبر الشبكة!".to_vec();
    
    let packet = MeshPacket::new(
        &key_pair,
        destination,
        MessageType::Data(payload),
        vec![],
    );

    println!("📦 معرّف الحزمة: {:?}", &packet.message_id[..4]);
    println!("🔄 TTL: {} | Hop Count: {}", packet.ttl, packet.hop_count);

    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let serialized = bincode::serialize(&packet)?;
    socket.send_to(&serialized, "127.0.0.1:8080")?;

    println!("✅ تم إرسال الحزمة بنجاح");
    Ok(())
}
