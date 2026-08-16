use chrono::Local as ChronoLocal;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

pub static LOG_BUFFER: Lazy<Arc<Mutex<Vec<String>>>> =
    Lazy::new(|| Arc::new(Mutex::new(Vec::new())));
pub static PROMPT_ACTIVE: AtomicBool = AtomicBool::new(false);

pub struct BufferWriter {
    pub buf: Arc<Mutex<Vec<String>>>,
}

impl std::io::Write for BufferWriter {
    fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
        if let Ok(s) = std::str::from_utf8(data) {
            let ts = ChronoLocal::now()
                .format("%Y-%m-%d %H:%M:%S%.3f")
                .to_string();
            let formatted = format!("{} {}\n", ts, s.trim_end());

            if PROMPT_ACTIVE.load(Ordering::SeqCst) {
                let mut err = std::io::stderr().lock();
                let _ = err.write_all(b"\n");
                let _ = err.flush();
            }

            let mut err = std::io::stderr().lock();
            let _ = err.write_all(formatted.as_bytes());
            let _ = err.flush();

            let mut lock = self.buf.lock().unwrap();
            lock.push(formatted.trim_end().to_string());
        }
        Ok(data.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
