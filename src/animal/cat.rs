use crate::animal::animal::Animal;
use crate::animal::structs::Cat;

/* Implementation of the Animal Interface with the Cat struct*/
impl Animal for Cat {

     fn new(pet_name: &str) -> Self {
          Self {
               name: (pet_name.to_owned()),
               age: 0,
          }
     }

     fn get_name(&self) -> &str {
          &self.name
     }

     fn set_name(&mut self, name: &str) {
          self.name = name.to_owned()
     }

     fn get_age(&self) -> &i32 {
          &self.age
     }

     fn set_age(&mut self, age: i32) {
          self.age = age
     }

     fn sound(&self) {
          println!("Meow!")
     }
}
