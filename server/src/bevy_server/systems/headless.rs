use bevy::prelude::{Res, Time};

pub fn log_headless_bevy_status(time: Res<Time>) {
    let secs = time.elapsed_secs_f64();
    if secs.fract() < 0.0001 {
        tracing::info!("bevy heartbeat: {:.1}s elapsed | server", secs);
    }
}
