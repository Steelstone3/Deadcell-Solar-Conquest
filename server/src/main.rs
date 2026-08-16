mod bevy_server;
mod console;

fn main() -> std::io::Result<()> {
    eprintln!("Deadcell Solar Conquest dedicated server");
    eprintln!("Press Ctrl+C to stop the dedicated server.");
    bevy_server::run_headless_bevy_server();
    Ok(())
}
