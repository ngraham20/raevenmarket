use std::error::Error;

/// A **Market** should have **Entries** and be able
/// to return them.
pub trait Market {
    type Entries;
    fn load_market_data(&mut self) -> Result<(), Box<dyn Error>>;
    fn market_data(&self) -> Result<&Self::Entries, Box<dyn Error>>;
}