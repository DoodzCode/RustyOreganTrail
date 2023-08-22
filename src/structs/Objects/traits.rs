pub trait TempResistance {
    fn get_cold_resistance(&self) -> u8;
    fn get_heat_resistance(&self) -> u8;
}

