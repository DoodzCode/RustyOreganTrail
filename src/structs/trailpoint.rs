use super::region::Region;
use super::terrain::Terrain;
use super::traits::Description;
use super::weather::Weather;

#[derive(Debug)]
pub struct TrailPoint {
    pub terrain: Terrain,
    pub weather: Weather,
    pub region: Region,
    pub coords: Coords,
}

#[derive(Debug)]
pub struct Coords (pub u8, pub u8);

impl TrailPoint {
    pub fn get_description(&self) -> String {
        format!(
            "{}{}",
            self.weather.get_description(),
            self.terrain.get_description()
        )
    }
}

pub fn _generate_tiny_trail() -> Vec<TrailPoint> {
    let mut trail: Vec<TrailPoint> = Vec::new();
    trail.push(TrailPoint {
        terrain: Terrain::Plains,
        weather: Weather::Clear,
        region: Region::Missouri,
        coords: Coords(50,50),
    });
    trail.push(TrailPoint {
        terrain: Terrain::Desert,
        weather: Weather::Rainy,
        region: Region::Missouri,
        coords: Coords(40, 50),

    });
    trail.push(TrailPoint {
        terrain: Terrain::Desert,
        weather: Weather::Rainy,
        region: Region::Missouri,
        coords: Coords(30, 50),

    });
    trail
}

// pub fn _generate_trail() -> Vec<TrailPoint> {
//     let mut trail: Vec<TrailPoint> = Vec::new();
//     let mut counter = 0;
//     loop {
//         if counter < 1 {
//             trail.push(TrailPoint {
//                 terrain: Terrain::Plains,
//                 weather: Weather::Clear,
//                 region: Region::Missouri,
//             });
//         } else if 1 < counter && counter < 3 {
//             trail.push(TrailPoint {
//                 terrain: Terrain::Hills,
//                 weather: Weather::Cloudy,
//                 region: Region::Missouri,
//             });
//         } else if 9 < counter && counter < 12 {
//             trail.push(TrailPoint {
//                 terrain: Terrain::Plains,
//                 weather: Weather::Rainy,
//                 region: Region::Kansas,
//             });
//         } else if 11 < counter && counter < 20 {
//             trail.push(TrailPoint {
//                 terrain: Terrain::Desert,
//                 weather: Weather::Clear,
//                 region: Region::Kansas,
//             });
//         } else if 19 < counter && counter < 25 {
//             trail.push(TrailPoint {
//                 terrain: Terrain::Mountain,
//                 weather: Weather::Foggy,
//                 region: Region::Wyoming,
//             });
//         }

//         counter += 1;

//         if counter > 2 {
//             return trail;
//         }
//     }
// }
