use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    pub in_path: String,
    pub static_path: String,
    pub out_path: String,

    #[arg(long, default_value = "web/components")]
    pub component_path: String,
    #[arg(long, default_value = "web/const.cfg")]
    pub const_path: String,
}

/*
const IN_PATH: &str = "web/static";
const STATIC_IN: &str = "web/static";
const OUT_PATH: &str = "web/dist/static";

const IN_PATH: &str = "web/template";
const STATIC_IN: &str = "web/template";
const OUT_PATH: &str = "web/dist/template";
*/