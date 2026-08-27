use crate::routing::MeshPacket;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use thiserror::Error;
use std::sync::Arc;

#[derive(Error, Debug)]
pub enum TransportError {
    #[error("فشل في تشغيل خادم الاستماع: {0}")]
    IoError(#[from] std::io::Error),
    #[error("فشل في تسلسل أو إلغاء تسلسل الحزمة")]
    SerializationError,
}

pub struct TransportLayer;

impl TransportLayer {
    pub fn listen<F>(address: &str, on_packet: F) -> Result<(), TransportError>
    where
        F: Fn(MeshPacket) + Send + Sync + 'static,
    {
        let listener = TcpListener::bind(address)?;
        println!("🌐 خادم الاتصال يستمع على العنوان: {}", address);
        let callback = Arc::new(on_packet);

        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        let cb = Arc::clone(&callback);
                        thread::spawn(move || {
                            let mut buffer = Vec::new();
                            if stream.read_to_end(&mut buffer).is_ok() {
                                if let Ok(packet) = bincode::deserialize::<MeshPacket>(&buffer) {
                                    cb(packet);
                                }
                            }
                        });
                    }
                    Err(e) => println!("❌ خطأ في اتصال العقدة: {}", e),
                }
            }
        });

        Ok(())
    }

    pub fn send_packet(target_address: &str, packet: &MeshPacket) -> Result<(), TransportError> {
        let mut stream = TcpStream::connect(target_address)?;
        let encoded = bincode::serialize(packet).map_err(|_| TransportError::SerializationError)?;
        stream.write_all(&encoded)?;
        stream.flush()?;
        Ok(())
    }
}
