use crate::structs::World::{
    trailpoint::{TrailPoint, Coords},
    terrain::Terrain,
};
use colored::Colorize;

struct TrailMap {
    terrain_map: Vec<Vec<Terrain>>,
}

impl TrailMap {
    pub fn to_string(&self, location: &TrailPoint) -> String {
        let coords: &Coords = &location.coords;
        let radius: u8 = location.weather.visibility();
    
        let row_start: u8 = coords.0 - radius;
        let col_start: u8 = coords.1 - radius;
    
        let row_end: u8 = coords.0 + radius;
        let col_end: u8 = coords.1 + radius;
    
        let player_token = '*';

        let mut string_vec: Vec<String> = Vec::new();
    
        for y_coord in row_start..row_end + 1 {
            let mut row_vec: Vec<String> = Vec::new();
    
            for x_coord in col_start..col_end + 1 {
                if x_coord == coords.1 && y_coord == coords.0 {
                    // print!("{} ", player_token.to_string().red().blink());
                    row_vec.push(player_token.to_string());
                } else {
                    // print_token(&map[usize::from(y_coord)][usize::from(x_coord)]);
                    row_vec.push(self.get_colored_token(&self.terrain_map[usize::from(y_coord)][usize::from(x_coord)]))
                }
            }
            // row_vec.push("\n".to_string());
            string_vec.push(row_vec.join(""))
        }
        string_vec.join("\n")
    }
    
    fn get_colored_token(&self, terrain: &Terrain) -> String {
        match terrain {
            Terrain::Plains => format!("{} ", terrain.get_token().to_string().bright_yellow()),
            Terrain::Desert => format!("{} ", terrain.get_token()),
            Terrain::Forest => format!("{} ", terrain.get_token()),
            Terrain::Hills => format!("{} ", terrain.get_token()),
            Terrain::Mountain => format!("{} ", terrain.get_token()),
            Terrain::Trail => format!("{} ", terrain.get_token()),
            _ => "X".to_string(),
        }
    }
    
    
}