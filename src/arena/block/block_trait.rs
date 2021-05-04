pub trait DisplayNamed {
    fn display_name(&self) -> &String;
    fn set_display_name(&mut self, name: String);
}
