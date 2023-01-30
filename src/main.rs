use crossterm::terminal;
use rvim::{config::drop::CleanUp, editor::editor::Editor};

fn main() -> crossterm::Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    let mut editor = Editor::new();
    while editor.run()? {}
    Ok(())
}
