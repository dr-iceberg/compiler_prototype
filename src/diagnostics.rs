#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    start: u32,
    end: u32,
}

impl Span {
    pub fn new_single_char(pos: usize) -> Self {
        Self::new(pos, pos)
    }

    pub fn new(start: usize, end: usize) -> Self {
        assert!(start <= end);
        Self {
            start: start as u32,
            end: end as u32,
        }
    }

    pub fn start(&self) -> u32 {
        self.start
    }

    pub fn end(&self) -> u32 {
        self.end
    }
}
