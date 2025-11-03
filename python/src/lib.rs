use pyo3::prelude::*;

/// Render a .shapdf script to a PDF file.
///
/// Args:
///     script (str): The .shapdf script content
///     output_path (str): Path to write the output PDF file
///
/// Raises:
///     RuntimeError: If PDF generation fails
///
/// Example:
///     >>> import pyshapdf
///     >>> script = '''
///     ... page letter
///     ... circle 100mm 150mm 20mm color=blue
///     ... line 50mm 50mm 150mm 100mm width=2mm color=red
///     ... '''
///     >>> pyshapdf.render_script(script, "output.pdf")
#[pyfunction]
fn render_script(script: &str, output_path: &str) -> PyResult<()> {
    shapdf::render_script_to_pdf(script, output_path)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("PDF generation failed: {}", e)))
}

/// Python bindings for shapdf - Create Shapes into PDF
///
/// This module provides Python bindings to the shapdf Rust library,
/// enabling efficient programmable generation of shapes in PDF format.
///
/// Functions:
///     render_script(script: str, output_path: str) -> None
///         Render a .shapdf script to a PDF file
///
/// Example:
///     >>> import pyshapdf
///     >>> script = '''
///     ... page letter
///     ... circle 100mm 150mm 20mm color=blue
///     ... rectangle 50mm 50mm 40mm 30mm color=green angle=45deg anchor=center
///     ... line 50mm 200mm 150mm 250mm width=2mm color=red cap=round
///     ... '''
///     >>> pyshapdf.render_script(script, "shapes.pdf")
#[pymodule]
fn pyshapdf(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(render_script, m)?)?;
    Ok(())
}
