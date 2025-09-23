#[derive(Default)]
pub struct ProfileDto {
    pub name: String,
    pub description: String,
}

impl ProfileDto {
    pub fn clear(&mut self) {
        self.name.clear();
        self.description.clear();
    }
}