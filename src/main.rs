use clap::{App, Arg};
use permutator::CartesianProductIterator;
use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use serde;
use serde::{Serialize, Deserialize};
use pbr::ProgressBar;
use std::collections::BTreeMap;
use regen::{Generator, Result};


#[derive(Debug, Deserialize, PartialEq)]
struct Builder {
    parts: Vec<FormatString>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
enum FormatString {
    Part(String),
    Regex(BTreeMap<String, String>),
}

fn main() {
    let matches = App::new("Captain Crunch")
        .version("0.1.0")
        .author("Thomas Graves <0o0o0o0o0@protonmail.ch>")
        .about("Captain Crunch is a modern wordlist generator that lets you specify a collection of character sets and then generate all possible permutations.")
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .value_name("FILE")
            .about("Sets the config file that defines how to generate words")
            .takes_value(true)
            .required(true))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .value_name("FILE")
            .about("Sets the file the wordlist will be written to")
            .takes_value(true)
            .required(true))
        .arg(Arg::new("progress")
            .short('p')
            .long("progress")
            .about("Display progress bar (VERY SLOW FOR LARGE SETS!)")
            .takes_value(false)
            .required(false))
        .get_matches();

    let format_strings = if let Some(i) = matches.value_of("config") {
        let f = std::fs::File::open(i).unwrap();
        let yaml: Builder = serde_yaml::from_reader(f).unwrap();
        parse_format_strings(yaml)
    } else {
        panic!();
    };

    let mut num_permus = 1;
    for arr in format_strings.iter() {
        num_permus *=  arr.len();
    }

    // Convert the `Vec<Vec<String>>` into a `Vec<Vec<&str>>`
    let tmp: Vec<Vec<&str>> = format_strings.iter()
        .map(|list| list.iter().map(AsRef::as_ref).collect::<Vec<&str>>())
        .collect();

    // Convert the `Vec<Vec<&str>>` into a `Vec<&[&str]>`
    let vector_of_arrays: Vec<&[&str]> = tmp.iter()
        .map(AsRef::as_ref).collect();

    let file = if let Some(i) = matches.value_of("output") {
        Arc::new(Mutex::new(OpenOptions::new()
                            .create(true)
                            .write(true)
                            .truncate(true)
                            .open(i)
                            .unwrap()))
    } else {
        panic!()
    };

    let progress = matches.is_present("progress");

    println!("Generating {:?} different permutations", num_permus);

    let pb = Arc::new(Mutex::new(ProgressBar::new(num_permus as u64)));

    CartesianProductIterator::new(&vector_of_arrays[..]).into_iter().par_bridge().for_each(|p| {
        writeln!(&mut file.lock().unwrap(), "{}", p.iter().map(|s| **s).collect::<Vec<&str>>().join(""));
        if progress {
            pb.lock().unwrap().inc();
        }
    });
    pb.lock().unwrap().finish_print("done");
}

fn parse_format_strings(builder: Builder) -> Vec<Vec<String>> {
    let mut all = Vec::new();

    for format_string in builder.parts.iter() {
        match format_string {
            FormatString::Part(part) => {
                let options = tokenize(part);
                all.push(options);
            },
            FormatString::Regex(regex) => {
                let mut strings = Vec::new();

                let mut out = Vec::new();
                let options = &regex["regex"];
                let mut gen = Generator::new(&options).unwrap();
                while gen.append_next(&mut out).is_some() {
                    let s = String::from_utf8_lossy(&out);
                    strings.push(String::from(s));
                    out.clear();
                }

                all.push(strings);
            }
        }
    }

    all
}

const SEPARATOR: char = '|';
const ESCAPE: char = '\\';

fn tokenize(string: &str) -> Vec<String> {
    let mut token = String::new();
    let mut tokens: Vec<String> = Vec::new();
    let mut chars = string.chars();
    while let Some(ch) = chars.next() {
        match ch {
            SEPARATOR => {
                tokens.push(token);
                token = String::new();
            },
            ESCAPE => {
                if let Some(next) = chars.next() {
                    token.push(next);
                }
            },
            _ => token.push(ch),
        }
    }
    tokens.push(token);
    tokens
}
