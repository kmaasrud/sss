#[derive(Debug)]
pub struct Slide {
    pub text: String,
    pub linecount: usize,
}

impl Slide {
    pub fn new() -> Self {
        Slide { text: "".to_string(), linecount: 0, }
    }

    pub fn push_line(&mut self, text: &str) {
        if self.linecount != 0 { self.text.push('\n'); }
        self.text.push_str(text);
        self.linecount += 1;
    }
}
