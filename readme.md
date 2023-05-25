# fmtastic ✨
A **fantastic** crate for **fmt**ing numbers using the appropriate unciode characters via the [`Display`] trait. ✨ \
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


[Vulgar Fractions]: https://en.wikipedia.org/wiki/Fraction_(mathematics)#Simple,_common,_or_vulgar_fractions
[`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
