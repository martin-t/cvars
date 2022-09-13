//! The in-game console which allows changing cvars at runtime.

// LATER Possible to depend only on fyrox-ui?

use fyrox_ui::{
    border::BorderBuilder,
    brush::Brush,
    core::{color::Color, pool::Handle},
    formatted_text::WrapMode,
    message::{KeyCode, MessageDirection, UiMessage},
    stack_panel::StackPanelBuilder,
    text::{TextBuilder, TextMessage},
    text_box::{TextBoxBuilder, TextCommitMode},
    widget::{WidgetBuilder, WidgetMessage},
    Orientation, UiNode, UserInterface, VerticalAlignment,
};

use cvars_console::{Console, CvarAccess};

/// In-game console for the Fyrox game engine.
pub struct FyroxConsole {
    is_open: bool,
    first_open: bool,
    was_mouse_grabbed: bool,
    console: Console,
    height: f32,
    history: Handle<UiNode>,
    prompt_text_box: Handle<UiNode>,
    layout: Handle<UiNode>,
}

impl FyroxConsole {
    pub fn new(user_interface: &mut UserInterface) -> Self {
        let history = TextBuilder::new(WidgetBuilder::new())
            // Word wrap doesn't work if there's an extremely long word.
            .with_wrap(WrapMode::Letter)
            .build(&mut user_interface.build_ctx());

        let prompt_arrow = TextBuilder::new(WidgetBuilder::new())
            .with_text("> ")
            .build(&mut user_interface.build_ctx());

        let prompt_text_box = TextBoxBuilder::new(WidgetBuilder::new())
            .with_text_commit_mode(TextCommitMode::Immediate)
            .with_skip_chars(vec!['-', '_'])
            .build(&mut user_interface.build_ctx());

        let prompt_line = StackPanelBuilder::new(
            WidgetBuilder::new().with_children([prompt_arrow, prompt_text_box]),
        )
        .with_orientation(Orientation::Horizontal)
        .build(&mut user_interface.build_ctx());

        // StackPanel doesn't support colored background so we wrap it in a Border.
        let layout = BorderBuilder::new(
            WidgetBuilder::new()
                .with_visibility(false)
                .with_background(Brush::Solid(Color::BLACK.with_new_alpha(220)))
                .with_child(
                    StackPanelBuilder::new(
                        WidgetBuilder::new()
                            .with_vertical_alignment(VerticalAlignment::Bottom)
                            .with_children([history, prompt_line]),
                    )
                    .with_orientation(Orientation::Vertical)
                    .build(&mut user_interface.build_ctx()),
                ),
        )
        .build(&mut user_interface.build_ctx());

        FyroxConsole {
            is_open: false,
            first_open: true,
            was_mouse_grabbed: false,
            console: Console::new(),
            height: 0.0,
            history,
            prompt_text_box,
            layout,
        }
    }

    pub fn resized(&mut self, user_interface: &mut UserInterface, width: f32, height: f32) {
        user_interface.send_message(WidgetMessage::width(
            self.layout,
            MessageDirection::ToWidget,
            width,
        ));

        self.height = height / 2.0;
        user_interface.send_message(WidgetMessage::height(
            self.layout,
            MessageDirection::ToWidget,
            self.height,
        ));

        // This actually goes beyond the screen but who cares.
        // It, however, still won't let me put the cursor at the end by clicking after the text:
        // https://github.com/FyroxEngine/Fyrox/issues/361
        user_interface.send_message(WidgetMessage::width(
            self.prompt_text_box,
            MessageDirection::ToWidget,
            width,
        ));

        // The number of lines that can fit might have changed - reprint history.
        self.update_ui_history(user_interface);
    }

    pub fn ui_message(
        &mut self,
        user_interface: &mut UserInterface,
        cvars: &mut impl CvarAccess,
        msg: UiMessage,
    ) {
        if msg.destination != self.prompt_text_box {
            return;
        }

        // We could just listen for KeyboardInput and get the text from the prompt via
        // ```
        // let node = user_interface.node(self.prompt_text_box);
        // let text = node.query_component::<TextBox>().unwrap().text();
        // ```
        // But this is the intended way to use the UI, even if it's more verbose.
        // At least it should reduce issues with the prompt reacting to some keys
        // but not others given KeyboardInput doesn't require focus.
        //
        // Note that it might still be better to read the text from the UI as the souce of truth
        // because right now the console doesn't know about any text we set from code.
        if let Some(TextMessage::Text(text)) = msg.data() {
            self.console.prompt = text.to_owned();
        }

        match msg.data() {
            Some(WidgetMessage::Unfocus) => {
                // As long as the console is open, always keep the prompt focused
                if self.is_open {
                    user_interface.send_message(WidgetMessage::focus(
                        self.prompt_text_box,
                        MessageDirection::ToWidget,
                    ));
                }
            }
            Some(WidgetMessage::KeyDown(KeyCode::Up)) => {
                self.console.history_back();
                self.update_ui_prompt(user_interface);
            }
            Some(WidgetMessage::KeyDown(KeyCode::Down)) => {
                self.console.history_forward();
                self.update_ui_prompt(user_interface);
            }
            Some(WidgetMessage::KeyDown(KeyCode::PageUp)) => {
                self.console.history_scroll_up(10);
                self.update_ui_history(user_interface);
            }
            Some(WidgetMessage::KeyDown(KeyCode::PageDown)) => {
                self.console.history_scroll_down(10);
                self.update_ui_history(user_interface);
            }
            Some(WidgetMessage::KeyDown(KeyCode::Return | KeyCode::NumpadEnter)) => {
                self.console.enter(cvars);
                self.update_ui_prompt(user_interface);
                self.update_ui_history(user_interface);
            }
            _ => (),
        }
    }

    fn update_ui_prompt(&mut self, user_interface: &mut UserInterface) {
        user_interface.send_message(TextMessage::text(
            self.prompt_text_box,
            MessageDirection::ToWidget,
            self.console.prompt.clone(),
        ));
    }

    fn update_ui_history(&mut self, user_interface: &mut UserInterface) {
        // LATER There should be a cleaner way to measure lines
        let line_height = 14;
        // Leave 1 line room for the prompt
        // LATER This is not exact for tiny windows but good enough for now.
        let max_lines = (self.height as usize / line_height).saturating_sub(1);

        let hi = self.console.history_view_end;
        let lo = hi.saturating_sub(max_lines);

        let mut hist = String::new();
        for line in &self.console.history[lo..hi] {
            if line.is_input {
                hist.push_str("> ");
            }
            hist.push_str(&line.text);
            hist.push('\n');
        }

        user_interface.send_message(TextMessage::text(
            self.history,
            MessageDirection::ToWidget,
            hist,
        ));
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    /// Open the console.
    ///
    /// If your game grabs the mouse, you can save the previous state here
    /// and get it back when closing.
    pub fn open(&mut self, user_interface: &mut UserInterface, was_mouse_grabbed: bool) {
        self.is_open = true;
        self.was_mouse_grabbed = was_mouse_grabbed;

        user_interface.send_message(WidgetMessage::visibility(
            self.layout,
            MessageDirection::ToWidget,
            true,
        ));

        user_interface.send_message(WidgetMessage::focus(
            self.prompt_text_box,
            MessageDirection::ToWidget,
        ));

        if self.first_open {
            // Currently it's not necessary to track the first opening,
            // the history will be empty so we could just print it when creating the console.
            // Eventually though, all stdout will be printed in the console
            // so if the message was at the top, nobody would see it.
            self.first_open = false;
            self.console.print("Type 'help' or '?' for basic info");
            self.update_ui_history(user_interface);
        }
    }

    /// Close the console. Returns whether the mouse was grabbed before opening the console.
    ///
    /// It's #[must_use] so you don't accidentally forget to restore it.
    /// You can safely ignore it intentionally.
    #[must_use]
    pub fn close(&mut self, user_interface: &mut UserInterface) -> bool {
        user_interface.send_message(WidgetMessage::visibility(
            self.layout,
            MessageDirection::ToWidget,
            false,
        ));
        user_interface.send_message(WidgetMessage::unfocus(
            self.prompt_text_box,
            MessageDirection::ToWidget,
        ));

        self.is_open = false;
        self.was_mouse_grabbed
    }
}
