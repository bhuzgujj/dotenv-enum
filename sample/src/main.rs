extern crate dotenv_enum;

use dotenv_enum::{env_enum, EnvironmentVariable, create_env_string};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

env_enum!(LocationsEnv, locations_test, [Folder, File, AnotherFile]);

fn main() {
    LocationsEnv::iter().for_each(|env_var| println!("{}", env_var.get_variable_name()));
}