#[derive(Debug)]
pub struct File {
    name: String,
    size: usize,
    mode: u16,
}

impl File {
    pub fn new(name: impl Into<String>, size: usize, mode: u16) -> Self {
        Self {
            name: name.into(),
            size,
            mode,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn mode(&self) -> u16 {
        self.mode
    }
}
