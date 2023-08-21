use std::{collections::HashMap, fs};

use clap::Parser;
use glob::glob;
use simple_config_parser::Config;

use crate::args::Args;

pub struct App {
    pub args: Args,
    pub consts: Config,
    pub components: HashMap<String, String>,
}

impl App {
    pub fn new() -> Self {
        let args = Args::parse();

        let consts = Config::new().file(&args.const_path).unwrap();
        println!(
            "Loaded Constants: {}",
            consts
                .data
                .iter()
                .map(|x| x[0].to_owned())
                .collect::<Vec<_>>()
                .join(", ")
        );

        let mut components = HashMap::new();
        for i in glob(&format!("{}/**/*.html", args.component_path))
            .unwrap()
            .map(|x| x.unwrap())
        {
            println!("[*] Loading Component `{}`", i.to_string_lossy());
            let name = i.to_string_lossy().replace('\\', "/");
            let name = name
                .strip_prefix(&args.component_path)
                .unwrap()
                .strip_suffix(".html")
                .unwrap();

            let value = fs::read_to_string(i).unwrap();
            components.insert(name[1..].to_owned(), value);
        }

        Self {
            args,
            consts,
            components,
        }
    }
}
