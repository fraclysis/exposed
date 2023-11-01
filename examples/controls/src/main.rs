fn main() -> Result<(), std::io::Error> {
    exposed::window::utility::run::<controls::App>(Default::default())
}
