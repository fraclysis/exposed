fn main() -> Result<(), std::io::Error> {
    exposed::window::utility::run::<triangle::App>(Default::default())
}
