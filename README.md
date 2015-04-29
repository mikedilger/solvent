# solvent
Solvent is a dependency resolver library written in rust.

[![Build Status](https://travis-ci.org/mikedilger/solvent.svg?branch=master)](https://travis-ci.org/mikedilger/solvent)

Documentation is available at https://mikedilger.github.io/solvent

Solvent helps you to resolve dependency orderings by building up a dependency
graph and then resolving the dependences of some target node in an order such
that each output depends only upon the previous outputs.

It is currently quite simple, but is still useful.

## Example

```rust
extern crate solvent;

use solvent::DepGraph;

fn main() {
    // Create a new empty DepGraph.  Must be `mut` or else it cannot
    // be used by the rest of the library.
    let mut depgraph: DepGraph = DepGraph::new();

    // You can register a dependency like this.  Solvent will
    // automatically create nodes for any term it has not seen before.
    // This means 'b' depends on 'd'
    depgraph.register_dependency("b","d");

    // You can also register multiple dependencies at once
    depgraph.register_dependencies("a",&["b","c","d"]);
    depgraph.register_dependencies("c",&["e"]);

    // Iterate through each dependency of "a".  The dependencies will be
    // returned in an order such that each output only depends on the
    // previous outputs (or nothing).  The target itself will be output
    // last.
    for node in depgraph.dependencies_of("a") {
        print!("{} ", node.unwrap());
    }
}
```

The above will output:  `d b e c a` or `e c d b a` or some other valid
dependency order.

The algorithm is not deterministic, and may give a different answer each
time it is run.  Beware.

The iterator dependencies_of() returns an `Option<Result<String,SolventError>>`.
The for loop handles the Option part for you, but you may want to check the
result for `SolventError`.  Once an error is returned, all subsequent calls to
the iterator next() will yield None.

You can also mark some elements as already satisfied, and the iterator
will take that into account:

```ignore
depgraph.mark_as_satisfied(["e","c"]);
```

Dependency cycles are detected and will return `SolventError::CycleDetected`.

## Use Cases
These kinds of calculations are useful in the following example situations:
* System package management: packages depending on other packages
* Build systems such as 'make' or 'cargo' to handle dependencies
  (note: neither cargo nor rustc use solvent)
* Complex software configurations such as Linux kernel configurations
* Database schema upgrades which don't need to be strictly sequential
  (e.g. multiple developers working on separate git branches being able
  to commit database schema upgrades independently, without merge
  conflicts) -- the author wrote solvent for this purpose.

## Other Details
While elements (nodes) are registered as slices (`&str`) and slices of
slices (`&[&str]`), these borrows do not persist beyond the lifetime of
the register function call, as they are internally copied into `String`s
and `Vec`s (and `HashMap`s).

Solvent does not yet handle boolean logic.  See issue [#1](https://github.com/mikedilger/solvent/issues/1).
