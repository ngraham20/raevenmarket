use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};

// input both fiber and cloth items up through T8
// input recipes
// get profits and losses

// using item name, retrieve recipe
// add recipe cost to total
// iterate through ingredients, adding the cost each time
// this will be a tree walk, but using a HashSet to store the item cost each time to reduce recursion depth


fn main() {
    let T2Cloth = MarketEntry {
        name: "Simple Cloth",
        cost: 28f64,
    };

    let T2ClothRecipe = Recipe {
        name: "Simple Cloth",
        cost: 0f64,
        ingredients: vec![("Cotton", 1)]
    };

    let cloth_ser = serde_json::to_string(&T2Cloth).unwrap();
    let rec_ser = serde_json::to_string(&T2ClothRecipe).unwrap();
    println!("{}", cloth_ser);
    println!("{}", rec_ser);
}

#[derive(Serialize, Deserialize)]
struct MarketEntry<'a> {
    name: &'a str,
    cost: f64
}

#[derive(Serialize, Deserialize)]
struct Recipe<'a> {
    name: &'a str,
    ingredients: Vec<(&'a str, usize)>,
    cost: f64
}