use std::io::{self, BufRead, Write};
use std::sync::mpsc;
use std::thread;

/// Start console I/O threads and return the stdin receiver and a response sender.
pub fn start_console() -> (mpsc::Receiver<String>, crossbeam::channel::Sender<String>) {
    let (tx, rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
    let reader_tx = tx.clone();

    let (response_tx, response_rx) = crossbeam::channel::unbounded::<String>();
    let response_printer = response_tx.clone();
    thread::spawn(move || {
        while let Ok(msg) = response_rx.recv() {
            if msg.trim().is_empty() {
                continue;
            }
            println!("{}", msg);
        }
    });

    thread::spawn(move || {
        let stdin = io::stdin();
        let mut reader = io::BufReader::new(stdin);

        loop {
            print!("server> ");
            let _ = io::stdout().flush();

            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => {
                    let _ = reader_tx.send("exit".to_string());
                    break;
                }
                Ok(_) => {
                    let cmd = line.trim();
                    if !cmd.is_empty() {
                        let _ = reader_tx.send(cmd.to_string());
                    }
                }
                Err(err) => {
                    eprintln!("Input error: {}", err);
                    let _ = reader_tx.send("exit".to_string());
                    break;
                }
            }
        }
    });

    let tx_ctrl = tx.clone();
    let _ = ctrlc::set_handler(move || {
        let _ = tx_ctrl.send("exit".to_string());
    });

    (rx, response_printer)
}
