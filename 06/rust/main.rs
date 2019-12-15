use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct OrbitalRelationship {
    orbitee: String,
    orbiter: String,
}

impl OrbitalRelationship {
    fn new(orbitee: &str, orbiter: &str) -> OrbitalRelationship {
        return OrbitalRelationship { orbitee: orbitee.to_string(), orbiter: orbiter.to_string() };
    }
}

impl std::str::FromStr for OrbitalRelationship {

    type Err = ParseOrbitalRelationshipError;

    fn from_str(input: &str) -> std::result::Result<OrbitalRelationship, ParseOrbitalRelationshipError> {

        let [orbitee, orbiter] = match input
            .split(")")
            .collect::<Vec<_>>()[..] {
                [tee, ter] => Ok([tee, ter]),
                _ => Err(ParseOrbitalRelationshipError::new(input)),
            }?;

        return Ok(OrbitalRelationship::new(orbitee, orbiter));
    }
}

#[derive(Debug)]
struct ParseOrbitalRelationshipError {
    input: String,
}

impl ParseOrbitalRelationshipError {
    fn new(input: &str) -> ParseOrbitalRelationshipError { 
        return ParseOrbitalRelationshipError { input: input.to_string() };
    }
}

impl fmt::Display for ParseOrbitalRelationshipError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Invalid orbital relationship: {}", self.input);
    }
}

impl Error for ParseOrbitalRelationshipError {}

struct OrbitGraph {
    orbiters: HashMap<String, OrbitGraph>,
}

impl OrbitGraph {

    fn new() -> OrbitGraph {
        return OrbitGraph {
            orbiters: HashMap::new(),
        };
    }

    fn get(&mut self, orbiter: &str) -> Option<&mut OrbitGraph> {

        return None;
    }

    fn nodes<'a>(&'a mut self) -> impl Iterator<Item=OrbitGraph> {
        return self.orbiters.values().flat_map(|o| o.nodes());
    }
}

struct OrbitGraphBuilder {
    root: String,
    graph: OrbitGraph,
    deferred: HashMap<String, Vec<String>>,
}

impl OrbitGraphBuilder {

    fn new(root: &str) -> OrbitGraphBuilder {
        return OrbitGraphBuilder {
            root: root.to_string(),
            graph: OrbitGraph::new(),
            deferred: HashMap::new(),
        };
    }

    fn add(&mut self, rel: OrbitalRelationship) {

        if self.root == rel.orbitee {
            self.graph.orbiters.insert(rel.orbiter.to_string(), OrbitGraph::new());
            self.add_deferred(&rel.orbiter);
            return;
        }
        
        if let Some(orbitee) = self.graph.get(&rel.orbitee) {
            // todo: check for duplicates
            orbitee.orbiters.insert(rel.orbiter.to_string(), OrbitGraph::new());
            self.add_deferred(&rel.orbiter);
            return;
        }

        self.deferred.entry(rel.orbitee)
            .or_insert(Vec::new())
            .push(rel.orbiter);
    }

    fn add_deferred(&mut self, orbiter: &str) {
        println!("{:?}", orbiter)
    }
}

fn print(graph: OrbitGraph, depth: usize) {
    for (k, v) in graph.orbiters {
        println!("{:1$}:", k, depth * 2);
        print(v, depth + 1);
    }
}

fn main() {
    
    let mut builder = OrbitGraphBuilder::new("COM");

    io::stdin().lock().lines()
        .map(|l| l.unwrap())
        .map(|l| l.parse::<OrbitalRelationship>().unwrap())
        .for_each(|rel| builder.add(rel));

    print(builder.graph, 0);
}