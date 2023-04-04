use pyo3::prelude::*;
pub mod fracdiff;
pub mod entropy;
pub mod sadf;
pub mod microstructure;
pub mod rollstats;
pub mod rollregression;
pub mod orderblock;

use crate::entropy::Entropy;
use crate::fracdiff::FracDiff;
use crate::microstructure::MicroStructucture;
use crate::rollstats::RollStats;
use crate::rollregression::RollRegression;
use crate::orderblock::OrderBlockDetecter;



#[pymodule]
#[pyo3(name = "features")]
fn features(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<MicroStructucture>()?;
    m.add_class::<FracDiff>()?;
    m.add_class::< Entropy>()?;
    m.add_class::<RollStats>()?;
    m.add_class::<RollRegression>()?;
    m.add_class::<OrderBlockDetecter>()?;
    Ok(())
}