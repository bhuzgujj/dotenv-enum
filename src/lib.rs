use std::env;
use std::env::VarError;
use std::fmt::Debug;
use std::str::FromStr;
use strum::IntoEnumIterator;

pub trait EnvironmentVariable {
    fn get_variable_name(&self) -> String;

    fn get_error_description(&self) -> String {
        format!("No {} in .env file", self.get_variable_name())
    }

    fn does_key_exist(value: &str) -> bool where Self: IntoEnumIterator {
        <Self as IntoEnumIterator>::iter().any(|env_var| env_var.get_variable_name() == value)
    }
}

pub fn get_env_var_result(env_var: impl EnvironmentVariable) -> Result<String, VarError> {
    env::var(env_var.get_variable_name())
}

pub fn get_env_var(env_var: impl EnvironmentVariable + Copy) -> String {
    get_env_var_result(env_var).unwrap_or_else(|_| { panic!("{}", env_var.get_error_description()) })
}

pub fn get_casted_env_var_result<T: FromStr>(env_var: impl EnvironmentVariable + Copy) -> Result<T, <T as FromStr>::Err>
    where <T as FromStr>::Err: Debug {
    get_env_var(env_var).parse::<T>()
}

pub fn create_env_string(enum_name: &str, enum_value: &str) -> String {
    let values = filter_unsuable_character(enum_value);
    let mut name = filter_unsuable_character(enum_name);
    let name_size = name.len() - 1;
    if name[name_size].eq("Env") {
        name.remove(name_size);
    }
    format!("{}_{}", name.join("_").to_uppercase(), values.join("_").to_uppercase())
}

pub fn filter_unsuable_character(enum_value: &str) -> Vec<String> {
    split_string_on_capitalize(enum_value.trim()
        .to_string()
        .chars()
        .filter(|character: &char| !character.eq(&'_'))
        .collect())
}

pub fn split_string_on_capitalize(value: String) -> Vec<String> {
    let mut enum_values: Vec<String> = vec![];
    for character in value.chars() {
        if character.is_uppercase() {
            enum_values.push(String::from(character));
        } else {
            let size = enum_values.len();
            if size == 0 {
                enum_values.push(String::from(character));
                continue;
            }
            enum_values[size - 1].push(character);
        }
    }
    enum_values
}

#[macro_export]
macro_rules! env_enum {
    ($enum_name: ident, $env_test_name: ident, [$($var_name: ident), *]) => {
        #[derive(Copy, Clone, EnumIter)]
        pub enum $enum_name {
            $($var_name,)*
        }

        impl EnvironmentVariable for $enum_name {
            fn get_variable_name(&self) -> String {
                match self {
                    $($enum_name::$var_name => create_env_string(stringify!($enum_name), stringify!($var_name)),)*
                }
            }
        }

        #[cfg(test)]
        mod $env_test_name {
            extern crate self as my_crate;
            use strum::IntoEnumIterator;
            use dotenv_enum::{env_enum, EnvironmentVariable};

            mod when_using_an_element_it_should_be_in_dotenv {
                extern crate self as my_crate;
                use dotenv_enum::{EnvironmentVariable, get_env_var};
                use test_case::test_case;

                $(#[test]
                fn $var_name() {
                    dotenv::dotenv().ok();
                    assert!(!get_env_var(my_crate::$enum_name::$var_name).is_empty());
                })*
            }

            #[test]
            fn when_comparing_elements_they_are_all_different() {
                my_crate::$enum_name::iter().enumerate().for_each(|(index, env_var)| {
                    my_crate::$enum_name::iter().enumerate()
                        .filter(|(index2, _)| index != *index2)
                        .for_each(|(_, env_var2)| assert_ne!(env_var.get_variable_name(), env_var2.get_variable_name()));
                });
            }
        }
    };
}
