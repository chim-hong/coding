use std::{error::Error, fs,env};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("args is not enougth!");
        }
        let query = &args[1];
        let filename = &args[2];
        Ok(Config {
            query: query.clone(),
            filename: filename.clone(),
            case_sensitive:env::var("CASE_SENSEITIVE").is_err(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        case_insensitive_search(&config.query, &contents)
    };
    for line in results {
        println!("{:#?}", line);
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }
    results
}

fn case_insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "hello";
        let contents = "\
        hello, world!
        This is a test file,
        which is include word!
      ";
        assert_eq!(vec!["hello, world!"], search(query, contents))
    }

    fn case_insensitive() {
        let query = "Hello";
        let contents = "\
      hello, world!
      This is a test file,
      which is include word!
    ";
        assert_eq!(
            vec!["hello, world!"],
            case_insensitive_search(query, contents)
        )
    }
}
