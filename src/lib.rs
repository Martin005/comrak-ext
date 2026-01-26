use pyo3::prelude::*;

// We renamed the Rust library to `comrak_lib`
use comrak_lib::{
    format_commonmark, format_html, format_xml, markdown_to_commonmark, markdown_to_commonmark_xml,
    markdown_to_html, parse_document, Arena, Options as ComrakOptions,
};

// Import the Python option classes we defined
mod options;
use options::{PyExtensionOptions, PyListStyleType, PyParseOptions, PyRenderOptions};
mod astnode;
use astnode::{
    PyAlert, PyAlertType, PyAstNode, PyBlockQuote, PyCode, PyCodeBlock, PyDescriptionDetails,
    PyDescriptionItem, PyDescriptionList, PyDescriptionTerm, PyDocument, PyEmph, PyEscaped,
    PyEscapedTag, PyFootnoteDefinition, PyFootnoteReference, PyFrontMatter, PyHeading, PyHeexBlock,
    PyHeexInline, PyHeexNode, PyHeexNodeComment, PyHeexNodeDirective, PyHeexNodeExpression,
    PyHeexNodeMultilineComment, PyHeexNodeTag, PyHighlight, PyHtmlBlock, PyHtmlInline, PyImage,
    PyItem, PyLineBreak, PyLineColumn, PyLink, PyList, PyListDelimType, PyListType, PyMath,
    PyMultilineBlockQuote, PyNodeAlert, PyNodeCode, PyNodeCodeBlock, PyNodeDescriptionItem,
    PyNodeFootnoteDefinition, PyNodeFootnoteReference, PyNodeHeading, PyNodeHeexBlock,
    PyNodeHtmlBlock, PyNodeLink, PyNodeList, PyNodeMath, PyNodeMultilineBlockQuote,
    PyNodeShortCode, PyNodeTable, PyNodeTaskItem, PyNodeValue, PyNodeWikiLink, PyParagraph, PyRaw,
    PyShortCode, PySoftBreak, PySourcepos, PySpoileredText, PyStrikethrough, PyStrong, PySubscript,
    PySubtext, PySuperscript, PyTable, PyTableAlignment, PyTableCell, PyTableRow, PyTaskItem,
    PyText, PyThematicBreak, PyUnderline, PyWikiLink,
};

/// Render a Markdown string to HTML, with optional Extension/Parse/Render overrides.
#[pyfunction(name = "markdown_to_html", signature=(text, extension_options=None, parse_options=None, render_options=None))]
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
#[pyfunction(name = "markdown_to_commonmark", signature=(text, extension_options=None, parse_options=None, render_options=None))]
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

#[pyfunction(name = "markdown_to_xml", signature=(text, extension_options=None, parse_options=None, render_options=None))]
fn render_markdown_to_commonmark_xml(
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

    let xml = markdown_to_commonmark_xml(text, &opts);
    Ok(xml)
}

// Parse a Markdown string into a document structure and return as PyAstNode.
#[pyfunction(name = "parse_document", signature=(text, extension_options=None, parse_options=None, render_options=None))]
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
    let py_node = PyAstNode::from_comrak_node(py, document, None);
    Ok(py_node)
}

#[pyfunction(name = "format_commonmark", signature=(root, extension_options=None, parse_options=None, render_options=None))]
fn ast_format_commonmark(
    py: Python,
    root: &PyAstNode,
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

    let mut output = String::new();
    let arena = Arena::new();
    let root_node = root.to_comrak_node(py, &arena);
    format_commonmark(root_node, &opts, &mut output).unwrap();
    Ok(output)
}

#[pyfunction(name = "format_html", signature=(root, extension_options=None, parse_options=None, render_options=None))]
fn ast_format_html(
    py: Python,
    root: &PyAstNode,
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

    let mut output = String::new();
    let arena = Arena::new();
    let root_node = root.to_comrak_node(py, &arena);
    format_html(root_node, &opts, &mut output).unwrap();
    Ok(output)
}

#[pyfunction(name = "format_xml", signature=(root, extension_options=None, parse_options=None, render_options=None))]
fn ast_format_xml(
    py: Python,
    root: &PyAstNode,
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

    let mut output = String::new();
    let arena = Arena::new();
    let root_node = root.to_comrak_node(py, &arena);
    format_xml(root_node, &opts, &mut output).unwrap();
    Ok(output)
}

#[pymodule]
fn comrak(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Expose the function
    m.add_function(wrap_pyfunction!(render_markdown, m)?)?;
    m.add_function(wrap_pyfunction!(parse_markdown, m)?)?;
    m.add_function(wrap_pyfunction!(render_markdown_to_commonmark, m)?)?;
    m.add_function(wrap_pyfunction!(render_markdown_to_commonmark_xml, m)?)?;
    m.add_function(wrap_pyfunction!(ast_format_commonmark, m)?)?;
    m.add_function(wrap_pyfunction!(ast_format_html, m)?)?;
    m.add_function(wrap_pyfunction!(ast_format_xml, m)?)?;

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
    m.add_class::<PyHeexNode>()?;
    m.add_class::<PyHeexNodeTag>()?;
    m.add_class::<PyHeexNodeExpression>()?;
    m.add_class::<PyHeexNodeDirective>()?;
    m.add_class::<PyHeexNodeComment>()?;
    m.add_class::<PyHeexNodeMultilineComment>()?;
    m.add_class::<PyHeexInline>()?;
    m.add_class::<PyHeexBlock>()?;
    m.add_class::<PyNodeHeexBlock>()?;
    m.add_class::<PyHighlight>()?;
    m.add_class::<PySubtext>()?;
    m.add_class::<PyNodeTaskItem>()?;
    Ok(())
}
