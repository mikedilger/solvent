// Dependency Graph Library in Rust

#![crate_type = "lib"]

// For log related macros:
#![feature(phase)]
#[phase(plugin, link)]
extern crate log;

use std::collections::hashmap::{HashMap,HashSet};

struct State<'a> {
    output: Vec<String>,
    remaining: HashSet<&'a str>,
}

pub fn dglr_get_ordered_dependencies_of<'c>(node: &'c str,
                          dependencies: &'c HashMap<&'c str,Vec<&'c str>>)
                          -> Vec<String>
{
    let mut state = State {
        output: Vec::new(),
        remaining: HashSet::new(),
    };

    // Scan the dependencies, and save all nodes in remaining
    for (n, dependsOn) in dependencies.iter() {
        debug!("preparing {}",n);
        state.remaining.insert(n.clone());
        for n2 in dependsOn.iter() {
            debug!("preparing {}",n2);
            state.remaining.insert(n2.clone());
        }
    }

    debug!("Recursing for the first time, with {}",node);
    dglr_deps_of(node, &mut state, dependencies);
    debug!("output is {}",state.output);
    state.output
}

fn dglr_deps_of<'a>(node: &'a str,
                   state: &mut State<'a>,
                   dependencies: &'a HashMap<&'a str,Vec<&'a str>>)
                   -> ()
{
    match dependencies.find(&node) {

        None => {
            debug!("{} has no dependencies",node);
        },

        Some(deplist) => {
            debug!("Handling the dependencies of {}",node);
            for n in deplist.iter() {
                // If node was not yet visited, recurse into it
                if state.remaining.contains(n) {

                    debug!("Recursing for {}",n);
                    dglr_deps_of(*n, state, dependencies);

                    debug!("Appending {} to output",*n);
                    state.output.push(String::from_str(*n));
                    state.remaining.remove(n);
                }
            }
        },
    }
}

#[test]
fn dglr_test() {
    let mut map: HashMap<&str,Vec<&str>> = HashMap::new();
    map.insert("a",vec!["b","c","g"]);
    map.insert("b",vec!["d"]);
    map.insert("c",vec!["e","m","g"]);
    map.insert("e",vec!["f"]);
    map.insert("g",vec!["h"]);
    map.insert("h",vec!["i"]);
    map.insert("i",vec!["j","k"]);
    map.insert("k",vec!["l","m"]);
    map.insert("m",vec!["n"]);

    let deps = dglr_get_ordered_dependencies_of("a",&map);
    println!("Deps of a = {}",deps);
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

    let deps2 = dglr_get_ordered_dependencies_of("i",&map);
    println!("Deps of i = {}",deps2);
    assert!( deps2 == vec![String::from_str("j"),
                           String::from_str("l"),
                           String::from_str("n"),
                           String::from_str("m"),
                           String::from_str("k")] );
}
