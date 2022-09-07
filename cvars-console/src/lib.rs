//! Engine-independant parts of in-game consoles.
//!
//! Parsing and executing commands, help, history, eventually cvarlist and tab completion, ...

use std::mem;

/// Engine-independant parts of the in-game console.
#[derive(Debug, Clone, Default)]
pub struct Console {
    /// The current contents of the prompt.
    ///
    /// Should always be kept in sync with what's displayed in the UI.
    pub prompt: String,

    /// Prompt to restore when using up and down keys. None if we're not currently walking through history.
    prompt_saved: Option<String>,

    /// Where we are in history when using up and down keys. None if we're not currently walking through history.
    prompt_history_index: Option<usize>,

    /// Input and output history.
    ///
    /// You should prepend input lines with "> " or something similar when displaying them.
    pub history: Vec<HistoryLine>,

    /// Where we are in the history view when scrolling using page up and down keys.
    ///
    /// This index is *one past* the last line to be displayed at the *bottom*
    /// so that you can use it as the high end of a range.
    pub history_view_end: usize,
}

impl Console {
    pub fn new() -> Self {
        Console {
            prompt: String::new(),
            prompt_saved: None,
            prompt_history_index: None,
            history: Vec::new(),
            history_view_end: 0,
        }
    }

    /// Go back in command history.
    ///
    /// Save the prompt so that users can go back in history,
    /// then come back to present and get what they typed back.
    pub fn history_back(&mut self) {
        let search_slice = if let Some(hi) = self.prompt_history_index {
            &self.history[0..hi]
        } else {
            &self.history[..]
        };
        if let Some(new_index) = search_slice
            .iter()
            .rposition(|hist_line| hist_line.is_input)
        {
            self.prompt_history_index = Some(new_index);
            if self.prompt_saved.is_none() {
                self.prompt_saved = Some(self.prompt.clone());
            }
            self.prompt = self.history[new_index].text.clone();
        }
    }

    /// Go forward in command history.
    ///
    /// Restore the saved prompt if get to the end.
    pub fn history_forward(&mut self) {
        if let Some(index) = self.prompt_history_index {
            // Start after the current, otherwise we'd immediately find the current, not the next.
            // It's ok to index 1 past the end.
            let begin = index + 1;
            let search_slice = &self.history[begin..];
            if let Some(local_index) = search_slice.iter().position(|hist_line| hist_line.is_input)
            {
                // `position` starts counting from the iterator's start,
                // not from history's start so we add the found index to what we skipped
                // instead of using it directly.
                let new_index = begin + local_index;
                self.prompt_history_index = Some(new_index);
                self.prompt = self.history[new_index].text.clone();
            } else {
                // We're at the end of history, restore the saved prompt.
                self.prompt_history_index = None;
                self.prompt = self.prompt_saved.take().unwrap();
            }
        }
    }

    pub fn history_scroll_up(&mut self, count: usize) {
        self.history_view_end = self.history_view_end.saturating_sub(count);
        if self.history_view_end == 0 && !self.history.is_empty() {
            // Keep at least one line in history when possible
            // because scrolling up to an empty view looks weird.
            self.history_view_end = 1;
        }
    }

    pub fn history_scroll_down(&mut self, count: usize) {
        self.history_view_end = (self.history_view_end + count).min(self.history.len());
    }

    /// The user pressed enter - process the line of text
    pub fn enter(&mut self, cvars: &mut impl CvarAccess) {
        let cmd = mem::take(&mut self.prompt);

        self.print_input(&cmd);

        // The actual command parsing logic
        let res = self.execute_command(cvars, &cmd);
        if let Err(msg) = res {
            self.print(msg);
        }

        // Entering a new command resets the user's position in history to the end.
        self.prompt_history_index = None;
    }

    /// Parse what the user typed and get or set a cvar
    fn execute_command(&mut self, cvars: &mut impl CvarAccess, cmd: &str) -> Result<(), String> {
        // Splitting on whitespace also in effect trims leading and trailing whitespace.
        let mut parts = cmd.split_whitespace();

        let cvar_name = match parts.next() {
            Some(name) => name,
            None => return Ok(()),
        };
        if cvar_name == "help" || cvar_name == "?" {
            self.print("Available actions:");
            self.print("    help                 Print this message");
            self.print("    <cvar name>          Print the cvar's value");
            self.print("    <cvar name> <value>  Set the cvar's value");
            return Ok(());
        }

        let cvar_value = match parts.next() {
            Some(val) => val,
            None => {
                let val = cvars.get_string(cvar_name)?;
                self.print(val);
                return Ok(());
            }
        };
        if let Some(rest) = parts.next() {
            return Err(format!("expected only cvar name and value, found {}", rest));
        }
        cvars.set_str(cvar_name, cvar_value)
    }

    pub fn print<S: Into<String>>(&mut self, text: S) {
        self.push_history_line(text.into(), false);
    }

    fn print_input<S: Into<String>>(&mut self, text: S) {
        self.push_history_line(text.into(), true);
    }

    fn push_history_line(&mut self, text: String, is_input: bool) {
        let hist_line = HistoryLine::new(text, is_input);
        self.history.push(hist_line);

        // LATER Make this configurable so adding new lines doesn't scroll the view.
        self.history_view_end += 1;
    }
}

#[derive(Debug, Clone)]
pub struct HistoryLine {
    pub text: String,
    /// Whether the line is input from the user or output from running a command.
    pub is_input: bool,
}

impl HistoryLine {
    pub fn new(text: String, is_input: bool) -> Self {
        Self { text, is_input }
    }
}

/// A mostly internal trait for writing generic code
/// that can access cvars but doesn't know the concrete Cvars struct.
pub trait CvarAccess {
    fn get_string(&self, cvar_name: &str) -> Result<String, String>;
    fn set_str(&mut self, cvar_name: &str, str_value: &str) -> Result<(), String>;
}
