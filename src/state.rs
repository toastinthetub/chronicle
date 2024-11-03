// TODO: implement state lols!
// TODO: implement UI mainloop
//      - should exist in state.rs or in terminal.rs? i am thinking
//        state.rs? and terminal.rs gets to hold all of our little
//        functions for manipulating shit! but must be careful not to
//        abstract too close to the sun...perhaps write a small TUI
//        library that sits on top of Crossterm for UI. Yes. I like.

use std::io::Write;

use crate::{
    constant::{HORIZONTAL_LINE_LOW, LEFT_UPPER_SHOULDER},
    entry::{DiaryEntries, Entry, SerializableEntry},
    terminal::{CanvasState, EntryBuffer},
};

use crossterm::{
    cursor::MoveTo,
    event::{self, KeyEvent, MouseEvent},
    execute,
    style::Stylize,
    terminal::{self, enable_raw_mode, Clear, ClearType, DisableLineWrap, EnterAlternateScreen},
    ExecutableCommand,
};

// TODO: Fix status bar, implement entrybuffer editing, more robust keyhandling

pub struct State {
    pub canvas: CanvasState,
    pub status: String,
}

impl State {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let canvas: CanvasState = match CanvasState::new_from_environment() {
            Ok(canvas) => canvas,
            Err(/*_*/ _e) => {
                panic!("compiler won't let me throw an error. my hands are in the air, i just do not care.")
                // return Err(Box::new(e));
            }
        };
        let status: String = String::new();
        Ok(Self { canvas, status })
    }

    pub fn event_listener(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            if event::poll(std::time::Duration::from_millis(33))? {
                match event::read()? {
                    event::Event::Key(key_event) => {
                        self.handle_key_event(key_event)?;
                    }
                    event::Event::Mouse(mouse_event) => {
                        self.handle_mouse_event(mouse_event)?;
                    }
                    event::Event::Resize(nw, nh) => {
                        self.handle_resize_event(nw, nh)?;
                    }
                    event::Event::Paste(data) => {
                        self.handle_paste_event(data)?;
                    }
                    _ => {
                        panic!("unhandled event.");
                        break;
                    }
                }
            }
            self.canvas.clear();
            self.render()?;
        }
        Ok(())
    }
    pub fn handle_key_event(
        &mut self,
        event: crossterm::event::KeyEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match event.code {
            crossterm::event::KeyCode::Esc => {
                crossterm::terminal::disable_raw_mode()?;
                std::process::exit(0);
            }
            crossterm::event::KeyCode::Enter => {
                match self.canvas.mode {
                    crate::terminal::Mode::MainMenu => {
                        match self.canvas.idx_buf {
                            0 => {
                                // new entry, must enter entry_editor()
                                self.change_mode(crate::terminal::Mode::EditEntryNormalMode)?;
                            }
                            1 => {
                                self.change_mode(crate::terminal::Mode::SelectExistingEntry)?;
                            }
                            2 => {
                                // TODO
                            }
                            _ => {
                                panic!("this was not supposed to happen.");
                            }
                        }
                    }
                    crate::terminal::Mode::EditEntryNormalMode => {
                        self.change_status_bar(String::from("you pressed enter!"))?;
                    }
                    crate::terminal::Mode::EditEntryCommandMode => {
                        // TODO: SubmitCommand
                        self.change_status_bar(String::from("you submitted a command!"))?;
                        //self.clear_status_bar()?;
                    }
                    _ => {
                        crossterm::terminal::disable_raw_mode()?;
                        std::process::exit(0);
                    }
                }
            }

            crossterm::event::KeyCode::Char(c) => {
                // handle those characters, bitch
                self.handle_char(c)?;
            }

            crossterm::event::KeyCode::Up => {
                match self.canvas.mode {
                    // determine canvas mode and do different shit
                    crate::terminal::Mode::MainMenu => {
                        // while in terminal mode, do this shit
                        if self.canvas.idx_buf <= 0 {
                            // do nothing
                            return Ok(());
                        } else {
                            self.canvas.idx_buf -= 1;
                        }
                    }

                    crate::terminal::Mode::EditEntryNormalMode => {
                        // do nothing
                    }

                    crate::terminal::Mode::EditEntryInsertMode => {
                        // do nothing
                    }

                    crate::terminal::Mode::EditEntryCommandMode => {
                        // do nothing
                    }

                    crate::terminal::Mode::SelectExistingEntry => {
                        // do nothing
                    }

                    _ => { // any other mode, do any of this shit
                    }
                }
            }

            // keycode::down. there has got to be a better way to do this
            crossterm::event::KeyCode::Down => {
                match self.canvas.mode {
                    crate::terminal::Mode::MainMenu => {
                        if self.canvas.idx_buf >= 2 {
                            return Ok(());
                        } else {
                            self.canvas.idx_buf += 1;
                        }
                    }
                    _ => {
                        // all other states undefined
                        // TODO
                    }
                }
            }
            crossterm::event::KeyCode::Left => {
                //
            }
            crossterm::event::KeyCode::Right => {
                //
            }
            _ => {}
        }

        Ok(())
    }
    pub fn handle_mouse_event(
        &mut self,
        event: crossterm::event::MouseEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        eprintln!("no mouse event handler");
        Ok(())
    }
    pub fn handle_resize_event(
        &mut self,
        nw: u16,
        nh: u16,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.canvas.recalculate_dimensions()?;
        Ok(())
    }
    pub fn handle_paste_event(&mut self, data: String) -> Result<(), Box<dyn std::error::Error>> {
        todo!();
        Ok(())
    }

    pub fn handle_char(&mut self, c: char) -> Result<(), Box<dyn std::error::Error>> {
        // this is it sluts, we're handling characters

        match c {
            ':' => {
                if self.canvas.mode == crate::terminal::Mode::EditEntryNormalMode {
                    self.canvas.last_mode = self.canvas.mode.clone();
                    self.change_mode(crate::terminal::Mode::EditEntryCommandMode)?;
                }
            }

            _ => match self.canvas.mode {
                crate::terminal::Mode::EditEntryInsertMode => {
                    self.canvas.entry_buffer.text_buffer.push(c);
                }
                crate::terminal::Mode::EditEntryCommandMode => {
                    self.status.push(c);
                }
                _ => {}
            },
        }

        Ok(())
    }

    pub fn change_mode(
        &mut self,
        mode: crate::terminal::Mode,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.canvas.mode = mode;
        self.change_status_bar(self.status.clone())?;

        Ok(())
    }

    pub fn change_status_bar(
        &mut self,
        mut new_status: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        /*        if self.status == new_status {
                    return Ok(());
                }
        */

        // if we already have a mode, remove it
        let first_five = new_status[..5].to_string();
        if first_five.contains(" - ") {
            new_status = new_status[5..].to_string()
        }

        // TODO: FIX STATUS

        self.status.clear();

        let mut str = format!("{} - {}", self.canvas.mode, new_status);

        while str.len() < self.canvas.size_x as usize - 2 {
            str.push(crate::constant::WHITESPACE);
        }

        self.status = str.bold().black().on_white().to_string();
        Ok(())
    }

    pub fn clear_status_bar(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.change_status_bar(String::from(""))?;
        Ok(())
    }

    pub fn draw_status_bar(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        crossterm::execute!(
            self.canvas.stdout,
            MoveTo(self.canvas.zero_x + 1, self.canvas.zero_y + 1)
        )?;
        self.canvas.stdout.write_all(self.status.as_bytes())?;

        Ok(())
    }

    pub fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.canvas.screen_square()?;
        self.draw_status_bar()?;

        // decide which buffer to draw
        match self.canvas.mode {
            crate::terminal::Mode::MainMenu => {
                self.canvas.draw_main_menu()?;
            }

            crate::terminal::Mode::EditEntryNormalMode => {
                self.canvas.draw_entry_buffer()?;
            }

            crate::terminal::Mode::EditEntryInsertMode => {
                self.canvas.draw_entry_buffer()?;
            }

            crate::terminal::Mode::EditEntryCommandMode => {
                self.canvas.draw_entry_buffer()?;
                // do something
            }

            _ => {}
        }

        // simple render test
        /*
                self.canvas
                    .screen_buffer
                    .write_char(0, 1, LEFT_UPPER_SHOULDER);
                for i in self.canvas.zero_x + 1..=self.canvas.size_x - 1 {
                    self.canvas
                        .screen_buffer
                        .write_char(i as usize, 1, HORIZONTAL_LINE_LOW);
                }
        */
        Ok(())
    }
}
