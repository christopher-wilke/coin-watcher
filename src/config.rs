use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    update_frequency_sec: u32,
    addresses: Vec<String>   
}

impl Config {
    pub fn new() -> String {
        "Hello from Config new :D".to_owned()
    }

    pub async fn read_file() {

        let tokio_file = tokio::fs::File::open("config.yml")
            .await
            .expect("Could not read file")
            .into_std()
            .await;

        // let file = std::fs::File::open("config.yml").expect("Could not read file");
        let serde: Config = serde_yaml::from_reader(tokio_file).unwrap();

        println!("{:?}", serde);
    }
}