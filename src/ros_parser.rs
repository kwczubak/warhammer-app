use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Roster {
    id: String,
    name: String,
    battle_scribe_version: String,
    game_system_id: String,
    game_system_name: String,
    game_system_revision: String,
    costs: Costs,
    forces: Forces
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Costs {
    #[serde(rename = "$value")]
    costs: Vec<Cost>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Forces{
    #[serde(rename = "$value")]
    forces: Vec<Force>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Cost {
    name: String,
    type_id: String,
    value: f32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Force {
    id: String,
    name: String,
    entry_id: String,
    catalogue_id: String,
    catalogue_revision: String,
    catalogue_name: String,
    selections: Selections,
    publications: Publications,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Selections {
    #[serde(rename = "$value")]
    selections: Vec<Selection>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Selection {
    id: String,
    name: String,
    entry_id: String,
    number: u32,
    r#type: String,
    entry_group_id: Option<String>,
    publication_id: Option<String>,
    selections: Option<Selections>,
    costs: Option<Costs>,
    profiles: Option<Profiles>,
    categories: Option<Categories>,
    rules: Option<Rules>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Profiles {
    #[serde(rename = "$value")]
    profiles: Vec<Profile>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    id: String,
    name: String,
    hidden: bool,
    type_id: String,
    type_name: String,
    characteristics: Characteristics,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Characteristics {
    #[serde(rename = "$value")]
    characteristics: Vec<Characteristic>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Characteristic {
    name: String,
    type_id: String,
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Categories {
    #[serde(rename = "$value")]
    categories: Vec<Category>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    id: String,
    name: String,
    entry_id: String,
    primary: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Rules {
    #[serde(rename = "$value")]
    categories: Vec<Rule>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    id: String,
    name: String,
    hidden: bool,
    publication_id: Option<String>,
    page: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Publications {
    #[serde(rename = "$value")]
    publications: Vec<Publication>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Publication {
    id: String,
    name: String,
}
