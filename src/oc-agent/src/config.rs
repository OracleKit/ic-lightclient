use ic_lightclient_types::Config;

pub fn load_config() -> Config {
    let config_file = "config.toml";
    let config_file_contents = std::fs::read_to_string(config_file)
        .expect("Failed to read config file");

    let config: Config = toml::from_str(&config_file_contents.as_str())
        .expect("Invalid config file");

    config
}