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
        if counter < 3 {
            trail.push(TrailPoint {
                terrain: Terrain::Plains,
                weather: Weather::Clear,
                region: Region::Missouri,
            });
        } else if 2 < counter && counter < 5 {
            trail.push(TrailPoint {
                terrain: Terrain::Hills,
                weather: Weather::Cloudy,
                region: Region::Missouri,
            });
        } else if 4 < counter && counter < 7 {
            trail.push(TrailPoint {
                terrain: Terrain::Plains,
                weather: Weather::Rainy,
                region: Region::Kansas,
            });
        }

        counter += 1;

        if counter >= 15 {
            return trail;
        }
    }
}
