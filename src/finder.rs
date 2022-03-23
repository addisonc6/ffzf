use crate::scorer::*;
use pyo3::{exceptions::PyValueError, prelude::*};
use rayon::prelude::*;

/// closest(target, candidates, /, algorithm='levenshtein', case_sensitive=False)
/// --
///
/// Find the closest match to the target string in the candidates.
#[pyfunction(
    algorithm = "\"levenshtein\"",
    case_sensitive = "false",
    remove_whitespace = "false",
    threshold = "0.0"
)]
pub fn closest(
    target: &str,
    options: Vec<&str>,
    algorithm: &str,
    case_sensitive: bool,
    remove_whitespace: bool,
    threshold: f32,
) -> PyResult<String> {
    if options.len() == 0 {
        return Err(PyValueError::new_err("No options provided."));
    }
    if !is_valid_algorithm_name(algorithm) {
        return Err(PyValueError::new_err(format!(
            "Unsupported algorithm: {}. Supported algorithms are: LEVENSHTEIN, JARO, JAROWINKLER, HAMMING",
            algorithm
        )));
    }
    let scorer = get_scorer(algorithm);
    if algorithm.to_uppercase().as_str() == "HAMMING" {
        for option in &options {
            if option.len() != target.len() {
                return Err(PyValueError::new_err(
                    "Words must be the same length to use Hamming distance.",
                ));
            }
        }
    }
    let mut score = f32::MAX;
    let mut best = "";
    let scores: Vec<(f32, &&str)> = options
        .par_iter()
        .map(|option| {
            (
                scorer(target, option, case_sensitive, remove_whitespace, threshold).unwrap(),
                option,
            )
        })
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
        score = f32::MIN;
        for (s, option) in scores {
            if s > score {
                score = s;
                best = option;
            }
        }
    }
    return Ok(best.to_owned());
}

/// n_closest(target, candidates, n, /, algorithm='levenshtein', case_sensitive=False)
/// --
///
/// Find the n closest matches to the target string in the candidates.
#[pyfunction(
    algorithm = "\"levenshtein\"",
    case_sensitive = "false",
    remove_whitespace = "false",
    threshold = "0.0"
)]
pub fn n_closest(
    target: &str,
    options: Vec<&str>,
    n: usize,
    algorithm: &str,
    case_sensitive: bool,
    remove_whitespace: bool,
    threshold: f32,
) -> PyResult<Vec<String>> {
    if options.len() == 0 {
        return Err(PyValueError::new_err("No options provided."));
    }
    if n < 1 {
        return Err(PyValueError::new_err("n must be greater than 0."));
    }
    if !is_valid_algorithm_name(algorithm) {
        return Err(PyValueError::new_err(format!(
            "Unsupported algorithm: {}. Supported algorithms are: LEVENSHTEIN, JARO, JAROWINKLER, HAMMING",
            algorithm
        )));
    }
    let scorer = get_scorer(algorithm);
    if algorithm.to_uppercase().as_str() == "HAMMING" {
        for option in &options {
            if option.len() != target.len() {
                return Err(PyValueError::new_err(
                    "Words must be the same length to use Hamming distance.",
                ));
            }
        }
    }
    let mut scores = options
        .par_iter()
        .map(|option| {
            (
                option,
                scorer(target, option, case_sensitive, remove_whitespace, threshold).unwrap(),
            )
        })
        .collect::<Vec<_>>();
    sort_scores(&mut scores, algorithm);
    let mut best: Vec<String> = Vec::new();
    for (option, _) in scores.iter().take(n) {
        best.push(String::from(**option));
    }
    return Ok(best);
}

#[pyfunction(
    algorithm = "\"levenshtein\"",
    case_sensitive = "false",
    remove_whitespace = "false",
    threshold = "0.0"
)]
pub fn closest_index_pair(
    target: &str,
    text: &str,
    algorithm: &str,
    case_sensitive: bool,
    remove_whitespace: bool,
    threshold: f32,
) -> PyResult<(usize, usize)> {
    if text.len() == 0 {
        return Ok((0, 0));
    }
    if !is_valid_algorithm_name(algorithm) {
        return Err(PyValueError::new_err(format!(
            "Unsupported algorithm: {}. Supported algorithms are: LEVENSHTEIN, JARO, JAROWINKLER, HAMMING",
            algorithm
        )));
    }
    let scorer = get_scorer(algorithm);

    let mut scores: Vec<(usize, f32)> = (0..text.len() - target.len() + 1)
        .into_par_iter()
        .map(|i| {
            (
                i,
                scorer(
                    target,
                    &text[i..i + target.len()],
                    case_sensitive,
                    remove_whitespace,
                    threshold,
                )
                .unwrap(),
            )
        })
        .collect::<Vec<_>>();
    sort_scores(&mut scores, algorithm);
    return Ok((scores[0].0, scores[0].0 + target.len()));
}

fn is_valid_algorithm_name(algorithm: &str) -> bool {
    return ["LEVENSHTEIN", "JARO", "JAROWINKLER", "HAMMING"]
        .contains(&algorithm.to_uppercase().as_str());
}

fn get_scorer(algorithm: &str) -> fn(&str, &str, bool, bool, f32) -> PyResult<f32> {
    return match algorithm.to_uppercase().as_str() {
        "JARO" => jaro_similarity,
        "JAROWINKLER" => jaro_winkler_similarity,
        "HAMMING" => hamming_distance,
        "LEVENSHTEIN" => levenshtein_distance,
        _ => unreachable!(),
    };
}

fn sort_scores<T: Send>(scores: &mut Vec<(T, f32)>, algorithm: &str) {
    if scores.len() > 1000 {
        return par_sort_scores(scores, algorithm);
    }
    if algorithm.to_uppercase().as_str() == "LEVENSHTEIN"
        || algorithm.to_uppercase().as_str() == "HAMMING"
    {
        scores.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    } else {
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    }
}

fn par_sort_scores<T: Send>(scores: &mut Vec<(T, f32)>, algorithm: &str) {
    if algorithm.to_uppercase().as_str() == "LEVENSHTEIN"
        || algorithm.to_uppercase().as_str() == "HAMMING"
    {
        scores.par_sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    } else {
        scores.par_sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    }
}
