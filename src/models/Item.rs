///

pub trait Item {
    fn search(&self);

    fn get_content(&self) -> Result<String, std::io::Error>;
}
