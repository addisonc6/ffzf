mod scorer;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use scorer::*;

#[pymodule]
fn ffzf(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(levenshtein_distance, m)?)?;
    m.add_function(wrap_pyfunction!(jaro_distance, m)?)?;
    m.add_function(wrap_pyfunction!(jaro_winkler_distance, m)?)?;
    m.add_function(wrap_pyfunction!(hamming_distance, m)?)?;
    m.add_function(wrap_pyfunction!(closest_string_matching, m)?)?;
    m.add_function(wrap_pyfunction!(n_closest_string_matching, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use super::*;
    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("", ""), 0.0);
        assert_eq!(levenshtein_distance("a", ""), 1.0);
        assert_eq!(levenshtein_distance("", "a"), 1.0);
        assert_eq!(levenshtein_distance("a", "a"), 0.0);
        assert_eq!(levenshtein_distance("a", "b"), 1.0);
        assert_eq!(levenshtein_distance("b", "a"), 1.0);
        assert_eq!(levenshtein_distance("a", "ab"), 1.0);
        assert_eq!(levenshtein_distance("ab", "a"), 1.0);
        assert_eq!(levenshtein_distance("a", "A"), 0.0);
        assert_eq!(levenshtein_distance("euphoria", "elation"), 7.0);
        assert_eq!(
            levenshtein_distance("triangle", "abcdefghijklmnopqrstuvwxyz"),
            24.0
        );
    }

    #[test]
    fn test_jaro_distance() {
        assert!(approx_eq!(
            f64,
            jaro_distance("subprime", "primers"),
            0.779762,
            epsilon = 0.001
        ));
        assert!(approx_eq!(
            f64,
            jaro_distance("SubPRIME", "Primers"),
            0.779762,
            epsilon = 0.001
        ));
        assert!(approx_eq!(
            f64,
            jaro_distance("SUBprime", "prImeRs"),
            0.779762,
            epsilon = 0.001
        ));
        assert!(approx_eq!(
            f64,
            jaro_distance("codify", "reify"),
            0.7,
            epsilon = 0.001
        ));
        assert!(approx_eq!(
            f64,
            jaro_distance("absolute", "resolute"),
            0.833333,
            epsilon = 0.001
        ));
        assert!(approx_eq!(
            f64,
            jaro_distance("anchors", "bank"),
            0.595238,
            epsilon = 0.001
        ));
        assert!(approx_eq!(
            f64,
            jaro_distance("out", "regaining"),
            0.0,
            epsilon = 0.001
        ));
    }

    #[test]
    fn test_jaro_winkler_distance() {
        assert!(approx_eq!(
            f64,
            jaro_winkler_distance("apples", "oranges"),
            0.642857,
            epsilon = 0.001
        ));
        assert!(approx_eq!(
            f64,
            jaro_winkler_distance("becoming", "trip"),
            0.458333,
            epsilon = 0.001
        ));
        assert!(approx_eq!(
            f64,
            jaro_winkler_distance("developers", "investment"),
            0.532682,
            epsilon = 0.001
        ));
        assert!(approx_eq!(
            f64,
            jaro_winkler_distance("trip", "drive"),
            0.633333,
            epsilon = 0.001
        ));
        assert!(approx_eq!(
            f64,
            jaro_winkler_distance("over", "out"),
            0.527778,
            epsilon = 0.001
        ));
    }

    #[test]
    fn test_hamming_distance() {
        assert_eq!(hamming_distance("apples", ""), 6.0);
        assert_eq!(hamming_distance("", ""), 0.0);
        assert_eq!(hamming_distance("a", ""), 1.0);
        assert_eq!(hamming_distance("", "a"), 1.0);
        assert_eq!(hamming_distance("batter", "bat"), 3.0); 
        assert_eq!(hamming_distance("ask", "mike"), 3.0);
        assert_eq!(hamming_distance("ask", "ask"), 0.0);
        assert_eq!(hamming_distance("ask", "asked"), 2.0);
        assert_eq!(hamming_distance("bask", "asked"), 5.0);
    }
}