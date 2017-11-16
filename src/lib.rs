//! Solvent is a dependency resolver library written in rust.
//!
//! Solvent helps you to resolve dependency orderings by building up a dependency graph and then
//! resolving the dependences of some target node in an order such that each output depends only
//! upon the previous outputs.
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
//!     // Create a new empty DepGraph.
//!     let mut depgraph: DepGraph<&str> = DepGraph::new();
//!
//!     // You can register a dependency like this. Solvent will automatically create nodes for
//!     // any term it has not seen before. This means 'b' depends on 'd'
//!     depgraph.register_dependency("b","d");
//!
//!     // You can also register multiple dependencies at once
//!     depgraph.register_dependencies("a",vec!["b","c","d"]);
//!     depgraph.register_dependencies("c",vec!["e"]);
//!
//!     // Iterate through each dependency of "a". The dependencies will be returned in an order
//!     // such that each output only depends on the previous outputs (or nothing). The target
//!     // itself will be output last.
//!     for node in depgraph.dependencies_of(&"a").unwrap() {
//!         match node {
//!             Ok(n) => print!("{} ", n),
//!             Err(e) => panic!("Solvent error detected: {:?}", e),
//!         }
//!     }
//! }
//! ```
//!
//! The above will output:  `d b e c a` or `e c d b a` or some other valid dependency order.
//!
//! The algorithm is not deterministic, and may give a different answer each time it is run. Beware.
//!
//! The iterator dependencies_of() returns an `Option<Result<T ,SolventError>>`.  The for loop
//! handles the `Option` part for you, but you may want to check the result for `SolventError`s.
//! Once an error is returned, all subsequent calls to the iterator `next()` will yield `None`.
//!
//! You can also mark some nodes as already satisfied, and the iterator will take that into
//! account
//!
//! ```ignore
//! depgraph.mark_as_satisfied(["e","c"]).unwrap();
//! ```
//!
//! Dependency cycles are detected and will return `SolventError::CycleDetected`.

pub mod error;
pub use error::SolventError;

use std::collections::{HashMap,HashSet};
use std::collections::hash_map::Entry;
use std::iter::{Iterator};

/// This is the dependency graph. The type `T` is intended to be a small type, or a
/// reference to a larger type that implements `Eq` (you will need to supply the type
/// and vectors of the type to functions).
#[derive(Debug,Clone)]
pub struct DepGraph<T: Eq> {
    // The nodes in the graph.  Each one is assigned a unique number.
    nodes: Vec<T>,

    // List of dependencies. The first node depends on the set of additional nodes.
    // We store indices into the nodes array.  This way we can have Eq + Copy + Hash
    // without any requirements on type T.
    dependencies: HashMap<usize, HashSet<usize>>,

    // The set of nodes already satisfied (by index into the nodes array).
    satisfied: HashSet<usize>,
}

impl<T: Eq> DepGraph<T> {

    /// Create an empty DepGraph.
    pub fn new() -> DepGraph<T> {
        DepGraph {
            nodes: Vec::new(),
            dependencies: HashMap::new(),
            satisfied: HashSet::new(),
        }
    }

    fn _pos(&self, node: &T) -> Option<usize> {
        self.nodes.iter().position(|x| x==node)
    }

    fn _register_node(&mut self, node: T) -> usize {
        match self._pos(&node) {
            Some(pos) => pos,
            None => {
                self.nodes.push(node);
                self.nodes.len() - 1
            }
        }
    }

    /// Register nodes in the graph. The `nodes` are added to any existing nodes,
    /// after checking to avoid duplicates.
    pub fn register_nodes(&mut self, mut nodes: Vec<T>) {
        for node in nodes.drain(..) {
            self.register_node(node);
        }
    }

    /// Register a node in the graph. The `node` is added to any existing nodes,
    /// after checking to avoid duplicates.
    pub fn register_node(&mut self, node: T) {
        self._register_node(node);
    }

    /// Add a dependency to a DepGraph. The node does not need to pre-exist, nor does the
    /// dependency node. If the node does pre-exist, the depends_on will be added to
    /// its existing dependency list. Otherwise it will be created.
    pub fn register_dependency(&mut self, node: T, depends_on: T) {

        let node_pos = self._register_node(node);
        let dep_pos = self._register_node(depends_on);

        match self.dependencies.entry( node_pos ) {
            Entry::Vacant(entry) => {
                let mut deps = HashSet::with_capacity(1);
                deps.insert( dep_pos );
                entry.insert( deps );
            },
            Entry::Occupied(mut entry) => {
                (*entry.get_mut()).insert( dep_pos );
            },
        }
    }

    /// Add multiple dependencies of one node to a DepGraph. The node does not need to
    /// pre-exist, nor does the dependency node. If the node does pre-exist, the
    /// depends_on will be added to its existing dependency list. Otherwise it will
    /// be created.
    pub fn register_dependencies(&mut self, node: T, mut depends_on: Vec<T>)
    {
        let node_pos = self._register_node(node);

        let mut dep_poses: Vec<usize> = Vec::new();
        for dp in depends_on.drain(..) {
            let pos = self._register_node(dp);
            dep_poses.push(pos);
        }

        match self.dependencies.entry( node_pos ) {
            Entry::Vacant(entry) => {
                let mut deps: HashSet<usize> = HashSet::with_capacity( dep_poses.len() );
                for pos in dep_poses.iter() {
                    deps.insert( *pos );
                }
                entry.insert( deps );
            },
            Entry::Occupied(mut entry) => {
                for pos in dep_poses.iter() {
                    (*entry.get_mut()).insert( *pos );
                }
            },
        }
    }

    /// This marks a node as satisfied. Iterators will not output such nodes. Nodes
    /// must exist.
    pub fn mark_as_satisfied(&mut self, nodes: &[T]) -> Result<(), SolventError>
    {
        for node in nodes.iter() {
            let node_pos = match self._pos(node) {
                None => return Err(SolventError::NoSuchNode),
                Some(pos) => pos
            };

            self.satisfied.insert( node_pos );
        }

        Ok(())
    }

    /// Get an iterator to iterate through the dependencies of the target node. Target
    /// node must exist.
    pub fn dependencies_of<'a>(&'a self, target: &T) -> Result<DepGraphIterator<'a, T>,
                                                               SolventError>
    {
        let pos = match self._pos(target) {
            None => return Err(SolventError::NoSuchNode),
            Some(p) => p
        };

        Ok(DepGraphIterator {
            depgraph: self,
            target: pos,
            satisfied: self.satisfied.clone(),
            curpath: HashSet::new(),
            halted: false,
        })
    }
}

/// This iterates through the dependencies of the DepGraph's target
pub struct DepGraphIterator<'a, T: Eq + 'a> {
    depgraph: &'a DepGraph<T>,

    // Target we are trying to satisfy
    target: usize,

    // Node positions already satisfied during this iterator's walk
    satisfied: HashSet<usize>,

    // Current path, for cycle detection
    curpath: HashSet<usize>,

    // Halted.  Used so that it can return None after an Err is returned.
    halted: bool,
}

impl<'a, T: Eq> DepGraphIterator<'a, T> {

    fn get_next_dependency(&mut self, pos: usize) -> Result<usize, SolventError>
    {
        if self.curpath.contains(&pos) {
            return Err(SolventError::CycleDetected);
        }
        self.curpath.insert(pos);

        let deplist = match self.depgraph.dependencies.get(&pos) {
            None => return Ok(pos),
            Some(deplist) => deplist
        };

        for n in deplist.iter() {
            // Prune satisfied nodes
            if self.satisfied.contains(n) {
                continue;
            }

            return self.get_next_dependency(*n);
        }

        // nodes dependencies are satisfied
        Ok(pos)
    }
}

impl<'a, T: Eq> Iterator for DepGraphIterator<'a, T> {
    type Item = Result<&'a T, SolventError>;

    // Get next dependency.  Returns None when finished.  If Some(Err(SolventError)) occurs,
    // all subsequent calls will return None.
    fn next(&mut self) -> Option<Result<&'a T, SolventError>>
    {
        if self.halted {
            return None;
        }

        let npos = self.target;
        if self.satisfied.contains(&npos) {
            self.halted = true;
            return None;
        }

        self.curpath.clear();
        let next = match self.get_next_dependency(npos) {
            Ok(d) => d,
            Err(e) => {
                self.halted = true;
                return Some(Err(e));
            }
        };
        self.satisfied.insert(next);
        Some(Ok(&self.depgraph.nodes[next]))
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use super::DepGraph;
    use super::SolventError;


    #[test]
    fn solvent_test_branching() {
        let mut depgraph: DepGraph<&str> = DepGraph::new();

        depgraph.register_nodes(vec!["a","b","c","d","e","f","g","h","i","j","k","l","m","n"]);

        depgraph.register_dependencies("a", vec!["b","c","d"]);
        depgraph.register_dependency("b","d");
        depgraph.register_dependencies("c", vec!["e","m","g"]);
        depgraph.register_dependency("e","f");
        depgraph.register_dependency("g","h");
        depgraph.register_dependency("h","i");
        depgraph.register_dependencies("i", vec!["j","k"]);
        depgraph.register_dependencies("k", vec!["l","m"]);
        depgraph.register_dependency("m","n");

        let mut results: Vec<&str> = Vec::new();

        for node in depgraph.dependencies_of(&"a").unwrap() {
            // detect infinite looping bugs
            assert!(results.len() < 30);

            let n = match node {
                Err(e) => panic!("Solvent error detected: {:?}", e),
                Ok(n) => n,
            };

            // Check that all of that nodes dependencies have already been output
            let pos = depgraph._pos(&n).unwrap();
            let deps: Option<&HashSet<usize>> = depgraph.dependencies.get(&pos);
            if deps.is_some() {
                for dep in deps.unwrap().iter() {
                    assert!( results.contains(&depgraph.nodes[*dep]) );
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
        let mut depgraph: DepGraph<&str> = DepGraph::new();

        depgraph.register_dependencies("a",vec!["b","c"]);
        depgraph.register_dependency("a","d");
        assert!(depgraph.dependencies.get(&0).unwrap().contains(&1));
        assert!(depgraph.dependencies.get(&0).unwrap().contains(&2));
        assert!(depgraph.dependencies.get(&0).unwrap().contains(&3));
    }

    #[test]
    fn solvent_test_circular() {

        let mut depgraph: DepGraph<&str> = DepGraph::new();
        depgraph.register_dependency("a","b");
        depgraph.register_dependency("b","c");
        depgraph.register_dependency("c","a");

        for node in depgraph.dependencies_of(&"a").unwrap() {
            assert!(node.is_err());
            assert!(node.unwrap_err() == SolventError::CycleDetected);
        }
    }

    #[test]
    fn solvent_test_satisfied_stoppage() {

        let mut depgraph: DepGraph<&str> = DepGraph::new();
        depgraph.register_dependencies("superconn", vec![]);
        depgraph.register_dependencies("owneruser", vec!["superconn"]);
        depgraph.register_dependencies("appuser", vec!["superconn"]);
        depgraph.register_dependencies("database", vec!["owneruser"]);
        depgraph.register_dependencies("ownerconn", vec!["database","owneruser"]);
        depgraph.register_dependencies("adminconn", vec!["database"]);
        depgraph.register_dependencies("extensions", vec!["database","adminconn"]);
        depgraph.register_dependencies("schema_table", vec!["database","ownerconn"]);
        depgraph.register_dependencies("schemas", vec!["ownerconn","extensions","schema_table","appuser"]);
        depgraph.register_dependencies("appconn", vec!["database","appuser","schemas"]);

        depgraph.mark_as_satisfied(&["owneruser","appuser"]).unwrap();
        assert_eq!( depgraph.satisfied.len() , 2 );

        let mut results: Vec<&str> = Vec::new();

        for node in depgraph.dependencies_of(&"appconn").unwrap() {
            assert!(results.len() < 30);
            match node {
                Ok(n) => results.push(n),
                Err(e) => panic!("Solvent error detected: {:?}",e),
            };
        }

        // Be sure we did not depend on these
        assert!( !results.contains( &"appuser") );
        assert!( !results.contains( &"owneruser") );
        assert!( !results.contains( &"superconn") );

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
}
