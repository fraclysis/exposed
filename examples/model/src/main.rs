use exposed::window::utility;

fn main() -> Result<(), std::io::Error> {
    utility::run::<model::App>(Default::default())
}
