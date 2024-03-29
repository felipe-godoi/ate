use std::env;

use ate::editor::Editor;
use crossterm::terminal;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not turn off Raw mode");
    }
}

fn main() -> crossterm::Result<()> {
    terminal::enable_raw_mode()?;
    let args: Vec<String> = env::args().collect();
    let _clean_up = CleanUp;
    let mut editor = Editor::new(&args);
    editor.run()?;
    Ok(())
}
