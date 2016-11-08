# colorstring.rs [![Build Status](https://travis-ci.org/ubnt-intrepid/colorstring.rs.svg?branch=master)](https://travis-ci.org/ubnt-intrepid/colorstring.rs)

`colorstring.rs` is an unofficial port of mitchellh's [colorstring](https://github.com/mitchellh/colorstring), written in Rust.

## Examples

```rust
extern crate colorstring;
use colorstring::Colorize;

let c = Colorize::new();
println!("{}", c.color("[blue]Hello, [blue]world"));
```


## License
This software is under the MIT license.
See [LICENSE](LICENSE) for details.
