use sovereign_mesh_core::crypto::NodeIdentity;
use sovereign_mesh_core::routing::MeshPacket;
use sovereign_mesh_core::transport::TransportLayer;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    println!("🛡️ بدء تشغيل الاختبار الشامل لشبكة Sovereign Mesh");

    // 1. توليد هويات العقد
    let sender = NodeIdentity::generate().expect("فشل توليد هوية المرسل");
    let receiver = NodeIdentity::generate().expect("فشل توليد هوية المستقبل");

    println!("🔑 تم إنشاء هوية المرسل والمستقبل بنجاح.");

    // قناة مزامنة لاختبار الاستقبال في الخيط الرئيسي
    let (tx, rx) = mpsc::channel();

    // 2. تشغيل خادم الاستقبال على المنفذ 8080
    TransportLayer::listen("127.0.0.1:8080", move |packet| {
        println!("📥 تم استلام حزمة بيانات عبر الشبكة!");
        let _ = tx.send(packet);
    }).expect("فشل في تشغيل خادم الاستقبال");

    // إعطاء فرصة لخادم الاستقبال ليبدأ بسلاسة
    std::thread::sleep(Duration::from_millis(500));

    // 3. إنشاء رسالة وحزمة موجهة وآمنة
    let payload = "رسالة سرية معبرة تمر عبر طبقة الاتصال السيادي".as_bytes().to_vec();
    let packet = MeshPacket::create(
        &sender,
        receiver.public_key(),
        payload,
    ).expect("فشل إنشاء الحزمة");

    println!("📦 جاري إرسال الحزمة عبر بروتوكول TCP...");
    TransportLayer::send_packet("127.0.0.1:8080", &packet)
        .expect("فشل إرسال الحزمة");

    // 4. انتظار واستقبال الحزمة والتحقق من صحتها
    match rx.recv_timeout(Duration::from_secs(3)) {
        Ok(received_packet) => {
            match received_packet.verify_packet() {
                Ok(_) => {
                    println!("✅ نجاح تام! تم استقبال الحزمة، التحقق من توقيعها Ed25519، وتأكيد سلامتها بنجاح.");
                    let message = String::from_utf8_lossy(&received_packet.payload);
                    println!("📝 محتوى الرسالة المستلمة: \"{}\"", message);
                }
                Err(e) => println!("❌ فشل التحقق من توقيع الحزمة المستلمة: {:?}", e),
            }
        }
        Err(_) => println!("❌ انتهى الوقت ولم يتم استلام الحزمة."),
    }
}
