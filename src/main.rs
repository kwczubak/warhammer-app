use std::{fs::File, io::BufReader};

use ros_parser::Characteristics;
use serde_xml_rs::{from_reader};
use warhammer_data::ProfileValue;


mod ros_parser;
mod warhammer_data;

fn main() -> std::io::Result<()> {

    let file = File::open("Astra_1000/Astra_1000.ros")?;
    let buf = BufReader::new(file);

    let roster : ros_parser::Roster = from_reader(buf).unwrap();

    // let profile = warhammer_data::parse_unit_profile(&roster.forces.forces[0].selections.selections[4].profiles.as_ref().unwrap().profiles[0].characteristics);
    // println!("{:#?}", profile);

    let profile = warhammer_data::parse_weapon_profile(&roster.forces.forces[0].selections.selections[4].selections.as_ref().unwrap().selections[0].profiles.as_ref().unwrap().profiles[0].characteristics);
    println!("{:#?}", profile);

    // let d = ProfileValue::new("2D3+1");
    // print!("{:#?}", d);

    Ok(())
}
