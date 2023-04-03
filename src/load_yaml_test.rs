// 加载yaml
#[cfg(test)]
mod tests {

    use std::fs::File;
    use std::io::prelude::*;
    use serde::{Deserialize, Serialize};
    use serde_yaml;

    #[derive(Debug, Deserialize)]
    struct Config {
        server: ServerConfig,
        database: DatabaseConfig,
    }

    #[derive(Debug, Deserialize)]
    struct ServerConfig {
        host: String,
        port: u16,
    }

    #[derive(Debug, Deserialize)]
    struct DatabaseConfig {
        url: String,
        username: String,
        password: String,
    }

    #[test]
    fn load_yaml_test() {
        let mut file = File::open("config.yaml").expect("config file not found");

        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("failed to read config file");

        let config: Config = serde_yaml::from_str(&contents).expect("failed to parse config");

        println!("{:#?}", config);
    }
}