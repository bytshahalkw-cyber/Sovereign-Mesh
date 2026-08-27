use super::{Discovery, DiscoveryError, NodeAddress, NodeId, NodeInfo};
use crate::crypto::NodeIdentity;
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

const SERVICE_TYPE: &str = "_sovereign-mesh._tcp.local.";

pub struct MdnsDiscovery {
    daemon: ServiceDaemon,
    neighbors: Arc<Mutex<HashMap<NodeId, NodeInfo>>>,
    identity: NodeIdentity,
    is_anchor: bool,
}

impl MdnsDiscovery {
    pub fn new(is_anchor: bool) -> Result<Self, DiscoveryError> {
        let daemon = ServiceDaemon::new().map_err(|e| DiscoveryError::NetworkError(e.to_string()))?;
        let identity = NodeIdentity::generate().map_err(|_| DiscoveryError::NetworkError("فشل توليد الهوية التشفيرية".into()))?;

        Ok(Self {
            daemon,
            neighbors: Arc::new(Mutex::new(HashMap::new())),
            identity,
            is_anchor,
        })
    }

    fn instance_name(&self) -> String {
        let pub_key = self.identity.public_key();
        let id_str: String = pub_key.iter().take(8).map(|b| format!("{:02x}", b)).collect();
        format!("node-{}", id_str)
    }
}

impl Discovery for MdnsDiscovery {
    fn start(&mut self) -> Result<(), DiscoveryError> {
        let instance_name = self.instance_name();
        let anchor_val = if self.is_anchor { "true" } else { "false" };
        
        let service_info = ServiceInfo::new(
            SERVICE_TYPE,
            &instance_name,
            "local.",
            "",
            8080,
            &[("is_anchor", anchor_val)][..],
        ).map_err(|e| DiscoveryError::NetworkError(e.to_string()))?;

        self.daemon.register(service_info).map_err(|e| DiscoveryError::NetworkError(e.to_string()))?;

        let receiver = self.daemon.browse(SERVICE_TYPE).map_err(|e| DiscoveryError::NetworkError(e.to_string()))?;
        let neighbors_clone = Arc::clone(&self.neighbors);

        std::thread::spawn(move || {
            while let Ok(event) = receiver.recv() {
                match event {
                    ServiceEvent::ServiceResolved(info) => {
                        let mut neighbors = neighbors_clone.lock().unwrap();
                        let id = [0u8; 32];
                        
                        let address = if let Some(ip) = info.get_addresses().iter().next() {
                            NodeAddress::IpV4(*ip)
                        } else {
                            continue;
                        };

                        let is_anchor = info.get_property("is_anchor")
                            .and_then(|p| p.val_str().parse::<bool>().ok())
                            .unwrap_or(false);

                        let node_info = NodeInfo {
                            id,
                            address,
                            signal_strength: 0,
                            last_seen: SystemTime::now(),
                            is_anchor,
                        };

                        neighbors.insert(id, node_info);
                        println!("✅ تم اكتشاف عقدة موثقة بهوية تشفيرية: {:?}", info.get_fullname());
                    }
                    ServiceEvent::ServiceRemoved(_, fullname) => {
                        println!("❌ غادرت العقدة الشبكة: {}", fullname);
                    }
                    _ => {}
                }
            }
        });

        Ok(())
    }

    fn get_neighbors(&self) -> Vec<NodeInfo> {
        let neighbors = self.neighbors.lock().unwrap();
        neighbors.values().cloned().collect()
    }

    fn send_heartbeat(&self, _target: &NodeId) -> Result<(), DiscoveryError> {
        Ok(())
    }
}
