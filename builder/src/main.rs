use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use glob::glob;
use simple_config_parser::Config;

mod app;
mod args;

const COMPONENT_PATH: &str = "web/components";
const CONST_PATH: &str = "web/const.cfg";

const IN_PATH: &str = "web/static";
const STATIC_IN: &str = "web/static";
const OUT_PATH: &str = "web/dist/static";

// const IN_PATH: &str = "web/template";
// const STATIC_IN: &str = "web/template";
// const OUT_PATH: &str = "web/dist/template";

fn main() {
    // Load constants
    let consts = Config::new().file(CONST_PATH).unwrap();
    println!(
        "Loaded Constants: {}",
        consts
            .data
            .iter()
            .map(|x| x[0].to_owned())
            .collect::<Vec<_>>()
            .join(", ")
    );

    // Load components
    let mut cmp = HashMap::new();
    for i in glob(&format!("{}/**/*.html", COMPONENT_PATH))
        .unwrap()
        .map(|x| x.unwrap())
    {
        println!("[*] Loading Component `{}`", i.to_string_lossy());
        let name = i.to_string_lossy().replace('\\', "/");
        let name = name
            .strip_prefix(COMPONENT_PATH)
            .unwrap()
            .strip_suffix(".html")
            .unwrap();

        let value = fs::read_to_string(i).unwrap();
        cmp.insert(name[1..].to_owned(), value);
    }

    // Remove Old Dist
    let _ = fs::remove_dir_all(OUT_PATH);
    fs::create_dir_all(OUT_PATH).unwrap();

    println!("[*] Copying Static Files");
    for i in glob(&format!("{}/**/*", STATIC_IN))
        .unwrap()
        .map(|x| x.unwrap())
    {
        if i.is_dir() {
            continue;
        }

        let new_path = PathBuf::from(OUT_PATH).join(i.strip_prefix(STATIC_IN).unwrap());

        fs::create_dir_all(new_path.parent().unwrap()).unwrap();
        fs::copy(&i, new_path).unwrap();
    }

    // Process Html files
    for i in glob(&format!("{}/**/*.html", IN_PATH))
        .unwrap()
        .map(|x| x.unwrap())
    {
        println!("[*] Processing Page `{}`", i.to_string_lossy());
        let value = fs::read_to_string(&i).unwrap();
        let mut new = substitute(&cmp, value);

        for i in consts.data.iter() {
            new = new.replace(&format!("{{{{{}}}}}", i[0].to_uppercase()), &i[1]);
        }

        let out_path = PathBuf::from(OUT_PATH).join(i.strip_prefix(IN_PATH).unwrap());
        fs::create_dir_all(out_path.parent().unwrap()).unwrap();
        fs::write(out_path, new).unwrap()
    }
}

fn substitute(cmp: &HashMap<String, String>, imp: String) -> String {
    let chars = imp.chars().collect::<Vec<_>>();
    let mut out = String::new();
    let mut working = String::new();
    let mut in_comment = false;

    let mut i = 0;
    while i < chars.len() - 4 {
        if Pattern("<!--") == &chars[i..i + 4] {
            in_comment = true;
            i += 4;
        }

        if Pattern("-->") == &chars[i..i + 3] {
            in_comment = false;
            i += 3;

            if let Some(i) = working.trim().strip_prefix("#INCLUDE").map(str::trim) {
                let format = cmp
                    .get(i)
                    .unwrap_or_else(|| panic!("Tried to include non existent file: `{}`", i));
                out.push_str(format);
                working.clear();
                continue;
            }

            out.push_str(&working);
            working.clear();
        }

        if in_comment {
            working.push(chars[i]);
            i += 1;
            continue;
        }

        out.push(chars[i]);
        i += 1;
    }

    out.push_str(&chars[(chars.len() - 4)..].iter().collect::<String>());
    out
}

struct Pattern(&'static str);

impl PartialEq<&[char]> for Pattern {
    fn eq(&self, other: &&[char]) -> bool {
        self.0.chars().zip(other.iter()).all(|(a, b)| a == *b)
    }
}
