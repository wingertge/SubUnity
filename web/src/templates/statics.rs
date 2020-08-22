/// A static file has a name (so its url can be recognized) and the
/// actual file contents.
///
/// The name includes a short (48 bits as 8 base64 characters) hash of
/// the content, to enable long-time caching of static resourses in
/// the clients.
#[allow(dead_code)]
pub struct StaticFile {
    pub content: &'static [u8],
    pub name: &'static str
}
#[allow(dead_code)]
impl StaticFile {
    /// Get a single `StaticFile` by name, if it exists.
    pub fn get(name: &str) -> Option<&'static Self> {
        if let Ok(pos) = STATICS.binary_search_by_key(&name, |s| s.name) {
            Some(STATICS[pos])
        } else {
            None
        }
    }
}

/// From "D:\\Coding\\Rust\\subtitle-thingy\\web\\assets\\test.png"
#[allow(non_upper_case_globals)]
pub static test_png: StaticFile = StaticFile {
    content: include_bytes!("D:\\Coding\\Rust\\subtitle-thingy\\web\\assets\\test.png"),
    name: "test-0v85n1IA.png"
};

pub static STATICS: &[&StaticFile] = &[&test_png];
