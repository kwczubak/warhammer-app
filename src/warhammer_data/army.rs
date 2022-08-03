use super::ability::{parse_abilities, Ability};
use super::unit::Unit;

use super::ros_parser::ros_parser::{Force, Roster};

#[derive(Debug)]
pub struct Detachment {
    pub name: String,
    pub abilities: Vec<Ability>,
    pub units: Vec<Unit>,
}

#[derive(Debug)]
pub struct Army {
    pub detachments: Vec<Detachment>,
    pub cp: f32,
}

impl Detachment {
    pub fn from_force(detachment_force: &Force) -> Result<Self, String> {
        let mut abilities: Vec<Ability> = Vec::new();
        let mut units: Vec<Unit> = Vec::new();

        // Should detachments have abilities? Or should it get passed down to the models?
        for selection in &detachment_force.selections.selections {
            match selection.r#type.as_str() {
                "upgrade" => {
                    
                }, 
                //abilities.append(&mut parse_abilities(&selection)),
                "model" | "unit" => units.push(Unit::from_selection(&selection).unwrap()),
                _ => {
                    return Err("Unknown selection type for force: ".to_string()
                        + selection.r#type.as_str())
                }
            };
        }

        Ok(Self {
            name: detachment_force.name.to_owned(),
            abilities,
            units,
        })
    }
}

impl Army {
    pub fn from_roster(roster: &Roster) -> Result<Self, String> {
        let mut detachments: Vec<Detachment> = Vec::new();
        
        for force in &roster.forces.forces {
            detachments.push(Detachment::from_force(force).unwrap());
        }

        let mut cp: f32 = 0.0;
        for cost in &roster.costs.costs {
            match cost.name.as_str() {
                "CP" => cp = cost.value,
                " PL" => (), // Power level
                "pts" => (), // points
                _ => (),
            }
        }

        Ok(Self {
            detachments,
            cp
        })
    }
}
