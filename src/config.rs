use bevy::prelude::Resource;
use serde::{
    Serialize,
    Deserialize,
};
use toml;


const DEFAULT_CLIENT_CONFIG: &str = r#"
    remote_url = "http://localhost:7777"

    [stars."Orion Nebula"]
    ra = 101.56875000000
    dec = -16.7514722222

    [stars.Sirius]
    ra = 83.826791666666
    dec = -5.39255555555
"#;

const DEFAULT_SERVER_CONFIG: &str = r#"
    ip = "http://localhost:7777"
    latitude = 40.416775
    longitude = -3.703790
    azimuth_steps_per_revolution = 20000
    altitude_steps_per_revolution = 20000
"#;

fn load_config_file(client: bool) -> String {
    let dir = std::env::var("XDG_CONFIG_HOME").unwrap_or(
        std::env::var("HOME").unwrap_or(String::from("~")) + "/.config"
    ) + "/radio-telescope-controller/";
    let file_name = if client {
        "client.toml"
    } else {
        "server.toml"
    };
    let path = dir.clone() + file_name;

    std::fs::create_dir_all(dir).unwrap();
    let write_default_config_and_return_content = | | -> String {
        println!("Creating missing config file in {}", &path);
        let config = if client {
            DEFAULT_CLIENT_CONFIG
        } else {
            DEFAULT_SERVER_CONFIG
        };
        std::fs::write(&path, config).unwrap();
        String::from(config)
    };

    std::fs::read_to_string(&path)
        .unwrap_or(write_default_config_and_return_content())
        .parse()
        .unwrap()
}

pub struct Star {
    pub name: String,
    pub ra: f32,
    pub dec: f32,
}

#[derive(Resource)]
pub struct ClientConfig {
    pub remote_url: String,
    pub stars: Vec<Star>,
}

impl Default for ClientConfig {
    fn default() -> ClientConfig {
        let config: toml::Value = toml::from_str(load_config_file(true).as_str()).unwrap();
        let mut client_config = ClientConfig {
            remote_url: String::from(config.get("remote_url").unwrap().as_str().unwrap()),
            stars: vec![],
        };
        
        if let Some(star_table) = config.get("stars") {
            for (star_name, star_data) in star_table.as_table().unwrap() {
                let star_ra = star_data.get("ra").expect(
                    &format!("Star {} is missing a right-ascension (ra) setting.", star_name)
                );
                let star_dec = star_data.get("dec").expect(
                    &format!("Star {} is missing a declination (dec) setting.", star_name)
                );
                client_config.stars.push(
                    Star {
                        name: star_name.clone(),
                        ra: star_ra.as_float().unwrap() as f32,
                        dec: star_dec.as_float().unwrap() as f32,
                    }
                );
            }
        }
        client_config
    }
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub ip: String,
    pub azimuth_steps_per_revolution: u32,
    pub altitude_steps_per_revolution: u32,
}

pub fn parse_server_config() -> ServerConfig {
    let config: ServerConfig = toml::from_str(load_config_file(false).as_str()).unwrap();
    config
}
