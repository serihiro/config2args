[![crate-name at crates.io](https://img.shields.io/crates/v/config2args.svg)](https://crates.io/crates/config2args)
[![Build Status](https://travis-ci.org/serihiro/config2args.svg?branch=master)](https://travis-ci.org/serihiro/config2args)

# About this repository
This is a CLI tool which converts config file (JSON is only supported for now) into [GNU option style](https://www.gnu.org/prep/standards/html_node/Command_002dLine-Interfaces.html) string

# Example
```sh
$ cat test.json
{
    "key1": 1,
    "key2": "hello",
    "key3": [2,3,4],
    "key4": 1.4,
    "key5": null,
    "a": "b"
}
$ config2args test.json
--key1 1 --key2 hello --key3 2 3 4 --key4 1.4 --key5 -a b
```

# How to install
cargo >= `1.32.0` is required. Using [rustup](https://rustup.rs/) is a good way to install rust build tools.

```sh
$ cargo install config2args
```

# How to build locally
cargo >= `1.32.0` is required. Using [rustup](https://rustup.rs/) is a good way to install rust build tools.

```sh
$ git clone git@github.com:serihiro/config2args.git
$ cd config2args
$ cargo build --release
```

# Features
## Supports JSON file as a config file 
- YAML may be supported in the future ?

## Supports both of long key name (with `--`) and short key name (with `-`)
```sh
$ cat test.json
{
    "k": 1,
    "key": "hello"
}
$ config2args test.json
-k 1 --key hello
```
## Supports string (which includes numeric) and array
```sh
$ cat test.json
{
    "key1": "a",
    "key2": 1,
    "key3": 1.4,
    "key4": ["a", "b", "c"],
    "key5": [1, 1.4, "c"]
}
$ config2args test.json
--key1 a --key2 1 --key3 1.4 --key4 a b c --key5 1 1.4 c
```

## Supports ignoring key name
```sh
$ cat test.json
{
    "_key1": "a",
    "_key2": "b",
    "key3": "c"
}
$ config2args test.json
a b --key3 c
```

## Supports not only JSON object, like `"aaaa"`, `[1, 2, 3]`.
```sh
$ cat test.json
"abcd"
$ config2args test.json
abcd
```

```sh
$ cat test.json
[1,2,3]
$ config2args test.json
1 2 3
```

## Supports netsted object
```sh
$ cat test.json
{
    "key1": 1,
    "key2": 2,
    "key3": 3,
    "key4": {
        "k1" : 4,
        "k2" : 5,
        "a": 6
    },
    "z": {
        "key5": 7,
        "b": 8
    }
}
$ config2args test.json
--key1 1 --key2 2 --key3 3 --key4.k1 4 --key4.k2 5 --key4.a 6 --z.key5 7 --z.b 8
```
## Supports [tera](https://tera.netlify.com/) template engine
If the file name of the input file ends with `.tera`, the file is evalued as a tera template.

```sh
$ cat test.json.tera
{
    "_setup_variable_for_tera": "{% set my_var = now() | date(format=\"%Y%m%d%H%M%S\") %}",
    "output": "logs/{{my_var}}"
}
$ config2args test.json.tera
--output logs/20190323005419
```

# Motivation
In many cases, machine learning scripts are implemented with many CLI options.  
For example, I often execute a python script like this:

```bash
$ python train_imagenet.py \
  "$imagenet1k_base/train.ssv" \
  "$imagenet1k_base/val.ssv" \
  --root "$imagenet1k_base" \
  --mean "$imagenet1k_base/mean.npy" \
  --gpu 0 \
  --arch resnet50 \
  --batchsize "$batch_size" \
  --val_batchsize "$batch_size" \
  --epoch "$epoch" \
  --loaderjob 2 \
  --out "$output_path"
```

Of course, this is written in one shell script.
But this style is not easy to read and update :(

So I wanted to manage these options with more comfortable format, like JSON or YAML.

```bash
$ python train_imagenet.py `config2args config.json`
```

# LICENSE
MIT
