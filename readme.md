# fmtastic ✨

[![Docs](https://img.shields.io/docsrs/fmtastic/latest)](https://docs.rs/fmtastic)
[![Crate Version](https://img.shields.io/crates/v/fmtastic)](https://crates.io/crates/fmtastic)

A **fantastic**, `#![no_std]`-friendly crate for **fmt**ing numbers using the appropriate unicode characters via the [`Display`] trait. ✨
Format as vulgar fractions, super- and subscript and more.

Contributions are welcome for more formats.

## Features

### [Vulgar Fractions]
Creates beautiful unicode fractions like ¼ or ¹⁰⁄₃.
```rust
use fmtastic::VulgarFraction;

assert_eq!("¹⁰⁄₃", format!("{}", VulgarFraction::new(10, 3)));
assert_eq!("¼", format!("{}", VulgarFraction::new(1, 4)));
```

### Sub- and superscript
Formats integers as sub- or superscript.

```rust
use fmtastic::{Subscript, Superscript};

assert_eq!("x₁", format!("x{}", Subscript(1)));
assert_eq!("n²", format!("n{}", Superscript(2)));
```

### Roman Numerals
Formats unsigned integers as Roman numerals.

```rust
use fmtastic::Roman;

assert_eq!("ⅾⅽⅽⅼⅹⅹⅹⅰⅹ", format!("{:#}", Roman::new(789_u16).unwrap())); // lowercase
assert_eq!("ⅯⅯⅩⅩⅠⅤ", format!("{}", Roman::new(2024_u16).unwrap()));
assert_eq!("MMXXIV", format!("{}", Roman::new(2024_u16).unwrap().ascii())); // ascii
assert_eq!("ⅠⅠⅠ", format!("{}", Roman::from(3_u8))); // u8's can always be formatted as Roman numeral
```

### Seven-Segment Digits
Formats an unsigned integer using seven-segment digits
from the [Legacy Computing] block.

```rust
use fmtastic::Segmented;

assert_eq!("🯶🯲🯸", format!("{}", Segmented(628_u32)));
```

### Outlined
Formats an unsigned integer using outlined digits
from the [Legacy Computing Supplement] block.

```rust
use fmtastic::Outlined;

assert_eq!("𜳶𜳲𜳸", format!("{}", Outlined(628_u32)));
```

### Tally Marks
Formats an unsigned integer as tally marks.
```rust
use fmtastic::TallyMarks;

assert_eq!("𝍷𝍷𝍷", TallyMarks(3_u32).to_string());
assert_eq!("𝍸𝍸𝍷𝍷", TallyMarks(12_u32).to_string());
```

### Ballot Box
Formats a boolean as a ballot box.

```rust
use fmtastic::BallotBox;

assert_eq!("☑ Buy bread", format!("{} Buy bread", BallotBox(true)));
assert_eq!("☐ Do the dishes", format!("{} Do the dishes", BallotBox(false)));
assert_eq!("☒ Laundry", format!("{:#} Laundry", BallotBox(true)));
```

## [Docs](https://docs.rs/fmtastic)

## License
Licensed under either of

* Unlicense
  ([unlicense.txt](unlicense.txt) or <https://unlicense.org/>)
* Apache License, Version 2.0
  ([license-apache.txt](license-apache.txt) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
  ([license-mit.txt](license-mit.txt) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
multi-licensed as above, without any additional terms or conditions.


[Legacy Computing]: https://www.unicode.org/charts/PDF/U1FB00.pdf
[Legacy Computing Supplement]: https://www.unicode.org/charts/PDF/U1CC00.pdf
[Vulgar Fractions]: https://en.wikipedia.org/wiki/Fraction_(mathematics)#Simple,_common,_or_vulgar_fractions
[`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
