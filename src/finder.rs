use crate::internal_scorer::*;
use crate::utils::char_vec;
use ordered_float::OrderedFloat;
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
    let closest_option;
    let processed_target = char_vec(target, case_sensitive, remove_whitespace);
    if algorithm.to_uppercase().as_str() == "LEVENSHTEIN"
        || algorithm.to_uppercase().as_str() == "HAMMING"
    {
        closest_option = options
            .into_par_iter()
            .min_by_key(|option| {
                OrderedFloat(
                    scorer(option, &processed_target, case_sensitive, remove_whitespace, threshold).expect(
                        format!(
                            "Could not calcuate score with algorithm {} between {} and {}",
                            algorithm, target, option
                        )
                        .as_str(),
                    ),
                )
            })
            .expect("No scored values were present.")
            .to_string();
    } else {
        closest_option = options
            .into_par_iter()
            .max_by_key(|option| {
                OrderedFloat(
                    scorer(option, &processed_target, case_sensitive, remove_whitespace, threshold).expect(
                        format!(
                            "Could not calcuate score with algorithm {} between {} and {}",
                            algorithm, target, option
                        )
                        .as_str(),
                    ),
                )
            })
            .expect("No scored values were present.")
            .to_string();
    }
    Ok(closest_option)
}

#[pyfunction(
    algorithm = "\"levenshtein\"",
    case_sensitive = "false",
    remove_whitespace = "false",
    threshold = "0.0"
)]
pub fn closest_with_score(
    target: &str,
    options: Vec<&str>,
    algorithm: &str,
    case_sensitive: bool,
    remove_whitespace: bool,
    threshold: f32,
) -> PyResult<(String, f32)> {
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
    let closest_option;
    let processed_target = char_vec(target, case_sensitive, remove_whitespace);
    if algorithm.to_uppercase().as_str() == "LEVENSHTEIN"
        || algorithm.to_uppercase().as_str() == "HAMMING"
    {
        closest_option = options
            .into_par_iter()
            .min_by_key(|option| {
                OrderedFloat(
                    scorer(option, &processed_target, case_sensitive, remove_whitespace, threshold).expect(
                        format!(
                            "Could not calcuate score with algorithm {} between {} and {}",
                            algorithm, target, option
                        )
                        .as_str(),
                    ),
                )
            })
            .expect("No scored values were present.")
            .to_string();
    } else {
        closest_option = options
            .into_par_iter()
            .max_by_key(|option| {
                OrderedFloat(
                    scorer(option, &processed_target, case_sensitive, remove_whitespace, threshold).expect(
                        format!(
                            "Could not calcuate score with algorithm {} between {} and {}",
                            algorithm, target, option
                        )
                        .as_str(),
                    ),
                )
            })
            .expect("No scored values were present.")
            .to_string();
    }
    Ok((closest_option.clone(), scorer(
        closest_option.as_str(),
        &processed_target,
        case_sensitive,
        remove_whitespace,
        threshold,
    ).expect(
        format!(
            "Could not calcuate score with algorithm {} between {} and {}",
            algorithm, target, closest_option
        )
        .as_str(),
    )))
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
    let processed_target = char_vec(target, case_sensitive, remove_whitespace);
    let mut scores = options
        .par_iter()
        .map(|option| {
            (
                option,
                scorer(option, &processed_target, case_sensitive, remove_whitespace, threshold).expect(
                    format!(
                        "Could not calcuate score with algorithm {} between {} and {}",
                        algorithm, target, option
                    )
                    .as_str(),
                ),
            )
        })
        .collect::<Vec<_>>();
    sort_scores(&mut scores, algorithm);
    let mut best = Vec::with_capacity(n);
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
pub fn n_closest_with_score(
    target: &str,
    options: Vec<&str>,
    n: usize,
    algorithm: &str,
    case_sensitive: bool,
    remove_whitespace: bool,
    threshold: f32,
) -> PyResult<Vec<(String, f32)>> {
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
    let processed_target = char_vec(target, case_sensitive, remove_whitespace);
    let mut scores = options
        .par_iter()
        .map(|option| {
            (
                option,
                scorer(option, &processed_target, case_sensitive, remove_whitespace, threshold).expect(
                    format!(
                        "Could not calcuate score with algorithm {} between {} and {}",
                        algorithm, target, option
                    )
                    .as_str(),
                ),
            )
        })
        .collect::<Vec<_>>();
    sort_scores(&mut scores, algorithm);
    let mut best = Vec::with_capacity(n);
    for (option, score) in scores.iter().take(n) {
        best.push((String::from(**option), *score));
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
    let processed_target = char_vec(target, case_sensitive, remove_whitespace);
    let mut scores: Vec<(usize, f32)> = (0..text.len() - target.len() + 1)
        .into_par_iter()
        .map(|i| {
            (
                i,
                scorer(
                    &text[i..i + target.len()],
                    &processed_target,
                    case_sensitive,
                    remove_whitespace,
                    threshold,
                )
                .expect(
                    format!(
                        "Could not calcuate score with algorithm {} between {} and {}",
                        algorithm,
                        target,
                        &text[i..i + target.len()]
                    )
                    .as_str(),
                ),
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

fn get_scorer(algorithm: &str) -> fn(&str, &Vec<char>, bool, bool, f32) -> PyResult<f32> {
    return match algorithm.to_uppercase().as_str() {
        "JARO" => jaro_similarity_target_preprocessed,
        "JAROWINKLER" => jaro_winkler_similarity_target_preprocessed,
        "HAMMING" => hamming_distance_target_preprocessed,
        "LEVENSHTEIN" => levenshtein_distance_target_preprocessed,
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
        scores.sort_by(|a, b| a.1.partial_cmp(&b.1).expect("Could not compare scores."));
    } else {
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).expect("Could not compare scores."));
    }
}

fn par_sort_scores<T: Send>(scores: &mut Vec<(T, f32)>, algorithm: &str) {
    if algorithm.to_uppercase().as_str() == "LEVENSHTEIN"
        || algorithm.to_uppercase().as_str() == "HAMMING"
    {
        scores
            .par_sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).expect("Could not compare scores."));
    } else {
        scores
            .par_sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).expect("Could not compare scores."));
    }
}
