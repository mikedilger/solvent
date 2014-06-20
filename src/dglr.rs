// Dependency Graph Library in Rust

//! dglr is a Dependency Graph Library in Rust.
//! In short, you register elements and their dependencies, and then
//! ask for the dependencies of any element in an order that satisfies.
//!
//! Elements are simply &str strings.  You register them with
//! register_dependency() or register_dependencies().
//!
//! Dependenies are (currently) just a list of other elements that
//! the element in question depends on.  [In the future, we may add
//! boolean logic, but currently they are all AND-ed together].
//!
//! Then call get_ordered_dependencies_of() and the library does the
//! magic and returns an order which will work.  It is possible that
//! other orders also work, but the library's algorithm is
//! deterministic, so you'll always get the same particular one.

#![crate_id = "dglr#0.1"]
#![crate_type = "lib"]

// Required for log and rustdoc:
#![feature(phase)]
#[phase(plugin, link)]

extern crate log;

use std::collections::hashmap::{HashMap,HashSet};

/// This is the dependency graph.  You can create it yourself, or
/// use the convenience methods of new(), register_dependency() and
/// register_dependencies() to build one up (that must be mutable).
pub struct DepGraph {
    /// List of dependencies.  Key is the element, values are the
    /// other elements that the key element depends upon.
    pub dependencies: HashMap<String,Vec<String>>,
}

// Internal structure keeping state between recursive functions
struct WalkState {
    // The path we have currently walked
    curpath: HashSet<String>,
    // The output so far
    output: Vec<String>,
}

impl DepGraph {

    /// Create an empty DepGraph.
    pub fn new() -> DepGraph
    {
        DepGraph {
            dependencies: HashMap::new(),
        }
    }

    /// Add a dependency to a DepGraph.  The thing does not need
    /// to pre-exist, nor do the dependency elements.  But if the
    /// thing does pre-exist, the dependsOn will be added to its
    /// existing dependency list.
    pub fn register_dependency<'a>( &mut self,
                                thing: &'a str,
                                dependsOn: &'a str )
    {
        self.dependencies.insert_or_update_with(
            String::from_str(thing),
            vec![String::from_str(dependsOn)],
            |_,v| { v.push(String::from_str(dependsOn)); }
            );
    }

    /// Add multiple dependencies of one thing to a DepGraph.  The
    /// thing does not need to pre-exist, nor do the dependency elements.
    /// But if the thing does pre-exist, the dependsOn will be added
    /// to its existing dependency list.
    pub fn register_dependencies<'a>( &mut self,
                                  thing: &'a str,
                                  dependsOn: &'a[&'a str] )
    {
        let newvec: Vec<String> = dependsOn.iter().map(
            |s| String::from_str(*s)).collect();

        self.dependencies.insert_or_update_with(
            String::from_str(thing),
            newvec.clone(),
            |_,v| { v.push_all(newvec.as_slice()); }
            );
    }

    /// This returns either a vector of elements (Strings) in the
    /// order which they must be resolved as dependencies of thing,
    /// or in the case where there is a dependency cycle, None is
    /// returned.
    pub fn get_ordered_dependencies_of(&self, thing: &str)
        -> Option<Vec<String>>
    {
        let mut state = WalkState {
            curpath: HashSet::new(),
            output: Vec::new(),
        };

        debug!("Recursing for the first time, with {}",thing);
        if ! self.get_deps_of_recurse(&String::from_str(thing), &mut state)
        {
            return None;
        }

        debug!("output is {}",state.output);
        Some(state.output)
    }

    // Internal function, recursion for get_ordered_dependencies_of
    fn get_deps_of_recurse(&self, thing: &String, state: &mut WalkState)
        -> bool
    {
        // If we find thing, we have a circular dependency:
        if state.curpath.contains(thing) {
            error!("Circular dependency detected at {}", thing);
            return false;
        }

        state.curpath.insert(thing.clone());

        match self.dependencies.find(thing) {
            None => {
                debug!("{} has no dependencies",thing);
            },

            Some(deplist) => {
                debug!("Handling the dependencies of {}",thing);
                for n in deplist.iter() {
                    // If thing was not yet visited, recurse into it
                    if !state.output.contains(n) {

                        debug!("Recursing for {}",n);
                        if ! self.get_deps_of_recurse(n, state) {
                            return false;
                        }

                        debug!("Appending {} to output",n);
                        state.output.push(n.clone());
                    }
                }
            },
        }

        state.curpath.remove(thing);

        return true;
    }
}

#[test]
fn dglr_test() {
    let mut depgraph: DepGraph = DepGraph::new();

    depgraph.register_dependencies("a",&["b","c","d"]);
    depgraph.register_dependency("b","d");
    depgraph.register_dependencies("c",&["e","m","g"]);
    depgraph.register_dependency("e","f");
    depgraph.register_dependency("g","h");
    depgraph.register_dependency("h","i");
    depgraph.register_dependencies("i",&["j","k"]);
    depgraph.register_dependencies("k",&["l","m"]);
    depgraph.register_dependency("m","n");

    let deps = depgraph.get_ordered_dependencies_of("a").unwrap();
    debug!("Deps of a = {}",deps);
    assert!( deps == vec![String::from_str("d"),
                          String::from_str("b"),
                          String::from_str("f"),
                          String::from_str("e"),
                          String::from_str("n"),
                          String::from_str("m"),
                          String::from_str("j"),
                          String::from_str("l"),
                          String::from_str("k"),
                          String::from_str("i"),
                          String::from_str("h"),
                          String::from_str("g"),
                          String::from_str("c")] );


    let deps2 = depgraph.get_ordered_dependencies_of("i").unwrap();
    debug!("Deps of i = {}",deps2);
    assert!( deps2 == vec![String::from_str("j"),
                           String::from_str("l"),
                           String::from_str("n"),
                           String::from_str("m"),
                           String::from_str("k")] );

    depgraph.register_dependency("i","g");
    let deps3 = depgraph.get_ordered_dependencies_of("a");
    assert!(deps3 == None);
    debug!("Circular dependency was detected.");
}
