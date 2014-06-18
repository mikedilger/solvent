// Dependency Graph Library in Rust
#![crate_type = "lib"]

// For log related macros:
#![feature(phase)]
#[phase(plugin, link)]
extern crate log;

use std::collections::hashmap::{HashMap,HashSet};

pub struct DepGraph {
    pub dependencies: HashMap<String,Vec<String>>,
}

struct WalkState {
    remaining: HashSet<String>,
    output: Vec<String>,
}

impl DepGraph {

    pub fn new() -> DepGraph
    {
        DepGraph {
            dependencies: HashMap::new(),
        }
    }

    pub fn add_dependency<'a>( &mut self,
                                thing: &'a str,
                                dependsOn: &'a str )
    {
        self.dependencies.insert_or_update_with(
            String::from_str(thing),
            vec![String::from_str(dependsOn)],
            |_,v| { v.push(String::from_str(dependsOn)); }
            );
    }

    pub fn add_dependencies<'a>( &mut self,
                                  thing: &'a str,
                                  dependsOn: Vec<&'a str> )
    {
        let newvec: Vec<String> = dependsOn.iter().map(
            |s| String::from_str(*s)).collect();

        self.dependencies.insert_or_update_with(
            String::from_str(thing),
            newvec.clone(),
            |_,v| { v.push_all(newvec.as_slice()); }
            );
    }

    pub fn get_ordered_dependencies_of(&self, thing: &str)
        -> Vec<String>
    {
        let mut state = WalkState {
            output: Vec::new(),
            remaining: HashSet::new(),
        };

        // Scan the dependencies, and save all nodes in remaining
        for (n, dependsOn) in self.dependencies.iter() {
            debug!("preparing {}",n);
            state.remaining.insert(n.clone());
            for n2 in dependsOn.iter() {
                debug!("preparing {}",n2);
                state.remaining.insert(n2.clone());
            }
        }

        debug!("Recursing for the first time, with {}",thing);
        self.get_deps_of_recurse(&String::from_str(thing), &mut state);
        debug!("output is {}",state.output);
        state.output
    }

    fn get_deps_of_recurse(&self, thing: &String, state: &mut WalkState)
    {
        match self.dependencies.find(thing) {
            None => {
                debug!("{} has no dependencies",thing);
            },

            Some(deplist) => {
                debug!("Handling the dependencies of {}",thing);
                for n in deplist.iter() {
                    // If thing was not yet visited, recurse into it
                    if state.remaining.contains(n) {

                        debug!("Recursing for {}",n);
                        self.get_deps_of_recurse(n, state);

                        debug!("Appending {} to output",n);
                        state.output.push(n.clone());
                        state.remaining.remove(n);
                    }
                }
            },
        }

    }
}

#[test]
fn dglr_test() {
    let mut depgraph: DepGraph = DepGraph::new();

    depgraph.add_dependencies("a",vec!["b","c","d"]);
    depgraph.add_dependencies("b",vec!["d"]);
    depgraph.add_dependencies("c",vec!["e","m","g"]);
    depgraph.add_dependencies("e",vec!["f"]);
    depgraph.add_dependencies("g",vec!["h"]);
    depgraph.add_dependencies("h",vec!["i"]);
    depgraph.add_dependencies("i",vec!["j","k"]);
    depgraph.add_dependencies("k",vec!["l","m"]);
    depgraph.add_dependencies("m",vec!["n"]);

    let deps = depgraph.get_ordered_dependencies_of("a");

    println!("Deps of i = {}",deps);

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


    let deps2 = depgraph.get_ordered_dependencies_of("i");
    println!("Deps of i = {}",deps2);
    assert!( deps2 == vec![String::from_str("j"),
                           String::from_str("l"),
                           String::from_str("n"),
                           String::from_str("m"),
                           String::from_str("k")] );
}
