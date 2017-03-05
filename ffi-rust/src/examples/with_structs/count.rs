pub struct Args {
    init: u32,
    by: u32,
}

pub struct Counter {
    val: u32,
    by: u32,
}

impl Counter {
    pub fn new(args: Args) -> Counter {
        Counter{
            val: args.init,
            by: args.by,
        }
    }

    pub fn get(&self) -> u32 {
        self.val
    }

    pub fn incr(&mut self) -> u32 {
        self.val += self.by;
        self.val
    }

    pub fn decr(&mut self) -> u32 {
        self.val -= self.by;
        self.val
    }
}
