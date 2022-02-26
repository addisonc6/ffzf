use pyo3::prelude::*;
use rayon::prelude::*;
use crate::scorer::*;

#[pyfunction(algorithm = "\"levenshtein\"")]
pub fn closest(target: &str, options: Vec<&str>, algorithm: &str) -> String {
    let scorer = match algorithm.to_uppercase().as_str() {
        "JARO" => jaro_distance,
        "JAROWINKLER" => jaro_winkler_distance,
        "HAMMING" => hamming_distance,
        "LEVENSHTEIN" => levenshtein_distance,
        _ => panic!("Invalid Algorithm"),
    };
    let mut score = f64::MAX;
    let mut best = "";
    let scores: Vec<(f64, &&str)> = options
        .par_iter()
        .map(|option| (scorer(target, option).unwrap(), option))
        .collect::<Vec<_>>();
    if algorithm.to_uppercase().as_str() == "LEVENSHTEIN"
        || algorithm.to_uppercase().as_str() == "HAMMING"
    {
        for (s, option) in scores {
            if s < score {
                score = s;
                best = option;
            }
        }
    } else {
        score = f64::MIN;
        for (s, option) in scores {
            if s > score {
                score = s;
                best = option;
            }
        }
    }
    return best.to_owned();
}

#[pyfunction(algorithm = "\"levenshtein\"")]
pub fn n_closest(
    target: &str,
    options: Vec<&str>,
    n: usize,
    algorithm: &str,
) -> Vec<String> {
    let scorer = match algorithm.to_uppercase().as_str() {
        "JARO" => jaro_distance,
        "JAROWINKLER" => jaro_winkler_distance,
        "HAMMING" => hamming_distance,
        "LEVENSHTEIN" => levenshtein_distance,
        _ => panic!("Invalid Algorithm"),
    };
    let mut scores = options
        .par_iter()
        .map(|option| (option, scorer(target, option).unwrap()))
        .collect::<Vec<_>>();
    if algorithm.to_uppercase().as_str() == "LEVENSHTEIN"
        || algorithm.to_uppercase().as_str() == "HAMMING"
    {
        scores.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    } else {
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    }
    let mut best: Vec<String> = Vec::new();
    for (option, _) in scores.iter().take(n) {
        best.push(String::from(**option));
    }
    return best;
}