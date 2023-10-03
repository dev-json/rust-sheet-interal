/*Interface*/
pub trait Animal
{
    fn new(name: &str) -> Self;
    fn get_name(&self) -> &str;
    fn set_name(&mut self, name: &str);
    fn get_age(&self) -> &i32;
    fn set_age(&mut self, age: i32);
    fn sound(&self);
}