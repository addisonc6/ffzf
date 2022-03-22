use crate::utils::char_vec;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::cmp::min;

/// levenshtein_distance(a, b, /, case_sensitive=False)
/// --
///
/// Calculate the Levenshtein distance between two strings.
#[pyfunction(case_sensitive = "false", _threshold = "0.0")]
pub fn levenshtein_distance(
    word1: &str,
    word2: &str,
    case_sensitive: bool,
    _threshold: f32,
) -> PyResult<f32> {
    let n = word1.len();
    let m = word2.len();
    let mut d = vec![vec![0; m + 1]; n + 1];
    let word1_chars = char_vec(word1, case_sensitive);
    let word2_chars = char_vec(word2, case_sensitive);
    for i in 0..=n {
        d[i][0] = i
    }
    for j in 0..=m {
        d[0][j] = j;
    }
    for i in 1..=n {
        for j in 1..=m {
            let sub_cost;
            if (i - 1 < n && j - 1 < m)
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
    Ok(d[n][m] as f32)
}

/// jaro_similarity(a, b, /, case_sensitive=False)
/// --
///
/// Calculate the Jaro similarity between two strings.
#[pyfunction(case_sensitive = "false", _threshold = "0.0")]
pub fn jaro_similarity(
    word1: &str,
    word2: &str,
    case_sensitive: bool,
    _threshold: f32,
) -> PyResult<f32> {
    if word1 == word2 {
        return Ok(1.0);
    }
    let n = word1.len();
    let m = word2.len();
    let word1_chars = char_vec(word1, case_sensitive);
    let word2_chars = char_vec(word2, case_sensitive);
    let max_dist = (usize::max(m, n) / 2) - 1;
    let mut matches = 0;
    let mut hash_word1 = vec![0; n];
    let mut hash_word2 = vec![0; m];
    for i in 0..n {
        let mut j = i32::max(i as i32 - max_dist as i32, 0);
        while j < usize::min(i + max_dist + 1, m) as i32 {
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
            if word1_chars[i] != word2_chars[point] {
                point += 1;
                transpositions += 1;
            } else {
                point += 1;
            }
        }
        transpositions /= 2;
    }
    let jaro_similarity = (matches as f32 / n as f32
        + matches as f32 / m as f32
        + (matches - transpositions) as f32 / matches as f32)
        / 3.0;
    Ok(jaro_similarity)
}

/// jaro_winkler_similarity(a, b, /, case_sensitive = False)
/// --
///
/// Calculate the Jaro-Winkler similarity between two strings.
#[pyfunction(case_sensitive = "false", threshold = "0.7")]
pub fn jaro_winkler_similarity(
    word1: &str,
    word2: &str,
    case_sensitive: bool,
    threshold: f32,
) -> PyResult<f32> {
    if threshold < 0.0 || threshold > 1.0 {
        return Err(PyValueError::new_err(
            "threshold must be between 0.0 and 1.0",
        ));
    }
    let mut jaro_similarity = jaro_similarity(word1, word2, case_sensitive, threshold).unwrap();
    let word1_chars = char_vec(word1, case_sensitive);
    let word2_chars = char_vec(word2, case_sensitive);
    if jaro_similarity > threshold {
        let mut prefix = 0;
        for i in 0..usize::min(word1.len(), word2.len()) {
            if word1_chars[i] != word2_chars[i] {
                break;
            }
            prefix += 1;
        }
        prefix = i32::min(4, prefix);
        jaro_similarity += 0.1 * prefix as f32 * (1.0 - jaro_similarity);
    }
    Ok(jaro_similarity)
}

/// hamming_distance(a, b, /, case_sensitive = False)
/// --
///
/// Calculate the Hamming distance between two strings.
#[pyfunction(case_sensitive = "false", _threshold = "0.0")]
pub fn hamming_distance(
    word1: &str,
    word2: &str,
    case_sensitive: bool,
    _threshold: f32,
) -> PyResult<f32> {
    let word1_chars = char_vec(word1, case_sensitive);
    let word2_chars = char_vec(word2, case_sensitive);
    if word1.len() != word2.len() {
        return Err(PyValueError::new_err(
            "Words must be the same length to use Hamming distance",
        ));
    }
    let mut distance = 0;
    for (i, j) in word1_chars.iter().zip(word2_chars.iter()) {
        if i != j {
            distance += 1;
        }
    }
    Ok(distance as f32)
}
