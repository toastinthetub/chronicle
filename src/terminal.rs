// TODO: write a ui for this lol
//          - changed my mind this is gonna be a library!:wqfuck

// double nevermind i aint got time to write a library

use crate::{
    constant::{
        CHRONICLE_RESOURCE_PATH, HORIZONTAL_LINE_HIGH, HORIZONTAL_LINE_LOW, LEFT_LOWER_SHOULDER,
        LEFT_UPPER_SHOULDER, MENU_OPTION_BROWSE_ENTRIES, MENU_OPTION_NEW_ENTRY, MENU_OPTION_QUIT,
        RIGHT_LOWER_SHOULDER, RIGHT_UPPER_SHOULDER, VERTICAL_LINE, WHITESPACE,
    },
    entry::{DiaryEntries, Entry},
};

use aes_gcm::aes::cipher::typenum::assert_type;
use crossterm::cursor::{
    DisableBlinking, Hide, MoveLeft, MoveToColumn, MoveToNextLine, SetCursorStyle,
};
use crossterm::style::Stylize;
use crossterm::terminal::{disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::QueueableCommand;
use crossterm::{
    cursor::{MoveDown, MoveTo, RestorePosition, SavePosition},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};

use std::{
    error::Error,
    fmt,
    io::{Lines, Read, Stdout, Write},
};

#[derive(PartialEq, Debug, Clone)]
pub enum Mode {
    MainMenu,
    SelectExistingEntry,

    EditEntryNormalMode,
    EditEntryInsertMode,
    EditEntryCommandMode,

    QuitAll,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Mode::MainMenu => write!(f, "MM"),
            Mode::SelectExistingEntry => write!(f, "SE"),
            Mode::EditEntryNormalMode => write!(f, "EN"),
            Mode::EditEntryInsertMode => write!(f, "EI"),
            Mode::EditEntryCommandMode => write!(f, "EC"),
            Mode::QuitAll => write!(f, "QA"),
        }
    }
}

pub struct CanvasState {
    pub stdout: Stdout,

    pub zero_x: u16,
    pub zero_y: u16,

    pub size_x: u16,
    pub size_y: u16,

    pub mode: Mode,
    pub last_mode: Mode, // for returning to original state
    pub idx_buf: i32,

    pub byte_buffer: [u8; 4],
    pub asset_buffer: Vec<String>,
    pub entry_buffer: EntryBuffer,

    pub asset_buffer_flag: i32,
}

impl CanvasState {
    pub fn new_from_environment() -> Result<Self, Box<dyn Error>> {
        let mut stdout: Stdout = std::io::stdout();

        if let Err(e) = execute!(stdout, EnterAlternateScreen) {
            return Err(Box::new(e));
        }

        if let Err(e) = crossterm::terminal::enable_raw_mode() {
            return Err(Box::new(e));
        }

        if let Err(e) = execute!(stdout, Clear(ClearType::All)) {
            return Err(Box::new(e));
        }

        let (w, h): (u16, u16) = match crossterm::terminal::size() {
            Ok((w, h)) => (w, h),
            Err(e) => return Err(Box::new(e)),
        };

        let zero_x = 0;
        let zero_y = 0;
        let size_x = w;
        let size_y = h;

        let mode: Mode = Mode::MainMenu;
        let last_mode: Mode = Mode::MainMenu;
        let idx_buf: i32 = 0;
        let byte_buffer: [u8; 4] = [0u8; 4];
        let asset_buffer: Vec<String> = Vec::new();
        let entry_buffer: EntryBuffer = EntryBuffer::no_entry();

        let asset_buffer_flag: i32 = 0;

        Ok(Self {
            stdout,
            zero_x,
            zero_y,
            size_x,
            size_y,
            mode,
            last_mode,
            idx_buf,
            byte_buffer,
            asset_buffer,
            entry_buffer,
            asset_buffer_flag, // prevents re-init of asset buffer
        })
    }

    pub fn recalculate_dimensions(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let (w, h) = crossterm::terminal::size()?;
        self.size_x = w;
        self.size_y = h;

        self.entry_buffer.size_x = (w * 90) / 100; // 90% of screen size
        self.entry_buffer.size_y = (h * 90) / 100;

        self.entry_buffer.zero_x = (self.size_x - self.entry_buffer.size_x) / 2;
        self.entry_buffer.zero_y = (self.size_y - self.entry_buffer.size_y) / 2;

        Ok(())
    }

    pub fn screen_square(&self) -> Result<(), Box<dyn std::error::Error>> {
        // this was written before we had a byte buffer as a field here
        let mut buffer = [0u8; 4];

        let mut stdout: Stdout = std::io::stdout();

        execute!(stdout, MoveTo(self.zero_x, self.zero_y))?;

        let byte_slice = LEFT_UPPER_SHOULDER.to_bytes(&mut buffer);
        stdout.write_all(byte_slice)?;

        let byte_slice = HORIZONTAL_LINE_HIGH.to_bytes(&mut buffer);
        for i in self.zero_x + 1..=self.size_x - 2 {
            execute!(stdout, MoveTo(i, self.zero_y))?;
            stdout.write_all(byte_slice)?;
        }

        let byte_slice = RIGHT_UPPER_SHOULDER.to_bytes(&mut buffer);
        execute!(stdout, MoveTo(self.size_x, self.zero_y))?;
        stdout.write_all(byte_slice)?;

        let byte_slice = VERTICAL_LINE.to_bytes(&mut buffer);
        for i in self.zero_y + 1..=self.size_y - 1 {
            execute!(stdout, MoveTo(self.zero_x, i))?;
            stdout.write_all(byte_slice)?;
            execute!(stdout, MoveTo(self.size_x, i))?;
            stdout.write_all(byte_slice)?;
        }

        let byte_slice = LEFT_LOWER_SHOULDER.to_bytes(&mut buffer);
        execute!(stdout, MoveTo(self.zero_x, self.size_y))?;
        stdout.write_all(byte_slice)?;

        let byte_slice = HORIZONTAL_LINE_HIGH.to_bytes(&mut buffer);
        for i in self.zero_x + 1..=self.size_x - 2 {
            execute!(stdout, MoveTo(i, self.size_y))?;
            stdout.write_all(byte_slice)?;
        }

        let byte_slice = RIGHT_LOWER_SHOULDER.to_bytes(&mut buffer);
        stdout.write_all(byte_slice)?;

        stdout.flush()?;

        execute!(stdout, MoveTo(self.size_x / 2, self.size_y / 2))?;

        Ok(())
    }

    pub fn draw_main_menu(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // each page gets to be a small event loop of its own, they're all
        // going to be relatively simple up until the buffer editing

        // 3 options;
        /*

        - create new entry ; idx = 0
        - browse entries   ; idx = 1
        - shred all entries; idx = 2

        - quit             ; idx = 3

        */

        let menu_size_x = (self.size_x as f32 * 0.5) as u16; // half of full size
        let menu_size_y = (self.size_y as f32 * 0.5) as u16;

        let menu_zero_x = (self.size_x - menu_size_x) / 2;
        let menu_zero_y = (self.size_y - menu_size_y) / 2;

        let menu_max_x = menu_zero_x + menu_size_x - 1;
        let menu_max_y = menu_zero_y + menu_size_y - 1;

        let byte_slice: &[u8] = LEFT_UPPER_SHOULDER.to_bytes(&mut self.byte_buffer);
        execute!(self.stdout, MoveTo(menu_zero_x, menu_zero_y))?;
        self.stdout.write_all(byte_slice)?;

        let byte_slice = HORIZONTAL_LINE_HIGH.to_bytes(&mut self.byte_buffer);
        for i in menu_zero_x + 1..menu_max_x {
            execute!(self.stdout, MoveTo(i, menu_zero_y))?;
            self.stdout.write_all(byte_slice)?;
        }

        let byte_slice = RIGHT_UPPER_SHOULDER.to_bytes(&mut self.byte_buffer);
        execute!(self.stdout, MoveTo(menu_max_x, menu_zero_y))?;
        self.stdout.write_all(byte_slice)?;

        let byte_slice = VERTICAL_LINE.to_bytes(&mut self.byte_buffer);
        for i in menu_zero_y + 1..=menu_max_y - 1 {
            execute!(self.stdout, MoveTo(menu_zero_x, i))?;
            self.stdout.write_all(byte_slice)?;
            execute!(self.stdout, MoveTo(menu_max_x, i))?;
            self.stdout.write_all(byte_slice)?;
        }

        let byte_slice = LEFT_LOWER_SHOULDER.to_bytes(&mut self.byte_buffer);
        execute!(self.stdout, MoveTo(menu_zero_x, menu_max_y))?;
        self.stdout.write_all(byte_slice)?;

        let byte_slice = HORIZONTAL_LINE_HIGH.to_bytes(&mut self.byte_buffer);
        for i in menu_zero_x + 1..menu_max_x {
            execute!(self.stdout, MoveTo(i, menu_max_y))?;
            self.stdout.write_all(byte_slice)?;
        }

        let byte_slice = RIGHT_LOWER_SHOULDER.to_bytes(&mut self.byte_buffer);
        execute!(self.stdout, MoveTo(menu_max_x, menu_max_y))?;
        self.stdout.write_all(byte_slice)?;

        // box is drawn

        self.stdout.flush()?;

        // by this point, we should only have rendered this much if we have at least 56 (chronicle + 2 on each side) characters.

        self.load_asset_buffer(CHRONICLE_RESOURCE_PATH)?;

        let asset_len_x = 52; // TODO: Calculate this dynamically
                              //       let asset_len_y = 5;

        let asset_x: u16 = menu_zero_x + (menu_max_x - menu_zero_x - asset_len_x) / 2;
        let mut asset_y: u16 = menu_zero_y + 2;

        for line in &self.asset_buffer {
            execute!(self.stdout, MoveTo(asset_x, asset_y))?;
            self.stdout.write_all(line.as_bytes())?;
            asset_y += 1;
        }

        self.draw_main_menu_options()?;

        Ok(())
    }

    pub fn draw_main_menu_options(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // use idx to figure out which of 3 options is selected, render it correctly.

        let mut selected_buf: String = String::new();

        // let option_one_x = (self.size_x / 2) - (MENU_OPTION_NEW_ENTRY.len() as u16 / 2);
        let option_two_x = (self.size_x / 2) - (MENU_OPTION_BROWSE_ENTRIES.len() as u16 / 2);
        // let option_three_x = (self.size_x / 2) - (MENU_OPTION_QUIT.len() as u16 / 2);

        let option_two_y = (self.size_y / 2) - 1;

        let str_max_len = MENU_OPTION_BROWSE_ENTRIES.len();

        let extend_str = |s: &mut String| {
            while s.len() < str_max_len {
                s.push(WHITESPACE)
            }
        };

        match self.idx_buf {
            0 => {
                //
                let mut str: String = String::from(MENU_OPTION_NEW_ENTRY);
                extend_str(&mut str);
                selected_buf = str.bold().black().on_white().to_string();
                execute!(self.stdout, MoveTo(option_two_x, option_two_y - 1))?;
                self.stdout.write_all(selected_buf.as_bytes())?;

                execute!(self.stdout, MoveTo(option_two_x, option_two_y))?;
                self.stdout
                    .write_all(MENU_OPTION_BROWSE_ENTRIES.as_bytes())?;

                execute!(self.stdout, MoveTo(option_two_x, option_two_y + 1))?;
                self.stdout.write_all(MENU_OPTION_QUIT.as_bytes())?;
            }
            1 => {
                //
                let mut str: String = String::from(MENU_OPTION_BROWSE_ENTRIES);
                extend_str(&mut str);

                selected_buf = str.bold().black().on_white().to_string();

                execute!(self.stdout, MoveTo(option_two_x, option_two_y - 1))?;
                self.stdout.write_all(MENU_OPTION_NEW_ENTRY.as_bytes())?;

                execute!(self.stdout, MoveTo(option_two_x, option_two_y))?;
                self.stdout.write_all(selected_buf.as_bytes())?;

                execute!(self.stdout, MoveTo(option_two_x, option_two_y + 1))?;
                self.stdout.write_all(MENU_OPTION_QUIT.as_bytes())?;
            }
            2 => {
                //
                let mut str: String = String::from(MENU_OPTION_QUIT);
                extend_str(&mut str);

                selected_buf = str.bold().black().on_white().to_string();

                execute!(self.stdout, MoveTo(option_two_x, option_two_y - 1))?;
                self.stdout.write_all(MENU_OPTION_NEW_ENTRY.as_bytes())?;

                execute!(self.stdout, MoveTo(option_two_x, option_two_y))?;
                self.stdout
                    .write_all(MENU_OPTION_BROWSE_ENTRIES.as_bytes())?;

                execute!(self.stdout, MoveTo(option_two_x, option_two_y + 1))?;
                self.stdout.write_all(selected_buf.as_bytes())?;
            }
            _ => {
                // do nothing, it should never be this.
            }
        }

        // TEST {
        let status: String = format!("idx: {} mode: {:?}", self.idx_buf.clone(), self.mode);

        execute!(
            self.stdout,
            MoveTo(
                (self.size_x / 2) - status.len() as u16 / 2,
                (self.size_y / 2) + ((self.size_y / 2) / 2) + 1
            )
        )?;

        self.stdout.write_all(status.as_bytes())?;

        // TEST }
        self.stdout.flush()?;

        Ok(())
    }

    pub fn draw_entry_buffer(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let str = String::from("you have selected the new_entry option :)");
        execute!(
            self.stdout,
            MoveTo((self.size_x / 2) - str.len() as u16 / 2, self.size_y / 2)
        )?;
        self.stdout.write_all(str.as_bytes())?;

        self.stdout.flush()?;

        Ok(())
    }

    pub fn draw_selection_buffer(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let str = String::from("you are selecting an entry! :)");
        execute!(
            self.stdout,
            MoveTo((self.size_x / 2) - str.len() as u16 / 2, self.size_y / 2)
        )?;
        self.stdout.write_all(str.as_bytes())?;

        self.stdout.flush()?;

        Ok(())
    }

    pub fn draw_quit_buffer(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = std::io::stdout();
        crossterm::terminal::disable_raw_mode()?;
        execute!(stdout, LeaveAlternateScreen)?;
        std::process::exit(0);
    }

    pub fn clear(&self) {
        let mut stdout: Stdout = std::io::stdout();
        execute!(stdout, Clear(ClearType::All)).unwrap();
    }

    pub fn load_asset_buffer(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        if path == CHRONICLE_RESOURCE_PATH && self.asset_buffer_flag == 1 {
            // sig for chronicle. this will not be implemented for every asset. i just wrote draw_main_menu() really badly
            return Ok(());
        }

        let mut str: String = String::new();
        let mut file: std::fs::File = std::fs::File::open(path)?;
        std::fs::File::read_to_string(&mut file, &mut str)?;

        let lines: Vec<String> = str.lines().map(|s| s.to_string()).collect();

        self.asset_buffer = lines;

        if path == CHRONICLE_RESOURCE_PATH {
            self.asset_buffer_flag = 1;
        }

        Ok(())
    }
}

pub struct EntryBuffer {
    /* we will have size here, but the buffer to be edited lives only
       inside the active_entry. We will be editing the entry directly
       and rendering the string that the entry holds. Ownership is
       gonna be a fucking cunt when we go to encrypt it though.
    */
    pub zero_x: u16,
    pub zero_y: u16,

    pub size_x: u16,
    pub size_y: u16,

    pub text_buffer: String,
    pub active_entry: Entry, // i need to rework the Entry struct before I write any of this.
}

impl EntryBuffer {
    pub fn no_entry() -> Self {
        let zero_x: u16 = 0;
        let zero_y: u16 = 0;

        let size_x: u16 = 0;
        let size_y: u16 = 0;
        let text_buffer: String = String::new();
        let active_entry: Entry = Entry::no_entry();

        Self {
            zero_x,
            zero_y,
            size_x,
            size_y,
            text_buffer,
            active_entry,
        }
    }

    pub fn load_entry(&mut self, entry: Entry) {
        self.active_entry = entry;
    }

    pub fn new_entry(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        todo!();
        Ok(())
    }
}

pub trait CharToBytes {
    fn to_bytes<'a>(&self, buffer: &'a mut [u8; 4]) -> &'a [u8];
}

impl CharToBytes for char {
    fn to_bytes<'a>(&self, buffer: &'a mut [u8; 4]) -> &'a [u8] {
        let len = self.encode_utf8(buffer).len();
        &buffer[..len]
    }
}
