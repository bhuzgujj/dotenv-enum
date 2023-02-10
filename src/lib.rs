use std::env;
use std::env::VarError;
use std::fmt::Debug;
use std::str::FromStr;
use strum::IntoEnumIterator;

pub trait EnvironmentVariable
    where Self: IntoEnumIterator
{
    fn get_key(&self) -> String;

    fn get_error_description(&self) -> String {
        format!("No {} in .env file", self.get_key())
    }

    fn does_key_exist(key: &str) -> bool where Self: IntoEnumIterator {
        <Self as IntoEnumIterator>::iter().any(|env_var| env_var.get_key() == key)
    }

    fn get_value_result(&self) -> Result<String, VarError> {
        env::var(self.get_key())
    }

    fn unwrap_value(&self) -> String {
        self.get_value_result().unwrap_or_else(|_| { panic!("{}", self.get_error_description()) })
    }

    fn get_casted_value<T: FromStr>(&self) -> Result<T, <T as FromStr>::Err>
        where <T as FromStr>::Err: Debug {
        self.unwrap_value().parse::<T>()
    }
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
    let mut size: usize;
    for character in value.chars() {
        size = enum_values.len();
        if character.is_uppercase() || size == 0 {
            enum_values.push(String::from(character));
        } else {
            enum_values[size - 1].push(character);
        }
    }
    enum_values
}

#[macro_export]
macro_rules! env_enum {
    ($enum_name: ident, $env_test_name: ident, [$($var_name: ident), *]) => {
        #[derive(Copy, Clone, strum_macros::EnumIter)]
        pub enum $enum_name {
            $($var_name,)*
        }

        impl EnvironmentVariable for $enum_name {
            fn get_key(&self) -> String {
                match self {
                    $($enum_name::$var_name => create_env_string(stringify!($enum_name), stringify!($var_name)),)*
                }
            }
        }

        #[cfg(test)]
        mod $env_test_name {
            extern crate self as my_crate;
            use strum::IntoEnumIterator;
            use dotenv_enum::EnvironmentVariable;

            #[allow(non_snake_case)]
            mod when_using_an_element_it_should_be_in_dotenv {
                extern crate self as my_crate;
                use dotenv_enum::EnvironmentVariable;

                $(#[test]
                fn $var_name() {
                    dotenv::dotenv().ok();
                    assert!(!my_crate::$enum_name::$var_name.unwrap_value().is_empty());
                })*
            }

            #[test]
            fn when_comparing_elements_they_are_all_different() {
                my_crate::$enum_name::iter().enumerate().for_each(|(index, env_var)| {
                    my_crate::$enum_name::iter().enumerate()
                        .filter(|(index2, _)| index != *index2)
                        .for_each(|(_, env_var2)| assert_ne!(env_var.get_key(), env_var2.get_key()));
                });
            }
        }
    };
}

#[cfg(test)] extern crate self as dotenv_enum;
#[cfg(test)] env_enum!(AnEnv, an_test, [Lol, TeamJaws, Mdr]);
#[cfg(test)] env_enum!(En, en_test, [Kappa, Pog, Mdr]);

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(AnEnv::Lol, ("AN_LOL", "waw"); "an lol")]
    #[test_case(AnEnv::TeamJaws, ("AN_TEAM_JAWS", "jason"); "an team jaws")]
    #[test_case(En::Pog, ("EN_POG", "champ"); "en pog")]
    fn when_using_the_getter_on_str_env_it_should_return_the_right_value(env: impl EnvironmentVariable + Copy, expected: (&str, &str)) {
        test_env::<String>(env, (expected.0, expected.1.to_string()));
    }

    #[test_case(AnEnv::Mdr, ("AN_MDR", 11); "an mdr")]
    #[test_case(En::Mdr, ("EN_MDR", 54); "en mdr")]
    #[test_case(En::Kappa, ("EN_KAPPA", 12); "en kappa")]
    fn when_using_the_getter_on_number_it_should_return_the_right_value(env: impl EnvironmentVariable + Copy, expected: (&str, u32)) {
        test_env::<u32>(env, expected);
    }

    fn test_env<T: FromStr + PartialEq + Debug>(env: impl EnvironmentVariable + Copy + Sized, expected: (&str, T)) where <T as std::str::FromStr>::Err: Debug {
        dotenv::dotenv().ok();
        assert_eq!(env.get_key(), expected.0);
        assert_eq!(env.get_casted_value::<T>().expect(env.get_error_description().as_str()), expected.1);
    }
}