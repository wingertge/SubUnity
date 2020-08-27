use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use crate::User;

pub fn profile_html<W>(mut _ructe_out_: &mut W, user: User, profile_picture: &str) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
_ructe_out_.write_all(b"<html lang=\"en\">\r\n    <body>\r\n        <p>")?;
user.username.to_html(&mut _ructe_out_)?;
_ructe_out_.write_all(b"</p>\r\n    </body>\r\n</html>")?;
Ok(())
}
