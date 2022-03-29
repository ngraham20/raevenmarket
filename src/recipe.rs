// if I input an item, I want it to return the recipe.
// the recipe should include the items required and the
// coin cost of using that recipe. This is a single step,
// not recursive.
// 
// Recipe itself could be a hashmap based off the item name.
// Recipe could be an item that returns its own contents.

pub struct Item {
    name: String,
    cost: f64,
}

pub struct Recipe {
    name: String,
    ingredients: Vec<(String, usize)>,
    cost: f64
}