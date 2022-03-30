use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use std::fs;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

// input both fiber and cloth items up through T8
// input recipes
// get profits and losses

// using item name, retrieve recipe
// add recipe cost to total
// iterate through ingredients, adding the cost each time
// this will be a tree walk, but using a HashSet to store the item cost each time to reduce recursion depth


fn main() {
    // let T2Cloth = MarketEntry {
    //     name: "Simple Cloth",
    //     cost: 28f64,
    // };

    // let T2ClothRecipe = Recipe {
    //     name: "Simple Cloth",
    //     cost: 0f64,
    //     ingredients: vec![("Cotton", 1)]
    // };

    // let cloth_ser = serde_json::to_string(&T2Cloth).unwrap();
    // let rec_ser = serde_json::to_string(&T2ClothRecipe).unwrap();
    // println!("{}", cloth_ser);
    // println!("{}", rec_ser);

    let mut bridgewatch: Market = read_market_file("./bridgewatch_market.json").unwrap();
    let mut recipes: HashMap<String, Recipe> = read_recipe_file("albion_recipe.json").unwrap();
    println!("{:#?}", bridgewatch);
    println!("{:#?}", recipes);
    
}

fn read_market_file<'a, P: AsRef<Path>>(path: P) -> Result<Market, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let m = serde_json::from_reader(reader)?;

    Ok(m)
}

fn read_recipe_file<'a, P: AsRef<Path>>(path: P) -> Result<HashMap<String, Recipe>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let m = serde_json::from_reader(reader)?;

    Ok(m)
}

#[derive(Serialize, Deserialize, Debug)]
struct Market {
    name: String,
    entries: Vec<Entry>
}

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    name: String,
    price: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct Recipe {
    ingredients: Vec<(String, usize)>,
    cost: usize,
    time: usize
}

struct Item<'a> {
    name: &'a str
}

// #[derive(Serialize, Deserialize)]
// struct MarketEntry<'a> {
//     name: &'a str,
//     cost: f64
// }

// #[derive(Serialize, Deserialize)]
// struct Recipe<'a> {
//     name: &'a str,
//     ingredients: Vec<(&'a str, usize)>,
//     cost: f64
// }