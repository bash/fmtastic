use core::fmt;

/// Formats a boolean as either a checked or unchecked ballot box.
/// ```
/// # use fmtastic::BallotBox;
/// assert_eq!("☑ Buy bread", format!("{} Buy bread", BallotBox(true)));
/// assert_eq!("☐ Do the dishes", format!("{} Do the dishes", BallotBox(false)));
/// assert_eq!("☒ Laundry", format!("{:#} Laundry", BallotBox(true)));
/// ```
///
/// ## Formatting Flags
/// ### Alternate `#`
/// By default a ballot box with a check (`☑`) is used.
/// The alternate flag `#` can be used to use a ballot box with an x instead (`☒`).
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct BallotBox(pub bool);

impl fmt::Display for BallotBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 && f.alternate() {
            write!(f, "☒")
        } else if self.0 {
            write!(f, "☑")
        } else {
            write!(f, "☐")
        }
    }
}

impl From<bool> for BallotBox {
    fn from(value: bool) -> Self {
        BallotBox(value)
    }
}
