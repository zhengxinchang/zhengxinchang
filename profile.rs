struct Xinchang {
    name: String,
    sex: String,
    current_location: String,
    reserch: Vec<String>,
    program_languages: Vec<String>,
    hobbies: Vec<String>,
}

impl Xinchang {
    fn init() -> Self {
        Xinchang {
            name: String::from("Xinchang Zheng"),
            sex: String::from("Male"),
            current_location: String::from("Houston, Texas"),
            reserch: vec![
                String::from("Bioinformatics & Computational Biology"),
                String::from("Cancer genomics"),
                String::from("Software engineering"),
                String::from("Biomedical database/webserver"),
                String::from("Data visualization"),
                String::from("Long read sequencing"),
            ],
            program_languages: vec![
                String::from("Python"),
                String::from("Rust"),
                String::from("R"),
                String::from("JavaScript/HTML/CSS"),
                String::from("Rust"),
                String::from("C/C++"),
                String::from("Shell"),
            ],
            hobbies: vec![String::from("Vedio games"), String::from("Pokemon!")],
        }
    }
    fn say_hi() -> String {
        "Hey there!".to_string()
    }
}

fn main() {}
