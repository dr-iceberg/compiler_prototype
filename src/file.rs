use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

#[derive(Debug)]
pub struct SourceFile {
    source: String,
    new_line_indexes: Vec<u32>,
}

impl SourceFile {
    pub fn new<P>(file: P) -> Result<Self, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(file)?;
        let mut source = String::new();
        {
            let mut reader = BufReader::new(file);
            reader.read_to_string(&mut source)?;
        }
        let mut new_line_indexes = Vec::new();
        for (pos, ch) in source.char_indices() {
            if ch == '\n' {
                new_line_indexes.push(pos as u32);
            }
        }

        Ok(Self {
            source,
            new_line_indexes,
        })
    }

    pub fn get_line_num(&self, position: u32) -> u32 {
        for (line, pos) in self.new_line_indexes().iter().enumerate() {
            if *pos >= position {
                return (line + 1) as u32;
            }
        }
        self.new_line_indexes.len() as u32
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn new_line_indexes(&self) -> &[u32] {
        &self.new_line_indexes
    }
}
