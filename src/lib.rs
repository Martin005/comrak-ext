use pyo3::prelude::*;

// We renamed the Rust library to `comrak_lib`
use comrak_lib::{markdown_to_html, markdown_to_commonmark, parse_document, Options as ComrakOptions, Arena};

// Import the Python option classes we defined
mod options;
use options::{PyExtensionOptions, PyParseOptions, PyRenderOptions, PyListStyleType};
mod astnode;
use astnode::{
    PyLineColumn, PySourcepos, PyNodeCode, PyNodeHtmlBlock, PyListDelimType, PyListType,
    PyTableAlignment, PyNodeList, PyNodeDescriptionItem, PyNodeCodeBlock, PyNodeHeading,
    PyNodeTable, PyNodeLink, PyNodeFootnoteDefinition, PyNodeFootnoteReference, PyNodeWikiLink,
    PyNodeShortCode, PyNodeMath, PyNodeMultilineBlockQuote, PyAlertType, PyNodeAlert,
    PyNodeValue, PyDocument, PyFrontMatter, PyBlockQuote, PyList, PyItem, PyDescriptionList,
    PyDescriptionItem, PyDescriptionTerm, PyDescriptionDetails, PyCodeBlock, PyHtmlBlock,
    PyParagraph, PyHeading, PyThematicBreak, PyFootnoteDefinition, PyTable, PyTableRow,
    PyTableCell, PyText, PyTaskItem, PySoftBreak, PyLineBreak, PyCode, PyHtmlInline, PyRaw,
    PyEmph, PyStrong, PyStrikethrough, PySuperscript, PyLink, PyImage, PyFootnoteReference,
    PyShortCode, PyMath, PyMultilineBlockQuote, PyEscaped, PyWikiLink, PyUnderline,
    PySubscript, PySpoileredText, PyEscapedTag, PyAlert, PyAstNode
};

/// Render a Markdown string to HTML, with optional Extension/Parse/Render overrides.
#[pyfunction(signature=(text, extension_options=None, parse_options=None, render_options=None))]
fn render_markdown(
    text: &str,
    extension_options: Option<PyExtensionOptions>,
    parse_options: Option<PyParseOptions>,
    render_options: Option<PyRenderOptions>,
) -> PyResult<String> {
    let mut opts = ComrakOptions::default();

    // If user provided custom extension options, apply them.
    if let Some(py_ext) = extension_options {
        py_ext.update_extension_options(&mut opts.extension);
    }

    if let Some(py_parse) = parse_options {
        py_parse.update_parse_options(&mut opts.parse);
    }

    if let Some(py_render) = render_options {
        py_render.update_render_options(&mut opts.render);
    }

    let html = markdown_to_html(text, &opts);
    Ok(html)
}

/// Convert a Markdown string to CommonMark format.
#[pyfunction(signature=(text, extension_options=None, parse_options=None, render_options=None))]
fn render_markdown_to_commonmark(
    text: &str,
    extension_options: Option<PyExtensionOptions>,
    parse_options: Option<PyParseOptions>,
    render_options: Option<PyRenderOptions>,
) -> PyResult<String> {
    let mut opts = ComrakOptions::default();

    // If user provided custom extension options, apply them.
    if let Some(py_ext) = extension_options {
        py_ext.update_extension_options(&mut opts.extension);
    }

    if let Some(py_parse) = parse_options {
        py_parse.update_parse_options(&mut opts.parse);
    }

    if let Some(py_render) = render_options {
        py_render.update_render_options(&mut opts.render);
    }

    let html = markdown_to_commonmark(text, &opts);
    Ok(html)
}

// Parse a Markdown string into a document structure and return as PyAstNode.
#[pyfunction(signature=(text, extension_options=None, parse_options=None, render_options=None))]
fn parse_markdown(
    py: Python,
    text: &str,
    extension_options: Option<PyExtensionOptions>,
    parse_options: Option<PyParseOptions>,
    render_options: Option<PyRenderOptions>,
) -> PyResult<Py<PyAstNode>> {
    let mut opts = ComrakOptions::default();

    // If user provided custom extension options, apply them.
    if let Some(py_ext) = extension_options {
        py_ext.update_extension_options(&mut opts.extension);
    }

    if let Some(py_parse) = parse_options {
        py_parse.update_parse_options(&mut opts.parse);
    }

    if let Some(py_render) = render_options {
        py_render.update_render_options(&mut opts.render);
    }

    let arena = Arena::new();
    let document = parse_document(&arena, text, &opts);
    let py_node = PyAstNode::from_comrak_node(py, document);
    Ok(py_node)
}

#[pymodule]
fn comrak(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Expose the function
    m.add_function(wrap_pyfunction!(render_markdown, m)?)?;
    m.add_function(wrap_pyfunction!(parse_markdown, m)?)?;
    m.add_function(wrap_pyfunction!(render_markdown_to_commonmark, m)?)?;

    // Expose the classes
    m.add_class::<PyExtensionOptions>()?;
    m.add_class::<PyParseOptions>()?;
    m.add_class::<PyRenderOptions>()?;
    m.add_class::<PyListStyleType>()?;
    m.add_class::<PyLineColumn>()?;
    m.add_class::<PySourcepos>()?;
    m.add_class::<PyNodeValue>()?;
    m.add_class::<PyDocument>()?;
    m.add_class::<PyFrontMatter>()?;
    m.add_class::<PyBlockQuote>()?;
    m.add_class::<PyList>()?;
    m.add_class::<PyItem>()?;
    m.add_class::<PyDescriptionList>()?;
    m.add_class::<PyDescriptionItem>()?;
    m.add_class::<PyDescriptionTerm>()?;
    m.add_class::<PyDescriptionDetails>()?;
    m.add_class::<PyCodeBlock>()?;
    m.add_class::<PyHtmlBlock>()?;
    m.add_class::<PyParagraph>()?;
    m.add_class::<PyHeading>()?;
    m.add_class::<PyThematicBreak>()?;
    m.add_class::<PyFootnoteDefinition>()?;
    m.add_class::<PyTable>()?;
    m.add_class::<PyTableRow>()?;
    m.add_class::<PyTableCell>()?;
    m.add_class::<PyText>()?;
    m.add_class::<PyTaskItem>()?;
    m.add_class::<PySoftBreak>()?;
    m.add_class::<PyLineBreak>()?;
    m.add_class::<PyCode>()?;
    m.add_class::<PyHtmlInline>()?;
    m.add_class::<PyRaw>()?;
    m.add_class::<PyEmph>()?;
    m.add_class::<PyStrong>()?;
    m.add_class::<PyStrikethrough>()?;
    m.add_class::<PySuperscript>()?;
    m.add_class::<PyLink>()?;
    m.add_class::<PyImage>()?;
    m.add_class::<PyFootnoteReference>()?;
    m.add_class::<PyShortCode>()?;
    m.add_class::<PyMath>()?;
    m.add_class::<PyMultilineBlockQuote>()?;
    m.add_class::<PyEscaped>()?;
    m.add_class::<PyWikiLink>()?;
    m.add_class::<PyUnderline>()?;
    m.add_class::<PySubscript>()?;
    m.add_class::<PySpoileredText>()?;
    m.add_class::<PyEscapedTag>()?;
    m.add_class::<PyAlert>()?;
    m.add_class::<PyNodeCode>()?;
    m.add_class::<PyNodeHtmlBlock>()?;
    m.add_class::<PyListDelimType>()?;
    m.add_class::<PyListType>()?;
    m.add_class::<PyTableAlignment>()?;
    m.add_class::<PyNodeList>()?;
    m.add_class::<PyNodeDescriptionItem>()?;
    m.add_class::<PyNodeCodeBlock>()?;
    m.add_class::<PyNodeHeading>()?;
    m.add_class::<PyNodeTable>()?;
    m.add_class::<PyNodeLink>()?;
    m.add_class::<PyNodeFootnoteDefinition>()?;
    m.add_class::<PyNodeFootnoteReference>()?;
    m.add_class::<PyNodeWikiLink>()?;
    m.add_class::<PyNodeShortCode>()?;
    m.add_class::<PyNodeMath>()?;
    m.add_class::<PyNodeMultilineBlockQuote>()?;
    m.add_class::<PyAlertType>()?;
    m.add_class::<PyNodeAlert>()?;
    m.add_class::<PyAstNode>()?;
    Ok(())
}
