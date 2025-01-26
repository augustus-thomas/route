use std::{
    //env,
    collections::HashMap,
    error::Error,
    //ffi::OsString,
    fs::File,
    //process,
};

struct Point {
    x: f32,
    y: f32,
    z: f32,
}

struct Node {
    label: String,
    coord: Point,
}

struct Edge {
    u: String,
    v: String,    
}

struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

// helper distance method
fn dist(source: Node, destination: Node) -> f32 {
    // cleaner way to do this? lol
    let del_x: f32 = destination.coord.x - source.coord.x;
    let del_y: f32 = destination.coord.y - source.coord.y;
    let del_z: f32 = destination.coord.z - source.coord.z;
    (del_x.powf(2.0) + del_y.powf(2.0) + del_z.powf(2.0)).sqrt()
}
fn read_nodes(file_path: &str) -> Result<Vec<Node>, Box<dyn Error>> {
    let mut n: Vec<Node> = vec![];
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let node: Node = match result {
            Ok(r) => Node{label: r[0].to_string(), coord: Point{x: r[1].parse::<f32>().unwrap(), y: r[2].parse::<f32>().unwrap(), z: r[3].parse::<f32>().unwrap()}},
            Err(_e) => panic!("why"),
        };
        n.push(node);
    }
    return Ok(n);
}

fn read_edges(file_path: &str) -> Result<Vec<Edge>, Box<dyn Error>> {
    let mut v: Vec<Edge> = vec![];
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let edge: Edge = match result {
            Ok(r) => Edge{u: r[0].to_string(), v: r[1].to_string(),},
            Err(_e) => panic!("what"), 
        };
        v.push(edge);
    }
    return Ok(v);
}

fn main() {
    // IO
    let g: Graph = Graph{nodes: read_nodes("../nodes.csv").expect("what"), edges: read_edges("../edges.csv").expect("what")};
    // Bellman-Ford
    let source: String = String::from("cat");
    let target: String = String::from("meow");
    let mut dist: HashMap<String, f32> = HashMap::new();
    // init
    for x in &g.nodes {
        dist.insert(String::clone(&x.label), 10000000000000.0);
    }


    for x in &g.nodes {
        for y in &g.edges {

        }
    }
}
