use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::search;
use minigrep::search_insensitive;

fn main() {
    let args: Vec<String> = env::args().collect();

    let conf = Conf::new(&args).unwrap_or_else(|err| {
        eprintln!("err parsing args {err}");
        process::exit(1);
    });

    if let Err(e) = run(conf) {
        eprintln!("app err: {e}");
        process::exit(1);
    }
}

fn run(conf: Conf) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(conf.file_path)?;

    let results: Vec<&str>;
    if conf.ignore_case {
        results = search_insensitive(&conf.query, &file_contents);
    } else {

        results = search(&conf.query, &file_contents);
    }

    for line in results {
        println!("{line}")
    }

    Ok(())
}

struct Conf {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Conf {
    // factory method. ok => conf, err => string literal (with a static lifetime)
    fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough args");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Conf{ query, file_path, ignore_case })
    }
}
