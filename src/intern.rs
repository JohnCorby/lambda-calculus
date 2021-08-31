use std::collections::HashSet;
use parking_lot::Mutex;

pub trait Intern {
    /// insert self into static list of strings, or return already existing
    ///
    /// prevents duplicates and lets us simply copy
    fn intern(self) -> &'static str;
}
impl Intern for String {
    fn intern(self) -> &'static str {
        static LOCK: Mutex<()> = Mutex::new(());
        let lock = LOCK.lock();

        let str = unsafe {
            // thread safe because we do locking above
            static mut STRINGS: Option<HashSet<String>> = None;
            if STRINGS.is_none() {
                STRINGS = Some(Default::default())
            }
            let strings = STRINGS.as_mut().unwrap_unchecked();
            // deref is safe because hashset only grows, meaning the strings only move, but not the underlying pointer to the data
            strings.get_or_insert(self).as_str()
        };

        drop(lock);
        str
    }
}
