extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main(){
    
    if args().len() != 3 {
        eprintln!("Usage: 'source' 'Target'");
    }

    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let mut output = File::create(args().nth(2).unwrap()).unwrap();

    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    output = encoder.finish().unwrap();

    println!(
        "Source len {:?}",input.get_ref().metadata().unwrap().len()
    );

    println!(
        "Target len {:?}",output.metadata().unwrap().len()
    );
    println!("Elapsed Time: {:?}",start.elapsed());
}