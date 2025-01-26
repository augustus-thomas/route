use std::{
    env,
    collections::HashMap,
    error::Error,
    fs::File,
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
    nodes: HashMap<String, Node>,
    edges: Vec<Edge>,
}

// helper distance method
fn r(source: &Node, destination: &Node) -> f32 {
    // cleaner way to do this? lol
    let del_x: f32 = destination.coord.x - source.coord.x;
    let del_y: f32 = destination.coord.y - source.coord.y;
    let del_z: f32 = destination.coord.z - source.coord.z;
    (del_x.powf(2.0) + del_y.powf(2.0) + del_z.powf(2.0)).sqrt()
}
fn read_nodes(file_path: &str) -> Result<HashMap<String, Node>, Box<dyn Error>> {
    let mut n: HashMap<String, Node> = HashMap::new();
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let node: Node = match result {
            Ok(r) => Node{label: r[0].to_string(), coord: Point{x: r[1].parse::<f32>().unwrap(), y: r[2].parse::<f32>().unwrap(), z: r[3].parse::<f32>().unwrap()}},
            Err(_e) => panic!("why"),
        };
        n.insert(node.label.clone(), node);
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
    let args: Vec<String> = env::args().collect();
    let g: Graph = Graph{nodes: read_nodes("../nodes.csv").expect("nodes file not found (place in ../)"), edges: read_edges("../edges.csv").expect("edges file not found (place in ../)")};
    let mut source: String = args[1].clone();
    let target: String = args[2].clone();
    // Bellman-Ford
    let mut dist: HashMap<String, f32> = HashMap::new();
    let mut dag: HashMap<String, String> = HashMap::new();
    // init
    for key in g.nodes.keys() {
        dist.insert(key.clone(), 10000000000000.0);
    }
    dist.insert(source.clone(), 0.0);

    for _x in &g.nodes {
        for y in &g.edges {
            let dist_u: f32 = match dist.get(&y.u) {
                Some(distance) => distance.clone(),
                None => panic!("{} has no distance", &y.u)
            };
            let dist_v: f32 = match dist.get(&y.v) {
                Some(distance) => distance.clone(),
                None => panic!("{} has no distance", &y.v)
            };
            let w: f32 = r(g.nodes.get(&y.u).expect("node not found"), g.nodes.get(&y.v).expect("node not found"));
            if dist_u + w < dist_v {
                dist.insert(y.v.clone(), dist_u + w);
                dag.insert(y.u.clone(), y.v.clone());
            }
        }
    }
    // IO
    print!("-> {} ", source);
    while source !=  target {
        let next = match dag.get(&source) {
            Some(next) => next,
            None => panic!("no!")
        };
        print!("-> {} ", next);
        source = next.to_string();
    }
    print!("{}", dist.get(&target).expect("target unreachable"));
}
