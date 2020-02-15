#[derive(Debug)]
struct Lexer {
    input_chars: Vec<char>,
    current_position: usize,
    read_position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input_chars: input.chars().collect(),
            current_position: 0,
            read_position: 0,
            current_char: None,
        }
        lexer.read_char();
    }

    fn read_char(&self) {
        self.current_position = self.read_position;
        if self.read_position < self.input_chars.len() {
            self.current_char = self.input_chars[read_position];
            self.read_position += 1;
        } else {
            self.current_char = None;
        }
    }
}