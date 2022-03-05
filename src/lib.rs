mod scorer;
mod finder;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use scorer::*;
use finder::*;

#[pymodule]
fn ffzf(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(levenshtein_distance))?;
    m.add_wrapped(wrap_pyfunction!(hamming_distance))?;
    m.add_wrapped(wrap_pyfunction!(jaro_similarity))?;
    m.add_wrapped(wrap_pyfunction!(jaro_winkler_similarity))?;
    m.add_wrapped(wrap_pyfunction!(closest))?;
    m.add_wrapped(wrap_pyfunction!(n_closest))?;
    m.add("LEVENSHTEIN", "LEVENSHTEIN")?;
    m.add("JARO", "JARO")?;
    m.add("JAROWINKLER", "JAROWINKLER")?;
    m.add("HAMMING", "HAMMING")?;
    Ok(())
}


