use std::io::Read;

pub fn get_inputd(year: u32, day: u32) -> String {
    let file = format!("data/input_{year}_{day}");
    std::fs::read_to_string(file).unwrap()
}

pub fn get_input(year: u32, day: u32) -> String {
    let cache_file = format!("./cache/input_{year}_{day}");

    if let Ok(mut file) = std::fs::File::open(&cache_file) {
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Error reading from cache file");
        buf
    } else {
        let cookie = std::env::var("ADVENT_OF_CODE_COOKIE")
            .expect("Could not read Session Cookie Environment variable");

        let client = reqwest::blocking::Client::new();
        let download = client
            .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
            .header(reqwest::header::COOKIE, format!("session={cookie}"))
            .send()
            .unwrap()
            .text()
            .unwrap();

        std::fs::create_dir_all("./cache").unwrap();
        std::fs::write(&cache_file, &download).unwrap();
        download
    }
}