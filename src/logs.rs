use log::LevelFilter;

pub fn setup() {
    env_logger::Builder::from_default_env()
        .filter_level(LevelFilter::Info)
        .init();
}
