use super::traits::Name;

#[derive(Debug)]
pub enum Region {
    Missouri,
    Kansas,
    Wyoming,
    Idaho,
    Oregon,
}

impl Name for Region {
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
