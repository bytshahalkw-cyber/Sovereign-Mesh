use std::{env, fs, io::{self, BufRead, BufReader, Write}, net::UdpSocket, os::unix::{fs::PermissionsExt, net::UnixListener}, thread, time::Duration};
use ring::signature::{self, KeyPair};
use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, CHACHA20_POLY1305};

const AUTH_TOKEN: &str = "aether-guard-secret-token-2026";

fn decrypt_onion_layer(encrypted_data: &[u8], unbound_key: UnboundKey) -> Option<(String, Vec<u8>)> {
    if encrypted_data.len() < 12 {
        return None;
    }
    let key = LessSafeKey::new(unbound_key);
    let (nonce_bytes, ciphertext_with_tag) = encrypted_data.split_at(12);
    let nonce = Nonce::try_assume_unique_for_key(nonce_bytes).ok()?;

    let mut in_out = ciphertext_with_tag.to_vec();
    match key.open_in_place(nonce, Aad::empty(), &mut in_out) {
        Ok(decrypted) => {
            if decrypted.len() > 16 {
                let next_hop = String::from_utf8_lossy(&decrypted[..16]).trim_matches('\0').to_string();
                let inner_payload = decrypted[16..].to_vec();
                Some((next_hop, inner_payload))
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

fn main() {
    let port: u16 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(8080);
    let socket = UdpSocket::bind(format!("127.0.0.1:{}", port)).expect("Bind failed");
    socket.set_nonblocking(true).expect("Failed to set nonblocking");

    let rng = ring::rand::SystemRandom::new();
    let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).expect("Key gen failed");
    let _key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).expect("Parse failed");

    println!("[Node Port {}] ", port);("🧅 Sidra Ring-Crypto Onion Node active on port: {} [Secure Native Crypto]", port);
    io::stdout().flush().unwrap();

    let socket_clone = socket.try_clone().expect("Clone failed");
    let current_port = port.clone();

    thread::spawn(move || {
        let mut buf = [0u8; 2048];
        let dummy_key_bytes = [0u8; 32];
        
        loop {
            let unbound_key = UnboundKey::new(&CHACHA20_POLY1305, &dummy_key_bytes).unwrap();
            match socket_clone.recv_from(&mut buf) {
                Ok((amt, src)) => {
                    let packet = &buf[..amt];
                    if let Some((next_hop, inner_payload)) = decrypt_onion_layer(packet, unbound_key) {
                        let msg_str = String::from_utf8_lossy(&inner_payload).trim().to_string();
                        println!("[Node Port {}] ", port);("📥 [Port {}] Decrypted Ring-Layer from {}. Next Hop: {} | Payload: {}", current_port, src, next_hop, msg_str);
                        io::stdout().flush().unwrap();
                    }
                }
                Err(_) => {
                    thread::sleep(Duration::from_millis(50));
                }
            }
        }
    });

    let socket_path = format!("mesh_{}.sock", port);
    let _ = fs::remove_file(&socket_path);
    let unix_listener = UnixListener::bind(&socket_path).expect("Failed to bind Unix socket");

    let metadata = fs::metadata(&socket_path).expect("Failed to get metadata");
    let mut perms = metadata.permissions();
    perms.set_mode(0o600);
    fs::set_permissions(&socket_path, perms).expect("Failed to set permissions");

    let control_port = port.clone();
    thread::spawn(move || {
        for stream in unix_listener.incoming() {
            match stream {
                Ok(mut sock) => {
                    let mut reader = BufReader::new(&sock);
                    let mut cmd = String::new();
                    if reader.read_line(&mut cmd).is_ok() {
                        let trimmed = cmd.trim();
                        let expected_prefix = format!("AUTH:{} ", AUTH_TOKEN);
                        
                        if trimmed.starts_with(&expected_prefix) {
                            let actual_cmd = &trimmed[expected_prefix.len()..];
                            if actual_cmd == "STATUS" {
                                let response = format!("Node {} Status: ACTIVE [Ring Crypto Onion OK]\n", control_port);
                                let _ = sock.write_all(response.as_bytes());
                            } else {
                                let _ = sock.write_all(b"ERROR: Unknown command\n");
                            }
                        } else {
                            let _ = sock.write_all(b"ERROR: Unauthorized\n");
                        }
                    }
                }
                Err(_) => break,
            }
        }
    });

    loop {
        thread::sleep(Duration::from_secs(5));
        let target = "127.0.0.1:8081";
        let raw_msg = format!("Ring-Encrypted-Payload from {}", port);
        
        let dummy_key_bytes = [0u8; 32];
        let unbound_key = UnboundKey::new(&CHACHA20_POLY1305, &dummy_key_bytes).unwrap();
        let cipher = LessSafeKey::new(unbound_key);
        let nonce_bytes = [1u8; 12];
        let nonce = Nonce::try_assume_unique_for_key(&nonce_bytes).unwrap();
        
        let mut in_out = vec![0u8; 16];
        let hop_bytes = b"127.0.0.1:8081  ";
        in_out[..16].copy_from_slice(&hop_bytes[..16]);
        in_out.extend_from_slice(raw_msg.as_bytes());

        if cipher.seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out).is_ok() {
            let mut packet = nonce_bytes.to_vec();
            packet.extend_from_slice(&in_out);
            let _ = socket.send_to(&packet, target);
        }
    }
}
