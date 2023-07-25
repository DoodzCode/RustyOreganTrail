use Uuid::new_V4;

pub struct Point{
    id: Uuid,
    zone_id: Uuid,
    biome_id: Uuid,
    weather_id: Uuid,
    description: String,
}s