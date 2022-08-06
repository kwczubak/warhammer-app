use super::{ros_parser::ros_parser::{Roster, Force}, army::{Army, Detachment}};

pub fn army_from_roster(roster: &Roster) -> Result<Army, String> {
    let mut cp;
    let mut pl;
    let mut pts;
    let mut detachments: Vec<Detachment> = Vec::new();

    // Get data from costs
    for cost in &roster.costs.costs {
        match cost.type_id.as_str() {
            "e356-c769-5920-6e14" => pl = cost.value,
            "2d3b-b544-ad49-fb75" => cp = cost.value,
            "points" => pts = cost.value,
            _ => return Err("Unknown cost type from roster.".to_string())
        }
    }

    // Get detachements
    for force in &roster.forces.forces {
        match detachment_from_force(force) {
            Ok(_) => todo!(),
            Err(_) => todo!(),
        }
    }

    let name: String = roster.name.to_owned();

    Ok(Army { detachments, cp, pl, pts, name })
}

pub fn detachment_from_force(force: &Force) -> Result<Detachment, String> {

}
