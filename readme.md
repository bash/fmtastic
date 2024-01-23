# fmtastic ✨

[![Docs](https://img.shields.io/docsrs/fmtastic/latest)](https://docs.rs/fmtastic)
[![Crate Version](https://img.shields.io/crates/v/fmtastic)](https://crates.io/crates/fmtastic)

A **fantastic** crate for **fmt**ing numbers using the appropriate unicode characters via the [`Display`] trait. ✨ \
Supports vulgar fractions, super- and subscript.

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

### Seven-Segment Digits
Formats an unsigned integer using seven-segment digits
from the [Legacy Computing] block.

```rust
use fmtastic::Segmented;

assert_eq!("🯶🯲🯸", format!("{}", Segmented(628_u32)));
```

### Tally Marks
Formats an unsigned integer as tally marks.
```rust
use fmtastic::TallyMarks;

assert_eq!("𝍷𝍷𝍷", TallyMarks(3_u32).to_string());
assert_eq!("𝍸𝍸𝍷𝍷", TallyMarks(12_u32).to_string());
```

[Legacy Computing]: https://www.unicode.org/charts/PDF/U1FB00.pdf

## [Docs](https://docs.rs/fmtastic)

## License
Licensed under either of

* Apache License, Version 2.0
  ([license-apache.txt](license-apache.txt) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
  ([license-mit.txt](license-mit.txt) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.


[Vulgar Fractions]: https://en.wikipedia.org/wiki/Fraction_(mathematics)#Simple,_common,_or_vulgar_fractions
[`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
