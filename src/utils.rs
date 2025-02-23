use termimad::{MadSkin, terminal_size};

pub fn display_markdown(text: &str) {
    let mut skin = MadSkin::default();
    skin.set_headers_fg(termimad::rgb(255, 187, 0));
    skin.bold.set_fg(termimad::rgb(255, 187, 0));
    skin.italic.set_fg(termimad::rgb(215, 215, 215));
    skin.code_block.set_fg(termimad::rgb(187, 187, 187));
    
    let (_width, _) = terminal_size();  // Changed to _width to silence warning
    skin.print_text(text);
}
