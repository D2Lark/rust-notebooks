第一次写macro，整个cheat sheet

item: an item, like a function, struct, module, etc.
block: a block (i.e. a block of statments and/or an expression, surrounded by braces)
stmt: a statement
pat: a pattern
expr: an expression
ty: a type
ident: an identifier
path: a path (e.g. foo, ::std::mem::replace, transmute::<_, int>, ...)
meta: a meta item; the things that go inside #[...] and #![...] attributes
tt: a single token tree
Source: libsyntax/ext/tt/macro_parser.rs

There's one other thing to be aware of: in the interests of future-proofing the language, the compiler restricts what tokens you're allowed to put after a matcher, depending on what kind it is. Typically, this comes up when trying to match expressions or statements; those can only be followed by one of =>, ,, and ;.

Source libsyntax/ext/tt/macro_rules.rs