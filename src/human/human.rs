use crate::animal::animal::Animal;

pub struct Human
{
    pub(self) name: String,
    pub(self) last_name: String,
    pub(self) age: u32,

    pub(self) animals: Vec<dyn Animal>,
}

impl Human {
    pub fn new() -> Self
    {
        Self
    }

    pub fn set_name(&mut self, name: String)
    {
        self.name = name;
    }

    pub fn get_name(&self) -> &String
    {
        &self.name
    }

    pub fn set_last_name(&mut self, name: String)
    {
        self.last_name = name;
    }

    pub fn get_last_name(&self) -> &String
    {
        &self.last_name
    }

    pub fn set_age(&mut self, age: u32)
    {
        self.age = age;
    }

    pub fn get_age(&self) -> &u32
    {
        &self.age
    }

    pub fn get_animals(&self) -> &Vec<dyn Animal>
    {
        &self.animals
    }

}