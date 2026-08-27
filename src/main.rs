use sovereign_mesh_core::crypto::NodeIdentity;
use sovereign_mesh_core::routing::MeshPacket;

fn main() {
    println!("🛡️ بدء تشغيل اختبار طبقة التوجيه والتشفير - Sovereign Mesh");

    let sender = NodeIdentity::generate().expect("فشل توليد هوية المرسل");
    let receiver = NodeIdentity::generate().expect("فشل توليد هوية المستقبل");

    println!("🔑 تم إنشاء مفتاح المرسل بنجاح (الحجم: {} بايت)", sender.public_key().len());
    println!("🔑 تم إنشاء مفتاح المستقبل بنجاح (الحجم: {} بايت)", receiver.public_key().len());

    let payload = b"رسالة سرية عبر شبكة الدرع السيادي المعزولة".to_vec();
    
    let packet = MeshPacket::create(
        &sender,
        receiver.public_key(),
        payload,
    ).expect("فشل في إنشاء الحزمة الموجهة");

    println!("📦 تم إنشاء الحزمة بنجاح وتوقيعها رقمياً.");

    match packet.verify_packet() {
        Ok(_) => println!("✅ تم التحقق من صحة الحزمة وتوقيعها بنجاح تام! الاتصال آمن وموثوق."),
        Err(e) => println!("❌ فشل التحقق من الحزمة: {:?}", e),
    }
}
