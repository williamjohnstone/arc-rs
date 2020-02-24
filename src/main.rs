#[macro_use]
extern crate clap;
use clap::{App, AppSettings};

fn main() {
    let yaml = load_yaml!("args.yml");
    let config = App::from_yaml(yaml)
                    .setting(AppSettings::SubcommandRequiredElseHelp)
                    .get_matches();

    let test = config.value_of("config_file").unwrap_or("test.conf");

    println!("Config is:\n{}", test);
}
