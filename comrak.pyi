from typing import Optional, Generic, TypeVar
from enum import Enum

T = TypeVar("T")

class NodeValue(Generic[T]):
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

class NodeHtmlBlock:
    block_type: int
    literal: str

class NodeList:
    list_type: ListType
    marker_offset: int
    padding: int
    start: int
    delimiter: ListDelimType
    bullet_char: int
    tight: bool
    is_task_list: bool

class NodeDescriptionItem:
    marker_offset: int
    padding: int
    tight: bool

class NodeCodeBlock:
    fenced: bool
    fence_char: int
    fence_length: int
    fence_offset: int
    info: str
    literal: str

class NodeHeading:
    level: int
    setext: bool

class NodeTable:
    alignments: list[TableAlignment]
    num_columns: int
    num_rows: int
    num_nonempty_cells: int

class NodeLink:
    url: str
    title: str

class NodeFootnoteDefinition:
    name: str
    total_references: int

class NodeFootnoteReference:
    name: str
    ref_num: int
    ix: int

class NodeWikiLink:
    url: str

class NodeShortCode:
    code: str
    emoji: str

class NodeMath:
    dollar_math: bool
    display_math: bool
    literal: str

class NodeMultilineBlockQuote:
    fence_length: int
    fence_offset: int

class NodeAlert:
    alert_type: AlertType
    title: Optional[str]
    multiline: bool
    fence_length: int
    fence_offset: int

class Document(NodeValue[None]):
    pass

class FrontMatter(NodeValue[str]):
    value: str

class BlockQuote(NodeValue[None]):
    pass

class List(NodeValue[NodeList]):
    value: NodeList

class Item(NodeValue[NodeList]):
    value: NodeList

class DescriptionList(NodeValue[None]):
    pass

class DescriptionItem(NodeValue[NodeDescriptionItem]):
    value: NodeDescriptionItem

class DescriptionTerm(NodeValue[None]):
    pass

class DescriptionDetails(NodeValue[None]):
    pass

class CodeBlock(NodeValue[NodeCodeBlock]):
    value: NodeCodeBlock

class HtmlBlock(NodeValue[NodeHtmlBlock]):
    value: NodeHtmlBlock

class Paragraph(NodeValue[None]):
    pass

class Heading(NodeValue[NodeHeading]):
    value: NodeHeading

class ThematicBreak(NodeValue[None]):
    pass

class FootnoteDefinition(NodeValue[NodeFootnoteDefinition]):
    value: NodeFootnoteDefinition

class Table(NodeValue[NodeTable]):
    value: NodeTable

class TableRow(NodeValue[bool]):
    value: bool

class TableCell(NodeValue[None]):
    pass

class Text(NodeValue[str]):
    value: str

class TaskItem(NodeValue[Optional[str]]):
    value: Optional[str]

class SoftBreak(NodeValue[None]):
    pass

class LineBreak(NodeValue[None]):
    pass

class Code(NodeValue[NodeCode]):
    value: NodeCode

class HtmlInline(NodeValue[str]):
    value: str

class Raw(NodeValue[str]):
    value: str

class Emph(NodeValue[None]):
    pass

class Strong(NodeValue[None]):
    pass

class Strikethrough(NodeValue[None]):
    pass

class Superscript(NodeValue[None]):
    pass

class Link(NodeValue[NodeLink]):
    value: NodeLink

class Image(NodeValue[NodeLink]):
    value: NodeLink

class FootnoteReference(NodeValue[NodeFootnoteReference]):
    value: NodeFootnoteReference

class ShortCode(NodeValue[NodeShortCode]):
    value: NodeShortCode

class Math(NodeValue[NodeMath]):
    value: NodeMath

class MultilineBlockQuote(NodeValue[NodeMultilineBlockQuote]):
    value: NodeMultilineBlockQuote

class Escaped(NodeValue[None]):
    pass

class WikiLink(NodeValue[NodeWikiLink]):
    value: NodeWikiLink

class Underline(NodeValue[None]):
    pass

class Subscript(NodeValue[None]):
    pass

class SpoileredText(NodeValue[None]):
    pass

class EscapedTag(NodeValue[str]):
    value: str

class Alert(NodeValue[NodeAlert]):
    value: NodeAlert

class LineColumn:
    line: int
    column: int

class Sourcepos:
    start: LineColumn
    end: LineColumn

class AstNode:
    node_value: NodeValue
    sourcepos: Sourcepos
    children: list[AstNode]
    def __init__(
        self,
        node_value: NodeValue,
        sourcepos: Sourcepos,
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
    def __init__(self) -> None: ...

class ParseOptions:
    smart: bool
    default_info_string: Optional[str]
    relaxed_tasklist_matching: bool
    relaxed_autolinks: bool
    def __init__(self) -> None: ...

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
    ignore_setext: bool
    ignore_empty_links: bool
    gfm_quirks: bool
    prefer_fenced: bool
    figure_with_caption: bool
    tasklist_classes: bool
    ol_width: int
    experimental_minimize_commonmark: bool
    def __init__(self) -> None: ...

def render_markdown(
    text: str,
    extension_options: Optional[ExtensionOptions] = None,
    parse_options: Optional[ParseOptions] = None,
    render_options: Optional[RenderOptions] = None,
) -> str: ...
def render_markdown_to_commonmark(
    text: str,
    extension_options: Optional[ExtensionOptions] = None,
    parse_options: Optional[ParseOptions] = None,
    render_options: Optional[RenderOptions] = None,
) -> str: ...
def parse_markdown(
    text: str,
    extension_options: Optional[ExtensionOptions] = None,
    parse_options: Optional[ParseOptions] = None,
    render_options: Optional[RenderOptions] = None,
) -> AstNode: ...
