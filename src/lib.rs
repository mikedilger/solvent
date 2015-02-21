//! Solvent is a dependency resolver library written in rust.
//!
//! Solvent helps you to resolve dependency orderings by building up a dependency
//! graph and then resolving the dependences of some target node in an order such
//! that each output depends only upon the previous outputs.
//!
//! It is currently quite simple, but is still useful.
//!
//! #Example
//!
//! ```rust
//! extern crate solvent;
//!
//! use solvent::DepGraph;
//!
//! fn main() {
//!     // Create a new empty DepGraph.  Must be `mut` or else it cannot
//!     // be used by the rest of the library.
//!     let mut depgraph: DepGraph = DepGraph::new();
//!
//!     // You can register a dependency like this.  Solvent will
//!     // automatically create nodes for any term it has not seen before.
//!     // This means 'b' depends on 'd'
//!     depgraph.register_dependency("b","d");
//!
//!     // You can also register multiple dependencies at once
//!     depgraph.register_dependencies("a",&["b","c","d"]);
//!     depgraph.register_dependencies("c",&["e"]);
//!
//!     // Iterate through each dependency of "a".  The dependencies will be
//!     // returned in an order such that each output only depends on the
//!     // previous outputs (or nothing).  The target itself will be output
//!     // last.
//!     for node in depgraph.dependencies_of("a") {
//!         match node {
//!             Ok(n) => print!("{} ", n),
//!             Err(e) => panic!("Solvent error detected: {:?}", e),
//!         }
//!     }
//! }
//! ```
//!
//! The above will output:  `d b e c a` or `e c d b a` or some other valid
//! dependency order.
//!
//! The algorithm is not deterministic, and may give a different answer each
//! time it is run.  Beware.
//!
//! The iterator dependencies_of() returns an Option<Result<String,SolventError>>.
//! The for loop handles the Option part for you, but you may want to check the
//! result for SolventErrors.  Once an error is returned, all subsequent calls to
//! the iterator next() will yield None.
//!
//! You can also mark some elements as already satisfied, and the iterator
//! will take that into account:
//!
//! ```ignore
//! depgraph.mark_as_satisfied(["e","c"]);
//! ```
//!
//! Dependency cycles are detected and will return SolventError::CycleDetected.

#![crate_name = "solvent"]
#![crate_type = "lib"]

// Silence warnings about unstable library features
#![feature(std_misc)]

#[macro_use] extern crate log;

use std::collections::{HashMap,HashSet};
use std::collections::hash_map::Entry;
use std::iter::{Iterator};
use std::borrow::ToOwned;

/// This is the dependency graph.
#[derive(Clone)]
pub struct DepGraph {
    /// List of dependencies.  Key is the element, values are the
    /// other elements that the key element depends upon.
    pub dependencies: HashMap<String,HashSet<String>>,

    /// Nodes already satisfied.  dependencies_of() will prune
    /// dependency searches at these nodes, and not output nodes
    /// registered here.
    pub satisfied: HashSet<String>,
}

#[derive(Copy,Debug,PartialEq)]
pub enum SolventError {
    CycleDetected,
    // TODO once we implement conflicts: Conflict(String)
}

/// This iterates through the dependencies of the DepGraph's target
pub struct DepGraphIterator<'a> {
    depgraph: &'a DepGraph,

    // Target we are trying to satisfy
    target: String,

    // Elements already satisfied during this iterator's walk
    satisfied: HashSet<String>,

    // Current path, for cycle detection
    curpath: HashSet<String>,

    // Halted.  Used so that it can return None after an Err is returned.
    halted: bool,
}

impl DepGraph {

    /// Create an empty DepGraph.
    pub fn new() -> DepGraph
    {
        DepGraph {
            dependencies: HashMap::new(),
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
        match self.dependencies.entry( node.to_string() ) {
            Entry::Vacant(entry) => {
                let mut deps = HashSet::with_capacity(1);
                deps.insert( depends_on.to_string() );
                entry.insert( deps );
            },
            Entry::Occupied(mut entry) => {
                (*entry.get_mut()).insert( depends_on.to_string() );
            },
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
        match self.dependencies.entry( node.to_string() ) {
            Entry::Vacant(entry) => {
                let mut deps = HashSet::with_capacity( depends_on.len() );
                for s in depends_on.iter() {
                    deps.insert( s.to_string() );
                }
                entry.insert( deps );
            },
            Entry::Occupied(mut entry) => {
                for s in depends_on.iter() {
                    (*entry.get_mut()).insert( s.to_string() );
                }
            },
        }
    }

    /// This marks a node as satisfied.  Iterators will not output
    /// such nodes.
    pub fn mark_as_satisfied<'a>( &mut self,
                                   nodes: &'a[&'a str] )
    {
        for node in nodes.iter() {
            self.satisfied.insert( node.to_string() );
        }
    }

    /// Get an iterator to iterate through the dependencies of
    /// the target node.
    pub fn dependencies_of<'a>(&'a self, target: &str) -> DepGraphIterator<'a>
    {
        // TODO: iterator's satisfied could start empty, and all checks
        //       could separately check depgraph's and iterator's.  That
        //       would avoid the copy.
        DepGraphIterator {
            depgraph: self,
            target: target.to_owned(),
            satisfied: self.satisfied.clone(),
            curpath: HashSet::new(),
            halted: false,
        }
    }
}

impl<'a> DepGraphIterator<'a> {

    fn get_next_dependency(&mut self, node: &String) -> Result<String,SolventError>
    {
        if self.curpath.contains(node) {
            return Err(SolventError::CycleDetected);
        }
        self.curpath.insert(node.clone());

        let deplist = match self.depgraph.dependencies.get(node) {
            None => return Ok(node.clone()),
            Some(deplist) => deplist.clone() // ouch
        };

        for n in deplist.iter() {
            // Prune satisfied nodes
            if self.satisfied.contains(n) {
                continue;
            }

            return self.get_next_dependency(n);
        }
        // nodes dependencies are satisfied
        Ok(node.clone())
    }
}

impl<'a> Iterator for DepGraphIterator<'a> {
    type Item = Result<String,SolventError>;

    /// Get next dependency.  Returns None when finished.  If
    /// Some(Err(SolventError)) occurs, all subsequent calls will
    /// return None.
    fn next(&mut self) -> Option< Result<String,SolventError> >
    {
        if self.halted {
            return None;
        }

        let node = self.target.clone();
        if self.satisfied.contains(&node) {
            return None;
        }

        self.curpath.clear();
        let next = match self.get_next_dependency(&node) {
            Ok(d) => d,
            Err(e) => {
                self.halted = true;
                return Some(Err(e));
            }
        };
        self.satisfied.insert(next.clone());
        Some(Ok(next))
    }
}

#[test]
fn solvent_test_branching() {
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

    let mut results: Vec<String> = Vec::new();

    for node in depgraph.dependencies_of("a") {
        // detect infinite looping bugs
        assert!(results.len() < 30);

        let n = match node {
            Err(e) => panic!("Solvent error detected: {:?}", e),
            Ok(n) => n,
        };

        // Check that all of that nodes dependencies have already been output
        let deps: Option<&HashSet<String>> = depgraph.dependencies.get(&n);
        if deps.is_some() {
            for dep in deps.unwrap().iter() {
                assert!( results.contains(dep) );
            }
        }

        results.push(n.clone());
    }

    // Be sure we actually output enough stuff
    assert!(results.len() == 14);

    // Be sure each output is unique
    for result in results.iter() {
        let mut count: usize = 0;
        for result2 in results.iter() {
            if result == result2 { count = count + 1; }
        }
        assert!(count == 1);
    }
}

#[test]
fn solvent_test_updating_dependencies() {
    let mut depgraph: DepGraph = DepGraph::new();

    depgraph.register_dependencies("a",&["b","c"]);
    depgraph.register_dependency("a","d");
    assert!(depgraph.dependencies.get("a").unwrap().contains("b"));
    assert!(depgraph.dependencies.get("a").unwrap().contains("c"));
    assert!(depgraph.dependencies.get("a").unwrap().contains("d"));
}

#[test]
fn solvent_test_circular() {

    let mut depgraph: DepGraph = DepGraph::new();
    depgraph.register_dependency("a","b");
    depgraph.register_dependency("b","c");
    depgraph.register_dependency("c","a");

    for node in depgraph.dependencies_of("a") {
        assert!(node.is_err());
        assert!(node.unwrap_err() == SolventError::CycleDetected);
    }
}

#[test]
fn solvent_test_satisfied_stoppage() {

    let mut depgraph: DepGraph = DepGraph::new();
    depgraph.register_dependencies("superconn", &[]);
    depgraph.register_dependencies("owneruser", &["superconn"]);
    depgraph.register_dependencies("appuser", &["superconn"]);
    depgraph.register_dependencies("database", &["owneruser"]);
    depgraph.register_dependencies("ownerconn", &["database","owneruser"]);
    depgraph.register_dependencies("adminconn", &["database"]);
    depgraph.register_dependencies("extensions", &["database","adminconn"]);
    depgraph.register_dependencies("schema_table", &["database","ownerconn"]);
    depgraph.register_dependencies("schemas", &["ownerconn","extensions","schema_table","appuser"]);
    depgraph.register_dependencies("appconn", &["database","appuser","schemas"]);

    depgraph.mark_as_satisfied(&["owneruser","appuser"]);

    let mut results: Vec<String> = Vec::new();

    for node in depgraph.dependencies_of("appconn") {
        assert!(results.len() < 30);
        match node {
            Ok(n) => results.push(n),
            Err(e) => panic!("Solvent error detected: {:?}",e),
        };
    }

    // Be sure we did not depend on these
    assert!( !results.contains( &"appuser".to_string()) );
    assert!( !results.contains( &"owneruser".to_string()) );
    assert!( !results.contains( &"superconn".to_string()) );

    // Be sure we actually output enough stuff
    assert!(results.len() == 7);

    // Be sure each output is unique
    for result in results.iter() {
        let mut count: usize = 0;
        for result2 in results.iter() {
            if result == result2 { count = count + 1; }
        }
        assert!(count == 1);
    }
}

