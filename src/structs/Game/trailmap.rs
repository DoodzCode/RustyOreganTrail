use crate::structs::World::{
    terrain::Terrain,
    trailpoint::{Coords, TrailPoint},
};
use colored::Colorize;

#[derive(Debug)]
pub struct TrailMap {
    terrain_map: Vec<Vec<Terrain>>,
}

impl TrailMap {
    pub fn create(rows: u8, cols: u8) -> TrailMap {
        let mut map: Vec<Vec<Terrain>> = Vec::new();

        for _ in 0..rows {
            let mut row: Vec<Terrain> = Vec::new();
            for _ in 0..cols {
                row.push(Terrain::Plains);
            }
            map.push(row);
        }

        TrailMap { terrain_map: map }
    }

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
                    row_vec.push(self.get_colored_token(
                        &self.terrain_map[usize::from(y_coord)][usize::from(x_coord)],
                    ))
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

    /// Generates a square of forest around a single point
    pub fn build_forest(&mut self, coords: (u8, u8), radius: u8) {
        let row_start: u8 = coords.0 - radius;
        let col_start: u8 = coords.1 - radius;

        let row_end: u8 = coords.0 + radius;
        let col_end: u8 = coords.1 + radius;

        let mut y: u8 = 0;
        let mut x: u8 = 0;

        for row in self.terrain_map.iter_mut() {
            for point in row.iter_mut() {
                if y >= row_start && y <= row_end {
                    if x >= col_start && x <= col_end {
                        if y == row_start || y == row_end || x == col_start || x == col_end {
                            if rand::random() {
                                *point = Terrain::Forest;
                            }
                        } else {
                            *point = Terrain::Forest;
                        }
                    }
                }
                x += 1;
            }
            y += 1;
            x = 0;
        }
    }
}
