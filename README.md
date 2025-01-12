# Giacrs

Bindings to the [Giac library](https://www-fourier.ujf-grenoble.fr/~parisse/giac_us.html) used in [xcas](https://www-fourier.ujf-grenoble.fr/~parisse/giac.html).

This library is under developpment, feels free to [contribute](#contributing) !

# Usage

```toml
[dependencies]
giacrs = "0.1.0"
```

There must be a giac library installed on your system:\
You can use `pacman -S giac` on arch.

# Examples

```rust
use giacrs::{
    context::{release_globals, Context},
    gen::Gen,
    GiacError,
};

fn main() -> Result<(), GiacError> {
    let ctx = Context::new();

    // Create a polynomial and factorize it
    let e = Gen::from_str("x^4", &ctx)?;
    let f = ctx.eval("x^5")?;
    let mut g = e.clone() * &f;
    g /= &e;
    println!("{}", g.factor(&ctx)?);

    // Compute the determinant of a matrix
    let mat = Gen::from_str("[[1,2],[3,4]]", &ctx)?;
    println!("{}", mat.det(&ctx)?.to_int()?);

    // Release giac globals variables to avoid memory leaks
    release_globals();
    Ok(())
}
```

# Contributing

See [CONTRIBUTING.md](https://github.com/supersurviveur/giacrs/blob/main/CONTRIBUTING.md)
