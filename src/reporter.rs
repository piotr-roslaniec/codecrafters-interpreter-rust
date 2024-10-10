#[derive(Clone)]
pub struct Reporter {
    pub errors: Vec<String>,
}

impl Reporter {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }
    pub fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message)
    }

    fn report(&mut self, line: usize, location: &str, message: &str) {
        let error = format!("[line {line}] Error {location}: {message}");
        eprintln!("{}", error);
        self.errors.push(error)
    }
}
