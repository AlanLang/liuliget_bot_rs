#[derive(Debug)]
pub struct Liuliget {
    working: bool,
}

impl Liuliget {
    pub fn new() -> Liuliget {
        Liuliget { working: false }
    }

    pub fn start(&mut self) {
        self.working = true;
    }
}
