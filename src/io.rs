use actix_web::web::Bytes;
use rand::{Rng, distr::Alphanumeric, rng};
use std::cell::RefCell;

pub struct PasteStore {
    db: sqlite::ConnectionThreadSafe,
}

impl PasteStore {
    pub fn new(db_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let db = sqlite::Connection::open_thread_safe(db_path)?;
        db.execute(r"CREATE TABLE IF NOT EXISTS pastes (id TEXT, data BLOB);")?;
        _ = db.execute(r"ALTER TABLE pastes ADD COLUMN encrypted BOOLEAN DEFAULT FALSE;");
        Ok(Self { db })
    }

    pub fn store_paste(&self, id: &str, content: &Bytes, encrypted: bool) {
        self.db
            .prepare("INSERT INTO pastes (id, data, encrypted) VALUES (?, ?, ?);")
            .and_then(|mut stmt| {
                stmt.bind((1, id))?;
                stmt.bind((2, content.iter().as_slice()))?;
                stmt.bind((3, i64::from(encrypted)))?;
                stmt.next()
            })
            .ok();
    }

    pub fn get_paste(&self, id: &str) -> Option<(Bytes, bool)> {
        self.db
            .prepare("SELECT data, encrypted FROM pastes WHERE id = ?;")
            .and_then(|mut stmt| {
                stmt.bind((1, id))?;
                if let sqlite::State::Row = stmt.next()? {
                    let data: Vec<u8> = stmt.read(0)?;
                    let encrypted: i64 = stmt.read(1)?;
                    let encrypted: bool = encrypted != 0;
                    Ok(Some((Bytes::from(data), encrypted)))
                } else {
                    Ok(None)
                }
            })
            .ok()
            .flatten()
    }
}

/// Generates a 'pronounceable' random ID using gpw
pub fn generate_id() -> String {
    thread_local!(static KEYGEN: RefCell<gpw::PasswordGenerator> = RefCell::new(gpw::PasswordGenerator::default()));

    KEYGEN.with(|k| k.borrow_mut().next()).unwrap_or_else(|| {
        rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_paste() {
        let store = PasteStore::new(":memory:").unwrap();
        store.store_paste("hello", &Bytes::from("world"), false);
        let (hello, encrypted) = store.get_paste("hello").unwrap();
        assert_eq!("world".as_bytes(), hello.as_ref());
        assert_eq!(false, encrypted);
    }

    #[test]
    fn test_binary_paste() {
        let store = PasteStore::new(":memory:").unwrap();
        store.store_paste("hello", &Bytes::from(vec![0x0, 0x1, 0x2, 0x3, 0xff]), false);
        let (hello, encrypted) = store.get_paste("hello").unwrap();
        assert_eq!(&[0x0, 0x1, 0x2, 0x3, 0xff], hello.as_ref());
        assert_eq!(false, encrypted);
    }

    #[test]
    fn test_empty() {
        let store = PasteStore::new(":memory:").unwrap();
        store.store_paste("hello", &Bytes::new(), false);
        let (hello, encrypted) = store.get_paste("hello").unwrap();
        assert!(hello.is_empty());
        assert_eq!(false, encrypted);
    }
}
