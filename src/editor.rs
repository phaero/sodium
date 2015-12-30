use cursor::Cursor;
use std::collections::VecDeque;
use graphics::StatusBar;
use options::Options;
use key_state::KeyState;
use redraw::RedrawTask;
use buffer::SplitBuffer;

#[cfg(feature = "orbital")]
use orbital::Window;

/// The current state of the editor, including the file, the cursor, the scrolling info, etc.
pub struct Editor {
    /// The current cursor
    pub current_cursor: u8,
    /// The cursors
    pub cursors: Vec<Cursor>,
    /// The buffer (document)
    pub buffer: SplitBuffer,
    /// The x coordinate of the scroll
    pub scroll_x: usize,
    /// The y coordinate of the scroll
    pub scroll_y: usize,
    /// The window
    pub window: Window,
    /// The status bar
    pub status_bar: StatusBar,
    /// The prompt
    pub prompt: String,
    /// The settings
    pub options: Options,
    /// The key state
    pub key_state: KeyState,
    /// Redraw
    pub redraw_task: RedrawTask,
}

impl Editor {
    /// Create new default state editor
    pub fn init() {


        #[cfg(feature = "orbital")]
        let window = Window::new(-1, -1, 700, 500, &"Sodium").unwrap();

        let mut editor = Editor {
            current_cursor: 0,
            cursors: vec![Cursor::new()],
            buffer: SplitBuffer::new(),
            scroll_x: 0,
            scroll_y: 0,
            window: *window,
            status_bar: StatusBar::new(),
            prompt: String::new(),
            options: Options::new(),
            key_state: KeyState::new(),
            redraw_task: RedrawTask::Null,
        };

        editor.redraw();

        loop {
            let inp = editor.get_inst();
            if let Inst(_, Cmd { key: Key::Quit }) = inp {
                break;
            }
            editor.exec(inp);
            editor.redraw();
            editor.status_bar.mode = editor.cursor().mode.to_string();
        }
    }

}
