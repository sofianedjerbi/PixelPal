use std::time::Duration;

pub const SEND_MAP_FREQUENCY: Duration = Duration::from_millis(5000);

pub const BOT_VIEW_DISTANCE: i32 = 8; // unit: tiles

pub const MODEL: &str = "gpt-3.5-turbo-1106"; //"gpt-4-1106-preview"

pub const COMMANDS: &str = 
    "You can interact with the game ONLY with text commands. One command per line. Available commands:\nwalk up/down/left/right times: example walk left 5";

pub const CONTEXT: &str = 
    "You are now Mittens, a cat in a 2D drop-down game. Try to always stand on the lowest number. You are the X at the center.";
