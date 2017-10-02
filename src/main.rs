
#![allow(dead_code)]
extern crate yaml_rust;
use yaml_rust::*;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

mod spec;
use spec::*;
fn main() {

    let _t : ClassSpec ;

   let mut file = match File::open("IBSMessage.yaml") {
       Err(why) => panic!("could not open file : {}",why.description()),
       Ok(file) => file,
   };

   let mut s = String::new();
   file.read_to_string(&mut s).unwrap();
   let docs = YamlLoader::load_from_str(&s).unwrap();
   let doc = &docs[0]; println!("{:?}", doc);

//    // Index access for map & array
//    assert_eq!(doc["foo"][0].as_str().unwrap(), "list1");
//    assert_eq!(doc["bar"][1].as_f64().unwrap(), 2.0);
//
//    // Chained key/array access is checked and won't panic,
//    // return BadValue if they are not exist.
//    assert!(doc["INVALID_KEY"][100].is_badvalue());
//
//    // Dump the YAML object
//    let mut out_str = String::new();
//    {
//        let mut emitter = YamlEmitter::new(&mut out_str);
//               emitter.dump(doc).unwrap(); // dump the YAML object to a String
//    }
//    println!("{}", out_str); println!("{:?}", doc);
//
//    // Index access for map & array
//    assert_eq!(doc["foo"][0].as_str().unwrap(), "list1");
//    assert_eq!(doc["bar"][1].as_f64().unwrap(), 2.0);
//
//    // Chained key/array access is checked and won't panic,
//    // return BadValue if they are not exist.
//    assert!(doc["INVALID_KEY"][100].is_badvalue());
//
//    // Dump the YAML object
//    let mut out_str = String::new();
//    {
//        let mut emitter = YamlEmitter::new(&mut out_str);
//        emitter.dump(doc).unwrap(); // dump the YAML object to a String
//    }
//    println!("{}", out_str);


 

}
