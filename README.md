# Tap Trait

[![Test](https://github.com/KSXGitHub/tap-trait/workflows/Test/badge.svg)](https://github.com/KSXGitHub/tap-trait/actions?query=workflow%3ATest)
[![Crates.io Version](https://img.shields.io/crates/v/tap-trait?logo=rust)](https://crates.io/crates/tap-trait)

Inspect and mutate values without leaving the method chain.

## Example

```rust
use tap_trait::Tap;
use pipe_trait::Pipe;
let result = 2i32
    .tap(|x| assert_eq!(x, 2))
    .tap_mut(|x| *x += 1)
    .tap(|x| assert_eq!(x, 3))
    .tap_mut(|x| *x *= 3)
    .tap(|x| assert_eq!(x, 9))
    .pipe(|x| -x)
    .tap(|x| assert_eq!(x, -9))
    .pipe_ref(ToString::to_string)
    .tap_ref(|x| assert_eq!(x, "-9"))
    .tap_mut(|x| *x += ".0")
    .tap_ref(|x| assert_eq!(x, "-9.0"));
assert_eq!(result, "-9.0");
```

## License

[MIT](https://git.io/J4otf) © [Hoàng Văn Khải](https://ksxgithub.github.io/).
