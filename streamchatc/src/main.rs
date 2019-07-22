mod layout;
use layout::Fringe;

mod window;
use window::Window;

mod config;
use config::Config;

mod args;
use args::Args;

use yansi::{Color, Paint};

fn main() {
    let mut config = Args::load_or_config();
    let color = std::env::var("NO_COLOR").is_err();

    if cfg!(windows) && !Paint::enable_windows_ascii() || !color {
        Paint::disable();
    }

    config.address = "localhost:51002".into();

    Window::run(config);
}
