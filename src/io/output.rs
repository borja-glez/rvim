use std::io::{stdout, Write};

use crossterm::{
    cursor,
    event::KeyCode,
    execute, queue,
    terminal::{self, ClearType},
};

use crate::{config::VERSION, cursor::cursor::CursorController, editor::content::EditorContents};

pub struct Output {
    pub win_size: (usize, usize),
    editor_contents: EditorContents,
    cursor_controller: CursorController,
}

impl Output {
    pub fn new() -> Self {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();
        Self {
            win_size,
            editor_contents: EditorContents::new(),
            cursor_controller: CursorController::new(win_size),
        }
    }

    pub fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    pub fn draw_rows(&mut self) {
        let screen_columns = self.win_size.0;
        let screen_rows = self.win_size.1;
        for i in 0..screen_rows {
            if i == screen_rows / 3 {
                let mut welcome = format!("rVim Editor --- Version {}", VERSION);
                if welcome.len() > screen_columns {
                    welcome.truncate(screen_columns)
                }
                let mut padding = (screen_columns - welcome.len()) / 2;
                if padding != 0 {
                    self.editor_contents.push('~');
                    padding -= 1
                }
                (0..padding).for_each(|_| self.editor_contents.push(' '));
                self.editor_contents.push_str(&welcome)
            } else {
                self.editor_contents.push('~');
            }
            queue!(
                self.editor_contents,
                terminal::Clear(ClearType::UntilNewLine)
            )
            .unwrap();
            if i < screen_rows - 1 {
                self.editor_contents.push_str("\r\n")
            }
        }
    }

    pub fn refresh_screen(&mut self) -> crossterm::Result<()> {
        queue!(self.editor_contents, cursor::Hide, cursor::MoveTo(0, 0))?;
        self.draw_rows();
        let cursor_x = u16::try_from(self.cursor_controller.cursor_x).unwrap();
        let cursor_y = u16::try_from(self.cursor_controller.cursor_y).unwrap();
        queue!(
            self.editor_contents,
            cursor::MoveTo(cursor_x, cursor_y),
            cursor::Show
        )?;
        self.editor_contents.flush()
    }

    pub fn move_cursor(&mut self, direction: KeyCode) {
        self.cursor_controller.move_cursor(direction);
    }
}
