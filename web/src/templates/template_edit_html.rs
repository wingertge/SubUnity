use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};

pub fn edit_html<W>(mut _ructe_out_: &mut W, video_id: &str, lang: &str) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
_ructe_out_.write_all(b"<html lang=\"en\">\r\n    <head>\r\n        <title>Subtitle Editor</title>\r\n        <meta charset=\"utf-8\" />\r\n        <link rel=\"stylesheet\" href=\"https://cdnjs.cloudflare.com/ajax/libs/normalize/8.0.1/normalize.min.css\" />\r\n    </head>\r\n    <body>\r\n        <div id=\"root\"></div>\r\n        <script src=\"https://www.youtube.com/iframe_api\" async></script>\r\n        <script>\r\n            window.VIDEO_ID = \"")?;
video_id.to_html(&mut _ructe_out_)?;
_ructe_out_.write_all(b"\";\r\n            window.SUBTITLE_LANG = \"")?;
lang.to_html(&mut _ructe_out_)?;
_ructe_out_.write_all(b"\";\r\n        </script>\r\n        ")?;
if std::env::var("ROCKET_ENV").map(|env| env == "development").unwrap() {
_ructe_out_.write_all(b"\r\n            <script type=\"module\" src=\"http://localhost:8080/_dist_/index.js\"></script>\r\n            <script>window.HMR_WEBSOCKET_URL = \"ws://localhost:8080\"</script>\r\n        ")?;
} else {
_ructe_out_.write_all(b"\r\n            <script type=\"module\" src=\"/js/build/__dist__/index.js\"></script>\r\n        ")?;
}
_ructe_out_.write_all(b"\r\n    </body>\r\n</html>")?;
Ok(())
}
