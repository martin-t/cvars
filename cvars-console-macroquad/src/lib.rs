//! The in-game console which allows changing cvars at runtime.

use cvars_console::{Console, CvarAccess};
use macroquad::{
    prelude::*,
    ui::{
        hash, root_ui,
        widgets::{Group, Label},
        Layout, Skin,
    },
};

#[derive(Debug, Clone, Default)]
pub struct MacroquadConsole {
    is_open: bool,
    console: Console,
    config: Config,
    input: ConsoleInput,
    input_prev: ConsoleInput,
}

impl MacroquadConsole {
    pub fn new() -> Self {
        Self {
            is_open: false,
            console: Console::new(),
            config: Config::default(),
            input: ConsoleInput::new(),
            input_prev: ConsoleInput::new(),
        }
    }

    pub fn update(&mut self, cvars: &mut dyn CvarAccess) {
        self.input_prev = self.input;
        self.input = get_input();

        self.open_close();

        if self.is_open {
            self.process_input();
            self.draw_console();
            if !self.input_prev.enter && self.input.enter && !self.console.prompt.is_empty() {
                self.console.enter(cvars);
            }
        }
    }

    /// Open or close the console based on user's input.
    fn open_close(&mut self) {
        let pressed_console = !self.input_prev.console && self.input.console;
        let pressed_escape = !self.input_prev.escape && self.input.escape;
        if !self.is_open && pressed_console {
            self.is_open = true;
            show_mouse(true);
        } else if self.is_open && (pressed_console || pressed_escape) {
            self.is_open = false;
            show_mouse(false);
        }
    }

    /// Sanitize input text, handle cycling through history, etc.
    fn process_input(&mut self) {
        // The semicolon (default console bind) gets typed into the console
        // when opening it (but interestingly not closing).
        // Just disallow it completely we don't allow multiple commands on one line
        // so there's currently no need for it.
        // This has the side effect, that the text cursor moves one char to the right
        // with each open/close cycle but that's OK.
        // LATER A less hacky input system would be great.
        self.console.prompt = self.console.prompt.replace(';', "");

        // Detect key pressed based on previous and current state.
        // MQ's UI doesn't seem to hae a built-in way to detecting keyboard events.
        let pressed_up = !self.input_prev.up && self.input.up;
        let pressed_down = !self.input_prev.down && self.input.down;
        let pressed_page_up = !self.input_prev.page_up && self.input.page_up;
        let pressed_page_down = !self.input_prev.page_down && self.input.page_down;

        if pressed_up {
            self.console.history_back();
        }

        // Go forward in history
        if pressed_down {
            self.console.history_forward();
        }

        // Scroll history up
        let count = 10; // LATER configurable
        if pressed_page_up {
            self.console.history_scroll_up(count);
        }
        if pressed_page_down {
            self.console.history_scroll_down(count);
        }
    }

    /// Draw the console and the UI elements it needs.
    fn draw_console(&mut self) {
        // Draw background
        // Floor aligns to pixels, otherwise text renders poorly.
        let console_height = (screen_height() * self.config.height_fraction).floor();
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            console_height,
            Color::new(0.0, 0.0, 0.0, self.config.background_alpha),
        );
        draw_line(
            0.0,
            console_height,
            screen_width(),
            console_height,
            1.0,
            RED,
        );

        // Draw history
        // This doesn't allow copying but in MQ's UI there's no way to print text
        // which allows copying while preventing editing.
        if self.console.history_view_end >= 1 {
            let mut i = self.console.history_view_end - 1;
            let mut y = console_height - self.config.history_y_offset;
            loop {
                let text = if self.console.history[i].is_input {
                    format!("> {}", self.console.history[i].text)
                } else {
                    self.console.history[i].text.clone()
                };
                draw_text(
                    &text,
                    self.config.history_x,
                    y,
                    self.config.history_line_font_size,
                    WHITE,
                );
                if i == 0 || y < 0.0 {
                    break;
                }
                i -= 1;
                y -= self.config.history_line_height;
            }
        }

        // Prompt style
        let bg_image = Image::gen_image_color(1, 1, BLANK);
        let style = root_ui()
            .style_builder()
            .background(bg_image)
            .color(BLANK) // This hides the faint rectangle around a Group
            .text_color(WHITE)
            .build();
        let skin = Skin {
            label_style: style.clone(),
            editbox_style: style.clone(),
            group_style: style,
            ..root_ui().default_skin()
        };
        root_ui().push_skin(&skin);

        // Draw prompt - this uses MQ's UI so i don't have to reimplement basic text editing ops.
        let id_prompt = 0;
        let label_y = console_height - self.config.prompt_label_y_offset;
        Label::new(">")
            .position(vec2(self.config.prompt_label_x, label_y))
            .ui(&mut root_ui());
        // Can't set position on an InputText so we wrap it in a Group.
        let group_y =
            screen_height() * self.config.height_fraction - self.config.prompt_group_y_offset;
        Group::new(hash!(), vec2(screen_width() - 8.0, 20.0))
            .position(vec2(self.config.prompt_group_x, group_y))
            .layout(Layout::Horizontal)
            .ui(&mut root_ui(), |ui| {
                ui.input_text(id_prompt, "", &mut self.console.prompt);
            });

        // The prompt should have focus all the time.
        root_ui().set_input_focus(id_prompt);
    }

    /// Whether the console is open right now.
    ///
    /// Useful for example to ignore game-related input
    /// while the player is typing into console.
    pub fn is_open(&self) -> bool {
        self.is_open
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub background_alpha: f32,
    pub prompt_group_x: f32,
    pub prompt_group_y_offset: f32,
    pub height_fraction: f32,
    pub history_line_font_size: f32,
    pub history_line_height: f32,
    pub history_x: f32,
    pub history_y_offset: f32,
    pub prompt_label_x: f32,
    pub prompt_label_y_offset: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            background_alpha: 0.8,
            prompt_group_x: 16.0,
            prompt_group_y_offset: 26.0,
            height_fraction: 0.45,
            history_line_font_size: 16.0,
            history_line_height: 14.0,
            history_x: 8.0,
            history_y_offset: 25.0,
            prompt_label_x: 8.0,
            prompt_label_y_offset: 22.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct ConsoleInput {
    console: bool,
    escape: bool,
    enter: bool,
    up: bool,
    down: bool,
    page_up: bool,
    page_down: bool,
}

impl ConsoleInput {
    fn new() -> Self {
        Self::default()
    }
}

fn get_input() -> ConsoleInput {
    let mut input = ConsoleInput::new();
    if are_keys_pressed(&[KeyCode::GraveAccent, KeyCode::Semicolon]) {
        input.console = true;
    }
    if are_keys_pressed(&[KeyCode::Escape]) {
        input.escape = true;
    }
    if are_keys_pressed(&[KeyCode::Enter, KeyCode::KpEnter]) {
        input.enter = true;
    }
    if are_keys_pressed(&[KeyCode::Up]) {
        input.up = true;
    }
    if are_keys_pressed(&[KeyCode::Down]) {
        input.down = true;
    }
    if are_keys_pressed(&[KeyCode::PageUp]) {
        input.page_up = true;
    }
    if are_keys_pressed(&[KeyCode::PageDown]) {
        input.page_down = true;
    }
    input
}

fn are_keys_pressed(key_codes: &[KeyCode]) -> bool {
    for &key_code in key_codes {
        if is_key_pressed(key_code) {
            return true;
        }
    }
    false
}
