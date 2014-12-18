# solvent
Solvent is a dependency resolver library written in rust.

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

    // You must set a target to resolve the dependencies of that target
    depgraph.set_target("a");

    // Iterate through each dependency.  The dependencies will be returned
    // in an order such that each output only depends on the previous
    // outputs (or nothing).  The target itself will be output last.
    for node in depgraph.satisfying_iter() {
        print!("{} ", node);
    }
}
```

The above will output:  `d b e c a`

You can also mark some elements as already satisfied, and the
iterator will take that into account:

```ignore
depgraph.mark_as_satisfied(["e","c"]);
```

The algorithm is deterministic, so while multiple sequences may
satisfy the dependency requirements, solvent will always yield the
same answer.

Dependency cycles are detected and will cause a panic!()

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
While elements (nodes) are registered as slices (&str) and slices of
slices (&[&str]), these borrows do not persist beyond the lifetime of
the register function call, as they are internally copied into Strings
and Vecs (and HashMaps).

Solvent does not yet handle boolean logic, e.g. `A` depends on `!B || B && !D`
but it is my intention to support boolean logic eventually, and I've worked
out how to do it in my head.  But as I haven't needed it for my schema
upgrade situation, I just haven't gotten around to it yet.

### TODO
* Boolean logic: A depends on !B || B && !D  (disjunctive normal form will
  likely be required at first)
* Node versioning, or the ability for library consumers to more easily
  manage node version such as gcc-4.7.4, gcc-4.8.3, gcc-4.9.2.
* Dependency filtering
* Compare to others:
** cargo/core/resolver/mod.rs
** PHP compoer's libsolver
** yum depsolve
** depresolve.go
