use parking_lot::Mutex;
use std::collections::HashSet;

pub trait Intern {
    /// insert self into static list of strings, or return already existing
    ///
    /// prevents duplicates and lets us simply copy
    fn intern(self) -> &'static str;
}
impl Intern for String {
    fn intern(self) -> &'static str {
        static STRINGS: Mutex<Option<HashSet<&str>>> = Mutex::new(None);
        let mut strings = STRINGS.lock();
        let strings = strings.get_or_insert_with(HashSet::new);
        if let Some(str) = strings.get(self.as_str()) {
            str
        } else {
            let str = Box::leak(self.into_boxed_str());
            strings.insert(str);
            str
        }
    }
}
