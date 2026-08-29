use std::collections::HashSet;
use std::sync::Mutex;

pub struct SeenCache {
    seen_ids: Mutex<HashSet<[u8; 16]>>,
    max_size: usize,
}

impl SeenCache {
    pub fn new(max_size: usize) -> Self {
        Self {
            seen_ids: Mutex::new(HashSet::new()),
            max_size,
        }
    }

    pub fn check_and_insert(&self, message_id: [u8; 16]) -> bool {
        let mut cache = self.seen_ids.lock().unwrap();
        
        if cache.contains(&message_id) {
            return false;
        }
        
        if cache.len() >= self.max_size {
            let to_remove: Vec<_> = cache.iter().take(self.max_size / 2).cloned().collect();
            for id in to_remove {
                cache.remove(&id);
            }
        }
        
        cache.insert(message_id);
        true
    }

    pub fn len(&self) -> usize {
        self.seen_ids.lock().unwrap().len()
    }
}
