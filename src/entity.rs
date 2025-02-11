use std::fmt::Display;

pub trait Entity {
    type Data : Display;
    fn created_time(&self) -> u128;
    fn name(&self) -> &str;
    fn data(&self) -> Self::Data;
    fn new(name: String, created_time: u128) -> Self;
    fn eq(&self, other: &Self) -> bool {
        self.created_time() == other.created_time()
    }
}
