mod arg;
mod constant;
mod encrypt;
mod entry;
mod state;
mod terminal;

use std::io::stdout;

use crossterm::*;

use chrono::{DateTime, Utc};
use encrypt::{decrypt_entry, encrypt_entry, EncryptionError};
use serde::{Deserialize, Serialize};
use state::State;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: implement argument parsing [arg.rs]

    let mut state: State = State::new().unwrap();

    state.change_status_bar(String::from("this here is a status bar."))?;
    state.event_listener()?;

    /*
    state.canvas.recalculate_dimensions().unwrap();

    state.canvas.test_square();
    */

    /*
    loop {
        if event::poll(std::time::Duration::from_millis(66))? {
            if let crossterm::event::Event::Key(key_event) = event::read()? {
                if key_event.code == crossterm::event::KeyCode::Enter {
                    break;
                }
            }
        }
        state.canvas.clear();
        state.canvas.recalculate_dimensions().unwrap();
        state.canvas.screen_square()?;
        state.canvas.draw_main_menu()?;
    }

    */

    kill();
    Ok(())
}

fn kill() {
    // let mut stdout = stdout();
    // crossterm::terminal::LeaveAlternateScreen;
    crossterm::terminal::disable_raw_mode().unwrap();
}
