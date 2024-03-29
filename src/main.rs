use std::{
    env, fs,
    io::{stdin, Read},
    path::PathBuf,
    process,
};

const HELP_MENU: &'static str = r#"
Sums up space and or newline delimited numbers (both integers and decimals) and prints result to stdout.
Input can be from stdin (no flag) or a file (-f flag).
Note: Commas in the numbers are allowed.
"#;

#[derive(Debug, PartialEq, Eq)]
enum Config {
    Stdin,
    CliArg(String),
    File(PathBuf),
    PrintHelp,
}

fn main() -> Result<(), String> {
    let config = parse_args(env::args().collect())?;

    let num_str = match config {
        Config::Stdin => {
            let mut buf = String::new();
            stdin().read_to_string(&mut buf).unwrap();
            buf
        }
        Config::CliArg(input) => input,
        Config::File(path) => fs::read_to_string(path).map_err(|e| e.to_string())?,
        Config::PrintHelp => {
            print_help();
            process::exit(0);
        }
    };

    let sum = parse_num_str(num_str)?.iter().sum::<f32>();

    println!("{}", sum);

    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<Config, String> {
    // Ignore very first argument given by OS
    let mut args_iter = args.iter().skip(1);

    let args_len = args_iter.len();

    if let Some(first_arg) = args_iter.next() {
        match first_arg.as_str() {
            "-f" => {
                let path = args_iter
                    .next()
                    .ok_or_else(|| "Missing path to file.".to_owned())?;

                Ok(Config::File(PathBuf::from(path)))
            }
            "-h" => Ok(Config::PrintHelp),
            _ => {
                let str_len = args.iter().map(|num_str| num_str.len()).sum::<usize>() + args_len;
                let num_str = args.into_iter().skip(1).enumerate().fold(
                    String::with_capacity(str_len),
                    |mut acc, (i, num_str)| {
                        acc.push_str(num_str.as_str());
                        if i < args_len - 1 {
                            acc.push(' ');
                        }
                        acc
                    },
                );

                Ok(Config::CliArg(num_str))
            }
        }
    } else {
        Ok(Config::Stdin)
    }
}

fn parse_num_str(num_str: String) -> Result<Vec<f32>, String> {
    let num_str = num_str.chars().filter(|&c| c != ',').collect::<String>();
    let num_str = num_str
        .trim()
        .split('\n')
        .map(|line| line.split(' '))
        .flatten()
        .collect::<Vec<&str>>();

    let parse_results: Vec<Result<f32, _>> = num_str.iter().map(|n| n.parse::<f32>()).collect();

    for (result, num_str) in parse_results.iter().zip(num_str.iter()) {
        if let Err(_) = result {
            return Err(format!("Failed to parse '{}'", num_str));
        }
    }

    Ok(parse_results.into_iter().flatten().collect())
}

fn print_help() {
    println!("{}", HELP_MENU);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_parse_cli_arg_config() {
        let args = vec!["./rsum".to_owned(), "1 2 3".to_owned()];

        let parsed_config = parse_args(args);
        let expected = Config::CliArg("1 2 3".to_owned());

        assert_eq!(parsed_config, Ok(expected));
    }

    #[test]
    fn can_parse_file_config() {
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
    fn can_parse_print_help_config() {
        let args = vec!["./rsum".to_owned(), "-h".to_owned()];

        let parsed_config = parse_args(args);
        let expected = Config::PrintHelp;

        assert_eq!(parsed_config, Ok(expected));
    }

    #[test]
    #[ignore = "Not sure how to implement this test yet."]
    fn can_parse_stdin_config() {
        unimplemented!()
    }

    #[test]
    fn can_parse_num_str_without_commas() {
        let num_str = "0.1 10 20.5 30000 40.".to_owned();

        let nums = parse_num_str(num_str);

        let expected = vec![0.1, 10., 20.5, 30_000., 40.];

        assert_eq!(nums, Ok(expected));
    }

    #[test]
    fn can_parse_num_str_with_commas() {
        let num_str = "0.1 10 20.5 30,000 40.".to_owned();

        let nums = parse_num_str(num_str);

        let expected = vec![0.1, 10., 20.5, 30_000., 40.];

        assert_eq!(nums, Ok(expected));
    }

    #[test]
    fn can_parse_num_str_space_delimited() {
        let num_str = "0.1 10 20.5 30,000 40.".to_owned();

        let nums = parse_num_str(num_str);

        let expected = vec![0.1, 10., 20.5, 30_000., 40.];

        assert_eq!(nums, Ok(expected));
    }

    #[test]
    fn can_parse_num_str_newline_delimited() {
        let num_str = "0.1\n10\n20.5\n30,000\n40.".to_owned();

        let nums = parse_num_str(num_str);

        let expected = vec![0.1, 10., 20.5, 30_000., 40.];

        assert_eq!(nums, Ok(expected));
    }

    #[test]
    fn can_parse_num_str_space_and_newline_delimited() {
        let num_str = "0.1 10\n20.5 30,000\n40.".to_owned();

        let nums = parse_num_str(num_str);

        let expected = vec![0.1, 10., 20.5, 30_000., 40.];

        assert_eq!(nums, Ok(expected));
    }
}
