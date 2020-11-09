pub mod asyncio;
pub mod http;
pub mod server;
pub mod utils;

use pyo3::prelude::*;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

///
/// Wraps all our existing pyobjects together in the module
///
#[pymodule]
fn pyre(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<server::protocols::h11::RustProtocol>()?;
    Ok(())
}
