use std::collections::{HashMap};



// input both fiber and cloth items up through T8
// input recipes
// get profits and losses

// using item name, retrieve recipe
// add recipe cost to total
// iterate through ingredients, adding the cost each time
// this will be a tree walk, but using a HashSet to store the item cost each time to reduce recursion depth

mod eve;
use eve::*;

fn main() {

    let mut s = Ship {
        shiptype: ShipType::Frigate("Punisher"),
        techlevel: 1,
        recipe: None,
        recipe_file: "eve_recipe.json"
    };

    s.load_recipe().unwrap();
    if let Some(r) = s.recipe().unwrap() {
        println!("{:?}", r);
    }
}