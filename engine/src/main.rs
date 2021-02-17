use std::fs::File;
use std::io::prelude::*;

mod atoms;
mod book;

use atoms::*;
use book::{Block::*, *};

pub fn main() {
    let doc = Section {
        title: Box::new(Text("SemDoc".to_string())),
        body: Box::new(SplitSequence(vec![
            Text("Hello, world!".to_string()),
            Text("This is a test.".to_string()),
        ])),
    };
    let atom = doc.to_atom();
    let bytes = atom.to_bytes();

    for chunk in bytes.chunks(8) {
        for i in 0..8 {
            print!("{:02x} ", chunk.get(i).unwrap());
        }
        println!();
    }

    let mut file = File::create("helloworld.sd").unwrap();
    file.write_all(&bytes).unwrap();

    let retrieved_atom = (&bytes[..]).to_atom().unwrap();
    println!("Retrieved atoms: {:?}", retrieved_atom);
    let retrieved_doc = retrieved_atom.to_block();
    println!("Retrieved doc: {:?}", retrieved_doc);
}
