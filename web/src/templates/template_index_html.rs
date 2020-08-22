use super::statics::test_png;
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature = "cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html, ToHtml};
use std::io::{self, Write};

pub fn index_html<W>(mut _ructe_out_: &mut W, name: &str, items: &[&str]) -> io::Result<()>
where
    W: ?Sized,
    for<'a> &'a mut W: Write
{
    _ructe_out_.write_all(b"<html lang=\"en\">\r\n    <head><title>")?;
    name.to_html(&mut _ructe_out_)?;
    _ructe_out_.write_all(b"</title></head>\r\n        <body>\r\n            ")?;
    if items.is_empty() {
        _ructe_out_.write_all(b"\r\n                <p>There are no items.</p>\r\n            ")?;
    } else {
        _ructe_out_.write_all(b"\r\n                <p>There are ")?;
        items.len().to_html(&mut _ructe_out_)?;
        _ructe_out_.write_all(b" items.</p>\r\n                <ul>\r\n                    ")?;
        for item in items {
            _ructe_out_.write_all(b"\r\n                        <li>")?;
            item.to_html(&mut _ructe_out_)?;
            _ructe_out_.write_all(b"</li>\r\n                    ")?;
        }
        _ructe_out_.write_all(b"</ul>\r\n            ")?;
    }
    _ructe_out_.write_all(b"\r\n            <img alt=\"Test\" src=\"/static/")?;
    test_png.name.to_html(&mut _ructe_out_)?;
    _ructe_out_.write_all(b"\" />\r\n        </body>\r\n</html>")?;
    Ok(())
}
