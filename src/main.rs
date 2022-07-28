use std::{fs::File, io::BufReader};

use serde_xml_rs::{from_reader};
use warhammer_data::ParseProfile;

mod ros_parser;
mod warhammer_data;

fn main() -> std::io::Result<()> {

    let file = File::open("Astra_1000/Astra_1000.ros")?;
    let buf = BufReader::new(file);

    let roster : ros_parser::Roster = from_reader(buf).unwrap();

    // yeah so you can't do this, cause as compiler said characteristics doesn't impl copy.
    // so either you can add #[derive(Copy)] to characteristics or do what i'll commit. 
    let profile = ParseProfile(&roster.forces.forces[0].selections.selections[4].selections.as_ref().unwrap().selections[1].profiles.as_ref().unwrap().profiles[0].characteristics);

    // println!("{:#?}", doc);
    Ok(())
}
