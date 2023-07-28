use serde::{Serialize, Deserialize};
use super::everecipe::EVEItem;
use std::error::Error;
use crate::market::{Market};


#[derive(Serialize, Deserialize, Debug)]
pub enum EVEStation {
    Jita,
    Dodixie,
    Hek,
    Rens,
    Amarr,
    PlayerStation(String),
    NPCStation(String)
}

impl EVEStation {
    fn station(&self) -> String {
        match *self {
            EVEStation::Jita => "Jita IV - Moon 4 - Caldari Navy Assembly Plant".to_string(),
            EVEStation::Dodixie => "Dodixie IX - Moon 20 - Federation Navy Assembly Plant".to_string(),
            EVEStation::Amarr => "Amarr VIII (Oris) - Emperor Family Academy".to_string(),
            EVEStation::Hek => "Hek VIII - Moon 12 - Boundless Creation Factory".to_string(),
            EVEStation::Rens => "Rens VI - Moon 8 - Brutor Tribe Treasury".to_string(),
            EVEStation::PlayerStation(n) |
            EVEStation::NPCStation(n) => n
        }
    }
}

impl Market for EVEStation {
    type Entries = Vec<EVEMarketEntry>;
    fn load_market_data(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    fn market_data(&self) -> Result<&Self::Entries, Box<dyn Error>> {
        Ok(&Self::Entries::new())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EVEMarketEntry {
    price: usize,
    quantity: usize,
}