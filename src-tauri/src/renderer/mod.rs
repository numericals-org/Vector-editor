pub struct Renderer {
    initialized: bool,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            initialized: false,
        }
    }

    pub fn initialize(&mut self) {
        println!("Initializing GPU renderer...");
        self.initialized = true;
    }

    pub fn is_ready(&self) -> bool {
        self.initialized
    }
}
