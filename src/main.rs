use std::{fs::File, io::BufReader};

use serde_xml_rs::{from_str, from_reader};

mod ros_parser;

fn main() {

    let file = File::open("Test_Roster.ros")?;
    // let buf = BufReader::new(f);

    // let doc : ros_parser::Selections = from_reader(buf).unwrap();

    // println!("{:#?}", doc);
}
