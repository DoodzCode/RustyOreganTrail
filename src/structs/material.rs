use super::traits::TempResistance;

pub enum Material {
    Fur,
    Wool,
    Leather,
    Cotton,
}

impl TempResistance for Material {
    fn get_cold_resistance(&self) -> u8 {
        match &self {
            Material::Fur => 10,
            Material::Wool => 6,
            Material::Leather => 3,
            Material::Cotton => 0,
        }
    }

    fn get_heat_resistance(&self) -> u8 {
        match &self {
            Material::Fur => 0,
            Material::Wool => 3,
            Material::Leather => 6,
            Material::Cotton => 10,
        }
    }
}
