use std::time::Duration;

use bevy_renet::renet::{ChannelConfig, SendType};

pub enum GameSyncChannels {
    Messages,
    SpaceTiles,
    Starships,
    Planets,
    SpaceFacilities,
    ServerObjects,
}

impl From<GameSyncChannels> for u8 {
    fn from(channel: GameSyncChannels) -> Self {
        match channel {
            GameSyncChannels::Messages => 0,
            GameSyncChannels::SpaceTiles => 1,
            GameSyncChannels::Starships => 2,
            GameSyncChannels::SpaceFacilities => 3,
            GameSyncChannels::Planets => 4,
            GameSyncChannels::ServerObjects => 5,
        }
    }
}

impl GameSyncChannels {
    pub fn config() -> Vec<ChannelConfig> {
        vec![
            ChannelConfig {
                channel_id: 0,
                max_memory_usage_bytes: 5 * 1024 * 1024,
                send_type: SendType::ReliableUnordered {
                    resend_time: Duration::from_millis(300),
                },
            },
            ChannelConfig {
                channel_id: 1,
                max_memory_usage_bytes: 5 * 1024 * 1024,
                send_type: SendType::ReliableUnordered {
                    resend_time: Duration::from_millis(300),
                },
            },
            ChannelConfig {
                channel_id: 2,
                max_memory_usage_bytes: 5 * 1024 * 1024,
                send_type: SendType::ReliableUnordered {
                    resend_time: Duration::from_millis(300),
                },
            },
            ChannelConfig {
                channel_id: 3,
                max_memory_usage_bytes: 5 * 1024 * 1024,
                send_type: SendType::ReliableUnordered {
                    resend_time: Duration::from_millis(300),
                },
            },
            ChannelConfig {
                channel_id: 4,
                max_memory_usage_bytes: 5 * 1024 * 1024,
                send_type: SendType::ReliableUnordered {
                    resend_time: Duration::from_millis(300),
                },
            },
            ChannelConfig {
                channel_id: 5,
                max_memory_usage_bytes: 5 * 1024 * 1024,
                send_type: SendType::ReliableOrdered {
                    resend_time: Duration::from_millis(300),
                },
            },
        ]
    }
}
