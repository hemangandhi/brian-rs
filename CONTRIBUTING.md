# Welcome!

So you want to contribute?!

Great!

There's a few things you should know before you try to:

1. Rust
1. Git
1. Github
1. Some software engineering terms (CI, unit testing, end-to-end testing)

Once you do, this should not be to hard. If you'd like to learn any of these by contributing here, feel free
to -- just know that the best way to get help is to ask, so if you want to add a feature, mention what you
don't know while requesting the feature (or requesting to contribute to the feature).

## The Github Layout

This repo will use issues, forks, and feature branches.

There will eventually be a CI action for the unit tests.

## The Repo Layout

Note: "crate" in this context means a directory with a `Cargo.toml` and a `src/` subdirectory under it with
Rust code.

This repo has two crates in it: the main Brian one, and the proc macro crate for the DSL. Many changes would
likely touch both crates. That is a misfortune we will live with.

The proc macros crate has its own docs which exist for developers (since users don't see anything but the
macros themselves).

The library structure in both repos, however, is quite flat, with all the files living under `src/`.

There are hopes to add a third crate consisting solely of examples (so an application, not a library).

## The Rust

Cargo does everything.

The tests live in the `lib.rs` files so far.
