# Developer-facing Documentation About the Procedural Macros

_Enter here all ye who abandon hope_

This is a bit of a self-satisfied rant about how procedural macros are implemented
in this project. For what the macros do and why, please consult the main README.

Furthermore, note that any and all Rustdoc here is developer-facing due to the nature
of procedural macros. Hence, here, I:

1. Document what the procedural macro bits are and how they work
1. Document special trickery employed for this project

## Proc macro bits

_Here be dragons_

Like the dragon book I guess?

Procedural macros (proc macros) are macros that are complicated enough to warrant
their own functions or powerful enough to pollute your namespace. In this project,
we are more interested in the latter functionality since our macros define new
types (which regular Rust macros are forbidden to do). However, this special
functionality comes at a price: one this README is budgetting.

* Procedural macros allow the `lib.rs` to only export the functions that are actually
  the macros themselves, so this crate only exports one function right now. The rest
  are there because I like my sanity.
  * As a result, proc macros are in a separate crate that the main crate imports.
* Procedural macros are hard to test. The compiler only emits AST-related structures
  as types in the proc_macro crate, and it is difficult get those types unless you're
  `rustc`.
  * This is remedied by some special crates:
    * syn: for parsing
	* proc_macro2: for testing and some better types
	* quote: for code generation.
* Macros are run after lexing, so having stray quotes and the like is difficult (yes,
  this is why derivatives are denoted with an `@` right now, sorry Newton).
	
Note: the AST types are duplicated between the crates quite frequently and crates define
seemingly synonymous traits. _C'est la vie_, so allons. Just have the documentation handy.

## Special Trickery

* The `syn::Ident` type is pretty handy for our bespoke keywords.
  * You can fake spans for the keywords through `Ident`: if you've parsed an
    `Ident` and it's not the keyword you need, you can just emit its `Span` as the
	erroneous one and `rustc` will highlight the right bit of code for you.
* When implementing `ToTokens` for code-generation, you can pass a lot by just saying
  it's an `Ident`. Note that `Ident`'s constructor is OK with Rust keywords.
