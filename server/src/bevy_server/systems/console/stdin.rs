use bevy::log::info;
use bevy::prelude::Res;

use crate::bevy_server::resources::{CommandResponseSender, StdinReceiver};

pub fn process_stdin_commands(
    receiver: Res<StdinReceiver>,
    responder: Option<Res<CommandResponseSender>>,
) {
    // Drain all available lines without holding the lock while processing
    let mut lines = Vec::new();
    {
        let guard = match receiver.0.lock() {
            Ok(guard) => guard,
            Err(_) => return, // If we can't lock, just skip processing this frame
        };
        loop {
            match guard.try_recv() {
                Ok(line) => lines.push(line),
                Err(std::sync::mpsc::TryRecvError::Empty) => break,
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    info!("Stdin channel disconnected");
                    std::process::exit(0);
                }
            }
        }
    }

    for raw in lines {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        info!("stdin command: {}", line);
        let send_resp = |s: String| {
            if let Some(r) = &responder {
                let _ = r.0.send(s);
            } else {
                info!("{}", s);
            }
        };

        match line {
            "quit" | "exit" => {
                send_resp("Exiting dedicated server window...".to_string());
                std::process::exit(0);
            }
            "status" => {
                send_resp("Server status: running".to_string());
            }
            "help" => {
                send_resp("Commands: status, help, quit".to_string());
            }
            _ => {
                send_resp(format!("Unhandled command: {}", line));
            }
        }
    }
}
