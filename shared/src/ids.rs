pub struct IdGenerator {
    next: u32,
}

impl IdGenerator {
    pub fn new() -> Self {
        Self { next: 0 }
    }
    pub fn next_id(&mut self) -> u32 {
        let id = self.next;
        self.next += 1;
        id
    }
}
