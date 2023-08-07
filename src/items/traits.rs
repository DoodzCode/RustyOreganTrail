pub trait Item {
    fn get_quantity(&self) -> u32;
    fn get_description(&self) -> String;
    fn get_name(&self) -> String;
    fn get_type(&self) -> Self;
}


// pub trait StatusEffectRemover<T> {
//     fn target<T>(&self) -> T;
    
// }