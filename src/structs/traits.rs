
trait HasDescription {
    fn get_description(&self) -> String;
}

trait HasName {
    fn get_name(&self) -> String;
}

enum Weather {
    Clear,
    Rainy,
    Cloudy,
    Foggy,
}

impl HasDescription for Weather {
    fn get_description(&self) -> String {
        match self {
            Weather::Clear => "The skies are clear and you can see ".to_string(),
            Weather::Rainy => "Rain falls heavily and obscures your vision, however you can make out ".to_string(),
            Weather::Cloudy => "The land is darkened by clouds, but in the distance you can see ".to_string(),
            Weather::Foggy => "Dense fog obscures your vision, you aren't able to see anything.".to_string(),
        }
    }
}

pub enum Terrain {
    Plains,
    Desert,
    Forest,
    Hills,
    Mountain,
}



impl HasDescription for Terrain {
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

enum Region {
    Missouri,
    Kansas,
    Wyoming,
    Idaho,
    Oregon
}

impl HasName for Region {
    fn get_name(&self) -> String {
        match self {
            Region::Missouri => "Missouri".to_string(),
            Region::Kansas => "Kansas".to_string(),
            Region::Wyoming => "Wyoming".to_string(),
            Region::Idaho => "Idaho".to_string(),
            Region::Oregon => "Oregon".to_string(),
        }
    }
}

struct TrailPoint {
    terrain: Terrain,
    weather: Weather,
    region: Region,
}

impl TrailPoint {
    pub fn get_description(&self) -> String {
        format!("{}{}", self.weather.get_description(), self.terrain.get_description())
    }
}

fn _generate_trail() -> Vec<TrailPoint> {
    let mut trail: Vec<TrailPoint> = Vec::new();
    let mut counter = 0;
    loop {
        if counter < 5 {
            trail.push(TrailPoint{ terrain: Terrain::Plains, weather: Weather::Clear, region: Region::Missouri });
        }
        else if 5 < counter && counter < 10 {
            trail.push(TrailPoint{ terrain: Terrain::Hills, weather: Weather::Cloudy, region: Region::Missouri });
        }
        else if 10 < counter && counter < 15 {
            trail.push(TrailPoint{ terrain: Terrain::Plains, weather: Weather::Rainy, region: Region::Kansas });
        }

        counter += 1;

        if counter >= 15 {
            return trail
        }
    }
}

fn look(location: &TrailPoint) {
    println!("\nYou scan your surroundings:");
    println!("{}\n", location.get_description());
}

fn where_am_i(location: &TrailPoint) {
    println!("According to the map, you are in {}", location.region.get_name())
}




#[cfg(test)]
    use super::*;
    
    #[test]
    fn test_terrain() {
        // let t_terrain = Terrain::Hills;
        // let desc_frag = t_terrain.get_description();

        // let t_weather = Weather::Rainy;
        // let desc_frag_1 = t_weather.get_description();

        let t_trail: Vec<TrailPoint> = _generate_trail();
        let t_trail = _generate_trail();
        let mut t_iter = t_trail.into_iter();
        // let iter_next_thing = t_iter.next();
        let current_location = t_iter.next().unwrap();

        look( &current_location );
        where_am_i(&current_location);

        let current_location = &t_iter.next().unwrap();
        let current_location = &t_iter.next().unwrap();
        let current_location = &t_iter.next().unwrap();
        let current_location = &t_iter.next().unwrap();
        let current_location = &t_iter.next().unwrap();
        let current_location = &t_iter.next().unwrap();

        look( &current_location );
        where_am_i(&current_location);
        
        let current_location = &t_iter.next().unwrap();
        let current_location = &t_iter.next().unwrap();
        let current_location = &t_iter.next().unwrap();
        let current_location = &t_iter.next().unwrap();
        let current_location = &t_iter.next().unwrap();
        let current_location = &t_iter.next().unwrap();

        look( &current_location );
        where_am_i(&current_location);
        

    }
