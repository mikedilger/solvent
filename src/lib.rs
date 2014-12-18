// Dependency Graph Library in Rust

//! dglr is a Dependency Graph Library in Rust.
//! In short, you register elements (nodes) and their dependencies,
//! set a target element, and then an iterator will give you a set
//! of elements in an order that satisfies the dependency requirement
//! of the target you specified.
//!
//! Elements are registered as &str strings.  You register them with
//! register_dependency() or register_dependencies().
//!
//! Dependenies are (currently) just a list of other elements that
//! the element in question depends on.  These are considered AND-ed
//! together.  Full boolean logic will be supported in the future.
//!
//! ```rust
//!  let mut depgraph: DepGraph = DepGraph::new();
//!  depgraph.register_dependencies("a",&["b","c","d"]);
//!  depgraph.register_dependency("b","d");
//!  depgraph.register_dependencies("c",&["e"]);
//!  depgraph.set_target("a");
//!  for node in depgraph.satisfying_iter() {
//!      print!("{} ", node);
//!  }
//! ```
//!
//! The above will output:  ```d b e c a```
//!
//! You can also mark some elements as already satisfied, and the
//! iterator will take that into account.
//!
//! ```
//! depgraph.mark_as_satisfied(["e","c"]);
//! ```
//!
//! The algorithm is deterministic, so while multiple sequences may
//! satify the dependency requirements, dglr will always answer with
//! the same one.
//!
//! Dependency cycles are detected and cause a panic!()

#![crate_name = "dglr"]
#![crate_type = "lib"]

// Required for log and rustdoc:
#![feature(phase)]
#[phase(plugin, link)]

extern crate log;

use std::collections::{HashMap,HashSet};
use std::collections::hash_map::{Occupied,Vacant};
use std::iter::{Iterator};
#[allow(unused_imports)]
use std::task;

/// This is the dependency graph.  It must be mutable, as the
/// library uses internal properties in the graph to do its
/// calculations.
pub struct DepGraph {
    /// List of dependencies.  Key is the element, values are the
    /// other elements that the key element depends upon.
    pub dependencies: HashMap<String,Vec<String>>,

    // (private) target we are trying to satisfy
    target: Option<String>,

    // (private) elements already satisfied
    satisfied: HashSet<String>,

    // (private) current path, for cycle detection
    curpath: HashSet<String>,
}

/// This iterates through the dependencies of the DepGraph's target
pub struct DepGraphIterator<'a> {
    depgraph: &'a mut DepGraph
}

/// This iterates through the dependencies of the DepGraph's target,
/// marking each element as satisfied as it is visited.
pub struct DepGraphSatisfyingIterator<'a> {
    depgraph: &'a mut DepGraph
}

impl DepGraph {

    /// Create an empty DepGraph.
    pub fn new() -> DepGraph
    {
        DepGraph {
            dependencies: HashMap::new(),
            target: None,
            curpath: HashSet::new(),
            satisfied: HashSet::new(),
        }
    }

    /// Add a dependency to a DepGraph.  The node does not need
    /// to pre-exist, nor do the dependency nodes.  But if the
    /// node does pre-exist, the depends_on will be added to its
    /// existing dependency list.
    pub fn register_dependency<'a>( &mut self,
                                node: &'a str,
                                depends_on: &'a str )
    {
        match self.dependencies.entry( String::from_str(node) ) {
            Vacant(entry) => { entry.set( vec![String::from_str(depends_on)] ); },
            Occupied(mut entry) => { (*entry.get_mut()).push(String::from_str(depends_on)); },
        }
    }

    /// Add multiple dependencies of one node to a DepGraph.  The
    /// node does not need to pre-exist, nor do the dependency elements.
    /// But if the node does pre-exist, the depends_on will be added
    /// to its existing dependency list.
    pub fn register_dependencies<'a>( &mut self,
                                  node: &'a str,
                                  depends_on: &'a[&'a str] )
    {
        let newvec: Vec<String> = depends_on.iter().map(
            |s| String::from_str(*s)).collect();

        match self.dependencies.entry( String::from_str(node) ) {
            Vacant(entry) => { entry.set( newvec.clone() ); },
            Occupied(mut entry) => { (*entry.get_mut()).push_all(newvec.as_slice()); },
        }
    }

    /// This sets the target node.  Iteratators on the graph always
    /// get the dependencies of the target node.
    pub fn set_target<'a>( &mut self, target: &'a str )
    {
        self.target = Some(String::from_str(target));
    }

    /// This marks a node as satisfied.  Iterators will not output
    /// such nodes.
    pub fn mark_as_satisfied<'a>( &mut self,
                                   nodes: &'a[&'a str] )
    {
        for node in nodes.iter() {
            self.satisfied.insert(String::from_str(*node));
        }
    }

    fn get_next_dependency(&mut self, node: &String) -> String
    {
        if self.curpath.contains(node) {
            panic!("Circular dependency graph at {}",node);
        }
        self.curpath.insert(node.clone());

        let deplist = match self.dependencies.get(node) {
            None => return node.clone(),
            Some(deplist) => deplist.clone() // ouch
        };

        for n in deplist.iter() {
            if self.satisfied.contains(n) {
                continue;
            }
            return self.get_next_dependency(n);
        }
        // nodes dependencies are satisfied
        node.clone()
    }

    /// Get an iterator to iterate through the dependencies of
    /// the target node
    pub fn iter<'a>(&'a mut self) -> DepGraphIterator<'a>
    {
        DepGraphIterator {
            depgraph: self
        }
    }

    /// Get an iterator to iterate through the dependencies of
    /// the target node, and also to mark those dependencies as
    /// satisfied as it goes.
    pub fn satisfying_iter<'a>(&'a mut self) -> DepGraphSatisfyingIterator<'a>
    {
        DepGraphSatisfyingIterator {
            depgraph: self
        }
    }
}

impl<'a> Iterator<String> for DepGraphIterator<'a> {
    fn next(&mut self) -> Option<String>
    {
        let node = match self.depgraph.target {
            None => return None,
            Some(ref node) => node.clone()
        };
        if self.depgraph.satisfied.contains(&node) {
            return None;
        }
        self.depgraph.curpath.clear();
        Some(self.depgraph.get_next_dependency(&node))
    }
}

impl<'a> Iterator<String> for DepGraphSatisfyingIterator<'a> {
    fn next(&mut self) -> Option<String>
    {
        let node = match self.depgraph.target {
            None => return None,
            Some(ref node) => node.clone()
        };
        if self.depgraph.satisfied.contains(&node) {
            return None;
        }
        self.depgraph.curpath.clear();
        let next = self.depgraph.get_next_dependency(&node);
        self.depgraph.mark_as_satisfied(&[next.as_slice()]);
        Some(next)
    }
}

#[test]
fn dglr_test_branching() {
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

    depgraph.set_target("a");

    let mut results: Vec<String> = Vec::new();

    loop {
        // detect infinite looping bugs
        assert!(results.len() < 30);

        let node = match depgraph.iter().next() {
            Some(x) => x,
            None => break,
        };
        depgraph.mark_as_satisfied([node.as_slice()]);
        results.push(node);
    }

    assert!( results == vec![String::from_str("d"),
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
                             String::from_str("c"),
                             String::from_str("a")] );
    //info!("Deps of a = {}",results);
}

#[test]
fn dglr_test_satisfying() {
    let mut depgraph: DepGraph = DepGraph::new();

    depgraph.register_dependencies("a",&["b","c","d"]);
    depgraph.register_dependency("b","d");
    depgraph.register_dependencies("c",&["e"]);

    depgraph.set_target("a");

    let mut results: Vec<String> = Vec::new();

    for node in depgraph.satisfying_iter() {
        // detect infinite looping bugs
        assert!(results.len() < 30);
        results.push(node);
    }

    assert!( results == vec![String::from_str("d"),
                             String::from_str("b"),
                             String::from_str("e"),
                             String::from_str("c"),
                             String::from_str("a")] );
    //info!("Deps of a = {}",results);
}

#[test]
fn dglr_test_circular() {

    let task_result = task::try(move|| {
        let mut depgraph: DepGraph = DepGraph::new();
        depgraph.register_dependency("a","b");
        depgraph.register_dependency("b","c");
        depgraph.register_dependency("c","a");
        depgraph.set_target("a");

        let mut results: Vec<String> = Vec::new();

        loop {
            // Detect infinite looping bugs by
            // breaking out successfully (successful
            // move|| means failed test!)
            if results.len() >= 30 { break; }

            let node = match depgraph.iter().next() {
                Some(x) => x,
                None => break,
            };
            depgraph.mark_as_satisfied([node.as_slice()]);
            results.push(node);
        }
    });
    match task_result {
        Ok(_) => panic!("Should have failed at the circular dependency!"),
        Err(_) => () //info!("Successfully detected the circular dependency (ignore task failure)"),
    };
}

#[test]
fn dglr_test_satisfied_stoppage() {

    let mut depgraph: DepGraph = DepGraph::new();
    depgraph.register_dependencies("superconn", []);
    depgraph.register_dependencies("owneruser", ["superconn"]);
    depgraph.register_dependencies("appuser", ["superconn"]);
    depgraph.register_dependencies("database", ["owneruser"]);
    depgraph.register_dependencies("ownerconn", ["database","owneruser"]);
    depgraph.register_dependencies("adminconn", ["database"]);
    depgraph.register_dependencies("extensions", ["database","adminconn"]);
    depgraph.register_dependencies("schema_table", ["database","ownerconn"]);
    depgraph.register_dependencies("schemas", ["ownerconn","extensions","schema_table","appuser"]);
    depgraph.register_dependencies("appconn", ["database","appuser","schemas"]);

    depgraph.set_target("appconn");
    depgraph.mark_as_satisfied(["owneruser","appuser"]);

    let mut results: Vec<String> = Vec::new();

    loop {
        assert!(results.len() < 30);

        let node = match depgraph.iter().next() {
            Some(x) => x,
            None => break,
        };
        depgraph.mark_as_satisfied([node.as_slice()]);
        results.push(node);
    }
    assert!( !results.contains(&String::from_str("superconn")) );
}
