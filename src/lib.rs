mod scorer;
mod finder;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use scorer::*;
use finder::*;

#[pymodule]
fn ffzf(py: Python, m: &PyModule) -> PyResult<()> {
    let scorers_module = PyModule::new(py, "scorers")?;
    scorers_module.add_wrapped(wrap_pyfunction!(levenshtein_distance))?;
    scorers_module.add_wrapped(wrap_pyfunction!(hamming_distance))?;
    scorers_module.add_wrapped(wrap_pyfunction!(jaro_similarity))?;
    scorers_module.add_wrapped(wrap_pyfunction!(jaro_winkler_similarity))?;
    let finders_module = PyModule::new(py, "finders")?;
    finders_module.add_wrapped(wrap_pyfunction!(closest))?;
    finders_module.add_wrapped(wrap_pyfunction!(n_closest))?;
    m.add_submodule(scorers_module)?;
    m.add_submodule(finders_module)?;
    
    // work around for bug registering submdules (https://github.com/PyO3/pyo3/issues/759)
    let mods = py.import("sys")?
        .getattr("modules")?;
    mods.set_item("ffzf.scorers", scorers_module)?;
    mods.set_item("ffzf.finders", finders_module)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use super::*;
    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("", "").unwrap(), 0.0);
        assert_eq!(levenshtein_distance("a", "").unwrap(), 1.0);
        assert_eq!(levenshtein_distance("", "a").unwrap(), 1.0);
        assert_eq!(levenshtein_distance("a", "a").unwrap(), 0.0);
        assert_eq!(levenshtein_distance("a", "b").unwrap(), 1.0);
        assert_eq!(levenshtein_distance("b", "a").unwrap(), 1.0);
        assert_eq!(levenshtein_distance("a", "ab").unwrap(), 1.0);
        assert_eq!(levenshtein_distance("ab", "a").unwrap(), 1.0);
        assert_eq!(levenshtein_distance("a", "A").unwrap(), 0.0);
        assert_eq!(levenshtein_distance("euphoria", "elation").unwrap(), 7.0);
        assert_eq!(
            levenshtein_distance("triangle", "abcdefghijklmnopqrstuvwxyz").unwrap(),
            24.0
        );
    }

    #[test]
    fn test_jaro_distance() {
        assert!(approx_eq!(
            f64,
            jaro_distance("subprime", "primers").unwrap(),
            0.779762,
            epsilon = 0.001
        ));
        assert!(approx_eq!(
            f64,
            jaro_distance("SubPRIME", "Primers").unwrap(),
            0.779762,
            epsilon = 0.001
        ));
        assert!(approx_eq!(
            f64,
            jaro_distance("SUBprime", "prImeRs").unwrap(),
            0.779762,
            epsilon = 0.001
        ));
        assert!(approx_eq!(
            f64,
            jaro_distance("codify", "reify").unwrap(),
            0.7,
            epsilon = 0.001
        ));
        assert!(approx_eq!(
            f64,
            jaro_distance("absolute", "resolute").unwrap(),
            0.833333,
            epsilon = 0.001
        ));
        assert!(approx_eq!(
            f64,
            jaro_distance("anchors", "bank").unwrap(),
            0.595238,
            epsilon = 0.001
        ));
        assert!(approx_eq!(
            f64,
            jaro_distance("out", "regaining").unwrap(),
            0.0,
            epsilon = 0.001
        ));
    }

    #[test]
    fn test_jaro_winkler_distance() {
        assert!(approx_eq!(
            f64,
            jaro_winkler_distance("apples", "oranges").unwrap(),
            0.642857,
            epsilon = 0.001
        ));
        assert!(approx_eq!(
            f64,
            jaro_winkler_distance("becoming", "trip").unwrap(),
            0.458333,
            epsilon = 0.001
        ));
        assert!(approx_eq!(
            f64,
            jaro_winkler_distance("developers", "investment").unwrap(),
            0.532682,
            epsilon = 0.001
        ));
        assert!(approx_eq!(
            f64,
            jaro_winkler_distance("trip", "drive").unwrap(),
            0.633333,
            epsilon = 0.001
        ));
        assert!(approx_eq!(
            f64,
            jaro_winkler_distance("over", "out").unwrap(),
            0.527778,
            epsilon = 0.001
        ));
    }

    #[test]
    fn test_hamming_distance() {
        assert_eq!(hamming_distance("apples", "").unwrap(), 6.0);
        assert_eq!(hamming_distance("", "").unwrap(), 0.0);
        assert_eq!(hamming_distance("a", "").unwrap(), 1.0);
        assert_eq!(hamming_distance("", "a").unwrap(), 1.0);
        assert_eq!(hamming_distance("batter", "bat").unwrap(), 3.0); 
        assert_eq!(hamming_distance("ask", "mike").unwrap(), 3.0);
        assert_eq!(hamming_distance("ask", "ask").unwrap(), 0.0);
        assert_eq!(hamming_distance("ask", "asked").unwrap(), 2.0);
        assert_eq!(hamming_distance("bask", "asked").unwrap(), 5.0);
    }
}