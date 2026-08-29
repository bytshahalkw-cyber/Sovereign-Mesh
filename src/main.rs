use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let port = if args.len() > 1 {
        args[1].clone()
    } else {
        env::var("PORT").unwrap_or_else(|_| "8080".to_string())
    };

    println!("🚀 Sovereign Mesh Node starting on UDP Port: {}", port);
    
    // ضبط متغيرات البيئة لضمان التقاطها بواسطة مكتبة الكور
    std::env::set_var("PORT", &port);
    std::env::set_var("BIND_PORT", &port);
}
