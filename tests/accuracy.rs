#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate tokei;
extern crate ignore;

use std::fs;

use regex::Regex;
use tokei::{Languages, Config};

lazy_static! {
    static ref LINES: Regex = Regex::new(r"\d+ lines").unwrap();
    static ref CODE: Regex = Regex::new(r"\d+ code").unwrap();
    static ref COMMENTS: Regex = Regex::new(r"\d+ comments").unwrap();
    static ref BLANKS: Regex = Regex::new(r"\d+ blanks").unwrap();
}

macro_rules! get_digit {
    ($regex:expr, $text:expr) => {{
        let matched = $regex.find(&$text).expect("Couldn't find category");
        matched.as_str().split_whitespace()
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap()
    }}
}

mod config {
    use tokei::*;

    /*
    #[test]
    fn extension_change() {
        use std::collections::HashMap;
        let mut languages = Languages::new();
        let config = Config {
            languages: {
                let mut map = HashMap::new();
                let mut config = LanguageConfig::new();
                config.extensions(vec![String::from("cpp")]);
                map.insert(LanguageType::C, config);

                Some(map)
            },
            ..Config::default()
        };

        languages.get_statistics(&["tests/data/cpp.cpp"], &[], &config);

        if languages.len() != 1 {
            panic!("wrong languages detected: expected just C, found {:?}",
                   languages.into_iter().collect::<Vec<_>>());
        }

        let (name, _) = languages.into_iter().next().unwrap();

        assert_eq!(LanguageType::C, name);
    }
    */

    #[test]
    fn treating_comments_as_code() {
        let mut languages = Languages::new();
        let config = Config {
            treat_doc_strings_as_comments: Some(true),
            ..Config::default()
        };

        languages.get_statistics(&["tests/data/python.py"], &[], &config);

        if languages.len() != 1 {
            panic!("wrong languages detected: expected just Python, found {:?}",
                   languages.into_iter().collect::<Vec<_>>());
        }

        let (_, language) = languages.into_iter().next().unwrap();

        assert_eq!(language.lines, 15);
        assert_eq!(language.code, 5);
        assert_eq!(language.comments, 7);
        assert_eq!(language.blanks, 3);
    }
}

include!(concat!(env!("OUT_DIR"), "/tests.rs"));
