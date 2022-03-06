#[cfg(not(test))]
extern crate serde_json;

extern crate tera;

#[cfg(test)]
#[macro_use]
extern crate serde_json;

use serde_json::Value;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        show_usage();
        std::process::exit(1);
    }

    let config_file_path = &args[0];
    let config = match parse_json_file(config_file_path) {
        Err(why) => panic!("failed to parse json file: {}", why),
        Ok(value) => value,
    };

    let raw_string = generate_args_string(&config, None);

    let is_tera_template = config_file_path.ends_with(".tera");
    if is_tera_template {
        let result = match eval_as_tera_template(&raw_string) {
            Err(why) => panic!("failed to eval as a tera template: {}", why),
            Ok(value) => value,
        };
        println!("{}", result);
    } else {
        println!("{}", raw_string);
    }
}

fn show_usage() {
    println!("usage: cofing2args /path/to/config.json");
}

fn parse_json_file(file_path: &str) -> anyhow::Result<Value> {
    let mut file = File::open(file_path)?;

    let mut raw_json_contents = String::new();
    file.read_to_string(&mut raw_json_contents)?;

    let config = serde_json::from_str(&raw_json_contents)?;

    Ok(config)
}

fn generate_args_string(config: &Value, prefix: Option<String>) -> String {
    let mut args = String::new();

    if config.is_object() {
        let keys = config.as_object().unwrap().keys();

        for key in keys {
            let mut key_name = prefix.clone().unwrap_or_default();
            key_name.push_str(key);

            let item = &config[key];
            if item.is_object() {
                key_name.push('.');
                let nested_args = generate_args_string(item, Some(key_name.clone()));
                args.push_str(format!("{} ", nested_args.as_str()).as_str());
                continue;
            }

            if key_name.find('_') != Some(0) {
                if key_name.len() == 1 {
                    args.push_str(format!("-{key_string} ", key_string = key_name).as_str());
                } else {
                    args.push_str(format!("--{key_string} ", key_string = key_name).as_str());
                }
            }

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
                let string_array = convert_vec_to_string_vec(item.as_array().unwrap());
                args.push_str(format!("{} ", string_array.as_slice().join(" ")).as_str());
                continue;
            }

            panic!("Only number, only string, array and object are supprted as an item of json config file.");
        }
    } else {
        if config.is_array() {
            let string_array = convert_vec_to_string_vec(config.as_array().unwrap());
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

fn convert_vec_to_string_vec(vec: &[Value]) -> Vec<String> {
    let mut result = Vec::new();
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

    result
}

fn eval_as_tera_template(template_string: &str) -> anyhow::Result<String> {
    let context = tera::Context::new();
    Ok(tera::Tera::one_off(template_string, &context, true)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_args_string_with_long_keys() {
        let config = json!({"key1": 1, "key2": "udon"});
        assert_eq!(generate_args_string(&config, None), "--key1 1 --key2 udon");
    }

    #[test]
    fn generate_args_string_with_short_keys() {
        let config = json!({"a": 1, "b": "udon"});
        assert_eq!(generate_args_string(&config, None), "-a 1 -b udon");
    }

    #[test]
    fn generate_args_string_with_array() {
        let config = json!({"key1": 1, "b": "udon", "key3": [1,2,3]});
        assert_eq!(
            generate_args_string(&config, None),
            "--key1 1 -b udon --key3 1 2 3"
        );
    }

    #[test]
    fn generate_args_string_with_string_value() {
        let config = json!("soba");
        assert_eq!(generate_args_string(&config, None), "soba");
    }

    #[test]
    fn generate_args_string_with_array_value() {
        let config = json!([1, 2, 3]);
        assert_eq!(generate_args_string(&config, None), "1 2 3");
    }

    #[test]
    fn generate_args_string_without_key() {
        let config = json!({"_skipped_key":1, "not_skipped_key": 2});
        assert_eq!(generate_args_string(&config, None), "1 --not_skipped_key 2");
    }

    #[test]
    fn generate_args_string_with_nested_object() {
        let config = json!({"key1":1, "key2": 2, "key3": { "k1": 3, "k2": 4, "k3": { "k4": 5 } }});
        assert_eq!(
            generate_args_string(&config, None),
            "--key1 1 --key2 2 --key3.k1 3 --key3.k2 4 --key3.k3.k4 5"
        );
    }

    #[test]
    #[should_panic]
    fn generate_args_string_with_nested_array() {
        let config = json!({"key1": 1, "b": "udon", "key3": [1,2,3, [4]]});
        generate_args_string(&config, None);
    }

    #[test]
    fn eval_as_a_tera_template() {
        let config = json!({"key1": "{% set my_var = [1, 2, 3, 4] %}{% for i in my_var %}{{i}} {% endfor %}"});
        let raw_string = generate_args_string(&config, None);
        assert_eq!(
            eval_as_tera_template(&raw_string).unwrap(),
            "--key1 1 2 3 4 "
        );
    }

    #[test]
    #[should_panic]
    fn eval_as_an_invalid_tera_template() {
        let config = json!({"key1": "{% set my_var = [1, 2, 3, 4] %}{% for i in my_var %}{{i}} {% endfor %"});
        let raw_string = generate_args_string(&config, None);
        assert_eq!(
            eval_as_tera_template(&raw_string).unwrap(),
            "--key1 1 2 3 4 "
        );
    }
}
