from typing import Optional, Generic, TypeVar
from enum import Enum

T = TypeVar("T")

class NodeValue(Generic[T]):
    pass

class HeexNode(Generic[T]):
    pass

class ListDelimType(Enum):
    Period = 1
    Paren = 2

class ListType(Enum):
    Bullet = 1
    Ordered = 2

class TableAlignment(Enum):
    None_ = 1
    Left = 2
    Center = 3
    Right = 4

class AlertType(Enum):
    Note = 1
    Tip = 2
    Important = 3
    Warning = 4
    Caution = 5

class ListStyleType(Enum):
    Dash = 45
    Plus = 43
    Star = 42

class NodeCode:
    num_backticks: int
    literal: str
    def __init__(self, num_backticks: int, literal: str) -> None: ...

class NodeHtmlBlock:
    block_type: int
    literal: str
    def __init__(self, block_type: int, literal: str) -> None: ...

class NodeList:
    list_type: ListType
    marker_offset: int
    padding: int
    start: int
    delimiter: ListDelimType
    bullet_char: int
    tight: bool
    is_task_list: bool
    def __init__(
        self,
        list_type: ListType,
        marker_offset: int,
        padding: int,
        start: int,
        delimiter: ListDelimType,
        bullet_char: int,
        tight: bool,
        is_task_list: bool,
    ) -> None: ...

class NodeDescriptionItem:
    marker_offset: int
    padding: int
    tight: bool
    def __init__(self, marker_offset: int, padding: int, tight: bool) -> None: ...

class NodeCodeBlock:
    fenced: bool
    fence_char: int
    fence_length: int
    fence_offset: int
    info: str
    literal: str
    closed: bool
    def __init__(
        self,
        fenced: bool,
        fence_char: int,
        fence_length: int,
        fence_offset: int,
        info: str,
        literal: str,
        closed: bool,
    ) -> None: ...

class NodeHeading:
    level: int
    setext: bool
    closed: bool
    def __init__(self, level: int, setext: bool, closed: bool) -> None: ...

class NodeTable:
    alignments: list[TableAlignment]
    num_columns: int
    num_rows: int
    num_nonempty_cells: int
    def __init__(
        self,
        alignments: list[TableAlignment] | None,
        num_columns: int,
        num_rows: int,
        num_nonempty_cells: int,
    ) -> None: ...

class NodeTaskItem:
    symbol: Optional[str]
    symbol_sourcepos: Sourcepos
    def __init__(self, symbol: Optional[str], symbol_sourcepos: Sourcepos) -> None: ...

class NodeLink:
    url: str
    title: str
    def __init__(self, url: str, title: str) -> None: ...

class NodeFootnoteDefinition:
    name: str
    total_references: int
    def __init__(self, name: str, total_references: int) -> None: ...

class NodeFootnoteReference:
    name: str
    texts: list[tuple[str, int]]
    ref_num: int
    ix: int
    def __init__(
        self, name: str, texts: list[tuple[str, int]], ref_num: int, ix: int
    ) -> None: ...

class NodeWikiLink:
    url: str
    def __init__(self, url: str) -> None: ...

class NodeShortCode:
    code: str
    emoji: str
    def __init__(self, code: str, emoji: str) -> None: ...

class NodeMath:
    dollar_math: bool
    display_math: bool
    literal: str
    def __init__(self, dollar_math: bool, display_math: bool, literal: str) -> None: ...

class NodeMultilineBlockQuote:
    fence_length: int
    fence_offset: int
    def __init__(self, fence_length: int, fence_offset: int) -> None: ...

class NodeAlert:
    alert_type: AlertType
    title: Optional[str]
    multiline: bool
    fence_length: int
    fence_offset: int
    def __init__(
        self,
        alert_type: AlertType,
        title: Optional[str],
        multiline: bool,
        fence_length: int,
        fence_offset: int,
    ) -> None: ...

class HeexNodeDirective(HeexNode[None]):
    def __init__(self) -> None: ...

class HeexNodeComment(HeexNode[None]):
    def __init__(self) -> None: ...

class HeexNodeMultilineComment(HeexNode[None]):
    def __init__(self) -> None: ...

class HeexNodeExpression(HeexNode[None]):
    def __init__(self) -> None: ...

class HeexNodeTag(HeexNode[str]):
    tag: str
    def __init__(self, tag: str) -> None: ...

class NodeHeexBlock:
    literal: str
    node: HeexNode
    def __init__(self, literal: str, node: HeexNode) -> None: ...

class HeexBlock(NodeValue[NodeHeexBlock]):
    value: NodeHeexBlock
    def __init__(self, value: NodeHeexBlock) -> None: ...

class HeexInline(NodeValue[str]):
    value: str
    def __init__(self, value: str) -> None: ...

class Document(NodeValue[None]):
    def __init__(self) -> None: ...

class FrontMatter(NodeValue[str]):
    value: str
    def __init__(self, value: str) -> None: ...

class BlockQuote(NodeValue[None]):
    def __init__(self) -> None: ...

class List(NodeValue[NodeList]):
    value: NodeList
    def __init__(self, value: NodeList) -> None: ...

class Item(NodeValue[NodeList]):
    value: NodeList
    def __init__(self, value: NodeList) -> None: ...

class DescriptionList(NodeValue[None]):
    def __init__(self) -> None: ...

class DescriptionItem(NodeValue[NodeDescriptionItem]):
    value: NodeDescriptionItem
    def __init__(self, value: NodeDescriptionItem) -> None: ...

class DescriptionTerm(NodeValue[None]):
    def __init__(self) -> None: ...

class DescriptionDetails(NodeValue[None]):
    def __init__(self) -> None: ...

class CodeBlock(NodeValue[NodeCodeBlock]):
    value: NodeCodeBlock
    def __init__(self, value: NodeCodeBlock) -> None: ...

class HtmlBlock(NodeValue[NodeHtmlBlock]):
    value: NodeHtmlBlock
    def __init__(self, value: NodeHtmlBlock) -> None: ...

class Paragraph(NodeValue[None]):
    def __init__(self) -> None: ...

class Heading(NodeValue[NodeHeading]):
    value: NodeHeading
    def __init__(self, value: NodeHeading) -> None: ...

class ThematicBreak(NodeValue[None]):
    def __init__(self) -> None: ...

class FootnoteDefinition(NodeValue[NodeFootnoteDefinition]):
    value: NodeFootnoteDefinition
    def __init__(self, value: NodeFootnoteDefinition) -> None: ...

class Table(NodeValue[NodeTable]):
    value: NodeTable
    def __init__(self, value: NodeTable) -> None: ...

class TableRow(NodeValue[bool]):
    value: bool
    def __init__(self, value: bool) -> None: ...

class TableCell(NodeValue[None]):
    def __init__(self) -> None: ...

class Text(NodeValue[str]):
    value: str
    def __init__(self, value: str) -> None: ...

class TaskItem(NodeValue[NodeTaskItem]):
    value: NodeTaskItem
    def __init__(self, value: NodeTaskItem) -> None: ...

class SoftBreak(NodeValue[None]):
    def __init__(self) -> None: ...

class LineBreak(NodeValue[None]):
    def __init__(self) -> None: ...

class Code(NodeValue[NodeCode]):
    value: NodeCode
    def __init__(self, value: NodeCode) -> None: ...

class HtmlInline(NodeValue[str]):
    value: str
    def __init__(self, value: str) -> None: ...

class Raw(NodeValue[str]):
    value: str
    def __init__(self, value: str) -> None: ...

class Emph(NodeValue[None]):
    def __init__(self) -> None: ...

class Strong(NodeValue[None]):
    def __init__(self) -> None: ...

class Strikethrough(NodeValue[None]):
    def __init__(self) -> None: ...

class Highlight(NodeValue[None]):
    def __init__(self) -> None: ...

class Superscript(NodeValue[None]):
    def __init__(self) -> None: ...

class Link(NodeValue[NodeLink]):
    value: NodeLink
    def __init__(self, value: NodeLink) -> None: ...

class Image(NodeValue[NodeLink]):
    value: NodeLink
    def __init__(self, value: NodeLink) -> None: ...

class FootnoteReference(NodeValue[NodeFootnoteReference]):
    value: NodeFootnoteReference
    def __init__(self, value: NodeFootnoteReference) -> None: ...

class ShortCode(NodeValue[NodeShortCode]):
    value: NodeShortCode
    def __init__(self, value: NodeShortCode) -> None: ...

class Math(NodeValue[NodeMath]):
    value: NodeMath
    def __init__(self, value: NodeMath) -> None: ...

class MultilineBlockQuote(NodeValue[NodeMultilineBlockQuote]):
    value: NodeMultilineBlockQuote
    def __init__(self, value: NodeMultilineBlockQuote) -> None: ...

class Escaped(NodeValue[None]):
    def __init__(self) -> None: ...

class WikiLink(NodeValue[NodeWikiLink]):
    value: NodeWikiLink
    def __init__(self, value: NodeWikiLink) -> None: ...

class Underline(NodeValue[None]):
    def __init__(self) -> None: ...

class Subscript(NodeValue[None]):
    def __init__(self) -> None: ...

class SpoileredText(NodeValue[None]):
    def __init__(self) -> None: ...

class EscapedTag(NodeValue[str]):
    value: str
    def __init__(self, value: str) -> None: ...

class Alert(NodeValue[NodeAlert]):
    value: NodeAlert
    def __init__(self, value: NodeAlert) -> None: ...

class Subtext(NodeValue[None]):
    def __init__(self) -> None: ...

class LineColumn:
    line: int
    column: int
    def __init__(self, line: int, column: int) -> None: ...

class Sourcepos:
    start: LineColumn
    end: LineColumn
    def __init__(self, start: LineColumn, end: LineColumn) -> None: ...

class AstNode:
    node_value: NodeValue
    sourcepos: Sourcepos
    parent: Optional[AstNode]
    children: list[AstNode]
    def __init__(
        self,
        node_value: NodeValue,
        sourcepos: Sourcepos,
        parent: Optional[AstNode],
        children: list[AstNode],
    ) -> None: ...

class ExtensionOptions:
    strikethrough: bool
    tagfilter: bool
    table: bool
    autolink: bool
    tasklist: bool
    superscript: bool
    header_ids: Optional[str]
    footnotes: bool
    inline_footnotes: bool
    description_lists: bool
    front_matter_delimiter: Optional[str]
    multiline_block_quotes: bool
    alerts: bool
    math_dollars: bool
    math_code: bool
    shortcodes: bool
    wikilinks_title_after_pipe: bool
    wikilinks_title_before_pipe: bool
    underline: bool
    subscript: bool
    spoiler: bool
    greentext: bool
    cjk_friendly_emphasis: bool
    subtext: bool
    highlight: bool
    phoenix_heex: bool
    def __init__(
        self,
        strikethrough: bool = False,
        tagfilter: bool = False,
        table: bool = False,
        autolink: bool = False,
        tasklist: bool = False,
        superscript: bool = False,
        header_ids: Optional[str] = None,
        footnotes: bool = False,
        inline_footnotes: bool = False,
        description_lists: bool = False,
        front_matter_delimiter: Optional[str] = None,
        multiline_block_quotes: bool = False,
        alerts: bool = False,
        math_dollars: bool = False,
        math_code: bool = False,
        shortcodes: bool = False,
        wikilinks_title_after_pipe: bool = False,
        wikilinks_title_before_pipe: bool = False,
        underline: bool = False,
        subscript: bool = False,
        spoiler: bool = False,
        greentext: bool = False,
        cjk_friendly_emphasis: bool = False,
        subtext: bool = False,
        highlight: bool = False,
        phoenix_heex: bool = False,
    ) -> None: ...

class ParseOptions:
    smart: bool
    default_info_string: Optional[str]
    relaxed_tasklist_matching: bool
    tasklist_in_table: bool
    relaxed_autolinks: bool
    ignore_setext: bool
    leave_footnote_definitions: bool
    escaped_char_spans: bool
    def __init__(
        self,
        smart: bool = False,
        default_info_string: Optional[str] = None,
        relaxed_tasklist_matching: bool = False,
        tasklist_in_table: bool = False,
        relaxed_autolinks: bool = False,
        ignore_setext: bool = False,
        leave_footnote_definitions: bool = False,
        escaped_char_spans: bool = False,
    ) -> None: ...

class RenderOptions:
    hardbreaks: bool
    github_pre_lang: bool
    full_info_string: bool
    width: int
    unsafe_: bool
    escape: bool
    list_style: ListStyleType
    sourcepos: bool
    escaped_char_spans: bool
    ignore_empty_links: bool
    gfm_quirks: bool
    prefer_fenced: bool
    figure_with_caption: bool
    tasklist_classes: bool
    ol_width: int
    experimental_minimize_commonmark: bool
    def __init__(
        self,
        hardbreaks: bool = False,
        github_pre_lang: bool = False,
        full_info_string: bool = False,
        width: int = 0,
        unsafe_: bool = False,
        escape: bool = False,
        list_style: ListStyleType = ListStyleType.Dash,
        sourcepos: bool = False,
        escaped_char_spans: bool = False,
        ignore_empty_links: bool = False,
        gfm_quirks: bool = False,
        prefer_fenced: bool = False,
        figure_with_caption: bool = False,
        tasklist_classes: bool = False,
        ol_width: int = 0,
        experimental_minimize_commonmark: bool = False,
    ) -> None: ...

def markdown_to_html(
    text: str,
    extension_options: Optional[ExtensionOptions] = None,
    parse_options: Optional[ParseOptions] = None,
    render_options: Optional[RenderOptions] = None,
) -> str: ...
def markdown_to_commonmark(
    text: str,
    extension_options: Optional[ExtensionOptions] = None,
    parse_options: Optional[ParseOptions] = None,
    render_options: Optional[RenderOptions] = None,
) -> str: ...
def markdown_to_xml(
    text: str,
    extension_options: Optional[ExtensionOptions] = None,
    parse_options: Optional[ParseOptions] = None,
    render_options: Optional[RenderOptions] = None,
) -> str: ...
def parse_document(
    text: str,
    extension_options: Optional[ExtensionOptions] = None,
    parse_options: Optional[ParseOptions] = None,
    render_options: Optional[RenderOptions] = None,
) -> AstNode: ...
def format_html(
    node: AstNode,
    extension_options: Optional[ExtensionOptions] = None,
    parse_options: Optional[ParseOptions] = None,
    render_options: Optional[RenderOptions] = None,
) -> str: ...
def format_commonmark(
    node: AstNode,
    extension_options: Optional[ExtensionOptions] = None,
    parse_options: Optional[ParseOptions] = None,
    render_options: Optional[RenderOptions] = None,
) -> str: ...
def format_xml(
    node: AstNode,
    extension_options: Optional[ExtensionOptions] = None,
    parse_options: Optional[ParseOptions] = None,
    render_options: Optional[RenderOptions] = None,
) -> str: ...
