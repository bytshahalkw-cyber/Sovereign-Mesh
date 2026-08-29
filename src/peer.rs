use std::net::SocketAddr;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PeerInfo {
    pub addr: SocketAddr,
    pub pub_key: Vec<u8>,
    pub last_seen: std::time::Instant,
}

pub struct PeerRegistry {
    peers: HashMap<Vec<u8>, PeerInfo>,
}

impl PeerRegistry {
    pub fn new() -> Self {
        Self { peers: HashMap::new() }
    }

    pub fn upsert_peer(&mut self, pub_key: Vec<u8>, addr: SocketAddr) {
        self.peers.insert(pub_key.clone(), PeerInfo {
            addr,
            pub_key,
            last_seen: std::time::Instant::now(),
        });
        println!("🌐 [الشبكة] تم تسجيل أو تحديث العقدة بنجاح على العنوان: {}", addr);
    }

    pub fn get_peer_addr(&self, pub_key: &[u8]) -> Option<&SocketAddr> {
        self.peers.get(pub_key).map(|p| &p.addr)
    }

    pub fn active_count(&self) -> usize {
        self.peers.len()
    }
}
