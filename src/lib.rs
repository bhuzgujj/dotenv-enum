use std::env;
use std::fmt::Debug;
use std::str::FromStr;
use strum::IntoEnumIterator;

/// # EnvironmentVariable
/// This trait is a link between the dotenv and your enums.
/// The macro env_enum simplifies significantly its creation and its safety.
/// The macro creates a suite of test verifying the enum value exist withing the dotenv.
///
/// ## Macro use
/// ```
/// use dotenv_enum::{env_enum, EnvironmentVariable};
/// use strum::IntoEnumIterator;
///
/// env_enum!(TheEnumNameEnv, enum_test_module, [ValueOne, ValueTwo]);
/// ```
///
/// ## Self made
/// ```
/// use dotenv_enum::EnvironmentVariable;
/// use strum::IntoEnumIterator;
///
/// #[derive(Copy, Clone, strum_macros::EnumIter, PartialEq, Debug)]
/// enum TheEnumNameEnv {
///     ValueOne,
///     ValueTwo,
///     // ...
/// }
///
/// impl EnvironmentVariable for TheEnumNameEnv {
///     fn get_key(&self) -> String {
///         match self {
///             TheEnumNameEnv::ValueOne => "THE_ENUM_NAME_VALUE_ONE".to_string(),
///             TheEnumNameEnv::ValueTwo => "THE_ENUM_NAME_VALUE_TWO".to_string(),
///             // ...
///         }
///     }
/// }
///
/// mod enum_test_module {
///     extern crate self as my_crate;
///     use strum::IntoEnumIterator;
///     use dotenv_enum::EnvironmentVariable;
///
///     #[allow(non_snake_case)]
///     mod when_using_an_element_it_should_be_in_dotenv {
///         extern crate self as my_crate;
///         use dotenv_enum::EnvironmentVariable;
///
///         #[test]
///         fn ValueOne() {
///             dotenv::dotenv().ok();
///             assert!(!my_crate::$enum_name::$var_name.unwrap_value().is_empty());
///         }
///
///         #[test]
///         fn ValueTwo() {
///             dotenv::dotenv().ok();
///             assert!(!my_crate::$enum_name::$var_name.unwrap_value().is_empty());
///         }
///         //...
///     }
///
///     #[test]
///     fn when_comparing_elements_they_are_all_different() {
///         my_crate::TheEnumNameEnv::iter().enumerate().for_each(|(index, env_var)| {
///             my_crate::TheEnumNameEnv::iter().enumerate()
///                 .filter(|(index2, _)| index != *index2)
///                 .for_each(|(_, env_var2)| assert_ne!(env_var.get_key(), env_var2.get_key()));
///         });
///     }
/// }
/// ```
pub trait EnvironmentVariable
    where Self: IntoEnumIterator
{
    /// Get the key string of this enum value
    /// ```
    /// use dotenv_enum::{env_enum, EnvironmentVariable};
    /// use strum::IntoEnumIterator;
    ///
    /// env_enum!(TheEnumNameEnv, enum_test_module, [ValueOne, ValueTwo]);
    /// assert_eq!(TheEnumNameEnv::ValueOne.get_key(), "THE_ENUM_NAME_VALUE_ONE".to_string());
    /// ```
    fn get_key(&self) -> String;

    /// Verify the key exist within the enum
    /// ```
    /// use dotenv_enum::{env_enum, EnvironmentVariable};
    /// use strum::IntoEnumIterator;
    ///
    /// env_enum!(TheEnumNameEnv, enum_test_module, [ValueOne, ValueTwo]);
    /// assert_eq!(TheEnumNameEnv::does_key_exist("THE_ENUM_NAME_VALUE_ONE"), true);
    /// assert_eq!(TheEnumNameEnv::does_key_exist("THE_ENUM_NAME_VALUE_THREE"), false);
    /// ```
    fn does_key_exist(key: &str) -> bool where Self: IntoEnumIterator {
        Self::get_enum_value_from_key(key).is_some()
    }

    /// Get the enum value having the key provided or None
    /// ```
    /// use dotenv_enum::{env_enum, EnvironmentVariable};
    /// use strum::IntoEnumIterator;
    ///
    /// env_enum!(TheEnumNameEnv, enum_test_module, [ValueOne, ValueTwo]);
    /// assert_eq!(TheEnumNameEnv::get_enum_value_from_key("THE_ENUM_NAME_VALUE_ONE"), Some(TheEnumNameEnv::ValueOne));
    /// assert_eq!(TheEnumNameEnv::get_enum_value_from_key("THE_ENUM_NAME_VALUE_THREE"), None);
    /// ```
    fn get_enum_value_from_key(key: &str) -> Option<Self> {
        <Self as IntoEnumIterator>::iter()
            .find(|enum_value| enum_value.get_key() == key)
    }

    /// Get the value from the .env related to the enum value
    /// ```
    /// use dotenv_enum::{env_enum, EnvironmentVariable};
    /// use strum::IntoEnumIterator;
    ///
    /// dotenv::dotenv().ok();
    /// env_enum!(TheEnumNameEnv, enum_test_module, [ValueOne, ValueTwo]);
    ///
    /// // Assuming the line:
    /// //      THE_ENUM_NAME_VALUE_ONE = "val"
    /// // exist in the .env
    /// assert_eq!(TheEnumNameEnv::ValueOne.get_value(), Ok("val".to_string()));
    ///
    /// // Assuming the key THE_ENUM_NAME_VALUE_TWO does not exist in .env
    /// assert_eq!(TheEnumNameEnv::ValueTwo.get_value(), Err("No THE_ENUM_NAME_VALUE_TWO in .env file".to_string()));
    /// ```
    fn get_value(&self) -> Result<String, String> {
        match env::var(self.get_key()) {
            Ok(var) => Result::Ok(var),
            Err(_) => Result::Err(format!("No {} in .env file", self.get_key()))
        }
    }

    /// Get the value from the .env related to the enum value and unwrap it
    /// This function will panic instead of sending an Err
    /// ```
    /// use dotenv_enum::{env_enum, EnvironmentVariable};
    /// use strum::IntoEnumIterator;
    ///
    /// dotenv::dotenv().ok();
    /// env_enum!(TheEnumNameEnv, enum_test_module, [ValueOne, ValueTwo]);
    ///
    /// // Assuming the line:
    /// //      THE_ENUM_NAME_VALUE_ONE = "val"
    /// // exist in the .env
    /// assert_eq!(TheEnumNameEnv::ValueOne.unwrap_value(), "val".to_string());
    /// ```
    fn unwrap_value(&self) -> String {
        self.get_value().unwrap_or_else(|message| { panic!("{}", message) })
    }

    /// Get the value from the .env related to the enum value and casted it into the type T
    /// ```
    /// use dotenv_enum::{env_enum, EnvironmentVariable};
    /// use strum::IntoEnumIterator;
    ///
    /// dotenv::dotenv().ok();
    /// env_enum!(TheEnumNameEnv, enum_test_module, [ValueOne, ValueTwo]);
    ///
    /// // Assuming the line:
    /// //      THE_ENUM_NAME_VALUE_ONE = "val"
    /// // exist in the .env
    /// assert_eq!(TheEnumNameEnv::ValueOne.get_casted_value::<String>(), Ok("val".to_string()));
    /// assert_eq!(TheEnumNameEnv::ValueOne.get_casted_value::<u32>(), Err("Cannot cast THE_ENUM_NAME_VALUE_ONE into u32".to_string()));
    ///
    /// // Assuming the key THE_ENUM_NAME_VALUE_TWO does not exist in .env
    /// assert_eq!(TheEnumNameEnv::ValueTwo.get_value(), Err("No THE_ENUM_NAME_VALUE_TWO in .env file".to_string()));
    /// ```
    fn get_casted_value<T: FromStr>(&self) -> Result<T, String>
        where <T as FromStr>::Err: Debug {
        match self.get_value()?.parse::<T>() {
            Ok(var) => Result::Ok(var),
            Err(_) => Result::Err(format!(
                "Cannot cast {} into {}",
                self.get_key(),
                std::any::type_name::<T>()
            )),
        }
    }

    /// Get the value from the .env related to the enum value and casted it into the type T
    /// This function will panic instead of sending an Err
    /// ```
    /// use dotenv_enum::{env_enum, EnvironmentVariable};
    /// use strum::IntoEnumIterator;
    ///
    /// dotenv::dotenv().ok();
    /// env_enum!(TheEnumNameEnv, enum_test_module, [ValueOne, ValueTwo]);
    ///
    /// // Assuming the line:
    /// //      THE_ENUM_NAME_VALUE_ONE = "val"
    /// // exist in the .env
    /// assert_eq!(TheEnumNameEnv::ValueOne.unwrap_casted_value::<String>(), "val".to_string());
    /// ```
    fn unwrap_casted_value<T: FromStr>(&self) -> T
        where <T as FromStr>::Err: Debug {
        self.get_casted_value::<T>().unwrap_or_else(|message| panic!("{}", message))
    }

    /// Create a full capitalize, seperated by underscored, without suffix Env, and merge name_value
    /// ```
    /// use dotenv_enum::{env_enum, EnvironmentVariable};
    /// use strum::IntoEnumIterator;
    ///
    /// env_enum!(TheEnumNameEnv, enum_test_module, [Value]);
    ///
    /// assert_eq!(TheEnumNameEnv::create_env_string("LolEnv", "AValue"), "LOL_A_VALUE".to_string())
    /// ```
    fn create_env_string(enum_name: &str, enum_value: &str) -> String {
        let values = Self::get_env_strings(enum_value);
        let mut name = Self::get_env_strings(enum_name);
        let name_size = name.len() - 1;
        if name[name_size].eq("Env") {
            name.remove(name_size);
        }
        format!("{}_{}", name.join("_").to_uppercase(), values.join("_").to_uppercase())
    }

    /// Create a vector of all the words seperated by either underscores or capital letters
    /// ```
    /// use dotenv_enum::{env_enum, EnvironmentVariable};
    /// use strum::IntoEnumIterator;
    ///
    /// env_enum!(TheEnumNameEnv, enum_test_module, [Value]);
    ///
    /// assert_eq!(TheEnumNameEnv::get_env_strings("LolEnv_Ok"), vec!["Lol", "Env", "Ok"])
    /// ```
    fn get_env_strings(enum_value: &str) -> Vec<String> {
        Self::split_string_on_capitalize(enum_value.trim()
            .to_string()
            .chars()
            .filter(|character: &char| !character.eq(&'_'))
            .collect())
    }

    /// Split a string on capital letter and keep it
    /// ```
    /// use dotenv_enum::{env_enum, EnvironmentVariable};
    /// use strum::IntoEnumIterator;
    ///
    /// env_enum!(TheEnumNameEnv, enum_test_module, [Value]);
    ///
    /// assert_eq!(TheEnumNameEnv::split_string_on_capitalize("LolEnv_Ok".to_string()), vec!["Lol", "Env_", "Ok"])
    /// ```
    fn split_string_on_capitalize(value: String) -> Vec<String> {
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
}

/// # Environment Enum
/// This macro creates an enum and implements the trait [EnvironmentVariable].
/// It also creates tests to verify that the keys from the enums exists within the macro
/// ```
/// use dotenv_enum::{env_enum, EnvironmentVariable};
/// use strum::IntoEnumIterator;
///
/// env_enum!(TheEnumNameEnv, enum_test_module, [Value]);
/// ```
#[macro_export]
macro_rules! env_enum {
    ($enum_name: ident, $env_test_name: ident, [$($var_name: ident), *]) => {
        #[derive(Copy, Clone, strum_macros::EnumIter, PartialEq, Debug)]
        pub enum $enum_name {
            $($var_name,)*
        }

        impl EnvironmentVariable for $enum_name {
            fn get_key(&self) -> String {
                match self {
                    $($enum_name::$var_name => Self::create_env_string(stringify!($enum_name), stringify!($var_name)),)*
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

#[cfg(test)]
extern crate self as dotenv_enum;
#[cfg(test)] env_enum!(AnEnv, an_test, [Lol, TeamJaws, Mdr]);
#[cfg(test)] env_enum!(En, en_test, [Kappa, Pog, Mdr]);

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(AnEnv::Lol, ("AN_LOL", Result::Ok("waw".to_string())); "an lol")]
    #[test_case(AnEnv::TeamJaws, ("AN_TEAM_JAWS", Result::Ok("jason".to_string())); "an team jaws")]
    #[test_case(En::Pog, ("EN_POG", Result::Ok("champ".to_string())); "en pog")]
    #[test_case(AnEnv::Mdr, ("AN_MDR", Result::Ok("11".to_string())); "an mdr")]
    #[test_case(En::Mdr, ("EN_MDR", Result::Ok("54".to_string())); "en mdr")]
    #[test_case(En::Kappa, ("EN_KAPPA", Result::Ok("12".to_string())); "en kappa")]
    fn when_using_the_getter_on_str_env_it_should_return_the_right_value(env: impl EnvironmentVariable + Copy, expected: (&str, Result<String, String>)) {
        test_env::<String>(env, (expected.0, expected.1));
    }

    #[test_case(AnEnv::Mdr, ("AN_MDR", Result::Ok(11)); "an mdr")]
    #[test_case(En::Mdr, ("EN_MDR", Result::Ok(54)); "en mdr")]
    #[test_case(En::Kappa, ("EN_KAPPA", Result::Ok(12)); "en kappa")]
    #[test_case(AnEnv::Lol, ("AN_LOL", Result::Err("Cannot cast AN_LOL into u32".to_string())); "an lol")]
    #[test_case(AnEnv::TeamJaws, ("AN_TEAM_JAWS", Result::Err("Cannot cast AN_TEAM_JAWS into u32".to_string())); "an team jaws")]
    #[test_case(En::Pog, ("EN_POG", Result::Err("Cannot cast EN_POG into u32".to_string())); "en pog")]
    fn when_using_the_getter_on_number_it_should_return_the_right_value(env: impl EnvironmentVariable + Copy, expected: (&str, Result<u32, String>)) {
        test_env::<u32>(env, expected);
    }

    #[test_case("AN_MDR", Some(AnEnv::Mdr); "is present")]
    #[test_case("LOLLL", None; "is not present")]
    fn when_looking_from_string_you_can_get_the_enum_value(key: &str, expected: Option<AnEnv>) {
        assert_eq!(AnEnv::get_enum_value_from_key(key), expected);
        assert_eq!(AnEnv::does_key_exist(key), expected.is_some())
    }

    fn test_env<T: FromStr + PartialEq + Debug>(env: impl EnvironmentVariable + Copy + Sized, expected: (&str, Result<T, String>)) where <T as std::str::FromStr>::Err: Debug {
        dotenv::dotenv().ok();
        assert_eq!(env.get_key(), expected.0);
        assert_eq!(env.get_casted_value::<T>(), expected.1);
    }
}