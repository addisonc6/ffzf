use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use rayon::prelude::*;
use std::cmp::min;

#[pyfunction(algorithm = "\"levenshtein\"")]
pub fn closest_string_matching(target: &str, options: Vec<&str>, algorithm: &str) -> String {
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
    if algorithm.to_uppercase().as_str() == "LEVENSHTEIN" || algorithm.to_uppercase().as_str() == "HAMMING" {
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
pub fn n_closest_string_matching(
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
    if algorithm.to_uppercase().as_str() == "LEVENSHTEIN" || algorithm.to_uppercase().as_str() == "HAMMING" {
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

#[pyfunction]
pub fn levenshtein_distance(word1: &str, word2: &str) -> PyResult<f64> {
    let n = word1.len();
    let m = word2.len();
    let mut d = vec![vec![0; m + 1]; n + 1];
    let word1_chars = word1
        .chars()
        .map(|c| c.to_ascii_uppercase())
        .collect::<Vec<char>>();
    let word2_chars = word2
        .chars()
        .map(|c| c.to_ascii_uppercase())
        .collect::<Vec<char>>();
    for i in 0..=n {
        d[i][0] = i
    }
    for j in 0..=m {
        d[0][j] = j;
    }
    for i in 1..=n {
        for j in 1..=m {
            let sub_cost;
            if (i - 1 < word1_chars.len() && j - 1 < word2_chars.len())
                && word1_chars[i - 1] == word2_chars[j - 1]
            {
                sub_cost = 0;
            } else {
                sub_cost = 1;
            }
            d[i][j] = min(
                d[i - 1][j] + 1,
                min(d[i][j - 1] + 1, d[i - 1][j - 1] + sub_cost),
            )
        }
    }
    Ok(d[n][m] as f64)
}

#[pyfunction]
pub fn jaro_distance(word1: &str, word2: &str) -> PyResult<f64> {
    if word1 == word2 {
        return Ok(1.0);
    }
    let n = word1.len();
    let m = word2.len();
    let word1_chars = word1
        .chars()
        .map(|c| c.to_ascii_uppercase())
        .collect::<Vec<char>>();
    let word2_chars = word2
        .chars()
        .map(|c| c.to_ascii_uppercase())
        .collect::<Vec<char>>();
    let max_dist: i32 = (i32::max(m as i32, n as i32) / 2) - 1;
    let mut matches = 0;
    let mut hash_word1 = vec![0; n];
    let mut hash_word2 = vec![0; m];
    for i in 0..n {
        let mut j = i32::max(i as i32 - max_dist, 0);
        while j < i32::min(i as i32 + max_dist + 1, m as i32) {
            if word1_chars[i] == word2_chars[j as usize] && hash_word2[j as usize] == 0 {
                hash_word1[i] = 1;
                hash_word2[j as usize] = 1;
                matches += 1;
                break;
            }
            j += 1;
        }
    }
    if matches == 0 {
        return Ok(0.0);
    }
    let mut transpositions = 0;
    let mut point = 0;
    for i in 0..n {
        if hash_word1[i] != 0 {
            while hash_word2[point] == 0 {
                point += 1;
            }
            if word1_chars[i] != word2_chars[point as usize] {
                point += 1;
                transpositions += 1;
            } else {
                point += 1;
            }
        }
        transpositions /= 2;
    }
    let jaro_distance = (matches as f64 / n as f64
        + matches as f64 / m as f64
        + (matches - transpositions) as f64 / matches as f64)
        / 3.0;
    Ok(jaro_distance)
}

#[pyfunction]
pub fn jaro_winkler_distance(word1: &str, word2: &str) -> PyResult<f64> {
    let mut jaro_distance = jaro_distance(word1, word2).unwrap();
    let word1_chars = word1
        .chars()
        .map(|c| c.to_ascii_uppercase())
        .collect::<Vec<char>>();
    let word2_chars = word2
        .chars()
        .map(|c| c.to_ascii_uppercase())
        .collect::<Vec<char>>();
    if jaro_distance > 0.7 {
        let mut prefix = 0;
        for i in 0..i32::min(word1.len() as i32, word2.len() as i32) {
            if word1_chars[i as usize] != word2_chars[i as usize] {
                break;
            }
            prefix += 1;
        }
        prefix = i32::min(4, prefix);
        jaro_distance += 0.1 * prefix as f64 * (1.0 - jaro_distance);
    }
    Ok(jaro_distance)
}

#[pyfunction]
pub fn hamming_distance(word1: &str, word2: &str) -> PyResult<f64> {
    if word1.len() != word2.len() {
        return Err(PyValueError::new_err("Words must be the same length"));
    }
    let mut distance = 0;
    for (i, j) in word1.chars().zip(word2.chars()) {
        if i != j {
            distance += 1;
        }
    }
    Ok(distance as f64)
}
