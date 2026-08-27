use sovereign_mesh_core::discovery::{mdns::MdnsDiscovery, Discovery};
use std::thread;
use std::time::Duration;

fn main() {
    println!("🛡️ بدء تشغيل الدرع السيادي (Sovereign Mesh) - نموذج الاكتشاف الأولي");
    
    let mut node = MdnsDiscovery::new(false).expect("فشل في تهيئة الاكتشاف");
    
    println!("📡 جاري البحث عن العقد المجاورة عبر mDNS...");
    node.start().expect("فشل في بدء خدمة الاكتشاف");

    loop {
        thread::sleep(Duration::from_secs(5));
        let neighbors = node.get_neighbors();
        println!("📊 عدد العقد المكتشفة حالياً: {}", neighbors.len());
        for n in neighbors {
            println!("   - عقدة (مرساة: {}): {:?}", n.is_anchor, n.address);
        }
    }
}
