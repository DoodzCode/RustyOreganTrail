use super::traits::TempResistance;

#[derive(Debug)]
pub enum Material {
    Fur,
    Wool,
    Leather,
    Cotton,
    None,
}

impl TempResistance for Material {
    fn get_cold_resistance(&self) -> u8 {
        match &self {
            Material::Fur => 10,
            Material::Wool => 6,
            Material::Leather => 3,
            Material::Cotton => 0,
            Material::None => 0,

        }
    }

    fn get_heat_resistance(&self) -> u8 {
        match &self {
            Material::Fur => 0,
            Material::Wool => 3,
            Material::Leather => 6,
            Material::Cotton => 10,
            Material::None => 0,
        }
    }
}
