use std::collections::HashMap;
use std::fs::read_to_string;
use coloredpp::Colorize;
use crate::Pulse;

impl Pulse {
    pub fn load_env(&mut self, file: &'static str) -> HashMap<String, String> {
        let mut secrets = HashMap::new();
        let contents = load_file(file).unwrap();
        for line in contents.lines() {
            if let Some((key, value)) = line.split_once('=') {
                secrets.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
        self.secrets = secrets.clone();
        secrets
    }
}

pub fn load_file(file: &'static str) -> Option<String> {
    match read_to_string(file) {
        Ok(contents) => Some(contents),
        Err(e) => {
            eprintln!("{}{}{}: {}",
                      "failed to open file '".red(),
                      file.yellow().bold(),
                      "'".red(),
                      e);
            None
        }
    }
}
