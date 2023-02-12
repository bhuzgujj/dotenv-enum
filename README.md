# dotenv Enums
This crate creates an enum from a `.env` variable and simplify acces to them.
The macro will create tests suite to check if the variable is present in the `.env` file.

*note: this is my first crate, so it may not be the best for production*

## Usage 
### With macro
```rust
use dotenv_enum::{env_enum, EnvironmentVariable};
use strum::IntoEnumIterator;

env_enum!(TheEnumNameEnv, enum_test_module, [
    ValueOne, 
    ValueTwo,
    //...
]);
```
*note: the comas on your last entry will break the macro*

### Without macro
```rust
use dotenv_enum::EnvironmentVariable;
use strum::IntoEnumIterator;

#[derive(Copy, Clone, strum_macros::EnumIter, PartialEq, Debug)]
enum TheEnumNameEnv {
     ValueOne,
     ValueTwo,
     // ...
}

impl EnvironmentVariable for TheEnumNameEnv {
    fn get_key(&self) -> String {
        match self {
            TheEnumNameEnv::ValueOne => "THE_ENUM_NAME_VALUE_ONE".to_string(),
            TheEnumNameEnv::ValueTwo => "THE_ENUM_NAME_VALUE_TWO".to_string(),
            // ...
        }
    }
}

mod enum_test_module {
    extern crate self as my_crate;
    use strum::IntoEnumIterator;
    use dotenv_enum::EnvironmentVariable;
    
    #[allow(non_snake_case)]
    mod when_using_an_element_it_should_be_in_dotenv {
        extern crate self as my_crate;
        use dotenv_enum::EnvironmentVariable;
        
        #[test]
        fn ValueOne() {
            dotenv::dotenv().ok();
            assert!(!my_crate::$enum_name::$var_name.unwrap_value().is_empty());
        }
        
        #[test]
        fn ValueTwo() {
            dotenv::dotenv().ok();
            assert!(!my_crate::$enum_name::$var_name.unwrap_value().is_empty());
        }
        //...
    }
    
    #[test]
    fn when_comparing_elements_they_are_all_different() {
        my_crate::TheEnumNameEnv::iter().enumerate().for_each(|(index, env_var)| {
            my_crate::TheEnumNameEnv::iter().enumerate()
                .filter(|(index2, _)| index != *index2)
                .for_each(|(_, env_var2)| assert_ne!(env_var.get_key(), env_var2.get_key()));
        });
    }
}
```

where all //... represent the same code as the previous one for all the new enum value you add.
