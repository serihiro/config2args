#[cfg(not(test))]
extern crate serde_json;

#[cfg(test)]
#[macro_use]
extern crate serde_json;

extern crate failure;

use serde_json::Value;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 0 {
        show_usage();
        std::process::exit(1);
    }

    let config_file_path = &args[0];
    let config = match parse_json_file(&config_file_path) {
        Err(why) => panic!("failed to parse json file: {}", why),
        Ok(value) => value,
    };

    println!("{}", generate_args_string(&config));
}

fn show_usage() {
    println!("usage: cofing2args /path/to/config.json");
}

fn parse_json_file(file_path: &str) -> Result<Value, failure::Error> {
    let mut file = File::open(file_path)?;

    let mut raw_json_contents = String::new();
    file.read_to_string(&mut raw_json_contents)?;

    let config: Value = serde_json::from_str(&raw_json_contents)?;

    return Ok(config);
}

fn generate_args_string(config: &Value) -> String {
    let mut args: String = String::new();

    if config.is_object() {
        let keys: serde_json::map::Keys = config.as_object().unwrap().keys();
        for key in keys {
            if key.len() == 1 {
                args.push_str(format!("-{key_string} ", key_string = key).as_str());
            } else {
                args.push_str(format!("--{key_string} ", key_string = key).as_str());
            }

            let item: &Value = &config[key];
            if item.is_number() {
                args.push_str(format!("{} ", item.as_f64().unwrap()).as_str());
                continue;
            }

            if item.is_string() {
                args.push_str(format!("{} ", item.as_str().unwrap()).as_str());
                continue;
            }

            if item.is_null() {
                continue;
            }

            if item.is_array() {
                let string_array: Vec<String> = convert_vec_to_string_vec(item.as_array().unwrap());
                args.push_str(format!("{} ", string_array.as_slice().join(" ")).as_str());
                continue;
            }

            panic!("Only number, string and array are supprted as an item of json config file.");
        }
    } else {
        if config.is_array() {
            let string_array: Vec<String> = convert_vec_to_string_vec(config.as_array().unwrap());
            args.push_str(format!("{} ", string_array.as_slice().join(" ")).as_str());
        }

        if config.is_number() {
            args.push_str(format!("{} ", config.as_f64().unwrap()).as_str());
        }

        if config.is_string() {
            args.push_str(format!("{} ", config.as_str().unwrap()).as_str());
        }
    }

    return args.trim_end().to_string();
}

fn convert_vec_to_string_vec(vec: &Vec<Value>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for item in vec {
        if item.is_number() {
            result.push(item.as_f64().unwrap().to_string());
            continue;
        }

        if item.is_string() {
            result.push(item.as_str().unwrap().to_string());
            continue;
        }

        panic!("Only number and string are supprted as an item of Array");
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_args_string_with_long_keys() {
        let config = json!({"key1": 1, "key2": "udon"});
        assert_eq!(generate_args_string(&config), "--key1 1 --key2 udon");
    }

    #[test]
    fn generate_args_string_with_short_keys() {
        let config = json!({"a": 1, "b": "udon"});
        assert_eq!(generate_args_string(&config), "-a 1 -b udon");
    }

    #[test]
    fn generate_args_string_with_array() {
        let config = json!({"key1": 1, "b": "udon", "key3": [1,2,3]});
        assert_eq!(
            generate_args_string(&config),
            "--key1 1 -b udon --key3 1 2 3"
        );
    }

    #[test]
    fn generate_args_string_with_string_value() {
        let config = json!("soba");
        assert_eq!(generate_args_string(&config), "soba");
    }

    #[test]
    fn generate_args_string_with_array_value() {
        let config = json!([1, 2, 3]);
        assert_eq!(generate_args_string(&config), "1 2 3");
    }

    #[test]
    #[should_panic]
    fn generate_args_string_with_nested_object() {
        let config = json!({"key1": 1, "b": "udon", "key3": [1,2,3], "key4": {"nested":"udon"} });
        generate_args_string(&config);
    }

    #[test]
    #[should_panic]
    fn generate_args_string_with_nested_array() {
        let config = json!({"key1": 1, "b": "udon", "key3": [1,2,3, [4]]});
        generate_args_string(&config);
    }
}
