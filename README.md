# antic-sys

Rust bindings to the [Antic](https://github.com/wbhart/antic) library.

Antic is an algebraic number theory library.

## Notes

`Antic-sys` has `flint-sys` as a dependency. At the moment, `flint-sys` requires the user to have [Flint](https://flintlib.org/doc/index.html) installed and accesible in your `$PATH`.

This crate will build Antic from source so it is not required to be pre-installed. This process is still experimental so please report any issues. Currently the build process is only tested on Ubuntu.

