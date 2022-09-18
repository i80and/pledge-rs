This helper generates `Promise` and its variants from the pledge(2) manual pages
hosted at [man.openbsd.org](https://man.openbsd.org). To update the generated
code when there’s a new OpenBSD release (run this from the repository root):

    ( cd variants; cargo run > ../src/promise.rs )

Unlike the library itself, this program is compatible with Rust 1.56.0+ only.
