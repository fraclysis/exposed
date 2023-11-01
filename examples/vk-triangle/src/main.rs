fn main() -> Result<(), std::io::Error> {
    exposed::window::utility::run::<vk_triangle::app::App>(Default::default())
}
