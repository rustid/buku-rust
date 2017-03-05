pub struct Counter {
    val: u32
}

impl Counter {
    pub fn new(val: u32) -> Counter {
        Counter{val: val}
    }

    pub fn get(&self) -> u32 {
        self.val
    }

    pub fn incr(&mut self, bys: &[u32]) -> u32 {
        for by in bys {
            self.val += *by;
        }
        self.val
    }

    pub fn decr(&mut self, bys: &[u32]) -> u32 {
        for by in bys {
            self.val -= *by;
        }
        self.val
    }
}
