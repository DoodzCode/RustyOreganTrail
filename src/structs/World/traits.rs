pub trait Description {
    fn get_description(&self) -> String;
}

pub trait Name {
    fn get_name(&self) -> String;
}

pub trait TempRating {
    fn get_heat_rating(&self) -> u8;
    fn get_cold_rating(&self) -> u8;
}

