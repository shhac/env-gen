extern crate yaml_rust;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use yaml_rust::yaml;

fn get_file_name() -> String {
  let args: Vec<_> = env::args().collect();
  if args.len() > 0 {
    let file_name: &str = &*args[1];
    return file_name.to_owned();
  } else {
    return "".to_owned();
  }
}

fn get_file_contents(file_name: &str) -> String {
  let mut f = File::open(&file_name).unwrap();
  let mut s = String::new();
  f.read_to_string(&mut s).unwrap();
  return s;
}

pub fn read() -> Vec<yaml::Yaml> {
    let file_name: &str = &*get_file_name();
    let file_contents = get_file_contents(file_name);
    let y: Vec<yaml::Yaml> = yaml::YamlLoader::load_from_str(&file_contents).unwrap();
    return y;
}

fn print_indent(indent: usize) {
    for _ in 0..indent {
        print!("    ");
    }
}

fn dump_node(doc: &yaml::Yaml, indent: usize) {
    match *doc {
        yaml::Yaml::Array(ref v) => {
            for x in v {
                dump_node(x, indent + 1);
            }
        }
        yaml::Yaml::Hash(ref h) => {
            for (k, v) in h {
                print_indent(indent);
                println!("{:?}:", k);
                dump_node(v, indent + 1);
            }
        }
        _ => {
            print_indent(indent);
            println!("{:?}", doc);
        }
    }
}

pub fn dump_yaml(docs: Vec<yaml::Yaml>) {
  for doc in &docs {
    println!("---");
    dump_node(doc, 0);
  }
}
