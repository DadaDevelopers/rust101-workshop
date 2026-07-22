use session12_destructors::Tracked;
use std::sync::{Arc, Mutex};

#[test]
fn test_drop_sets_flag() {
    let flag = Arc::new(Mutex::new(false));
    {
        let _t = Tracked::new("resource", Arc::clone(&flag));
        assert!(!*flag.lock().unwrap());
    }
    assert!(*flag.lock().unwrap());
}
