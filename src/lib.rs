//! This library provides rust implementations for some stemmer algorithms
//! written in the [snowball language](https://snowballstem.org/).
//!
//!
//! All algorithms expect the input to already be lowercased.
//!
//! # Usage
//! ```toml
//! [dependencies]
//! rust-stemmers = "^1.0"
//! ```
//!
//! ```rust
//! extern crate rust_stemmers;
//!
//! use rust_stemmers::{Algorithm, Stemmer};
//!
//! fn main() {
//!    let en_stemmer = Stemmer::create(Algorithm::English);
//!    assert_eq!(en_stemmer.stem("fruitlessly"), "fruitless");
//! }
//! ```
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::borrow::Cow;

mod snowball;

use snowball::SnowballEnv;
use snowball::algorithms;

/// Enum of all supported algorithms.
/// Check the [Snowball-Website](https://snowballstem.org/) for details.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
pub enum Algorithm {
    Arabic,
    Armenian,
    Basque,
    Catalan,
    Czech,
    Danish,
    Dutch,
    DutchPorter,
    English,
    Esperanto,
    Estonian,
    Finnish,
    French,
    German,
    Greek,
    Hindi,
    Hungarian,
    Indonesian,
    Irish,
    Italian,
    Lithuanian,
    Nepali,
    Norwegian,
    Persian,
    Polish,
    Porter,
    Portuguese,
    Romanian,
    Russian,
    Serbian,
    Sesotho,
    Spanish,
    Swedish,
    Tamil,
    Turkish,
    Yiddish,
}

/// Wrapps a usable interface around the actual stemmer implementation
pub struct Stemmer {
    stemmer: fn(&mut SnowballEnv) -> bool,
}

impl Stemmer {
    /// Create a new stemmer from an algorithm
    pub fn create(lang: Algorithm) -> Self {
        match lang {
            Algorithm::Arabic => Stemmer { stemmer: algorithms::arabic_stemmer::stem },
            Algorithm::Armenian => Stemmer { stemmer: algorithms::armenian_stemmer::stem },
            Algorithm::Basque => Stemmer { stemmer: algorithms::basque_stemmer::stem },
            Algorithm::Catalan => Stemmer { stemmer: algorithms::catalan_stemmer::stem },
            Algorithm::Czech => Stemmer { stemmer: algorithms::czech_stemmer::stem },
            Algorithm::Danish => Stemmer { stemmer: algorithms::danish_stemmer::stem },
            Algorithm::Dutch => Stemmer { stemmer: algorithms::dutch_stemmer::stem },
            Algorithm::DutchPorter => Stemmer { stemmer: algorithms::dutch_porter_stemmer::stem },
            Algorithm::English => Stemmer { stemmer: algorithms::english_stemmer::stem },
            Algorithm::Esperanto => Stemmer { stemmer: algorithms::esperanto_stemmer::stem },
            Algorithm::Estonian => Stemmer { stemmer: algorithms::estonian_stemmer::stem },
            Algorithm::Finnish => Stemmer { stemmer: algorithms::finnish_stemmer::stem },
            Algorithm::French => Stemmer { stemmer: algorithms::french_stemmer::stem },
            Algorithm::German => Stemmer { stemmer: algorithms::german_stemmer::stem },
            Algorithm::Greek => Stemmer { stemmer: algorithms::greek_stemmer::stem },
            Algorithm::Hindi => Stemmer { stemmer: algorithms::hindi_stemmer::stem },
            Algorithm::Hungarian => Stemmer { stemmer: algorithms::hungarian_stemmer::stem },
            Algorithm::Indonesian => Stemmer { stemmer: algorithms::indonesian_stemmer::stem },
            Algorithm::Irish => Stemmer { stemmer: algorithms::irish_stemmer::stem },
            Algorithm::Italian => Stemmer { stemmer: algorithms::italian_stemmer::stem },
            Algorithm::Lithuanian => Stemmer { stemmer: algorithms::lithuanian_stemmer::stem },
            Algorithm::Nepali => Stemmer { stemmer: algorithms::nepali_stemmer::stem },
            Algorithm::Norwegian => Stemmer { stemmer: algorithms::norwegian_stemmer::stem },
            Algorithm::Persian => Stemmer { stemmer: algorithms::persian_stemmer::stem },
            Algorithm::Polish => Stemmer { stemmer: algorithms::polish_stemmer::stem },
            Algorithm::Porter => Stemmer { stemmer: algorithms::porter_stemmer::stem },
            Algorithm::Portuguese => Stemmer { stemmer: algorithms::portuguese_stemmer::stem },
            Algorithm::Romanian => Stemmer { stemmer: algorithms::romanian_stemmer::stem },
            Algorithm::Russian => Stemmer { stemmer: algorithms::russian_stemmer::stem },
            Algorithm::Serbian => Stemmer { stemmer: algorithms::serbian_stemmer::stem },
            Algorithm::Sesotho => Stemmer { stemmer: algorithms::sesotho_stemmer::stem },
            Algorithm::Spanish => Stemmer { stemmer: algorithms::spanish_stemmer::stem },
            Algorithm::Swedish => Stemmer { stemmer: algorithms::swedish_stemmer::stem },
            Algorithm::Tamil => Stemmer { stemmer: algorithms::tamil_stemmer::stem },
            Algorithm::Turkish => Stemmer { stemmer: algorithms::turkish_stemmer::stem },
            Algorithm::Yiddish => Stemmer { stemmer: algorithms::yiddish_stemmer::stem },
        }
    }

    /// Stem a single word
    /// Please note, that the input is expected to be all lowercase (if that is applicable).
    pub fn stem<'a>(&self, input: &'a str) -> Cow<'a, str> {
        let mut env = SnowballEnv::create(input);
        (self.stemmer)(&mut env);
        env.get_current()
    }
}



#[cfg(test)]
mod tests {
    use super::{Stemmer, Algorithm};

    fn stemms_to(lhs: &str, rhs: &str, stemmer: Algorithm) {
        assert_eq!(Stemmer::create(stemmer).stem(lhs), rhs);
    }

    #[test]
    fn german_test() {
        use std::fs;
        use std::io;
        use std::io::BufRead;

        let vocab = io::BufReader::new(fs::File::open("test_data/voc_ger.txt").unwrap());
        let result = io::BufReader::new(fs::File::open("test_data/res_ger.txt").unwrap());

        let lines = vocab.lines().zip(result.lines());

        for (voc, res) in lines {
            stemms_to(voc.unwrap().as_str(),
                      res.unwrap().as_str(),
                      Algorithm::German);
        }
    }

    #[test]
    fn english_test() {
        use std::fs;
        use std::io;
        use std::io::BufRead;

        let vocab = io::BufReader::new(fs::File::open("test_data/voc_en.txt").unwrap());
        let result = io::BufReader::new(fs::File::open("test_data/res_en.txt").unwrap());

        let lines = vocab.lines().zip(result.lines());

        for (voc, res) in lines {
            stemms_to(voc.unwrap().as_str(),
                      res.unwrap().as_str(),
                      Algorithm::English);
        }
    }

    #[test]
    fn porter_test() {
        let cases = [
            ("caresses", "caress"),
            ("ponies", "poni"),
            ("ties", "ti"),
            ("cats", "cat"),
            ("feed", "feed"),
            ("agreed", "agre"),
            ("disabled", "disabl"),
            ("matting", "mat"),
            ("mating", "mate"),
            ("meeting", "meet"),
            ("milling", "mill"),
            ("messing", "mess"),
            ("meetings", "meet"),
        ];

        for &(word, expected) in &cases {
            stemms_to(word, expected, Algorithm::Porter);
        }
    }

    #[test]
    fn additional_algorithms_test() {
        let cases = [
            (Algorithm::Basque, "abadeak", "abade"),
            (Algorithm::Catalan, "abacial", "abac"),
            (Algorithm::DutchPorter, "aachen", "aach"),
            (Algorithm::Esperanto, "0-dimensia", "0-dimensi"),
            (Algorithm::Estonian, "aadlike", "aadliku"),
            (Algorithm::Hindi, "अँगरेजी", "अँगरेज"),
            (Algorithm::Indonesian, "aadalah", "aada"),
            (Algorithm::Irish, "abairteach", "abairt"),
            (Algorithm::Lithuanian, "abatas", "abat"),
            (Algorithm::Nepali, "अँकाइनु", "अँकाइ"),
            (Algorithm::Persian, "آئتیوس", "آیتیوس"),
            (Algorithm::Polish, "aaronem", "aaron"),
            (Algorithm::Serbian, "abadžija", "abadžij"),
            (Algorithm::Sesotho, "abamnyama", "abamnyam"),
            (Algorithm::Yiddish, "אַבֿ־בית־דין", "אב־בית־די"),
        ];

        for &(algorithm, word, expected) in &cases {
            stemms_to(word, expected, algorithm);
        }
    }

    #[test]
    fn czech_test() {
        use std::fs;
        use std::io;
        use std::io::BufRead;

        let vocab = io::BufReader::new(fs::File::open("test_data/voc_cs.txt").unwrap());
        let result = io::BufReader::new(fs::File::open("test_data/res_cs.txt").unwrap());

        let lines = vocab.lines().zip(result.lines());

        for (voc, res) in lines {
            stemms_to(voc.unwrap().as_str(),
                      res.unwrap().as_str(),
                      Algorithm::Czech);
        }
    }

    #[test]
    fn french_test() {
        use std::fs;
        use std::io;
        use std::io::BufRead;

        let vocab = io::BufReader::new(fs::File::open("test_data/voc_fr.txt").unwrap());
        let result = io::BufReader::new(fs::File::open("test_data/res_fr.txt").unwrap());

        let lines = vocab.lines().zip(result.lines());

        for (voc, res) in lines {
            stemms_to(voc.unwrap().as_str(),
                      res.unwrap().as_str(),
                      Algorithm::French);
        }
    }

    #[test]
    fn spanish_test() {
        use std::fs;
        use std::io;
        use std::io::BufRead;

        let vocab = io::BufReader::new(fs::File::open("test_data/voc_es.txt").unwrap());
        let result = io::BufReader::new(fs::File::open("test_data/res_es.txt").unwrap());

        let lines = vocab.lines().zip(result.lines());

        for (voc, res) in lines {
            stemms_to(voc.unwrap().as_str(),
                      res.unwrap().as_str(),
                      Algorithm::Spanish);
        }
    }

    #[test]
    fn portuguese_test() {
        use std::fs;
        use std::io;
        use std::io::BufRead;

        let vocab = io::BufReader::new(fs::File::open("test_data/voc_pt.txt").unwrap());
        let result = io::BufReader::new(fs::File::open("test_data/res_pt.txt").unwrap());

        let lines = vocab.lines().zip(result.lines());

        for (voc, res) in lines {
            stemms_to(voc.unwrap().as_str(),
                      res.unwrap().as_str(),
                      Algorithm::Portuguese);
        }
    }

    #[test]
    fn italian_test() {
        use std::fs;
        use std::io;
        use std::io::BufRead;

        let vocab = io::BufReader::new(fs::File::open("test_data/voc_it.txt").unwrap());
        let result = io::BufReader::new(fs::File::open("test_data/res_it.txt").unwrap());

        let lines = vocab.lines().zip(result.lines());

        for (voc, res) in lines {
            stemms_to(voc.unwrap().as_str(),
                      res.unwrap().as_str(),
                      Algorithm::Italian);
        }
    }

    #[test]
    fn romanian_test() {
        use std::fs;
        use std::io;
        use std::io::BufRead;

        let vocab = io::BufReader::new(fs::File::open("test_data/voc_ro.txt").unwrap());
        let result = io::BufReader::new(fs::File::open("test_data/res_ro.txt").unwrap());

        let lines = vocab.lines().zip(result.lines());

        for (voc, res) in lines {
            stemms_to(voc.unwrap().as_str(),
                      res.unwrap().as_str(),
                      Algorithm::Romanian);
        }
    }

    #[test]
    fn russian_test() {
        use std::fs;
        use std::io;
        use std::io::BufRead;

        let vocab = io::BufReader::new(fs::File::open("test_data/voc_ru.txt").unwrap());
        let result = io::BufReader::new(fs::File::open("test_data/res_ru.txt").unwrap());

        let lines = vocab.lines().zip(result.lines());

        for (voc, res) in lines {
            stemms_to(voc.unwrap().as_str(),
                      res.unwrap().as_str(),
                      Algorithm::Russian);
        }
    }

    #[test]
    fn arabic_test() {
        use std::fs;
        use std::io;
        use std::io::BufRead;

        let vocab = io::BufReader::new(fs::File::open("test_data/voc_ar.txt").unwrap());
        let result = io::BufReader::new(fs::File::open("test_data/res_ar.txt").unwrap());

        let lines = vocab.lines().zip(result.lines());

        for (voc, res) in lines {
            stemms_to(voc.unwrap().as_str(),
                      res.unwrap().as_str(),
                      Algorithm::Arabic);
        }
    }

    #[test]
    fn finnish_test() {
        use std::fs;
        use std::io;
        use std::io::BufRead;

        let vocab = io::BufReader::new(fs::File::open("test_data/voc_fi.txt").unwrap());
        let result = io::BufReader::new(fs::File::open("test_data/res_fi.txt").unwrap());

        let lines = vocab.lines().zip(result.lines());

        for (voc, res) in lines {
            stemms_to(voc.unwrap().as_str(),
                      res.unwrap().as_str(),
                      Algorithm::Finnish);
        }
    }

    #[test]
    fn greek_test() {
        use std::fs;
        use std::io;
        use std::io::BufRead;

        let vocab = io::BufReader::new(fs::File::open("test_data/voc_el.txt").unwrap());
        let result = io::BufReader::new(fs::File::open("test_data/res_el.txt").unwrap());

        let lines = vocab.lines().zip(result.lines());

        for (voc, res) in lines {
            stemms_to(voc.unwrap().as_str(),
                      res.unwrap().as_str(),
                      Algorithm::Greek);
        }
    }

    #[test]
    fn norwegian_test() {
        use std::fs;
        use std::io;
        use std::io::BufRead;

        let vocab = io::BufReader::new(fs::File::open("test_data/voc_no.txt").unwrap());
        let result = io::BufReader::new(fs::File::open("test_data/res_no.txt").unwrap());

        let lines = vocab.lines().zip(result.lines());

        for (voc, res) in lines {
            stemms_to(voc.unwrap().as_str(),
                      res.unwrap().as_str(),
                      Algorithm::Norwegian);
        }
    }

}
