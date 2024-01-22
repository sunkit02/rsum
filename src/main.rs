use std::{
    env, fs,
    io::{stdin, Read},
    path::PathBuf,
};

const HELP_MENU: &'static str = r#"
Sums up space and or newline delimeted numbers (both integers and decimals) and prints result to stdout.
Input can be from stdin (no flag) or a file (-f flag).
"#;

#[derive(Debug, PartialEq, Eq)]
enum Config {
    Stdin,
    CliArg(String),
    File(PathBuf),
}

fn main() -> Result<(), String> {
    let config = parse_args(env::args().collect())?;

    let input = match config {
        Config::Stdin => {
            let mut buf = String::new();
            stdin().read_to_string(&mut buf).unwrap();
            buf
        }
        Config::CliArg(input) => input,
        Config::File(path) => fs::read_to_string(path).map_err(|e| e.to_string())?,
    };

    dbg!(&input);

    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<Config, String> {
    let mut args = args.into_iter();

    // Ignore very first argument given by OS
    args.next()
        .expect("No name of command invoking this binary given.");

    if let Some(first_arg) = args.next() {
        match first_arg.as_str() {
            "-f" => {
                if let Some(second_arg) = args.next() {
                    Ok(Config::File(PathBuf::from(second_arg)))
                } else {
                    Err("Missing path to file.".to_owned())
                }
            }
            _ => Ok(Config::CliArg(first_arg)),
        }
    } else {
        Ok(Config::Stdin)
    }
}

fn print_help() {
    println!("{}", HELP_MENU);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cli_arg_config() {
        let args = vec!["./rsum".to_owned(), "1 2 3".to_owned()];

        let parsed_config = parse_args(args);
        let expected = Config::CliArg("1 2 3".to_owned());

        assert_eq!(parsed_config, Ok(expected));
    }

    #[test]
    fn file_config() {
        let args = vec![
            "./rsum".to_owned(),
            "-f".to_owned(),
            "numbers.txt".to_owned(),
        ];

        let parsed_config = parse_args(args);
        let expected = Config::File(PathBuf::from("numbers.txt".to_owned()));

        assert_eq!(parsed_config, Ok(expected));
    }

    #[test]
    #[ignore = "Not sure how to implement this test yet."]
    fn stdin_config() {
        unimplemented!()
    }
}