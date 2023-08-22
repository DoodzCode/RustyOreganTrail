use super::traits::Description;
use colored::Colorize;


#[derive(Debug)]
pub enum Terrain {
    Plains,
    Desert,
    Forest,
    Hills,
    Mountain,
    Trail,
}

impl Terrain {
    /// Returns a tuple representing base resource values ( Food, Water, Wood )
    /// 
    pub fn base_resource_availability(&self) -> (u8, u8, u8) {
        match self {
            Terrain::Plains => (15, 10, 5),
            Terrain::Desert => (5, 5, 5),
            Terrain::Forest => (15, 15, 15),
            Terrain::Hills => (10, 15, 5),
            Terrain::Mountain => (5, 15, 10),
            _ => (0,0,0),
        }
    }


    // TODO maybe upgrade the map to a Vec<String>?
    pub fn get_token(&self) -> colored::ColoredString {
        match self {
            Terrain::Plains => '~'.to_string().yellow(),
            Terrain::Desert => '-'.to_string().yellow(),
            Terrain::Forest => '!'.to_string().bright_green(),
            Terrain::Hills => ','.to_string().normal(),
            Terrain::Mountain => '^'.to_string().white(),
            Terrain::Trail => '-'.to_string().magenta(),
        }
    }

    // pub fn token(&self) -> String {
    //     match self
    // } 

    pub fn base_travel_cost(&self) -> u8 {
        match self {
            Terrain::Plains => 1,
            Terrain::Desert => 1,
            Terrain::Forest => 2,
            Terrain::Hills => 2,
            Terrain::Mountain => 4,
            Terrain::Trail => 0,
        }
    }
}

impl Description for Terrain {
    fn get_description(&self) -> String {
        match self {
            Terrain::Plains => "open, vast and nearly endless plains".to_string(),
            Terrain::Desert => "lifeless with mountains in the distance".to_string(),
            Terrain::Forest => "lush, green with rivers".to_string(),
            Terrain::Hills => " a hilly landscape. Easy to cross, however due to the lack of visability, hills make it easy for outlaws and indians to ambush oncoming caravans".to_string(),
            Terrain::Mountain => "harsh and rocky, terrain that makes traveling across, difficult and dangerous.".to_string(),
            _ => "invalid descriptor".to_string(),
        }
    }
}
