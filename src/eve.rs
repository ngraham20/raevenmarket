use serde::{Serialize, Deserialize};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader};
use std::collections::{HashMap};

#[derive(Serialize, Deserialize, Debug)]
pub enum EVEStation {
    Jita(String),
    Dodixie(String),
    Hek(String),
    Rens(String)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EVEMarketEntry<'a> {
    #[serde(borrow)]
    item: EVEItem<'a>,
    price: usize,
    location: EVEStation
}

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
    pub recipe: Option<EVERecipe>,
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

    fn shipname(&self) -> &'a str {
        match *self {
            ShipType::Frigate(n) => n,
            ShipType::Destroyer(n) => n,
            ShipType::Cruiser(n) => n
        }
    }
}

pub trait Recipe {
    type Data;
    fn load_recipe(&mut self) -> Result<(), Box<dyn Error>>;
    fn recipe(&self) -> Result<&Self::Data, Box<dyn Error>>;
    fn cost(&self) -> Result<usize, Box<dyn Error>>;
}

impl<'a> Recipe for Ship<'a> {
    type Data = Option<EVERecipe>;
    fn load_recipe(&mut self) -> Result<(), Box<dyn Error>> {
        let file = File::open(&self.recipe_file)?;
        let reader = BufReader::new(file);

        let jdata: HashMap<String, EVERecipe> = serde_json::from_reader(reader)?;
        self.recipe = Some(jdata.get(&format!("{}-T{}", self.shiptype.typestring(), self.techlevel)).unwrap().to_owned());
        Ok(())
    }

    fn recipe(&self) -> Result<&Self::Data, Box<dyn Error>> {
        Ok(&self.recipe)
    }
    fn cost(&self) -> Result<usize, Box<dyn Error>> {
        Ok(0usize)
    }
}