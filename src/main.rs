use std::env;
use std::fs;
use std::collections::LinkedList;


#[derive(Debug)]
struct Location {
    lat: f32,
    lon: f32,
    vert: f32,
}

#[derive(Debug)]
struct Observation {
    loc: Location, 
    qty: u32,
    typ: u32,

}

fn create_obs(loc: Location, qty: u32, typ: u32) -> Observation { Observation {
    loc,
    qty,
    typ
}
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Somthing went wrong reading the file");

    println!("Contents of obs_seq\n{}", contents);

    let loc1 = Location {
                lat: 34.0,
                lon: 70.1,
                vert: 12.2
    };
        
    let obs1 = create_obs(loc1,12, 14);
    println!("obs1 loc {:?}, qty {}, typ {}", obs1.loc, obs1.qty, obs1.typ);

    println!("obs1 struct {:#?}", obs1);

    let mut sequence: LinkedList<Observation> = LinkedList::new();

    sequence.push_back(obs1);

}
