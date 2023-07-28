// if I input an item, I want it to return the recipe.
// the recipe should include the items required and the
// coin cost of using that recipe. This is a single step,
// not recursive.
// 
// Recipe itself could be a hashmap based off the item name.
// Recipe could be an item that returns its own contents.

use std::error::Error;

/// A recipe should be pretty basic, with the **Data**, which
/// is the basic information that makes up a recipe, and the
/// **Modifiers**, which defines how a recipe can be modified
/// by the game.
pub trait Recipe {
    type Data;
    type Modifiers;
    fn load_recipe(&mut self) -> Result<(), Box<dyn Error>>;
    fn recipe(&self) -> Result<&Self::Data, Box<dyn Error>>;
    fn apply_modifiers(&mut self, modifiers: Self::Modifiers) -> Result<(), Box<dyn Error>>;
    fn cost(&self) -> Result<usize, Box<dyn Error>>;
}