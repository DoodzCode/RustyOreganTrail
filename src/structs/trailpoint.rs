use super::region::Region;
use super::terrain::Terrain;
use super::traits::Description;
use super::weather::Weather;

#[derive(Debug)]
pub struct TrailPoint {
    pub terrain: Terrain,
    pub weather: Weather,
    pub region: Region,
}

impl TrailPoint {
    pub fn get_description(&self) -> String {
        format!(
            "{}{}",
            self.weather.get_description(),
            self.terrain.get_description()
        )
    }
}

pub fn _generate_trail() -> Vec<TrailPoint> {
    let mut trail: Vec<TrailPoint> = Vec::new();
    let mut counter = 0;
    loop {
        if counter < 5 {
            trail.push(TrailPoint {
                terrain: Terrain::Plains,
                weather: Weather::Clear,
                region: Region::Missouri,
            });
        } else if 4 < counter && counter < 10 {
            trail.push(TrailPoint {
                terrain: Terrain::Hills,
                weather: Weather::Cloudy,
                region: Region::Missouri,
            });
        } else if 9 < counter && counter < 12 {
            trail.push(TrailPoint {
                terrain: Terrain::Plains,
                weather: Weather::Rainy,
                region: Region::Kansas,
            });
        } else if 11 < counter && counter < 20 {
            trail.push(TrailPoint {
                terrain: Terrain::Desert,
                weather: Weather::Clear,
                region: Region::Kansas,
            });
        } else if 19 < counter && counter < 25 {
            trail.push(TrailPoint {
                terrain: Terrain::Mountain,
                weather: Weather::Foggy,
                region: Region::Wyoming,
            });
        }

        counter += 1;

        if counter >= 24 {
            return trail;
        }
    }
}
