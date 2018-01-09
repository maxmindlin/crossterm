use Construct;
use super::base_terminal::{ClearType, ITerminal};
use super::{NoTerminal, UnixTerminal, WinApiTerminal};

/// Struct with the terminal on wits terminal realated actions can be performed.
pub struct Terminal {
    terminal: Option<Box<ITerminal>>,
}

impl Terminal {
    /// Instantiate an color implementation whereon color related actions can be performed.
    pub fn init(&mut self) {
        if let None = self.terminal {
            self.terminal = get_terminal();
        }
    }

    /// Clear the current cursor by specifying the clear type
    /// 
    /// #Example
    ///
    /// ```rust
    /// 
    /// let mut terminal = terminal::get();
    /// 
    /// // clear all cells in terminal.
    /// terminal.clear(ClearType::All);
    ///  //clear all cells after the cursor position in terminal.
    /// terminal.clear(ClearType::AfterCursor);
    /// // clear all cells before cursor in terminal.
    /// terminal.clear(ClearType::BeforeCursor);
    /// // clear current line cells in terminal.
    /// terminal.clear(ClearType::CurrentLine);
    /// // clear all cells from cursor position until new line in terminal.
    /// terminal.clear(ClearType::UntilNewLine);
    /// 
    /// ```
    pub fn clear(&mut self, clear_type: ClearType) {
        &self.init();
        if let Some(ref terminal) = self.terminal {
            terminal.clear(clear_type);
        }
    }

    /// Get the terminal size (x,y).
    /// 
    /// #Example
    ///
    /// ```rust
    /// 
    /// let mut terminal = terminal::get();
    /// 
    /// let size = terminal.terminal_size();
    /// println!("{:?}", size);
    /// 
    /// ```
    pub fn terminal_size(&mut self) -> Option<(u16, u16)> {
        &self.init();
        if let Some(ref terminal) = self.terminal {
            let a = terminal.terminal_size();
            a
        } else {
            None
        }
    }

    /// Scroll `n` lines up in the current terminal.
    pub fn scroll_up(&mut self, count: i16) {
        for i in 0..100 {
            println!("Ik ben timon en dit is een test {}", i)
        }

        &self.init();
        if let Some(ref terminal) = self.terminal {
            terminal.scroll_up(count);
        }
    }

    /// Scroll `n` lines up in the current terminal.
    pub fn scroll_down(&self) {}
}

/// Get the concrete ITerminal implementation based on the current operating system.
fn get_terminal() -> Option<Box<ITerminal>> {
    if cfg!(target_os = "linux") {
        Some(UnixTerminal::new())
    } else if cfg!(target_os = "windows") {
        Some(WinApiTerminal::new())
    } else {
        Some(NoTerminal::new())
    }
}

/// Get terminal whereon terminal related actions can be performed.
pub fn get() -> Box<Terminal> {
    Box::from(Terminal {
        terminal: get_terminal(),
    })
}
