This is a Dependency Graph Library written in Rust.

It is currently quite simple, but useful.

You use it like this:

1. Create a DepGraph
2. Register dependencies (nodes, and the nodes they depend on)
3. (optionally) mark some nodes as satisfied
4. Set a target node (the thing you want to get the dependencies of)
5. Iterate through the graph, and you will get in-order dependencies of the
   target.

These kinds of calculations are useful in the following example situations:
* System package management: packages depending on other packages
* Build systems such as 'make' or 'cargo' to handle dependencies
  (note: neither cargo nor rustc use dglr)
* Complex software configurations such as Linux kernel configurations
* Database schema upgrades which don't need to be strictly sequential
  (e.g. multiple developers working on separate git branches being able
  to commit database schema upgrades independently, without merge
  conflicts) -- the author wrote dlgr for this purpose.

For the technical "how to" details of using dglr, refer to any of the
test functions in lib.rs, or "cargo doc".

Elements (nodes) are registered from &str strings: `"B"`
Dependencies are registered from slices of &str strings: `&["B","C"]`
Internally, dglr copies this data into `String`s, `Vec`s, and `HashMap`s.

The algorithm is determinstic.  Even if multiple correct answers exist,
it will always yield the same one.

Circular dependency graphs are detected and will cause a `panic!`

dglr does not yet handle boolean logic, e.g. `A` depends on `(!B || (B && !D))`
but it is my intention to support boolean logic eventually, and I've worked
out how to do it in my head.  But as I haven't needed it for my schema
upgrade situation, I just haven't gotten around to it yet.

TODO:
* boolean logic
* optional dependencies and dependency filtering
* compare to cargo/core/resolver/mod.rs
