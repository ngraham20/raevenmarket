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

    let mut e = EVEItem {
        category: EVEItemCategory::Ship(Ship::Frigate("Slasher".to_string())),
        tech: 1,
        recipe: HashMap::new(),
        recipe_path: "test_recipe.json".to_string()
    };

    e.recipe.insert("Slasher".to_string(), vec![
        ("Tritanium".to_string(), 1),
        ("Pyerite".to_string(), 2),
        ("Mexallon".to_string(), 3),
        ("Isogen".to_string(), 4)
    ]);

    e.store_recipe().unwrap();
}