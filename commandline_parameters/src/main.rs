use std::env;


const APPLICATION_VERSION: &str = "0.1.0";

#[derive(Debug)]
struct ConfigurationInformation {
    input_file: String,
    output_file: String,
}

impl ConfigurationInformation {
    fn new() -> Self {
        Self {
            input_file: "input.xml".to_string(),
            output_file: "output.svg".to_string(),
        }
    }
    fn get_input_file_name<'a>(&'a self) -> &'a str {
        &self.input_file
    }

    fn get_outputfile_name<'a>(&'a self) -> &'a str {
        &self.output_file
    }

    fn set_output_file(&mut self, parameter: Option<&String>) {
        match parameter {
            Some(filename) => {
                self.output_file = filename.clone();
            }
            None => { println!("EEE the parameter was not there set_output_file()") }
        }
    }

    fn set_input_file(&mut self, parameter: Option<&String>) {
        match parameter {
            Some(filename) => {
                self.input_file = filename.clone();
            }
            None => { println!("EEE the parameter was not there set_output_file()") }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut args_iterator = args.iter();

    let mut local_configuration = ConfigurationInformation::new();

    // Skip the application name thing ($0)
    args_iterator.next();

    loop {
        match args_iterator.next() {
            Some(parameter) => {
                match parameter.as_str() {
                    "--help" => show_help(),
                    "--input-file" => local_configuration.set_input_file(args_iterator.next()),
                    "--output-file" => local_configuration.set_output_file(args_iterator.next()),
                    "--version" => println!("{}", APPLICATION_VERSION),
                    _ => println!("EEE unknown option: '{}'", parameter),
                }
            }
            None => {
                break;
            }
        }
    }

    println!("command line parameters - {} data:", APPLICATION_VERSION);
    println!("  Input  file: {}", local_configuration.get_input_file_name());
    println!("  Output file: {}", local_configuration.get_outputfile_name());
}


fn show_help() {
    println!("Command line parameters - help text");
    println!("  --help                    : this text");
    println!("  --input-file  <file_name> : set the input file name");
    println!("  --output-file <file_name> : set the output file name");
    println!("  --version                 : show application version number({})", APPLICATION_VERSION);
}