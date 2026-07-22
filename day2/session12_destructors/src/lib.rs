/// A resource that logs when it is dropped.
pub struct Tracked {
    pub name: String,
    pub dropped: std::sync::Arc<std::sync::Mutex<bool>>,
}

impl Tracked {
    pub fn new(name: &str, dropped: std::sync::Arc<std::sync::Mutex<bool>>) -> Self {
        Tracked { name: name.to_string(), dropped }
    }
}

/// TODO: implement the Drop trait for Tracked.
/// When dropped, set the `dropped` flag to true.
impl Drop for Tracked {
    fn drop(&mut self) {
        todo!()
    }
}
