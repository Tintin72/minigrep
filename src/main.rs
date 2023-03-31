use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    let conf = Config::build(&args).unwrap_or_else(|err| println!("Something is up, {}", err));
    // println!(
    //     "the query is {}, and the file path is {}",
    //     conf.query, conf.file_path
    // );

    // let contents =
    //     fs::read_to_string(conf.file_path).expect("Should have been able to read the file");

    // println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// fn parse_config(args: &[String]) -> Config {

// }
