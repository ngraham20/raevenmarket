use serde::{Serialize, Deserialize};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader};
use std::collections::{HashMap};
use crate::recipe::{Recipe};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EVERecipe {
    ingredients: Vec<(String, usize)>,
    manufacture_time: usize
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EVEItem<'a> {
    #[serde(borrow)]
    Module(Module<'a>),
    Ship(Ship<'a>),

    Mineral(Mineral),
    Component(String),
    Material(String)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Mineral {
    Tritanium,
    Pyerite,
    Mexallon,
    Isogen,
    Nocxium,
    Zydrine,
    Megacyte
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Module<'a> {
    #[serde(borrow)]
    modtype: ModuleType<'a>,
    techlevel: usize,
    recipe: EVERecipe,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ModuleType<'a> {
    Autocannon(&'a str)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ship<'a> {
    #[serde(borrow)]
    pub shiptype: ShipType<'a>,
    pub techlevel: usize,
    pub recipe: EVERecipe,
    pub recipe_file: &'a str
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ShipType<'a> {
    Frigate(&'a str),
    Destroyer(&'a str),
    Cruiser(&'a str)
}

impl<'a> ShipType<'a> {
    fn typestring(&self) -> &'a str {
        match *self {
            ShipType::Frigate(_) => "Frigate",
            ShipType::Destroyer(_) => "Destroyer",
            ShipType::Cruiser(_) => "Cruiser"
        }
    }

    fn _shipname(&self) -> &'a str {
        match *self {
            ShipType::Frigate(n) |
            ShipType::Destroyer(n) |
            ShipType::Cruiser(n) => n
        }
    }
}

pub struct EVEBlueprint {
    pub time_efficiency: usize,
    pub material_efficiency: usize,
}

impl<'a> Recipe for Ship<'a> {
    type Data = EVERecipe;
    type Modifiers = EVEBlueprint;
    fn load_recipe(&mut self) -> Result<(), Box<dyn Error>> {
        let file = File::open(&self.recipe_file)?;
        let reader = BufReader::new(file);

        let jdata: HashMap<String, EVERecipe> = serde_json::from_reader(reader)?;
        self.recipe = jdata.get(&format!("{}-T{}", self.shiptype.typestring(), self.techlevel)).unwrap().to_owned();
        Ok(())
    }

    fn recipe(&self) -> Result<&Self::Data, Box<dyn Error>> {
        Ok(&self.recipe)
    }

    fn apply_modifiers(&mut self, modifiers: Self::Modifiers) -> Result<(), Box<dyn Error>> {
        let m = modifiers.material_efficiency;
        let t = modifiers.time_efficiency;
        self.recipe.ingredients.iter_mut().for_each(|(_, y)|{*y -= *y * m/100});
        self.recipe.manufacture_time -= self.recipe.manufacture_time * t/100;

        Ok(())
    }
    fn cost(&self) -> Result<usize, Box<dyn Error>> {
        Ok(0usize)
    }
}