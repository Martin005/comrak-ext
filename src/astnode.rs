use std::fmt::Display;

use comrak::nodes::*;
use pyo3::{prelude::*, pyclass, IntoPyObjectExt, Python};

#[pyclass(name = "LineColumn", get_all, set_all, eq, str)]
#[derive(Clone, Debug, PartialEq, Eq)]
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

impl Display for PyLineColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LineColumn(line={}, column={})", self.line, self.column)
    }
}

#[pymethods]
impl PyLineColumn {
    #[new]
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "LineColumn(line={}, column={})",
            self.line, self.column
        ))
    }
}

#[pyclass(name = "Sourcepos", get_all, set_all, eq, str)]
#[derive(Clone, Debug, PartialEq, Eq)]
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

impl Display for PySourcepos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sourcepos(start={}, end={})", self.start, self.end)
    }
}

#[pymethods]
impl PySourcepos {
    #[new]
    pub fn new(start: PyLineColumn, end: PyLineColumn) -> Self {
        Self { start, end }
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Sourcepos(start={}, end={})", self.start, self.end))
    }
}

#[pyclass(name = "NodeCode", get_all, set_all, eq, str)]
#[derive(Clone, Debug, PartialEq, Eq)]
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

impl Display for PyNodeCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NodeCode(num_backticks={}, literal={})",
            self.num_backticks, self.literal
        )
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
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "NodeCode(num_backticks={}, literal={})",
            self.num_backticks, self.literal
        ))
    }
}

#[pyclass(name = "NodeHtmlBlock", get_all, set_all, eq, str)]
#[derive(Clone, Debug, PartialEq, Eq)]
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

impl Display for PyNodeHtmlBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NodeHtmlBlock(block_type={}, literal={})",
            self.block_type, self.literal
        )
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
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "NodeHtmlBlock(block_type={}, literal={})",
            self.block_type, self.literal
        ))
    }
}

#[pyclass(name = "ListDelimType", eq, eq_int, str)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PyListDelimType {
    Period,
    Paren,
}

impl Display for PyListDelimType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ListDelimType::{:?}", self)
    }
}

impl From<ListDelimType> for PyListDelimType {
    fn from(d: ListDelimType) -> Self {
        match d {
            ListDelimType::Period => PyListDelimType::Period,
            ListDelimType::Paren => PyListDelimType::Paren,
        }
    }
}

#[pyclass(name = "ListType", eq, eq_int, str)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PyListType {
    Bullet,
    Ordered,
}

impl Display for PyListType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ListType::{:?}", self)
    }
}

impl From<ListType> for PyListType {
    fn from(t: ListType) -> Self {
        match t {
            ListType::Bullet => PyListType::Bullet,
            ListType::Ordered => PyListType::Ordered,
        }
    }
}

#[pyclass(name = "TableAlignment", eq, eq_int, str)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PyTableAlignment {
    #[pyo3(name = "None_")] // named 'None_' because 'None' is reserved in Python
    None,
    Left,
    Center,
    Right,
}

impl Display for PyTableAlignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TableAlignment::{:?}", self)
    }
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

#[pyclass(name = "NodeList", get_all, set_all, eq, str)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

impl Display for PyNodeList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NodeList(list_type={:?}, marker_offset={}, padding={}, start={}, delimiter={:?}, bullet_char={}, tight={}, is_task_list={})",
            self.list_type, self.marker_offset, self.padding, self.start, self.delimiter, self.bullet_char, self.tight, self.is_task_list
        )
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
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "NodeList(list_type={:?}, marker_offset={}, padding={}, start={}, delimiter={:?}, bullet_char={}, tight={}, is_task_list={})",
            self.list_type, self.marker_offset, self.padding, self.start, self.delimiter, self.bullet_char, self.tight, self.is_task_list
        ))
    }
}

#[pyclass(name = "NodeDescriptionItem", get_all, set_all, eq, str)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

impl Display for PyNodeDescriptionItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NodeDescriptionItem(marker_offset={}, padding={}, tight={})",
            self.marker_offset, self.padding, self.tight
        )
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
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "NodeDescriptionItem(marker_offset={}, padding={}, tight={})",
            self.marker_offset, self.padding, self.tight
        ))
    }
}

#[pyclass(name = "NodeCodeBlock", get_all, set_all, eq, str)]
#[derive(Clone, Debug, PartialEq, Eq)]
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

impl Display for PyNodeCodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NodeCodeBlock(fenced={}, fence_char={}, fence_length={}, fence_offset={}, info={}, literal={}, closed={})",
            self.fenced, self.fence_char, self.fence_length, self.fence_offset, self.info, self.literal, self.closed
        )
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
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "NodeCodeBlock(fenced={}, fence_char={}, fence_length={}, fence_offset={}, info={}, literal={}, closed={})",
            self.fenced, self.fence_char, self.fence_length, self.fence_offset, self.info, self.literal, self.closed
        ))
    }
}

#[pyclass(name = "NodeHeading", get_all, set_all, eq, str)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

impl Display for PyNodeHeading {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NodeHeading(level={}, setext={}, closed={})",
            self.level, self.setext, self.closed
        )
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
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "NodeHeading(level={}, setext={}, closed={})",
            self.level, self.setext, self.closed
        ))
    }
}

#[pyclass(name = "NodeTable", get_all, set_all, eq, str)]
#[derive(Clone, Debug, PartialEq, Eq)]
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

impl Display for PyNodeTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NodeTable(alignments={:?}, num_columns={}, num_rows={}, num_nonempty_cells={})",
            self.alignments, self.num_columns, self.num_rows, self.num_nonempty_cells
        )
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
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "NodeTable(alignments={:?}, num_columns={}, num_rows={}, num_nonempty_cells={})",
            self.alignments, self.num_columns, self.num_rows, self.num_nonempty_cells
        ))
    }
}

#[pyclass(name = "NodeTaskItem", get_all, set_all, eq, str)]
#[derive(Clone, Debug, PartialEq, Eq)]
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

impl Display for PyNodeTaskItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NodeTaskItem(symbol={:?}, symbol_sourcepos={:?})",
            self.symbol, self.symbol_sourcepos
        )
    }
}

#[pymethods]
impl PyNodeTaskItem {
    #[new]
    pub fn new(symbol: Option<char>, symbol_sourcepos: PySourcepos) -> Self {
        Self {
            symbol,
            symbol_sourcepos,
        }
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "NodeTaskItem(symbol={:?}, symbol_sourcepos={:?})",
            self.symbol, self.symbol_sourcepos
        ))
    }
}

#[pyclass(name = "NodeLink", get_all, set_all, eq, str)]
#[derive(Clone, Debug, PartialEq, Eq)]
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

impl Display for PyNodeLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NodeLink(url={}, title={})", self.url, self.title)
    }
}

#[pymethods]
impl PyNodeLink {
    #[new]
    pub fn new(url: String, title: String) -> Self {
        Self { url, title }
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("NodeLink(url={}, title={})", self.url, self.title))
    }
}

#[pyclass(name = "NodeFootnoteDefinition", get_all, set_all, eq, str)]
#[derive(Clone, Debug, PartialEq, Eq)]
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

impl Display for PyNodeFootnoteDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NodeFootnoteDefinition(name={}, total_references={})",
            self.name, self.total_references
        )
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
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "NodeFootnoteDefinition(name={}, total_references={})",
            self.name, self.total_references
        ))
    }
}

#[pyclass(name = "NodeFootnoteReference", get_all, set_all, eq, str)]
#[derive(Clone, Debug, PartialEq, Eq)]
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

impl Display for PyNodeFootnoteReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NodeFootnoteReference(name={}, texts={:?}, ref_num={}, ix={})",
            self.name, self.texts, self.ref_num, self.ix
        )
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
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "NodeFootnoteReference(name={}, texts={:?}, ref_num={}, ix={})",
            self.name, self.texts, self.ref_num, self.ix
        ))
    }
}

#[pyclass(name = "NodeWikiLink", get_all, set_all, eq, str)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PyNodeWikiLink {
    pub url: String,
}

impl From<&NodeWikiLink> for PyNodeWikiLink {
    fn from(w: &NodeWikiLink) -> Self {
        Self { url: w.url.clone() }
    }
}

impl Display for PyNodeWikiLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NodeWikiLink(url={})", self.url)
    }
}

#[pymethods]
impl PyNodeWikiLink {
    #[new]
    pub fn new(url: String) -> Self {
        Self { url }
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("NodeWikiLink(url={})", self.url))
    }
}

#[pyclass(name = "NodeShortCode", get_all, set_all, eq, str)]
#[derive(Clone, Debug, PartialEq, Eq)]
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

impl Display for PyNodeShortCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NodeShortCode(code={}, emoji={})", self.code, self.emoji)
    }
}

#[pymethods]
impl PyNodeShortCode {
    #[new]
    pub fn new(code: String, emoji: String) -> Self {
        Self { code, emoji }
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "NodeShortCode(code={}, emoji={})",
            self.code, self.emoji
        ))
    }
}

#[pyclass(name = "NodeMath", get_all, set_all, eq, str)]
#[derive(Clone, Debug, PartialEq, Eq)]
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

impl Display for PyNodeMath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NodeMath(dollar_math={}, display_math={}, literal={})",
            self.dollar_math, self.display_math, self.literal
        )
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
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "NodeMath(dollar_math={}, display_math={}, literal={})",
            self.dollar_math, self.display_math, self.literal
        ))
    }
}

#[pyclass(name = "NodeMultilineBlockQuote", get_all, set_all, eq, str)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

impl Display for PyNodeMultilineBlockQuote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NodeMultilineBlockQuote(fence_length={}, fence_offset={})",
            self.fence_length, self.fence_offset
        )
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
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "NodeMultilineBlockQuote(fence_length={}, fence_offset={})",
            self.fence_length, self.fence_offset
        ))
    }
}

#[pyclass(name = "AlertType", eq, eq_int, str)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PyAlertType {
    Note,
    Tip,
    Important,
    Warning,
    Caution,
}

impl Display for PyAlertType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AlertType::{:?}", self)
    }
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

#[pyclass(name = "NodeAlert", get_all, set_all, eq, str)]
#[derive(Clone, Debug, PartialEq, Eq)]
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

impl Display for PyNodeAlert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NodeAlert(alert_type={:?}, title={:?}, multiline={}, fence_length={}, fence_offset={})",
            self.alert_type, self.title, self.multiline, self.fence_length, self.fence_offset
        )
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
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "NodeAlert(alert_type={:?}, title={:?}, multiline={}, fence_length={}, fence_offset={})",
            self.alert_type, self.title, self.multiline, self.fence_length, self.fence_offset
        ))
    }
}

#[pyclass(name = "NodeBlockDirective", get_all, set_all, eq, str)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PyNodeBlockDirective {
    pub fence_length: usize,
    pub fence_offset: usize,
    pub info: String,
}

impl From<&NodeBlockDirective> for PyNodeBlockDirective {
    fn from(bd: &NodeBlockDirective) -> Self {
        Self {
            fence_length: bd.fence_length,
            fence_offset: bd.fence_offset,
            info: bd.info.clone(),
        }
    }
}

impl Display for PyNodeBlockDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NodeBlockDirective(fence_length={}, fence_offset={}, info={})",
            self.fence_length, self.fence_offset, self.info
        )
    }
}

#[pymethods]
impl PyNodeBlockDirective {
    #[new]
    pub fn new(fence_length: usize, fence_offset: usize, info: String) -> Self {
        Self {
            fence_length,
            fence_offset,
            info,
        }
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "NodeBlockDirective(fence_length={}, fence_offset={}, info={})",
            self.fence_length, self.fence_offset, self.info
        ))
    }
}

#[pyclass(name = "HeexNode", subclass, eq, str)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PyHeexNode {}

impl Display for PyHeexNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HeexNode()")
    }
}

#[pymethods]
impl PyHeexNode {
    #[new]
    pub fn new() -> Self {
        Self {}
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok("HeexNode()".to_string())
    }
}

#[pyclass(name = "HeexNodeDirective", extends=PyHeexNode, eq, str)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PyHeexNodeDirective {}

impl Display for PyHeexNodeDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HeexNodeDirective()")
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

#[pymethods]
impl PyHeexNodeDirective {
    #[new]
    pub fn new() -> (Self, PyHeexNode) {
        (Self {}, PyHeexNode::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok("HeexNodeDirective()".to_string())
    }
}

#[pyclass(name = "HeexNodeComment", extends=PyHeexNode, eq, str)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PyHeexNodeComment {}

impl Display for PyHeexNodeComment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HeexNodeComment()")
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

#[pymethods]
impl PyHeexNodeComment {
    #[new]
    pub fn new() -> (Self, PyHeexNode) {
        (Self {}, PyHeexNode::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok("HeexNodeComment()".to_string())
    }
}

#[pyclass(name = "HeexNodeMultilineComment", extends=PyHeexNode, eq, str)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PyHeexNodeMultilineComment {}

impl Display for PyHeexNodeMultilineComment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HeexNodeMultilineComment()")
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

#[pymethods]
impl PyHeexNodeMultilineComment {
    #[new]
    pub fn new() -> (Self, PyHeexNode) {
        (Self {}, PyHeexNode::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok("HeexNodeMultilineComment()".to_string())
    }
}

#[pyclass(name = "HeexNodeExpression", extends=PyHeexNode, eq, str)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PyHeexNodeExpression {}

impl Display for PyHeexNodeExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HeexNodeExpression()")
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

#[pymethods]
impl PyHeexNodeExpression {
    #[new]
    pub fn new() -> (Self, PyHeexNode) {
        (Self {}, PyHeexNode::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok("HeexNodeExpression()".to_string())
    }
}

#[pyclass(name = "HeexNodeTag", extends=PyHeexNode, get_all, set_all, eq, str)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PyHeexNodeTag {
    pub tag: String,
}

impl Display for PyHeexNodeTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HeexNodeTag(tag={})", self.tag)
    }
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
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("HeexNodeTag(tag={})", self.tag))
    }
}

#[pyclass(name = "NodeHeexBlock", get_all, set_all, eq, str)]
#[derive(Debug)]
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

impl Display for PyNodeHeexBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NodeHeexBlock(literal={}, node={:?})",
            self.literal, self.node
        )
    }
}

#[pymethods]
impl PyNodeHeexBlock {
    #[new]
    pub fn new(literal: String, node: Py<PyAny>) -> Self {
        Self { literal, node }
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "NodeHeexBlock(literal={}, node={:?})",
            self.literal, self.node
        ))
    }
}

#[pyclass(name = "NodeValue", subclass, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyNodeValue {}

impl Display for PyNodeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NodeValue()")
    }
}

#[pymethods]
impl PyNodeValue {
    #[new]
    pub fn new() -> Self {
        Self {}
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok("NodeValue()".to_string())
    }
}

#[pyclass(name = "Document", extends=PyNodeValue, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyDocument {}

impl Display for PyDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Document()")
    }
}

#[pymethods]
impl PyDocument {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok("Document()".to_string())
    }
}

#[pyclass(name = "FrontMatter", extends=PyNodeValue, get_all, set_all, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyFrontMatter {
    pub value: String,
}

impl Display for PyFrontMatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FrontMatter(value={})", self.value)
    }
}

#[pymethods]
impl PyFrontMatter {
    #[new]
    pub fn new(value: String) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("FrontMatter(value={})", self.value))
    }
}

#[pyclass(name = "BlockQuote", extends=PyNodeValue, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyBlockQuote {}

impl Display for PyBlockQuote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BlockQuote()")
    }
}

#[pymethods]
impl PyBlockQuote {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok("BlockQuote()".to_string())
    }
}

#[pyclass(name = "List", extends=PyNodeValue, get_all, set_all, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyList {
    pub value: PyNodeList,
}

impl Display for PyList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "List(value={:?})", self.value)
    }
}

#[pymethods]
impl PyList {
    #[new]
    pub fn new(value: PyNodeList) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("List(value={:?})", self.value))
    }
}

#[pyclass(name = "Item", extends=PyNodeValue, get_all, set_all, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyItem {
    pub value: PyNodeList,
}

impl Display for PyItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Item(value={:?})", self.value)
    }
}

#[pymethods]
impl PyItem {
    #[new]
    pub fn new(value: PyNodeList) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Item(value={:?})", self.value))
    }
}

#[pyclass(name = "DescriptionList", extends=PyNodeValue, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyDescriptionList {}

impl Display for PyDescriptionList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DescriptionList()")
    }
}

#[pymethods]
impl PyDescriptionList {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok("DescriptionList()".to_string())
    }
}

#[pyclass(name = "DescriptionItem", extends=PyNodeValue, get_all, set_all, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyDescriptionItem {
    pub value: PyNodeDescriptionItem,
}

impl Display for PyDescriptionItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DescriptionItem(value={:?})", self.value)
    }
}

#[pymethods]
impl PyDescriptionItem {
    #[new]
    pub fn new(value: PyNodeDescriptionItem) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("DescriptionItem(value={:?})", self.value))
    }
}

#[pyclass(name = "DescriptionTerm", extends=PyNodeValue, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyDescriptionTerm {}

impl Display for PyDescriptionTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DescriptionTerm()")
    }
}

#[pymethods]
impl PyDescriptionTerm {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok("DescriptionTerm()".to_string())
    }
}

#[pyclass(name = "DescriptionDetails", extends=PyNodeValue, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyDescriptionDetails {}

impl Display for PyDescriptionDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DescriptionDetails()")
    }
}

#[pymethods]
impl PyDescriptionDetails {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok("DescriptionDetails()".to_string())
    }
}

#[pyclass(name = "CodeBlock", extends=PyNodeValue, get_all, set_all, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyCodeBlock {
    pub value: PyNodeCodeBlock,
}

impl Display for PyCodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CodeBlock(value={:?})", self.value)
    }
}

#[pymethods]
impl PyCodeBlock {
    #[new]
    pub fn new(value: PyNodeCodeBlock) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("CodeBlock(value={:?})", self.value))
    }
}

#[pyclass(name = "HtmlBlock", extends=PyNodeValue, get_all, set_all, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyHtmlBlock {
    pub value: PyNodeHtmlBlock,
}

impl Display for PyHtmlBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HtmlBlock(value={:?})", self.value)
    }
}

#[pymethods]
impl PyHtmlBlock {
    #[new]
    pub fn new(value: PyNodeHtmlBlock) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("HtmlBlock(value={:?})", self.value))
    }
}

#[pyclass(name = "HeexBlock", extends=PyNodeValue, get_all, set_all, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyHeexBlock {
    pub value: PyNodeHeexBlock,
}

impl Display for PyHeexBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HeexBlock(value={:?})", self.value)
    }
}

#[pymethods]
impl PyHeexBlock {
    #[new]
    pub fn new(value: PyNodeHeexBlock) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("HeexBlock(value={:?})", self.value))
    }
}

#[pyclass(name = "Paragraph", extends=PyNodeValue, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyParagraph {}

impl Display for PyParagraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Paragraph()")
    }
}

#[pymethods]
impl PyParagraph {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok("Paragraph()".to_string())
    }
}

#[pyclass(name = "Heading", extends=PyNodeValue, get_all, set_all, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyHeading {
    pub value: PyNodeHeading,
}

impl Display for PyHeading {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Heading(value={:?})", self.value)
    }
}

#[pymethods]
impl PyHeading {
    #[new]
    pub fn new(value: PyNodeHeading) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Heading(value={:?})", self.value))
    }
}

#[pyclass(name = "ThematicBreak", extends=PyNodeValue, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyThematicBreak {}

impl Display for PyThematicBreak {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ThematicBreak()")
    }
}

#[pymethods]
impl PyThematicBreak {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok("ThematicBreak()".to_string())
    }
}

#[pyclass(name = "FootnoteDefinition", extends=PyNodeValue, get_all, set_all, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyFootnoteDefinition {
    pub value: PyNodeFootnoteDefinition,
}

impl Display for PyFootnoteDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FootnoteDefinition(value={:?})", self.value)
    }
}

#[pymethods]
impl PyFootnoteDefinition {
    #[new]
    pub fn new(value: PyNodeFootnoteDefinition) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("FootnoteDefinition(value={:?})", self.value))
    }
}

#[pyclass(name = "Table", extends=PyNodeValue, get_all, set_all, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyTable {
    pub value: PyNodeTable,
}

impl Display for PyTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Table(value={:?})", self.value)
    }
}

#[pymethods]
impl PyTable {
    #[new]
    pub fn new(value: PyNodeTable) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Table(value={:?})", self.value))
    }
}

#[pyclass(name = "TableRow", extends=PyNodeValue, get_all, set_all, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyTableRow {
    pub value: bool,
}

impl Display for PyTableRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TableRow(value={})", self.value)
    }
}

#[pymethods]
impl PyTableRow {
    #[new]
    pub fn new(value: bool) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("TableRow(value={})", self.value))
    }
}

#[pyclass(name = "TableCell", extends=PyNodeValue, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyTableCell {}

impl Display for PyTableCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TableCell()")
    }
}

#[pymethods]
impl PyTableCell {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok("TableCell()".to_string())
    }
}

#[pyclass(name = "Text", extends=PyNodeValue, get_all, set_all, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyText {
    pub value: String,
}

impl Display for PyText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Text(value={})", self.value)
    }
}

#[pymethods]
impl PyText {
    #[new]
    pub fn new(value: String) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Text(value={})", self.value))
    }
}

#[pyclass(name = "TaskItem", extends=PyNodeValue, get_all, set_all, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyTaskItem {
    pub value: PyNodeTaskItem,
}

impl Display for PyTaskItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TaskItem(value={:?})", self.value)
    }
}

#[pymethods]
impl PyTaskItem {
    #[new]
    pub fn new(value: PyNodeTaskItem) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("TaskItem(value={:?})", self.value))
    }
}

#[pyclass(name = "SoftBreak", extends=PyNodeValue, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PySoftBreak {}

impl Display for PySoftBreak {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SoftBreak()")
    }
}

#[pymethods]
impl PySoftBreak {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok("SoftBreak()".to_string())
    }
}

#[pyclass(name = "LineBreak", extends=PyNodeValue, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyLineBreak {}

impl Display for PyLineBreak {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LineBreak()")
    }
}

#[pymethods]
impl PyLineBreak {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok("LineBreak()".to_string())
    }
}

#[pyclass(name = "Code", extends=PyNodeValue, get_all, set_all, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyCode {
    pub value: PyNodeCode,
}

impl Display for PyCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Code(value={:?})", self.value)
    }
}

#[pymethods]
impl PyCode {
    #[new]
    pub fn new(value: PyNodeCode) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Code(value={:?})", self.value))
    }
}

#[pyclass(name = "HtmlInline", extends=PyNodeValue, get_all, set_all, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyHtmlInline {
    pub value: String,
}

impl Display for PyHtmlInline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HtmlInline(value={})", self.value)
    }
}

#[pymethods]
impl PyHtmlInline {
    #[new]
    pub fn new(value: String) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("HtmlInline(value={})", self.value))
    }
}

#[pyclass(name = "HeexInline", extends=PyNodeValue, get_all, set_all, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyHeexInline {
    pub value: String,
}

impl Display for PyHeexInline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HeexInline(value={})", self.value)
    }
}

#[pymethods]
impl PyHeexInline {
    #[new]
    pub fn new(value: String) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("HeexInline(value={})", self.value))
    }
}

#[pyclass(name = "Raw", extends=PyNodeValue, get_all, set_all, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyRaw {
    pub value: String,
}

impl Display for PyRaw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Raw(value={})", self.value)
    }
}

#[pymethods]
impl PyRaw {
    #[new]
    pub fn new(value: String) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Raw(value={})", self.value))
    }
}

#[pyclass(name = "Emph", extends=PyNodeValue, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyEmph {}

impl Display for PyEmph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Emph()")
    }
}

#[pymethods]
impl PyEmph {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok("Emph()".to_string())
    }
}

#[pyclass(name = "Strong", extends=PyNodeValue, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyStrong {}

impl Display for PyStrong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Strong()")
    }
}

#[pymethods]
impl PyStrong {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok("Strong()".to_string())
    }
}

#[pyclass(name = "Strikethrough", extends=PyNodeValue, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyStrikethrough {}

impl Display for PyStrikethrough {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Strikethrough()")
    }
}

#[pymethods]
impl PyStrikethrough {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok("Strikethrough()".to_string())
    }
}

#[pyclass(name = "Highlight", extends=PyNodeValue, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyHighlight {}

impl Display for PyHighlight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Highlight()")
    }
}

#[pymethods]
impl PyHighlight {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok("Highlight()".to_string())
    }
}

#[pyclass(name = "Insert", extends=PyNodeValue, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyInsert {}

impl Display for PyInsert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Insert()")
    }
}

#[pymethods]
impl PyInsert {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok("Insert()".to_string())
    }
}

#[pyclass(name = "Superscript", extends=PyNodeValue, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PySuperscript {}

impl Display for PySuperscript {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Superscript()")
    }
}

#[pymethods]
impl PySuperscript {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok("Superscript()".to_string())
    }
}

#[pyclass(name = "Link", extends=PyNodeValue, get_all, set_all, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyLink {
    pub value: PyNodeLink,
}

impl Display for PyLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Link(value={:?})", self.value)
    }
}

#[pymethods]
impl PyLink {
    #[new]
    pub fn new(value: PyNodeLink) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Link(value={:?})", self.value))
    }
}

#[pyclass(name = "Image", extends=PyNodeValue, get_all, set_all, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyImage {
    pub value: PyNodeLink,
}

impl Display for PyImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Image(value={:?})", self.value)
    }
}

#[pymethods]
impl PyImage {
    #[new]
    pub fn new(value: PyNodeLink) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Image(value={:?})", self.value))
    }
}

#[pyclass(name = "FootnoteReference", extends=PyNodeValue, get_all, set_all, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyFootnoteReference {
    pub value: PyNodeFootnoteReference,
}

impl Display for PyFootnoteReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FootnoteReference(value={:?})", self.value)
    }
}

#[pymethods]
impl PyFootnoteReference {
    #[new]
    pub fn new(value: PyNodeFootnoteReference) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("FootnoteReference(value={:?})", self.value))
    }
}

#[pyclass(name = "ShortCode", extends=PyNodeValue, get_all, set_all, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyShortCode {
    pub value: PyNodeShortCode,
}

impl Display for PyShortCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ShortCode(value={:?})", self.value)
    }
}

#[pymethods]
impl PyShortCode {
    #[new]
    pub fn new(value: PyNodeShortCode) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("ShortCode(value={:?})", self.value))
    }
}

#[pyclass(name = "Math", extends=PyNodeValue, get_all, set_all, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyMath {
    pub value: PyNodeMath,
}

impl Display for PyMath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Math(value={:?})", self.value)
    }
}

#[pymethods]
impl PyMath {
    #[new]
    pub fn new(value: PyNodeMath) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Math(value={:?})", self.value))
    }
}

#[pyclass(name = "MultilineBlockQuote", extends=PyNodeValue, get_all, set_all, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyMultilineBlockQuote {
    pub value: PyNodeMultilineBlockQuote,
}

impl Display for PyMultilineBlockQuote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MultilineBlockQuote(value={:?})", self.value)
    }
}

#[pymethods]
impl PyMultilineBlockQuote {
    #[new]
    pub fn new(value: PyNodeMultilineBlockQuote) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("MultilineBlockQuote(value={:?})", self.value))
    }
}

#[pyclass(name = "Escaped", extends=PyNodeValue, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyEscaped {}

impl Display for PyEscaped {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Escaped()")
    }
}

#[pymethods]
impl PyEscaped {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok("Escaped()".to_string())
    }
}

#[pyclass(name = "WikiLink", extends=PyNodeValue, get_all, set_all, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyWikiLink {
    pub value: PyNodeWikiLink,
}

impl Display for PyWikiLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WikiLink(value={:?})", self.value)
    }
}

#[pymethods]
impl PyWikiLink {
    #[new]
    pub fn new(value: PyNodeWikiLink) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("WikiLink(value={:?})", self.value))
    }
}

#[pyclass(name = "Underline", extends=PyNodeValue, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyUnderline {}

impl Display for PyUnderline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Underline()")
    }
}

#[pymethods]
impl PyUnderline {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok("Underline()".to_string())
    }
}

#[pyclass(name = "Subscript", extends=PyNodeValue, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PySubscript {}

impl Display for PySubscript {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Subscript()")
    }
}

#[pymethods]
impl PySubscript {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok("Subscript()".to_string())
    }
}

#[pyclass(name = "SpoileredText", extends=PyNodeValue, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PySpoileredText {}

impl Display for PySpoileredText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SpoileredText()")
    }
}

#[pymethods]
impl PySpoileredText {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok("SpoileredText()".to_string())
    }
}

#[pyclass(name = "EscapedTag", extends=PyNodeValue, get_all, set_all, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyEscapedTag {
    pub value: String,
}

impl Display for PyEscapedTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EscapedTag(value={})", self.value)
    }
}

#[pymethods]
impl PyEscapedTag {
    #[new]
    pub fn new(value: String) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("EscapedTag(value={})", self.value))
    }
}

#[pyclass(name = "Alert", extends=PyNodeValue, get_all, set_all, eq, str)]
#[derive(PartialEq, Debug, Eq)]
pub struct PyAlert {
    pub value: PyNodeAlert,
}

impl Display for PyAlert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Alert(value={:?})", self.value)
    }
}

#[pymethods]
impl PyAlert {
    #[new]
    pub fn new(value: PyNodeAlert) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Alert(value={:?})", self.value))
    }
}

#[pyclass(name = "Subtext", extends=PyNodeValue, eq, str)]
#[derive(PartialEq, Debug, Eq)]

pub struct PySubtext {}

impl Display for PySubtext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Subtext()")
    }
}

#[pymethods]
impl PySubtext {
    #[new]
    pub fn new() -> (Self, PyNodeValue) {
        (Self {}, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok("Subtext()".to_string())
    }
}

#[pyclass(name = "BlockDirective", extends=PyNodeValue, get_all, set_all, eq, str)]
#[derive(Debug, PartialEq, Eq)]
pub struct PyBlockDirective {
    pub value: PyNodeBlockDirective,
}

impl Display for PyBlockDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BlockDirective(value={:?})", self.value)
    }
}

#[pymethods]
impl PyBlockDirective {
    #[new]
    pub fn new(value: PyNodeBlockDirective) -> (Self, PyNodeValue) {
        (Self { value }, PyNodeValue::new())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("BlockDirective(value={:?})", self.value))
    }
}

#[pyclass(name = "AstNode", get_all, set_all, str)]
#[derive(Debug)]
pub struct PyAstNode {
    pub node_value: Py<PyNodeValue>,
    pub sourcepos: PySourcepos,
    pub parent: Option<Py<PyAstNode>>,
    pub children: Vec<Py<PyAstNode>>,
}

impl Display for PyAstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = Python::attach(|py| {
            let node_value_repr = self
                .node_value
                .bind(py)
                .repr()
                .map(|s| s.to_string())
                .unwrap_or_else(|_| "?".to_string());
            let sourcepos_repr = format!("{}", self.sourcepos);

            let parent_repr = match &self.parent {
                Some(p) => p
                    .borrow(py)
                    .node_value
                    .bind(py)
                    .repr()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|_| "?".to_string()),
                None => "None".to_string(),
            };

            let children_repr: Vec<String> = self
                .children
                .iter()
                .map(|child| {
                    child
                        .borrow(py)
                        .node_value
                        .bind(py)
                        .repr()
                        .map(|s| s.to_string())
                        .unwrap_or_else(|_| "?".to_string())
                })
                .collect();

            format!(
                "AstNode(node_value={}, sourcepos={}, parent={}, children=[{}])",
                node_value_repr,
                sourcepos_repr,
                parent_repr,
                children_repr.join(", ")
            )
        });

        write!(f, "{}", s)
    }
}

#[pymethods]
impl PyAstNode {
    #[new]
    pub fn new(
        node_value: Py<PyNodeValue>,
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
    fn __repr__(&self) -> PyResult<String> {
        let s = Python::attach(|py| {
            let node_value_repr = self
                .node_value
                .bind(py)
                .repr()
                .map(|s| s.to_string())
                .unwrap_or_else(|_| "?".to_string());
            let sourcepos_repr = format!("{}", self.sourcepos);

            let parent_repr = match &self.parent {
                Some(p) => p
                    .borrow(py)
                    .node_value
                    .bind(py)
                    .repr()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|_| "?".to_string()),
                None => "None".to_string(),
            };

            let children_repr: Vec<String> = self
                .children
                .iter()
                .map(|child| {
                    child
                        .borrow(py)
                        .node_value
                        .bind(py)
                        .repr()
                        .map(|s| s.to_string())
                        .unwrap_or_else(|_| "?".to_string())
                })
                .collect();

            format!(
                "AstNode(node_value={}, sourcepos={}, parent={}, children=[{}])",
                node_value_repr,
                sourcepos_repr,
                parent_repr,
                children_repr.join(", ")
            )
        });

        Ok(s)
    }
}

fn create_py_node_value(py: Python, value: &comrak::nodes::NodeValue) -> Py<PyNodeValue> {
    match value {
        comrak::nodes::NodeValue::Document => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyDocument {}),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::FrontMatter(s) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {})
                .add_subclass(PyFrontMatter { value: s.clone() }),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::BlockQuote => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyBlockQuote {}),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::List(l) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyList {
                value: PyNodeList::from(l),
            }),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::Item(i) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyItem {
                value: PyNodeList::from(i),
            }),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::DescriptionList => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyDescriptionList {}),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::DescriptionItem(d) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyDescriptionItem {
                value: PyNodeDescriptionItem::from(d),
            }),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::DescriptionTerm => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyDescriptionTerm {}),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::DescriptionDetails => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyDescriptionDetails {}),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::CodeBlock(c) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyCodeBlock {
                value: PyNodeCodeBlock::from(c.as_ref()),
            }),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::HtmlBlock(h) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyHtmlBlock {
                value: PyNodeHtmlBlock::from(h),
            }),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::HeexBlock(h) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyHeexBlock {
                value: PyNodeHeexBlock::from((py, h.as_ref())),
            }),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::Paragraph => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyParagraph {}),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::Heading(h) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyHeading {
                value: PyNodeHeading::from(h),
            }),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::ThematicBreak => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyThematicBreak {}),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::FootnoteDefinition(f) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyFootnoteDefinition {
                value: PyNodeFootnoteDefinition::from(f),
            }),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::Table(t) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyTable {
                value: PyNodeTable::from(t.as_ref()),
            }),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::TableRow(is_header) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyTableRow { value: *is_header }),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::TableCell => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyTableCell {}),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::Text(t) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyText {
                value: t.to_string(),
            }),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::TaskItem(c) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyTaskItem {
                value: PyNodeTaskItem::from(c),
            }),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::SoftBreak => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PySoftBreak {}),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::LineBreak => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyLineBreak {}),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::Code(c) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyCode {
                value: PyNodeCode::from(c),
            }),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::HtmlInline(s) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {})
                .add_subclass(PyHtmlInline { value: s.clone() }),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::HeexInline(s) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {})
                .add_subclass(PyHeexInline { value: s.clone() }),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::Raw(s) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyRaw { value: s.clone() }),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::Emph => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyEmph {}),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::Strong => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyStrong {}),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::Strikethrough => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyStrikethrough {}),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::Highlight => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyHighlight {}),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::Insert => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyInsert {}),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::Superscript => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PySuperscript {}),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::Link(l) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyLink {
                value: PyNodeLink::from(l.as_ref()),
            }),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::Image(i) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyImage {
                value: PyNodeLink::from(i.as_ref()),
            }),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::FootnoteReference(f) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyFootnoteReference {
                value: PyNodeFootnoteReference::from(f.as_ref()),
            }),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::ShortCode(s) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyShortCode {
                value: PyNodeShortCode::from(s.as_ref()),
            }),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::Math(m) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyMath {
                value: PyNodeMath::from(m),
            }),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::MultilineBlockQuote(m) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyMultilineBlockQuote {
                value: PyNodeMultilineBlockQuote::from(m),
            }),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::Escaped => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyEscaped {}),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::WikiLink(w) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyWikiLink {
                value: PyNodeWikiLink::from(w),
            }),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::Underline => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyUnderline {}),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::Subscript => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PySubscript {}),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::SpoileredText => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PySpoileredText {}),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::EscapedTag(s) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyEscapedTag {
                value: s.to_string(),
            }),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::Alert(a) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyAlert {
                value: PyNodeAlert::from(a.as_ref()),
            }),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::Subtext => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PySubtext {}),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
        comrak::nodes::NodeValue::BlockDirective(d) => Py::new(
            py,
            PyClassInitializer::from(PyNodeValue {}).add_subclass(PyBlockDirective {
                value: PyNodeBlockDirective::from(d.as_ref()),
            }),
        )
        .unwrap()
        .into_bound(py)
        .into_super()
        .unbind(),
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
) -> comrak::nodes::NodeValue {
    let any = node_value.as_ref();

    // Try each concrete Python subclass in turn and build the matching NodeValue
    if let Ok(_v) = any.extract::<pyo3::PyRef<PyDocument>>(py) {
        return comrak::nodes::NodeValue::Document;
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyFrontMatter>>(py) {
        return comrak::nodes::NodeValue::FrontMatter(v.value.clone());
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PyBlockQuote>>(py) {
        return comrak::nodes::NodeValue::BlockQuote;
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

        return comrak::nodes::NodeValue::List(NodeList {
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

        return comrak::nodes::NodeValue::Item(NodeList {
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
        return comrak::nodes::NodeValue::DescriptionList;
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyDescriptionItem>>(py) {
        let di = &v.value;
        return comrak::nodes::NodeValue::DescriptionItem(NodeDescriptionItem {
            marker_offset: di.marker_offset,
            padding: di.padding,
            tight: di.tight,
        });
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PyDescriptionTerm>>(py) {
        return comrak::nodes::NodeValue::DescriptionTerm;
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PyDescriptionDetails>>(py) {
        return comrak::nodes::NodeValue::DescriptionDetails;
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyCodeBlock>>(py) {
        let cb = &v.value;
        return comrak::nodes::NodeValue::CodeBlock(Box::new(NodeCodeBlock {
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
        return comrak::nodes::NodeValue::HtmlBlock(NodeHtmlBlock {
            block_type: hb.block_type,
            literal: hb.literal.clone(),
        });
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyHeexBlock>>(py) {
        let hb = &v.value;
        // Convert HeexNode Py<PyAny> -> HeexNode
        let py_node_any = hb.node.as_ref();
        if let Ok(_) = py_node_any.extract::<pyo3::PyRef<PyHeexNodeDirective>>(py) {
            return comrak::nodes::NodeValue::HeexBlock(Box::new(NodeHeexBlock {
                literal: hb.literal.clone(),
                node: HeexNode::Directive,
            }));
        }
        if let Ok(_) = py_node_any.extract::<pyo3::PyRef<PyHeexNodeComment>>(py) {
            return comrak::nodes::NodeValue::HeexBlock(Box::new(NodeHeexBlock {
                literal: hb.literal.clone(),
                node: HeexNode::Comment,
            }));
        }
        if let Ok(_) = py_node_any.extract::<pyo3::PyRef<PyHeexNodeMultilineComment>>(py) {
            return comrak::nodes::NodeValue::HeexBlock(Box::new(NodeHeexBlock {
                literal: hb.literal.clone(),
                node: HeexNode::MultilineComment,
            }));
        }
        if let Ok(_) = py_node_any.extract::<pyo3::PyRef<PyHeexNodeExpression>>(py) {
            return comrak::nodes::NodeValue::HeexBlock(Box::new(NodeHeexBlock {
                literal: hb.literal.clone(),
                node: HeexNode::Expression,
            }));
        }
        if let Ok(tag) = py_node_any.extract::<pyo3::PyRef<PyHeexNodeTag>>(py) {
            return comrak::nodes::NodeValue::HeexBlock(Box::new(NodeHeexBlock {
                literal: hb.literal.clone(),
                node: HeexNode::Tag(tag.tag.clone()),
            }));
        }
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PyParagraph>>(py) {
        return comrak::nodes::NodeValue::Paragraph;
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyHeading>>(py) {
        let h = &v.value;
        return comrak::nodes::NodeValue::Heading(NodeHeading {
            level: h.level,
            setext: h.setext,
            closed: h.closed,
        });
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PyThematicBreak>>(py) {
        return comrak::nodes::NodeValue::ThematicBreak;
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyFootnoteDefinition>>(py) {
        let f = &v.value;
        return comrak::nodes::NodeValue::FootnoteDefinition(NodeFootnoteDefinition {
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

        return comrak::nodes::NodeValue::Table(Box::new(NodeTable {
            alignments,
            num_columns: t.num_columns,
            num_rows: t.num_rows,
            num_nonempty_cells: t.num_nonempty_cells,
        }));
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyTableRow>>(py) {
        return comrak::nodes::NodeValue::TableRow(v.value);
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PyTableCell>>(py) {
        return comrak::nodes::NodeValue::TableCell;
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyText>>(py) {
        return comrak::nodes::NodeValue::Text(v.value.clone().into());
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyTaskItem>>(py) {
        let ti = &v.value;
        return comrak::nodes::NodeValue::TaskItem(NodeTaskItem {
            symbol: ti.symbol,
            symbol_sourcepos: py_sourcepos_to_sourcepos(&ti.symbol_sourcepos),
        });
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PySoftBreak>>(py) {
        return comrak::nodes::NodeValue::SoftBreak;
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PyLineBreak>>(py) {
        return comrak::nodes::NodeValue::LineBreak;
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyCode>>(py) {
        let c = &v.value;
        return comrak::nodes::NodeValue::Code(NodeCode {
            num_backticks: c.num_backticks,
            literal: c.literal.clone(),
        });
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyHtmlInline>>(py) {
        return comrak::nodes::NodeValue::HtmlInline(v.value.clone());
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyHeexInline>>(py) {
        return comrak::nodes::NodeValue::HeexInline(v.value.clone());
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyRaw>>(py) {
        return comrak::nodes::NodeValue::Raw(v.value.clone());
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PyEmph>>(py) {
        return comrak::nodes::NodeValue::Emph;
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PyStrong>>(py) {
        return comrak::nodes::NodeValue::Strong;
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PyStrikethrough>>(py) {
        return comrak::nodes::NodeValue::Strikethrough;
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PyHighlight>>(py) {
        return comrak::nodes::NodeValue::Highlight;
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PyInsert>>(py) {
        return comrak::nodes::NodeValue::Insert;
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PySuperscript>>(py) {
        return comrak::nodes::NodeValue::Superscript;
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyLink>>(py) {
        return comrak::nodes::NodeValue::Link(Box::new(NodeLink {
            url: v.value.url.clone(),
            title: v.value.title.clone(),
        }));
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyImage>>(py) {
        return comrak::nodes::NodeValue::Image(Box::new(NodeLink {
            url: v.value.url.clone(),
            title: v.value.title.clone(),
        }));
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyFootnoteReference>>(py) {
        let fr = &v.value;
        return comrak::nodes::NodeValue::FootnoteReference(Box::new(NodeFootnoteReference {
            name: fr.name.clone(),
            texts: fr.texts.clone(),
            ref_num: fr.ref_num,
            ix: fr.ix,
        }));
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyShortCode>>(py) {
        return comrak::nodes::NodeValue::ShortCode(Box::new(NodeShortCode {
            code: v.value.code.clone(),
            emoji: v.value.emoji.clone(),
        }));
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyMath>>(py) {
        return comrak::nodes::NodeValue::Math(NodeMath {
            dollar_math: v.value.dollar_math,
            display_math: v.value.display_math,
            literal: v.value.literal.clone(),
        });
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyMultilineBlockQuote>>(py) {
        return comrak::nodes::NodeValue::MultilineBlockQuote(NodeMultilineBlockQuote {
            fence_length: v.value.fence_length,
            fence_offset: v.value.fence_offset,
        });
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PyEscaped>>(py) {
        return comrak::nodes::NodeValue::Escaped;
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyWikiLink>>(py) {
        return comrak::nodes::NodeValue::WikiLink(NodeWikiLink {
            url: v.value.url.clone(),
        });
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PyUnderline>>(py) {
        return comrak::nodes::NodeValue::Underline;
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PySubscript>>(py) {
        return comrak::nodes::NodeValue::Subscript;
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PySpoileredText>>(py) {
        return comrak::nodes::NodeValue::SpoileredText;
    }

    if let Ok(v) = any.extract::<pyo3::PyRef<PyEscapedTag>>(py) {
        return comrak::nodes::NodeValue::EscapedTag(Box::leak(v.value.clone().into_boxed_str()));
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

        return comrak::nodes::NodeValue::Alert(Box::new(NodeAlert {
            alert_type,
            title: a.title.clone(),
            multiline: a.multiline,
            fence_length: a.fence_length,
            fence_offset: a.fence_offset,
        }));
    }

    if let Ok(_v) = any.extract::<pyo3::PyRef<PySubtext>>(py) {
        return comrak::nodes::NodeValue::Subtext;
    }

    // Fallback: default to Document if unknown
    comrak::nodes::NodeValue::Document
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
        arena: &'a comrak::Arena<'a>,
    ) -> &'a comrak::nodes::AstNode<'a> {
        let node_value = self.node_value.as_ref();
        let ast_node_value = create_comrak_node_value(py, node_value);

        let node_in_arena = arena.alloc(
            comrak::nodes::Ast::new_with_sourcepos(
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
