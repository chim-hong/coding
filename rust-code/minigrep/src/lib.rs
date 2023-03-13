use std::{error::Error, fs,env::{self, Args}};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("args is not enougth!");
        }
        args.next();
        let query = match args.next() {
            Some(arg)=>arg,
            None=>return Err("Didn't get a query string!")
        };
        let filename = match args.next() {
            Some(arg)=>arg,
            None=>return Err("Didn't get a filename string!")
        };
        Ok(Config {
            query,
            filename,
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
    contents.lines().filter(|line| line.contains(query)).collect()
}

fn case_insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.to_lowercase().contains(query)).collect()

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
