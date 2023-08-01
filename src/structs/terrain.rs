use super::traits::Description;

#[derive(Debug)]
pub enum Terrain {
    Plains,
    Desert,
    Forest,
    Hills,
    Mountain,
}

impl Description for Terrain {
    fn get_description(&self) -> String {
        match self {
            Terrain::Plains => "open, vast and nearly endless plains".to_string(),
            Terrain::Desert => "lifeless with mountains in the distance".to_string(),
            Terrain::Forest => "lush, green with rivers".to_string(),
            Terrain::Hills => " a hilly landscape. Easy to cross, however due to the lack of visability, hills make it easy for outlaws and indians to ambush oncoming caravans".to_string(),
            Terrain::Mountain => "harsh and rocky, terrain that makes traveling across, difficult and dangerous.".to_string(),
        }
    }
}
