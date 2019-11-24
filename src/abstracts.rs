pub trait Loadable {
    fn load(&mut self) -> Result<(), diesel::result::Error>;
}
