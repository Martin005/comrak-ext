use comrak_lib::nodes::*;
use pyo3::{prelude::*, pyclass, IntoPyObjectExt};

#[pyclass(name = "LineColumn", get_all, set_all, eq)]
#[derive(Clone, PartialEq, Eq)]
pub struct PyLineColumn {
    pub line: usize,
    pub column: usize,
}

impl From<&LineColumn> for PyLineColumn {
    fn from(lc: &LineColumn) -> Self {
        Self {
            line: lc.line,
            column: lc.column,
        }
    }
}

#[pymethods]
impl PyLineColumn {
    #[new]
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

#[pyclass(name = "Sourcepos", get_all, set_all, eq)]
#[derive(Clone, PartialEq, Eq)]
pub struct PySourcepos {
    pub start: PyLineColumn,
    pub end: PyLineColumn,
}

impl From<&Sourcepos> for PySourcepos {
    fn from(sourcepos: &Sourcepos) -> Self {
        Self {
            start: PyLineColumn::from(&sourcepos.start),
            end: PyLineColumn::from(&sourcepos.end),
        }
    }
}

#[pymethods]
impl PySourcepos {
    #[new]
    pub fn new(start: PyLineColumn, end: PyLineColumn) -> Self {
        Self { start, end }
    }
}

#[pyclass(name = "NodeCode", get_all, set_all, eq)]
#[derive(Clone, PartialEq, Eq)]
pub struct PyNodeCode {
    pub num_backticks: usize,
    pub literal: String,
}

impl From<&NodeCode> for PyNodeCode {
    fn from(code: &NodeCode) -> Self {
        Self {
            num_backticks: code.num_backticks,
            literal: code.literal.clone(),
        }
    }
}

#[pymethods]
impl PyNodeCode {
    #[new]
    pub fn new(num_backticks: usize, literal: String) -> Self {
        Self {
            num_backticks,
            literal,
        }
    }
}

#[pyclass(name = "NodeHtmlBlock", get_all, set_all, eq)]
#[derive(Clone, PartialEq, Eq)]
pub struct PyNodeHtmlBlock {
    pub block_type: u8,
    pub literal: String,
}

impl From<&NodeHtmlBlock> for PyNodeHtmlBlock {
    fn from(block: &NodeHtmlBlock) -> Self {
        Self {
            block_type: block.block_type,
            literal: block.literal.clone(),
        }
    }
}

#[pymethods]
impl PyNodeHtmlBlock {
    #[new]
    pub fn new(block_type: u8, literal: String) -> Self {
        Self {
            block_type,
            literal,
        }
    }
}

#[pyclass(name = "ListDelimType", eq, eq_int)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PyListDelimType {
    Period,
    Paren,
}

impl From<ListDelimType> for PyListDelimType {
    fn from(d: ListDelimType) -> Self {
        match d {
            ListDelimType::Period => PyListDelimType::Period,
            ListDelimType::Paren => PyListDelimType::Paren,
        }
    }
}

#[pyclass(name = "ListType", eq, eq_int)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PyListType {
    Bullet,
    Ordered,
}

impl From<ListType> for PyListType {
    fn from(t: ListType) -> Self {
        match t {
            ListType::Bullet => PyListType::Bullet,
            ListType::Ordered => PyListType::Ordered,
        }
    }
}

#[pyclass(name = "TableAlignment", eq, eq_int)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PyTableAlignment {
    #[pyo3(name = "None_")] // named 'None_' because 'None' is reserved in Python
    None,
    Left,
    Center,
    Right,
}

impl From<TableAlignment> for PyTableAlignment {
    fn from(a: TableAlignment) -> Self {
        match a {
            TableAlignment::None => PyTableAlignment::None,
            TableAlignment::Left => PyTableAlignment::Left,
            TableAlignment::Center => PyTableAlignment::Center,
            TableAlignment::Right => PyTableAlignment::Right,
        }
    }
}

#[pyclass(name = "NodeList", get_all, set_all, eq)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PyNodeList {
    pub list_type: PyListType,
    pub marker_offset: usize,
    pub padding: usize,
    pub start: usize,
    pub delimiter: PyListDelimType,
    pub bullet_char: u8,
    pub tight: bool,
    pub is_task_list: bool,
}

impl From<&NodeList> for PyNodeList {
    fn from(list: &NodeList) -> Self {
        Self {
            list_type: PyListType::from(list.list_type),
            marker_offset: list.marker_offset,
            padding: list.padding,
            start: list.start,
            delimiter: PyListDelimType::from(list.delimiter),
            bullet_char: list.bullet_char,
            tight: list.tight,
            is_task_list: list.is_task_list,
        }
    }
}

#[pymethods]
impl PyNodeList {
    #[new]
    pub fn new(
        list_type: PyListType,
        marker_offset: usize,
        padding: usize,
        start: usize,
        delimiter: PyListDelimType,
        bullet_char: u8,
        tight: bool,
        is_task_list: bool,
    ) -> Self {
        Self {
            list_type,
            marker_offset,
            padding,
            start,
            delimiter,
            bullet_char,
            tight,
            is_task_list,
        }
    }
}

#[pyclass(name = "NodeDescriptionItem", get_all, set_all, eq)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PyNodeDescriptionItem {
    pub marker_offset: usize,
    pub padding: usize,
    pub tight: bool,
}

impl From<&NodeDescriptionItem> for PyNodeDescriptionItem {
    fn from(item: &NodeDescriptionItem) -> Self {
        Self {
            marker_offset: item.marker_offset,
            padding: item.padding,
            tight: item.tight,
        }
    }
}

#[pymethods]
impl PyNodeDescriptionItem {
    #[new]
    pub fn new(marker_offset: usize, padding: usize, tight: bool) -> Self {
        Self {
            marker_offset,
            padding,
            tight,
        }
    }
}

#[pyclass(name = "NodeCodeBlock", get_all, set_all, eq)]
#[derive(Clone, PartialEq, Eq)]
pub struct PyNodeCodeBlock {
    pub fenced: bool,
    pub fence_char: u8,
    pub fence_length: usize,
    pub fence_offset: usize,
    pub info: String,
    pub literal: String,
    pub closed: bool,
}

impl From<&NodeCodeBlock> for PyNodeCodeBlock {
    fn from(cb: &NodeCodeBlock) -> Self {
        Self {
            info: cb.info.clone(),
            literal: cb.literal.clone(),
            fenced: cb.fenced,
            fence_char: cb.fence_char,
            fence_length: cb.fence_length,
            fence_offset: cb.fence_offset,
            closed: cb.closed,
        }
    }
}

#[pymethods]
impl PyNodeCodeBlock {
    #[new]
    pub fn new(
        fenced: bool,
        fence_char: u8,
        fence_length: usize,
        fence_offset: usize,
        info: String,
        literal: String,
        closed: bool,
    ) -> Self {
        Self {
            fenced,
            fence_char,
            fence_length,
            fence_offset,
            info,
            literal,
            closed,
        }
    }
}

#[pyclass(name = "NodeHeading", get_all, set_all, eq)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PyNodeHeading {
    pub level: u8,
    pub setext: bool,
    pub closed: bool,
}

impl From<&NodeHeading> for PyNodeHeading {
    fn from(h: &NodeHeading) -> Self {
        Self {
            level: h.level,
            setext: h.setext,
            closed: h.closed,
        }
    }
}

#[pymethods]
impl PyNodeHeading {
    #[new]
    pub fn new(level: u8, setext: bool, closed: bool) -> Self {
        Self {
            level,
            setext,
            closed,
        }
    }
}

#[pyclass(name = "NodeTable", get_all, set_all, eq)]
#[derive(Clone, PartialEq, Eq)]
pub struct PyNodeTable {
    pub alignments: Vec<PyTableAlignment>,
    pub num_columns: usize,
    pub num_rows: usize,
    pub num_nonempty_cells: usize,
}

impl From<&NodeTable> for PyNodeTable {
    fn from(table: &NodeTable) -> Self {
        Self {
            alignments: table
                .alignments
                .iter()
                .map(|a| PyTableAlignment::from(*a))
                .collect(),
            num_columns: table.num_columns,
            num_rows: table.num_rows,
            num_nonempty_cells: table.num_nonempty_cells,
        }
    }
}

#[pymethods]
impl PyNodeTable {
    #[new]
    pub fn new(
        alignments: Vec<PyTableAlignment>,
        num_columns: usize,
        num_rows: usize,
        num_nonempty_cells: usize,
    ) -> Self {
        Self {
            alignments,
            num_columns,
            num_rows,
            num_nonempty_cells,
        }
    }
}

#[pyclass(name = "NodeTaskItem", get_all, set_all, eq)]
#[derive(Clone, PartialEq, Eq)]
pub struct PyNodeTaskItem {
    pub symbol: Option<char>,
    pub symbol_sourcepos: PySourcepos,
}

impl From<&NodeTaskItem> for PyNodeTaskItem {
    fn from(ti: &NodeTaskItem) -> Self {
        Self {
            symbol: ti.symbol,
            symbol_sourcepos: PySourcepos::from(&ti.symbol_sourcepos),
        }
    }
}

#[pyclass(name = "NodeLink", get_all, set_all, eq)]
#[derive(Clone, PartialEq, Eq)]
pub struct PyNodeLink {
    pub url: String,
    pub title: String,
}

impl From<&NodeLink> for PyNodeLink {
    fn from(link: &NodeLink) -> Self {
        Self {
            url: link.url.clone(),
            title: link.title.clone(),
        }
    }
}

#[pymethods]
impl PyNodeLink {
    #[new]
    pub fn new(url: String, title: String) -> Self {
        Self { url, title }
    }
}

#[pyclass(name = "NodeFootnoteDefinition", get_all, set_all, eq)]
#[derive(Clone, PartialEq, Eq)]
pub struct PyNodeFootnoteDefinition {
    pub name: String,
    pub total_references: u32,
}

impl From<&NodeFootnoteDefinition> for PyNodeFootnoteDefinition {
    fn from(f: &NodeFootnoteDefinition) -> Self {
        Self {
            name: f.name.clone(),
            total_references: f.total_references,
        }
    }
}

#[pymethods]
impl PyNodeFootnoteDefinition {
    #[new]
    pub fn new(name: String, total_references: u32) -> Self {
        Self {
            name,
            total_references,
        }
    }
}

#[pyclass(name = "NodeFootnoteReference", get_all, set_all, eq)]
#[derive(Clone, PartialEq, Eq)]
pub struct PyNodeFootnoteReference {
    pub name: String,
    pub texts: Vec<(String, usize)>,
    pub ref_num: u32,
    pub ix: u32,
}

impl From<&NodeFootnoteReference> for PyNodeFootnoteReference {
    fn from(f: &NodeFootnoteReference) -> Self {
        Self {
            name: f.name.clone(),
            texts: f.texts.clone(),
            ref_num: f.ref_num,
            ix: f.ix,
        }
    }
}

#[pymethods]
impl PyNodeFootnoteReference {
    #[new]
    pub fn new(name: String, texts: Vec<(String, usize)>, ref_num: u32, ix: u32) -> Self {
        Self {
            name,
            texts,
            ref_num,
            ix,
        }
    }
}

#[pyclass(name = "NodeWikiLink", get_all, set_all, eq)]
#[derive(Clone, PartialEq, Eq)]
pub struct PyNodeWikiLink {
    pub url: String,
}

impl From<&NodeWikiLink> for PyNodeWikiLink {
    fn from(w: &NodeWikiLink) -> Self {
        Self { url: w.url.clone() }
    }
}

#[pymethods]
impl PyNodeWikiLink {
    #[new]
    pub fn new(url: String) -> Self {
        Self { url }
    }
}

#[pyclass(name = "NodeShortCode", get_all, set_all, eq)]
#[derive(Clone, PartialEq, Eq)]
pub struct PyNodeShortCode {
    pub code: String,
    pub emoji: String,
}

impl From<&NodeShortCode> for PyNodeShortCode {
    fn from(sc: &NodeShortCode) -> Self {
        Self {
            code: sc.code.clone(),
            emoji: sc.emoji.clone(),
        }
    }
}

#[pymethods]
impl PyNodeShortCode {
    #[new]
    pub fn new(code: String, emoji: String) -> Self {
        Self { code, emoji }
    }
}

#[pyclass(name = "NodeMath", get_all, set_all, eq)]
#[derive(Clone, PartialEq, Eq)]
pub struct PyNodeMath {
    pub dollar_math: bool,
    pub display_math: bool,
    pub literal: String,
}

impl From<&NodeMath> for PyNodeMath {
    fn from(m: &NodeMath) -> Self {
        Self {
            dollar_math: m.dollar_math,
            display_math: m.display_math,
            literal: m.literal.clone(),
        }
    }
}

#[pymethods]
impl PyNodeMath {
    #[new]
    pub fn new(dollar_math: bool, display_math: bool, literal: String) -> Self {
        Self {
            dollar_math,
            display_math,
            literal,
        }
    }
}

#[pyclass(name = "NodeMultilineBlockQuote", get_all, set_all, eq)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PyNodeMultilineBlockQuote {
    pub fence_length: usize,
    pub fence_offset: usize,
}

impl From<&NodeMultilineBlockQuote> for PyNodeMultilineBlockQuote {
    fn from(mbq: &NodeMultilineBlockQuote) -> Self {
        Self {
            fence_length: mbq.fence_length,
            fence_offset: mbq.fence_offset,
        }
    }
}

#[pymethods]
impl PyNodeMultilineBlockQuote {
    #[new]
    pub fn new(fence_length: usize, fence_offset: usize) -> Self {
        Self {
            fence_length,
            fence_offset,
        }
    }
}

#[pyclass(name = "AlertType", eq, eq_int)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PyAlertType {
    Note,
    Tip,
    Important,
    Warning,
    Caution,
}

impl From<AlertType> for PyAlertType {
    fn from(a: AlertType) -> Self {
        match a {
            AlertType::Note => PyAlertType::Note,
            AlertType::Tip => PyAlertType::Tip,
            AlertType::Important => PyAlertType::Important,
            AlertType::Warning => PyAlertType::Warning,
            AlertType::Caution => PyAlertType::Caution,
        }
    }
}

#[pyclass(name = "NodeAlert", get_all, set_all, eq)]
#[derive(Clone, PartialEq, Eq)]
pub struct PyNodeAlert {
    pub alert_type: PyAlertType,
    pub title: Option<String>,
    pub multiline: bool,
    pub fence_length: usize,
    pub fence_offset: usize,
}

impl From<&NodeAlert> for PyNodeAlert {
    fn from(a: &NodeAlert) -> Self {
        Self {
            alert_type: PyAlertType::from(a.alert_type),
            title: a.title.clone(),
            multiline: a.multiline,
            fence_length: a.fence_length,
            fence_offset: a.fence_offset,
        }
    }
}

#[pymethods]
impl PyNodeAlert {
    #[new]
    pub fn new(
        alert_type: PyAlertType,
        title: Option<String>,
        multiline: bool,
        fence_length: usize,
        fence_offset: usize,
    ) -> Self {
        Self {
            alert_type,
            title,
            multiline,
            fence_length,
            fence_offset,
        }
    }
}

#[pyclass(name = "HeexNode", subclass, eq)]
#[derive(Clone, PartialEq, Eq)]
pub struct PyHeexNode {}

#[pymethods]
impl PyHeexNode {
    #[new]
    pub fn new() -> Self {
        Self {}
    }
}

#[pyclass(name = "HeexNodeDirective", extends=PyHeexNode, eq)]
#[derive(Clone, PartialEq, Eq)]
pub struct PyHeexNodeDirective {}

#[pymethods]
impl PyHeexNodeDirective {
    #[new]
    pub fn new() -> (Self, PyHeexNode) {
        (Self {}, PyHeexNode::new())
    }
}

impl<'py> IntoPyObject<'py> for PyHeexNodeDirective {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        // Invoking PyHeexNodeDirective(self.bar) via the Python interface
        py.get_type::<PyHeexNodeDirective>().call1(())
    }
}

#[pyclass(name = "HeexNodeComment", extends=PyHeexNode, eq)]
#[derive(Clone, PartialEq, Eq)]
pub struct PyHeexNodeComment {}

#[pymethods]
impl PyHeexNodeComment {
    #[new]
    pub fn new() -> (Self, PyHeexNode) {
        (Self {}, PyHeexNode::new())
    }
}

impl<'py> IntoPyObject<'py> for PyHeexNodeComment {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        // Invoking PyHeexNodeComment(self.bar) via the Python interface
        py.get_type::<PyHeexNodeComment>().call1(())
    }
}

#[pyclass(name = "HeexNodeMultilineComment", extends=PyHeexNode, eq)]
#[derive(Clone, PartialEq, Eq)]
pub struct PyHeexNodeMultilineComment {}

#[pymethods]
impl PyHeexNodeMultilineComment {
    #[new]
    pub fn new() -> (Self, PyHeexNode) {
        (Self {}, PyHeexNode::new())
    }
}

impl<'py> IntoPyObject<'py> for PyHeexNodeMultilineComment {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        // Invoking PyHeexNodeMultilineComment(self.bar) via the Python interface
        py.get_type::<PyHeexNodeMultilineComment>().call1(())
    }
}

#[pyclass(name = "HeexNodeExpression", extends=PyHeexNode, eq)]
#[derive(Clone, PartialEq, Eq)]
pub struct PyHeexNodeExpression {}

#[pymethods]
impl PyHeexNodeExpression {
    #[new]
    pub fn new() -> (Self, PyHeexNode) {
        (Self {}, PyHeexNode::new())
    }
}

impl<'py> IntoPyObject<'py> for PyHeexNodeExpression {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        // Invoking PyHeexNodeExpression(self.bar) via the Python interface
        py.get_type::<PyHeexNodeExpression>().call1(())
    }
}

#[pyclass(name = "HeexNodeTag", extends=PyHeexNode, get_all, set_all, eq)]
#[derive(Clone, PartialEq, Eq)]
pub struct PyHeexNodeTag {
    pub tag: String,
}

impl<'py> IntoPyObject<'py> for PyHeexNodeTag {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        // Invoking PyHeexNodeTag(self.bar) via the Python interface
        py.get_type::<PyHeexNodeTag>().call1((self.tag,))
    }
}

#[pymethods]
impl PyHeexNodeTag {
    #[new]
    pub fn new(tag: String) -> (Self, PyHeexNode) {
        (Self { tag }, PyHeexNode::new())
    }
}

#[pyclass(name = "NodeHeexBlock", get_all, set_all, eq)]
pub struct PyNodeHeexBlock {
    pub literal: String,
    pub node: Py<PyAny>, // PyHeexNode subclass
}

impl Clone for PyNodeHeexBlock {
    fn clone(&self) -> Self {
        Python::attach(|py| Self {
            literal: self.literal.clone(),
            node: self.node.clone_ref(py),
        })
    }
}

impl PartialEq for PyNodeHeexBlock {
    fn eq(&self, other: &Self) -> bool {
        self.literal == other.literal && self.node.as_ptr() == other.node.as_ptr()
    }
}

impl Eq for PyNodeHeexBlock {}

impl From<(Python<'_>, &NodeHeexBlock)> for PyNodeHeexBlock {
    fn from((py, hb): (Python, &NodeHeexBlock)) -> Self {
        let py_node = match &hb.node {
            HeexNode::Directive => PyHeexNodeDirective {}.into_py_any(py),
            HeexNode::Comment => PyHeexNodeComment {}.into_py_any(py),
            HeexNode::MultilineComment => PyHeexNodeMultilineComment {}.into_py_any(py),
            HeexNode::Expression => PyHeexNodeExpression {}.into_py_any(py),
            HeexNode::Tag(s) => PyHeexNodeTag { tag: s.clone() }.into_py_any(py),
        };

        Self {
            literal: hb.literal.clone(),
            node: py_node.unwrap().into(),
        }
    }
}

#[pymethods]
impl PyNodeHeexBlock {
    #[new]
    pub fn new(literal: String, node: Py<PyAny>) -> Self {
        Self { literal, node }
    }
}

#[pyclass(name = "NodeValue", subclass, eq)]
#[derive(PartialEq, Eq)]
pub struct PyNodeValue {}

#[pymethods]
impl PyNodeValue {
    #[new]
    pub fn new() -> Self {
        Self {}
    }
}

#[pyclass(name = "Document", extends=PyNodeValue, eq)]
#[derive(PartialEq, Eq)]
pub struct PyDocument {}

#[pymethods]
impl PyDocument {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
}

#[pyclass(name = "FrontMatter", extends=PyNodeValue, get_all, set_all, eq)]
#[derive(PartialEq, Eq)]
pub struct PyFrontMatter {
    pub value: String,
}

#[pymethods]
impl PyFrontMatter {
    #[new]
    pub fn new(value: String) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
}

#[pyclass(name = "BlockQuote", extends=PyNodeValue, eq)]
#[derive(PartialEq, Eq)]
pub struct PyBlockQuote {}

#[pymethods]
impl PyBlockQuote {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
}

#[pyclass(name = "List", extends=PyNodeValue, get_all, set_all, eq)]
#[derive(PartialEq, Eq)]
pub struct PyList {
    pub value: PyNodeList,
}

#[pymethods]
impl PyList {
    #[new]
    pub fn new(value: PyNodeList) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
}

#[pyclass(name = "Item", extends=PyNodeValue, get_all, set_all, eq)]
#[derive(PartialEq, Eq)]
pub struct PyItem {
    pub value: PyNodeList,
}

#[pymethods]
impl PyItem {
    #[new]
    pub fn new(value: PyNodeList) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
}

#[pyclass(name = "DescriptionList", extends=PyNodeValue, eq)]
#[derive(PartialEq, Eq)]
pub struct PyDescriptionList {}

#[pymethods]
impl PyDescriptionList {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
}

#[pyclass(name = "DescriptionItem", extends=PyNodeValue, get_all, set_all, eq)]
#[derive(PartialEq, Eq)]
pub struct PyDescriptionItem {
    pub value: PyNodeDescriptionItem,
}

#[pymethods]
impl PyDescriptionItem {
    #[new]
    pub fn new(value: PyNodeDescriptionItem) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
}

#[pyclass(name = "DescriptionTerm", extends=PyNodeValue, eq)]
#[derive(PartialEq, Eq)]
pub struct PyDescriptionTerm {}

#[pymethods]
impl PyDescriptionTerm {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
}

#[pyclass(name = "DescriptionDetails", extends=PyNodeValue, eq)]
#[derive(PartialEq, Eq)]
pub struct PyDescriptionDetails {}

#[pymethods]
impl PyDescriptionDetails {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
}

#[pyclass(name = "CodeBlock", extends=PyNodeValue, get_all, set_all, eq)]
#[derive(PartialEq, Eq)]
pub struct PyCodeBlock {
    pub value: PyNodeCodeBlock,
}

#[pymethods]
impl PyCodeBlock {
    #[new]
    pub fn new(value: PyNodeCodeBlock) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
}

#[pyclass(name = "HtmlBlock", extends=PyNodeValue, get_all, set_all, eq)]
#[derive(PartialEq, Eq)]
pub struct PyHtmlBlock {
    pub value: PyNodeHtmlBlock,
}

#[pymethods]
impl PyHtmlBlock {
    #[new]
    pub fn new(value: PyNodeHtmlBlock) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
}

#[pyclass(name = "HeexBlock", extends=PyNodeValue, get_all, set_all, eq)]
#[derive(PartialEq, Eq)]
pub struct PyHeexBlock {
    pub value: PyNodeHeexBlock,
}

#[pymethods]
impl PyHeexBlock {
    #[new]
    pub fn new(value: PyNodeHeexBlock) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
}

#[pyclass(name = "Paragraph", extends=PyNodeValue, eq)]
#[derive(PartialEq, Eq)]
pub struct PyParagraph {}

#[pymethods]
impl PyParagraph {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
}

#[pyclass(name = "Heading", extends=PyNodeValue, get_all, set_all, eq)]
#[derive(PartialEq, Eq)]
pub struct PyHeading {
    pub value: PyNodeHeading,
}

#[pymethods]
impl PyHeading {
    #[new]
    pub fn new(value: PyNodeHeading) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
}

#[pyclass(name = "ThematicBreak", extends=PyNodeValue, eq)]
#[derive(PartialEq, Eq)]
pub struct PyThematicBreak {}

#[pymethods]
impl PyThematicBreak {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
}

#[pyclass(name = "FootnoteDefinition", extends=PyNodeValue, get_all, set_all, eq)]
#[derive(PartialEq, Eq)]
pub struct PyFootnoteDefinition {
    pub value: PyNodeFootnoteDefinition,
}

#[pymethods]
impl PyFootnoteDefinition {
    #[new]
    pub fn new(value: PyNodeFootnoteDefinition) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
}

#[pyclass(name = "Table", extends=PyNodeValue, get_all, set_all, eq)]
#[derive(PartialEq, Eq)]
pub struct PyTable {
    pub value: PyNodeTable,
}

#[pymethods]
impl PyTable {
    #[new]
    pub fn new(value: PyNodeTable) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
}

#[pyclass(name = "TableRow", extends=PyNodeValue, get_all, set_all, eq)]
#[derive(PartialEq, Eq)]
pub struct PyTableRow {
    pub value: bool,
}

#[pymethods]
impl PyTableRow {
    #[new]
    pub fn new(value: bool) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
}

#[pyclass(name = "TableCell", extends=PyNodeValue, eq)]
#[derive(PartialEq, Eq)]
pub struct PyTableCell {}

#[pymethods]
impl PyTableCell {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
}

#[pyclass(name = "Text", extends=PyNodeValue, get_all, set_all, eq)]
#[derive(PartialEq, Eq)]
pub struct PyText {
    pub value: String,
}

#[pymethods]
impl PyText {
    #[new]

    pub fn new(value: String) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
}

#[pyclass(name = "TaskItem", extends=PyNodeValue, get_all, set_all, eq)]
#[derive(PartialEq, Eq)]
pub struct PyTaskItem {
    pub value: PyNodeTaskItem,
}

#[pymethods]
impl PyTaskItem {
    #[new]
    pub fn new(value: PyNodeTaskItem) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
}

#[pyclass(name = "SoftBreak", extends=PyNodeValue, eq)]
#[derive(PartialEq, Eq)]
pub struct PySoftBreak {}

#[pymethods]
impl PySoftBreak {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
}

#[pyclass(name = "LineBreak", extends=PyNodeValue, eq)]
#[derive(PartialEq, Eq)]
pub struct PyLineBreak {}

#[pymethods]
impl PyLineBreak {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
}

#[pyclass(name = "Code", extends=PyNodeValue, get_all, set_all, eq)]
#[derive(PartialEq, Eq)]
pub struct PyCode {
    pub value: PyNodeCode,
}

#[pymethods]
impl PyCode {
    #[new]
    pub fn new(value: PyNodeCode) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
}

#[pyclass(name = "HtmlInline", extends=PyNodeValue, get_all, set_all, eq)]
#[derive(PartialEq, Eq)]
pub struct PyHtmlInline {
    pub value: String,
}

#[pymethods]
impl PyHtmlInline {
    #[new]
    pub fn new(value: String) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
}

#[pyclass(name = "HeexInline", extends=PyNodeValue, get_all, set_all, eq)]
#[derive(PartialEq, Eq)]
pub struct PyHeexInline {
    pub value: String,
}

#[pymethods]
impl PyHeexInline {
    #[new]
    pub fn new(value: String) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
}

#[pyclass(name = "Raw", extends=PyNodeValue, get_all, set_all, eq)]
#[derive(PartialEq, Eq)]
pub struct PyRaw {
    pub value: String,
}

#[pymethods]
impl PyRaw {
    #[new]
    pub fn new(value: String) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
}

#[pyclass(name = "Emph", extends=PyNodeValue, eq)]
#[derive(PartialEq, Eq)]
pub struct PyEmph {}

#[pymethods]
impl PyEmph {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
}

#[pyclass(name = "Strong", extends=PyNodeValue, eq)]
#[derive(PartialEq, Eq)]
pub struct PyStrong {}

#[pymethods]
impl PyStrong {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
}

#[pyclass(name = "Strikethrough", extends=PyNodeValue, eq)]
#[derive(PartialEq, Eq)]
pub struct PyStrikethrough {}

#[pymethods]
impl PyStrikethrough {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
}

#[pyclass(name = "Highlight", extends=PyNodeValue, eq)]
#[derive(PartialEq, Eq)]
pub struct PyHighlight {}

#[pymethods]
impl PyHighlight {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
}

#[pyclass(name = "Superscript", extends=PyNodeValue, eq)]
#[derive(PartialEq, Eq)]
pub struct PySuperscript {}

#[pymethods]
impl PySuperscript {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
}

#[pyclass(name = "Link", extends=PyNodeValue, get_all, set_all, eq)]
#[derive(PartialEq, Eq)]
pub struct PyLink {
    pub value: PyNodeLink,
}

#[pymethods]
impl PyLink {
    #[new]
    pub fn new(value: PyNodeLink) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
}

#[pyclass(name = "Image", extends=PyNodeValue, get_all, set_all, eq)]
#[derive(PartialEq, Eq)]
pub struct PyImage {
    pub value: PyNodeLink,
}

#[pymethods]
impl PyImage {
    #[new]
    pub fn new(value: PyNodeLink) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
}

#[pyclass(name = "FootnoteReference", extends=PyNodeValue, get_all, set_all, eq)]
#[derive(PartialEq, Eq)]
pub struct PyFootnoteReference {
    pub value: PyNodeFootnoteReference,
}

#[pymethods]
impl PyFootnoteReference {
    #[new]
    pub fn new(value: PyNodeFootnoteReference) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
}

#[pyclass(name = "ShortCode", extends=PyNodeValue, get_all, set_all, eq)]
#[derive(PartialEq, Eq)]
pub struct PyShortCode {
    pub value: PyNodeShortCode,
}

#[pymethods]
impl PyShortCode {
    #[new]
    pub fn new(value: PyNodeShortCode) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
}

#[pyclass(name = "Math", extends=PyNodeValue, get_all, set_all, eq)]
#[derive(PartialEq, Eq)]
pub struct PyMath {
    pub value: PyNodeMath,
}

#[pymethods]
impl PyMath {
    #[new]
    pub fn new(value: PyNodeMath) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
}

#[pyclass(name = "MultilineBlockQuote", extends=PyNodeValue, get_all, set_all, eq)]
#[derive(PartialEq, Eq)]
pub struct PyMultilineBlockQuote {
    pub value: PyNodeMultilineBlockQuote,
}

#[pymethods]
impl PyMultilineBlockQuote {
    #[new]
    pub fn new(value: PyNodeMultilineBlockQuote) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
}

#[pyclass(name = "Escaped", extends=PyNodeValue, eq)]
#[derive(PartialEq, Eq)]
pub struct PyEscaped {}

#[pymethods]
impl PyEscaped {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
}

#[pyclass(name = "WikiLink", extends=PyNodeValue, get_all, set_all, eq)]
#[derive(PartialEq, Eq)]
pub struct PyWikiLink {
    pub value: PyNodeWikiLink,
}

#[pymethods]
impl PyWikiLink {
    #[new]
    pub fn new(value: PyNodeWikiLink) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
}

#[pyclass(name = "Underline", extends=PyNodeValue, eq)]
#[derive(PartialEq, Eq)]
pub struct PyUnderline {}

#[pymethods]
impl PyUnderline {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
}

#[pyclass(name = "Subscript", extends=PyNodeValue, eq)]
#[derive(PartialEq, Eq)]
pub struct PySubscript {}

#[pymethods]
impl PySubscript {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
}

#[pyclass(name = "SpoileredText", extends=PyNodeValue, eq)]
#[derive(PartialEq, Eq)]
pub struct PySpoileredText {}

#[pymethods]
impl PySpoileredText {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
}

#[pyclass(name = "EscapedTag", extends=PyNodeValue, get_all, set_all, eq)]
#[derive(PartialEq, Eq)]
pub struct PyEscapedTag {
    pub value: String,
}

#[pymethods]
impl PyEscapedTag {
    #[new]
    pub fn new(value: String) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
}

#[pyclass(name = "Alert", extends=PyNodeValue, get_all, set_all, eq)]
#[derive(PartialEq, Eq)]
pub struct PyAlert {
    pub value: PyNodeAlert,
}

#[pymethods]
impl PyAlert {
    #[new]
    pub fn new(value: PyNodeAlert) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
}

#[pyclass(name = "Subtext", extends=PyNodeValue, eq)]
#[derive(PartialEq, Eq)]

pub struct PySubtext {}

#[pymethods]
impl PySubtext {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
}

#[pyclass(name = "AstNode", get_all, set_all)]
pub struct PyAstNode {
    pub node_value: Py<PyAny>,
    pub sourcepos: PySourcepos,
    pub parent: Option<Py<PyAstNode>>,
    pub children: Vec<Py<PyAstNode>>,
}

#[pymethods]
impl PyAstNode {
    #[new]
    pub fn new(
        node_value: Py<PyAny>,
        sourcepos: PySourcepos,
        parent: Option<Py<PyAstNode>>,
        children: Vec<Py<PyAstNode>>,
    ) -> Self {
        Self {
            node_value,
            sourcepos,
            parent,
            children,
        }
    }
}

fn create_py_node_value(py: Python, value: &comrak_lib::nodes::NodeValue) -> Py<PyAny> {
    match value {
        comrak_lib::nodes::NodeValue::Document => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyDocument {}),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::FrontMatter(s) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {})
                .add_subclass(PyFrontMatter { value: s.clone() }),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::BlockQuote => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyBlockQuote {}),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::List(l) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyList {
                value: PyNodeList::from(l),
            }),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::Item(i) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyItem {
                value: PyNodeList::from(i),
            }),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::DescriptionList => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyDescriptionList {}),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::DescriptionItem(d) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyDescriptionItem {
                value: PyNodeDescriptionItem::from(d),
            }),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::DescriptionTerm => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyDescriptionTerm {}),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::DescriptionDetails => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyDescriptionDetails {}),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::CodeBlock(c) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyCodeBlock {
                value: PyNodeCodeBlock::from(c.as_ref()),
            }),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::HtmlBlock(h) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyHtmlBlock {
                value: PyNodeHtmlBlock::from(h),
            }),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::HeexBlock(h) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyHeexBlock {
                value: PyNodeHeexBlock::from((py, h.as_ref())),
            }),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::Paragraph => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyParagraph {}),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::Heading(h) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyHeading {
                value: PyNodeHeading::from(h),
            }),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::ThematicBreak => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyThematicBreak {}),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::FootnoteDefinition(f) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyFootnoteDefinition {
                value: PyNodeFootnoteDefinition::from(f),
            }),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::Table(t) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyTable {
                value: PyNodeTable::from(t.as_ref()),
            }),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::TableRow(is_header) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyTableRow { value: *is_header }),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::TableCell => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyTableCell {}),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::Text(t) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyText {
                value: t.to_string(),
            }),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::TaskItem(c) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyTaskItem {
                value: PyNodeTaskItem::from(c),
            }),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::SoftBreak => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PySoftBreak {}),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::LineBreak => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyLineBreak {}),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::Code(c) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyCode {
                value: PyNodeCode::from(c),
            }),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::HtmlInline(s) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {})
                .add_subclass(PyHtmlInline { value: s.clone() }),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::HeexInline(s) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {})
                .add_subclass(PyHeexInline { value: s.clone() }),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::Raw(s) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyRaw { value: s.clone() }),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::Emph => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyEmph {}),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::Strong => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyStrong {}),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::Strikethrough => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyStrikethrough {}),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::Highlight => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyHighlight {}),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::Superscript => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PySuperscript {}),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::Link(l) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyLink {
                value: PyNodeLink::from(l.as_ref()),
            }),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::Image(i) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyImage {
                value: PyNodeLink::from(i.as_ref()),
            }),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::FootnoteReference(f) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyFootnoteReference {
                value: PyNodeFootnoteReference::from(f.as_ref()),
            }),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::ShortCode(s) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyShortCode {
                value: PyNodeShortCode::from(s.as_ref()),
            }),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::Math(m) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyMath {
                value: PyNodeMath::from(m),
            }),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::MultilineBlockQuote(m) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyMultilineBlockQuote {
                value: PyNodeMultilineBlockQuote::from(m),
            }),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::Escaped => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyEscaped {}),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::WikiLink(w) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyWikiLink {
                value: PyNodeWikiLink::from(w),
            }),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::Underline => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyUnderline {}),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::Subscript => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PySubscript {}),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::SpoileredText => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PySpoileredText {}),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::EscapedTag(s) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyEscapedTag {
                value: s.to_string(),
            }),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::Alert(a) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyAlert {
                value: PyNodeAlert::from(a.as_ref()),
            }),
        )
        .unwrap()
        .into(),
        comrak_lib::nodes::NodeValue::Subtext => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PySubtext {}),
        )
        .unwrap()
        .into(),
    }
}

fn py_sourcepos_to_sourcepos(sp: &PySourcepos) -> Sourcepos {
    Sourcepos {
        start: LineColumn {
            line: sp.start.line,
            column: sp.start.column,
        },
        end: LineColumn {
            line: sp.end.line,
            column: sp.end.column,
        },
    }
}

fn create_comrak_node_value<'a>(
    py: Python<'a>,
    node_value: &Py<PyAny>,
) -> comrak_lib::nodes::NodeValue {
    let any = node_value.as_ref();

    // Try each concrete Python subclass in turn and build the matching NodeValue
    if let Ok(_v) = any.extract::<pyo3::PyRef<PyDocument>>(py) {
        return comrak_lib::nodes::NodeValue::Document;
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyFrontMatter>>(py) {
        return comrak_lib::nodes::NodeValue::FrontMatter(v.value.clone());
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PyBlockQuote>>(py) {
        return comrak_lib::nodes::NodeValue::BlockQuote;
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyList>>(py) {
        let pl = &v.value;
        let list_type = match pl.list_type {
            PyListType::Bullet => ListType::Bullet,
            PyListType::Ordered => ListType::Ordered,
        };
        let delim = match pl.delimiter {
            PyListDelimType::Period => ListDelimType::Period,
            PyListDelimType::Paren => ListDelimType::Paren,
        };

        return comrak_lib::nodes::NodeValue::List(NodeList {
            list_type,
            marker_offset: pl.marker_offset,
            padding: pl.padding,
            start: pl.start,
            delimiter: delim,
            bullet_char: pl.bullet_char,
            tight: pl.tight,
            is_task_list: pl.is_task_list,
        });
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyItem>>(py) {
        let pl = &v.value;
        let list_type = match pl.list_type {
            PyListType::Bullet => ListType::Bullet,
            PyListType::Ordered => ListType::Ordered,
        };
        let delim = match pl.delimiter {
            PyListDelimType::Period => ListDelimType::Period,
            PyListDelimType::Paren => ListDelimType::Paren,
        };

        return comrak_lib::nodes::NodeValue::Item(NodeList {
            list_type,
            marker_offset: pl.marker_offset,
            padding: pl.padding,
            start: pl.start,
            delimiter: delim,
            bullet_char: pl.bullet_char,
            tight: pl.tight,
            is_task_list: pl.is_task_list,
        });
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PyDescriptionList>>(py) {
        return comrak_lib::nodes::NodeValue::DescriptionList;
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyDescriptionItem>>(py) {
        let di = &v.value;
        return comrak_lib::nodes::NodeValue::DescriptionItem(NodeDescriptionItem {
            marker_offset: di.marker_offset,
            padding: di.padding,
            tight: di.tight,
        });
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PyDescriptionTerm>>(py) {
        return comrak_lib::nodes::NodeValue::DescriptionTerm;
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PyDescriptionDetails>>(py) {
        return comrak_lib::nodes::NodeValue::DescriptionDetails;
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyCodeBlock>>(py) {
        let cb = &v.value;
        return comrak_lib::nodes::NodeValue::CodeBlock(Box::new(NodeCodeBlock {
            fenced: cb.fenced,
            fence_char: cb.fence_char,
            fence_length: cb.fence_length,
            fence_offset: cb.fence_offset,
            info: cb.info.clone(),
            literal: cb.literal.clone(),
            closed: cb.closed,
        }));
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyHtmlBlock>>(py) {
        let hb = &v.value;
        return comrak_lib::nodes::NodeValue::HtmlBlock(NodeHtmlBlock {
            block_type: hb.block_type,
            literal: hb.literal.clone(),
        });
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyHeexBlock>>(py) {
        let hb = &v.value;
        // Convert HeexNode Py<PyAny> -> HeexNode
        let py_node_any = hb.node.as_ref();
        if let Ok(_) = py_node_any.extract::<pyo3::PyRef<PyHeexNodeDirective>>(py) {
            return comrak_lib::nodes::NodeValue::HeexBlock(Box::new(NodeHeexBlock {
                literal: hb.literal.clone(),
                node: HeexNode::Directive,
            }));
        }
        if let Ok(_) = py_node_any.extract::<pyo3::PyRef<PyHeexNodeComment>>(py) {
            return comrak_lib::nodes::NodeValue::HeexBlock(Box::new(NodeHeexBlock {
                literal: hb.literal.clone(),
                node: HeexNode::Comment,
            }));
        }
        if let Ok(_) = py_node_any.extract::<pyo3::PyRef<PyHeexNodeMultilineComment>>(py) {
            return comrak_lib::nodes::NodeValue::HeexBlock(Box::new(NodeHeexBlock {
                literal: hb.literal.clone(),
                node: HeexNode::MultilineComment,
            }));
        }
        if let Ok(_) = py_node_any.extract::<pyo3::PyRef<PyHeexNodeExpression>>(py) {
            return comrak_lib::nodes::NodeValue::HeexBlock(Box::new(NodeHeexBlock {
                literal: hb.literal.clone(),
                node: HeexNode::Expression,
            }));
        }
        if let Ok(tag) = py_node_any.extract::<pyo3::PyRef<PyHeexNodeTag>>(py) {
            return comrak_lib::nodes::NodeValue::HeexBlock(Box::new(NodeHeexBlock {
                literal: hb.literal.clone(),
                node: HeexNode::Tag(tag.tag.clone()),
            }));
        }
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PyParagraph>>(py) {
        return comrak_lib::nodes::NodeValue::Paragraph;
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyHeading>>(py) {
        let h = &v.value;
        return comrak_lib::nodes::NodeValue::Heading(NodeHeading {
            level: h.level,
            setext: h.setext,
            closed: h.closed,
        });
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PyThematicBreak>>(py) {
        return comrak_lib::nodes::NodeValue::ThematicBreak;
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyFootnoteDefinition>>(py) {
        let f = &v.value;
        return comrak_lib::nodes::NodeValue::FootnoteDefinition(NodeFootnoteDefinition {
            name: f.name.clone(),
            total_references: f.total_references,
        });
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyTable>>(py) {
        let t = &v.value;
        let alignments = t
            .alignments
            .iter()
            .map(|a| match a {
                PyTableAlignment::None => TableAlignment::None,
                PyTableAlignment::Left => TableAlignment::Left,
                PyTableAlignment::Center => TableAlignment::Center,
                PyTableAlignment::Right => TableAlignment::Right,
            })
            .collect();

        return comrak_lib::nodes::NodeValue::Table(Box::new(NodeTable {
            alignments,
            num_columns: t.num_columns,
            num_rows: t.num_rows,
            num_nonempty_cells: t.num_nonempty_cells,
        }));
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyTableRow>>(py) {
        return comrak_lib::nodes::NodeValue::TableRow(v.value);
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PyTableCell>>(py) {
        return comrak_lib::nodes::NodeValue::TableCell;
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyText>>(py) {
        return comrak_lib::nodes::NodeValue::Text(v.value.clone().into());
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyTaskItem>>(py) {
        let ti = &v.value;
        return comrak_lib::nodes::NodeValue::TaskItem(NodeTaskItem {
            symbol: ti.symbol,
            symbol_sourcepos: py_sourcepos_to_sourcepos(&ti.symbol_sourcepos),
        });
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PySoftBreak>>(py) {
        return comrak_lib::nodes::NodeValue::SoftBreak;
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PyLineBreak>>(py) {
        return comrak_lib::nodes::NodeValue::LineBreak;
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyCode>>(py) {
        let c = &v.value;
        return comrak_lib::nodes::NodeValue::Code(NodeCode {
            num_backticks: c.num_backticks,
            literal: c.literal.clone(),
        });
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyHtmlInline>>(py) {
        return comrak_lib::nodes::NodeValue::HtmlInline(v.value.clone());
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyHeexInline>>(py) {
        return comrak_lib::nodes::NodeValue::HeexInline(v.value.clone());
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyRaw>>(py) {
        return comrak_lib::nodes::NodeValue::Raw(v.value.clone());
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PyEmph>>(py) {
        return comrak_lib::nodes::NodeValue::Emph;
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PyStrong>>(py) {
        return comrak_lib::nodes::NodeValue::Strong;
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PyStrikethrough>>(py) {
        return comrak_lib::nodes::NodeValue::Strikethrough;
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PyHighlight>>(py) {
        return comrak_lib::nodes::NodeValue::Highlight;
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PySuperscript>>(py) {
        return comrak_lib::nodes::NodeValue::Superscript;
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyLink>>(py) {
        return comrak_lib::nodes::NodeValue::Link(Box::new(NodeLink {
            url: v.value.url.clone(),
            title: v.value.title.clone(),
        }));
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyImage>>(py) {
        return comrak_lib::nodes::NodeValue::Image(Box::new(NodeLink {
            url: v.value.url.clone(),
            title: v.value.title.clone(),
        }));
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyFootnoteReference>>(py) {
        let fr = &v.value;
        return comrak_lib::nodes::NodeValue::FootnoteReference(Box::new(NodeFootnoteReference {
            name: fr.name.clone(),
            texts: fr.texts.clone(),
            ref_num: fr.ref_num,
            ix: fr.ix,
        }));
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyShortCode>>(py) {
        return comrak_lib::nodes::NodeValue::ShortCode(Box::new(NodeShortCode {
            code: v.value.code.clone(),
            emoji: v.value.emoji.clone(),
        }));
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyMath>>(py) {
        return comrak_lib::nodes::NodeValue::Math(NodeMath {
            dollar_math: v.value.dollar_math,
            display_math: v.value.display_math,
            literal: v.value.literal.clone(),
        });
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyMultilineBlockQuote>>(py) {
        return comrak_lib::nodes::NodeValue::MultilineBlockQuote(NodeMultilineBlockQuote {
            fence_length: v.value.fence_length,
            fence_offset: v.value.fence_offset,
        });
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PyEscaped>>(py) {
        return comrak_lib::nodes::NodeValue::Escaped;
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyWikiLink>>(py) {
        return comrak_lib::nodes::NodeValue::WikiLink(NodeWikiLink {
            url: v.value.url.clone(),
        });
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PyUnderline>>(py) {
        return comrak_lib::nodes::NodeValue::Underline;
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PySubscript>>(py) {
        return comrak_lib::nodes::NodeValue::Subscript;
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PySpoileredText>>(py) {
        return comrak_lib::nodes::NodeValue::SpoileredText;
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyEscapedTag>>(py) {
        return comrak_lib::nodes::NodeValue::EscapedTag(Box::leak(
            v.value.clone().into_boxed_str(),
        ));
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyAlert>>(py) {
        let a = &v.value;
        let alert_type = match a.alert_type {
            PyAlertType::Note => AlertType::Note,
            PyAlertType::Tip => AlertType::Tip,
            PyAlertType::Important => AlertType::Important,
            PyAlertType::Warning => AlertType::Warning,
            PyAlertType::Caution => AlertType::Caution,
        };

        return comrak_lib::nodes::NodeValue::Alert(Box::new(NodeAlert {
            alert_type,
            title: a.title.clone(),
            multiline: a.multiline,
            fence_length: a.fence_length,
            fence_offset: a.fence_offset,
        }));
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PySubtext>>(py) {
        return comrak_lib::nodes::NodeValue::Subtext;
    }

    // Fallback: default to Document if unknown
    comrak_lib::nodes::NodeValue::Document
}

impl PyAstNode {
    pub fn from_comrak_node<'a>(
        py: Python<'a>,
        node: &'a AstNode<'a>,
        parent: Option<Py<PyAstNode>>,
    ) -> Py<PyAstNode> {
        let ast = node.data.borrow();
        let node_value = create_py_node_value(py, &ast.value);
        let sourcepos: PySourcepos = PySourcepos::from(&ast.sourcepos);
        // Create the current PyAstNode with the owned parent handle (if any).
        let current = Py::new(
            py,
            PyAstNode {
                node_value,
                sourcepos,
                parent: parent.as_ref().map(|p| p.clone_ref(py)),
                children: Vec::new(),
            },
        )
        .unwrap();

        // Build children with `current` as their parent, then append them.
        for child in node.children() {
            let child_py = Self::from_comrak_node(py, child, Some(current.clone_ref(py)));
            // Borrow the PyAstNode instance mutably to push the child.
            let mut current_ref = current.borrow_mut(py);
            current_ref.children.push(child_py);
        }

        current
    }

    pub fn to_comrak_node<'a>(
        &self,
        py: Python<'a>,
        arena: &'a comrak_lib::Arena<'a>,
    ) -> &'a comrak_lib::nodes::AstNode<'a> {
        let node_value = self.node_value.as_ref();
        let ast_node_value = create_comrak_node_value(py, node_value);

        let node_in_arena = arena.alloc(
            comrak_lib::nodes::Ast::new_with_sourcepos(
                ast_node_value,
                py_sourcepos_to_sourcepos(&self.sourcepos),
            )
            .into(),
        );

        for child in &self.children {
            let child_handle = child.clone_ref(py);
            let child_ref = child_handle.borrow(py);
            let child_ast_node = child_ref.to_comrak_node(py, arena);
            node_in_arena.append(child_ast_node);
        }

        node_in_arena
    }
}
