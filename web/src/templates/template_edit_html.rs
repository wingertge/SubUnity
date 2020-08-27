use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};

pub fn edit_html<W>(mut _ructe_out_: &mut W, video_id: &str) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
_ructe_out_.write_all(b"<html lang=\"en\">\r\n    <head>\r\n        <title>Hello World!</title>\r\n        <script id=\"js-init\">\r\n            window.VIDEO_ID = \"")?;
video_id.to_html(&mut _ructe_out_)?;
_ructe_out_.write_all(b"\"\r\n            document.getElementById(\"js-init\").remove()\r\n        </script>\r\n    </head>\r\n    <body>\r\n        <script src=\"/js/bundle.js\" async></script>\r\n    </body>\r\n</html>")?;
Ok(())
}
