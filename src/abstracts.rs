pub trait Loadable {
    fn load(&mut self) -> Result<(), diesel::result::Error>;
}

pub trait Saveable {
    fn save(&self) -> Result<(), diesel::result::Error>;
}
