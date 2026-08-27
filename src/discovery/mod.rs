use std::net::IpAddr;
use std::time::SystemTime;
use thiserror::Error;

pub type NodeId = [u8; 32];

#[derive(Debug, Clone, PartialEq)]
pub enum NodeAddress {
    IpV4(IpAddr),
    IpV6(IpAddr),
    MacAddress([u8; 6]),
}

#[derive(Debug, Clone)]
pub struct NodeInfo {
    pub id: NodeId,
    pub address: NodeAddress,
    pub signal_strength: i32,
    pub last_seen: SystemTime,
    pub is_anchor: bool,
}

pub trait Discovery {
    fn start(&mut self) -> Result<(), DiscoveryError>;
    fn get_neighbors(&self) -> Vec<NodeInfo>;
    fn send_heartbeat(&self, target: &NodeId) -> Result<(), DiscoveryError>;
}

#[derive(Error, Debug)]
pub enum DiscoveryError {
    #[error("محول الشبكة غير موجود أو معطل")]
    AdapterNotFound,
    #[error("تم رفض الإذن للوصول إلى الشبكة")]
    PermissionDenied,
    #[error("خطأ في الشبكة: {0}")]
    NetworkError(String),
}

pub mod mdns;
