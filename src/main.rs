use std::{fs::File, io::BufReader};

use serde_xml_rs::from_reader;

use crate::warhammer_data::ros_parser::ros_parser::Roster;

mod warhammer_data;

fn main() -> std::io::Result<()> {
    let file = File::open("Astra_1000/Astra_1000.ros")?;
    let buf = BufReader::new(file);

    let roster: Roster = from_reader(buf).unwrap();
    // println!("{:#?}", roster);

    // let profile = warhammer_data::unit::ModelProfile::from_profile(
    //     &roster.forces.forces[0].selections.selections[4]
    //         .profiles
    //         .as_ref()
    //         .unwrap()
    //         .profiles[0]
    // );
    // println!("{:#?}", profile);

    // let profile = warhammer_data::weapon::parse_weapon_profile(
    //     &roster.forces.forces[0].selections.selections[4]
    //         .selections
    //         .as_ref()
    //         .unwrap()
    //         .selections[0]
    //         .profiles
    //         .as_ref()
    //         .unwrap()
    //         .profiles[0]
    //         .characteristics,
    // );
    // println!("{:#?}", profile);

    // let model = warhammer_data::unit::parse_model(
    //     &roster
    //         .forces
    //         .forces
    //         .get(0)
    //         .unwrap()
    //         .selections
    //         .selections
    //         .get(4)
    //         .unwrap(),
    // );
    // println!("{:#?}", model);

    // let unit = warhammer_data::unit::Unit::from_selection(
    //     &roster
    //         .forces
    //         .forces
    //         .get(0)
    //         .unwrap()
    //         .selections
    //         .selections
    //         .get(11)
    //         .unwrap(),
    // );
    // println!("{:#?}", unit);

    // let detachment = warhammer_data::army::Detachment::from_force(
    //     &roster
    //         .forces
    //         .forces
    //         .get(0)
    //         .unwrap()
    // );
    // println!("{:#?}", detachment);

    let army = warhammer_data::army::Army::from_roster(
        &roster
    );
    println!("{:#?}", army);

    // let d = warhammer_data::profile::ProfileValue::from_str("2D3+1");
    // print!("{:#?}", d);

    Ok(())
}
