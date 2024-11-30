pub struct Settings {
    pub path: String,
}

impl Settings {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}
