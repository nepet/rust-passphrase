const DICT_FILE: &str = include_str!("./en.txt");

pub enum Lang {
    EN,
}

pub fn new_dict(lang: Lang) -> Box<dyn Dict> {
    match lang {
        Lang::EN => Box::new(DictFromFile::new(DICT_FILE)),
    }
}

pub trait Dict {
    fn size(&self) -> usize;
    fn word(&self, pos: usize) -> String;
}

pub struct DictFromFile {
    words: Vec<String>,
}

impl DictFromFile {
    fn new(words: &str) -> Self {
        let dict: Vec<String> = words.split("\n").map(|s| s.to_string()).collect();
        Self { words: dict }
    }
}

impl Dict for DictFromFile {
    fn size(&self) -> usize {
        self.words.len()
    }

    fn word(&self, pos: usize) -> String {
       self.words[pos].clone()
    }
}
