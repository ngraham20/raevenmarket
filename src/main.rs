// input both fiber and cloth items up through T8
// input recipes
// get profits and losses

// using item name, retrieve recipe
// add recipe cost to total
// iterate through ingredients, adding the cost each time
// this will be a tree walk, but using a HashSet to store the item cost each time to reduce recursion depth

mod eve;
mod recipe;
mod market;
use eve::{Ship, ShipType, EVEBlueprint};
use recipe::{Recipe};

fn main() {

    let mut s = Ship {
        shiptype: ShipType::Frigate("Punisher"),
        techlevel: 1,
        recipe: None,
        recipe_file: "eve_recipe.json"
    };
    let bp = EVEBlueprint {
        material_efficiency: 10,
        time_efficiency: 20
    };

    s.load_recipe().unwrap();
    if let Some(r) = s.recipe().unwrap() {
        println!("{:?}", r);
    }
    s.apply_modifiers(bp).unwrap();
    if let Some(r) = s.recipe().unwrap() {
        println!("{:?}", r);
    }
}