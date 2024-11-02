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
    event::{self, KeyEvent, MouseEvent},
    execute,
    terminal::{self, enable_raw_mode, Clear, ClearType, DisableLineWrap, EnterAlternateScreen},
    ExecutableCommand,
};

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
            crossterm::event::KeyCode::Enter => {
                crossterm::terminal::disable_raw_mode()?;
                std::process::exit(0);
            }
            _ => Ok(()),
        }
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

    pub fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.canvas.screen_square()?;
        self.canvas.draw_main_menu()?;
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
