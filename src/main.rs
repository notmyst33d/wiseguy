use reqwest::blocking::Client;
use std::env;
use std::fs::write;
use std::io::Read;
use std::path::Path;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let text = match args.get(1) {
        Some(value) => value,
        None => {
            println!("You need to specify the text for TTS");
            return;
        }
    };

    let mut data: Vec<u8> = vec![];
    let client = Client::new();

    client
        .get("http://api.voiceforge.com/swift_engine")
        .header("http_x_api_key", "8b3f76a8539")
        .query(&[("voice", "Wiseguy"), ("msg", &text), ("email", "null")])
        .send()
        .unwrap()
        .read_to_end(&mut data)
        .unwrap();

    let mut collision_counter = 0;
    let mut path_string = format!("audio{}.wav", collision_counter);
    let mut path = Path::new(&path_string);
    while Path::new(&path).exists() {
        collision_counter += 1;
        path_string = format!("audio{}.wav", collision_counter);
        path = Path::new(&path_string);
    }

    write(path, data).unwrap();

    println!("Audio written to {}", path.display());
}
