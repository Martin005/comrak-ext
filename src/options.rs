use pyo3::prelude::*;

// Import the Comrak (Rust) types under `comrak_lib::`
use comrak_lib::options::{
    Extension as ComrakExtensionOptions, Parse as ComrakParseOptions, Render as ComrakRenderOptions,
};

/// Python class that mirrors Comrak’s `ExtensionOptions`
#[pyclass(name = "ExtensionOptions", get_all, set_all, eq)]
#[derive(Clone, PartialEq, Eq)]
pub struct PyExtensionOptions {
    pub strikethrough: bool,
    pub tagfilter: bool,
    pub table: bool,
    pub autolink: bool,
    pub tasklist: bool,
    pub superscript: bool,
    pub header_ids: Option<String>,
    pub footnotes: bool,
    pub inline_footnotes: bool,
    pub description_lists: bool,
    pub front_matter_delimiter: Option<String>,
    pub multiline_block_quotes: bool,
    pub alerts: bool,
    pub math_dollars: bool,
    pub math_code: bool,
    pub shortcodes: bool, // if your comrak_lib has the "shortcodes" feature
    pub wikilinks_title_after_pipe: bool,
    pub wikilinks_title_before_pipe: bool,
    pub underline: bool,
    pub subscript: bool,
    pub spoiler: bool,
    pub greentext: bool,
    pub cjk_friendly_emphasis: bool,
    pub subtext: bool,
    pub highlight: bool,
    pub phoenix_heex: bool,
}

impl PyExtensionOptions {
    /// **Rust-only** helper to copy from `PyExtensionOptions` into a real `ComrakExtensionOptions`.
    pub fn update_extension_options(&self, opts: &mut ComrakExtensionOptions<'_>) {
        opts.strikethrough = self.strikethrough;
        opts.tagfilter = self.tagfilter;
        opts.table = self.table;
        opts.autolink = self.autolink;
        opts.tasklist = self.tasklist;
        opts.superscript = self.superscript;
        opts.header_ids = self.header_ids.clone();
        opts.footnotes = self.footnotes;
        opts.inline_footnotes = self.inline_footnotes;
        opts.description_lists = self.description_lists;
        opts.front_matter_delimiter = self.front_matter_delimiter.clone();
        opts.multiline_block_quotes = self.multiline_block_quotes;
        opts.alerts = self.alerts;
        opts.math_dollars = self.math_dollars;
        opts.math_code = self.math_code;
        opts.shortcodes = self.shortcodes;
        opts.wikilinks_title_after_pipe = self.wikilinks_title_after_pipe;
        opts.wikilinks_title_before_pipe = self.wikilinks_title_before_pipe;
        opts.underline = self.underline;
        opts.subscript = self.subscript;
        opts.spoiler = self.spoiler;
        opts.greentext = self.greentext;
        opts.cjk_friendly_emphasis = self.cjk_friendly_emphasis;
        opts.subtext = self.subtext;
        opts.highlight = self.highlight;
        opts.phoenix_heex = self.phoenix_heex;
    }
}

#[pymethods]
impl PyExtensionOptions {
    #[new]
    #[pyo3(signature = (
        strikethrough=None,
        tagfilter=None,
        table=None,
        autolink=None,
        tasklist=None,
        superscript=None,
        header_ids=None,
        footnotes=None,
        inline_footnotes=None,
        description_lists=None,
        front_matter_delimiter=None,
        multiline_block_quotes=None,
        alerts=None,
        math_dollars=None,
        math_code=None,
        shortcodes=None, // if your comrak_lib has the "shortcodes" feature
        wikilinks_title_after_pipe=None,
        wikilinks_title_before_pipe=None,
        underline=None,
        subscript=None,
        spoiler=None,
        greentext=None,
        cjk_friendly_emphasis=None,
        subtext=None,
        highlight=None,
        phoenix_heex=None,
    ))]
    pub fn new(
        strikethrough: Option<bool>,
        tagfilter: Option<bool>,
        table: Option<bool>,
        autolink: Option<bool>,
        tasklist: Option<bool>,
        superscript: Option<bool>,
        header_ids: Option<String>,
        footnotes: Option<bool>,
        inline_footnotes: Option<bool>,
        description_lists: Option<bool>,
        front_matter_delimiter: Option<String>,
        multiline_block_quotes: Option<bool>,
        alerts: Option<bool>,
        math_dollars: Option<bool>,
        math_code: Option<bool>,
        shortcodes: Option<bool>, // if your comrak_lib has the "shortcodes" feature
        wikilinks_title_after_pipe: Option<bool>,
        wikilinks_title_before_pipe: Option<bool>,
        underline: Option<bool>,
        subscript: Option<bool>,
        spoiler: Option<bool>,
        greentext: Option<bool>,
        cjk_friendly_emphasis: Option<bool>,
        subtext: Option<bool>,
        highlight: Option<bool>,
        phoenix_heex: Option<bool>,
    ) -> Self {
        let defaults = ComrakExtensionOptions::default();
        Self {
            strikethrough: strikethrough.unwrap_or(defaults.strikethrough),
            tagfilter: tagfilter.unwrap_or(defaults.tagfilter),
            table: table.unwrap_or(defaults.table),
            autolink: autolink.unwrap_or(defaults.autolink),
            tasklist: tasklist.unwrap_or(defaults.tasklist),
            superscript: superscript.unwrap_or(defaults.superscript),
            header_ids: header_ids.or(defaults.header_ids.clone()),
            footnotes: footnotes.unwrap_or(defaults.footnotes),
            inline_footnotes: inline_footnotes.unwrap_or(defaults.inline_footnotes),
            description_lists: description_lists.unwrap_or(defaults.description_lists),
            front_matter_delimiter: front_matter_delimiter
                .or(defaults.front_matter_delimiter.clone()),
            multiline_block_quotes: multiline_block_quotes
                .unwrap_or(defaults.multiline_block_quotes),
            alerts: alerts.unwrap_or(defaults.alerts),
            math_dollars: math_dollars.unwrap_or(defaults.math_dollars),
            math_code: math_code.unwrap_or(defaults.math_code),
            shortcodes: shortcodes.unwrap_or(defaults.shortcodes),
            wikilinks_title_after_pipe: wikilinks_title_after_pipe
                .unwrap_or(defaults.wikilinks_title_after_pipe),
            wikilinks_title_before_pipe: wikilinks_title_before_pipe
                .unwrap_or(defaults.wikilinks_title_before_pipe),
            underline: underline.unwrap_or(defaults.underline),
            subscript: subscript.unwrap_or(defaults.subscript),
            spoiler: spoiler.unwrap_or(defaults.spoiler),
            greentext: greentext.unwrap_or(defaults.greentext),
            cjk_friendly_emphasis: cjk_friendly_emphasis.unwrap_or(defaults.cjk_friendly_emphasis),
            subtext: subtext.unwrap_or(defaults.subtext),
            highlight: highlight.unwrap_or(defaults.highlight),
            phoenix_heex: phoenix_heex.unwrap_or(defaults.phoenix_heex),
        }
    }
}

/// Python class that mirrors Comrak’s `ParseOptions`
#[pyclass(name = "ParseOptions", get_all, set_all, eq)]
#[derive(Clone, PartialEq, Eq)]
pub struct PyParseOptions {
    pub smart: bool,
    pub default_info_string: Option<String>,
    pub relaxed_tasklist_matching: bool,
    pub tasklist_in_table: bool,
    pub relaxed_autolinks: bool,
    pub ignore_setext: bool,
    pub leave_footnote_definitions: bool,
    pub escaped_char_spans: bool,
}

impl PyParseOptions {
    /// Rust-only helper
    pub fn update_parse_options(&self, opts: &mut ComrakParseOptions<'_>) {
        opts.smart = self.smart;
        opts.default_info_string = self.default_info_string.clone();
        opts.relaxed_tasklist_matching = self.relaxed_tasklist_matching;
        opts.tasklist_in_table = self.tasklist_in_table;
        opts.relaxed_autolinks = self.relaxed_autolinks;
        opts.ignore_setext = self.ignore_setext;
        opts.leave_footnote_definitions = self.leave_footnote_definitions;
        opts.escaped_char_spans = self.escaped_char_spans;
    }
}

#[pymethods]
impl PyParseOptions {
    #[new]
    #[pyo3(signature = (
        smart=None,
        default_info_string=None,
        relaxed_tasklist_matching=None,
        tasklist_in_table=None,
        relaxed_autolinks=None,
        ignore_setext=None,
        leave_footnote_definitions=None,
        escaped_char_spans=None
    ))]
    pub fn new(
        smart: Option<bool>,
        default_info_string: Option<String>,
        relaxed_tasklist_matching: Option<bool>,
        tasklist_in_table: Option<bool>,
        relaxed_autolinks: Option<bool>,
        ignore_setext: Option<bool>,
        leave_footnote_definitions: Option<bool>,
        escaped_char_spans: Option<bool>,
    ) -> Self {
        let defaults = ComrakParseOptions::default();
        Self {
            smart: smart.unwrap_or(defaults.smart),
            default_info_string: default_info_string.or(defaults.default_info_string.clone()),
            relaxed_tasklist_matching: relaxed_tasklist_matching
                .unwrap_or(defaults.relaxed_tasklist_matching),
            tasklist_in_table: tasklist_in_table.unwrap_or(defaults.tasklist_in_table),
            relaxed_autolinks: relaxed_autolinks.unwrap_or(defaults.relaxed_autolinks),
            ignore_setext: ignore_setext.unwrap_or(defaults.ignore_setext),
            leave_footnote_definitions: leave_footnote_definitions
                .unwrap_or(defaults.leave_footnote_definitions),
            escaped_char_spans: escaped_char_spans.unwrap_or(defaults.escaped_char_spans),
        }
    }
}

#[pyclass(name = "ListStyleType", eq, eq_int)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PyListStyleType {
    Dash = 45,
    Plus = 43,
    Star = 42,
}

/// Python class that mirrors Comrak’s `RenderOptions`
#[pyclass(name = "RenderOptions", get_all, set_all, eq)]
#[derive(Clone, PartialEq, Eq)]
pub struct PyRenderOptions {
    pub hardbreaks: bool,
    pub github_pre_lang: bool,
    pub full_info_string: bool,
    pub width: usize,
    pub r#unsafe: bool, // named 'unsafe_' because 'unsafe' is reserved
    pub escape: bool,
    pub list_style: PyListStyleType,
    pub sourcepos: bool,
    pub escaped_char_spans: bool,
    pub ignore_empty_links: bool,
    pub gfm_quirks: bool,
    pub prefer_fenced: bool,
    pub figure_with_caption: bool,
    pub tasklist_classes: bool,
    pub ol_width: usize,
    pub experimental_minimize_commonmark: bool,
}

impl PyRenderOptions {
    /// Rust-only helper
    pub fn update_render_options(&self, opts: &mut ComrakRenderOptions) {
        opts.hardbreaks = self.hardbreaks;
        opts.github_pre_lang = self.github_pre_lang;
        opts.full_info_string = self.full_info_string;
        opts.width = self.width;
        opts.r#unsafe = self.r#unsafe;
        opts.escape = self.escape;
        // convert integer to ListStyleType
        opts.list_style = match self.list_style {
            PyListStyleType::Dash => comrak_lib::options::ListStyleType::Dash,
            PyListStyleType::Plus => comrak_lib::options::ListStyleType::Plus,
            PyListStyleType::Star => comrak_lib::options::ListStyleType::Star,
        };
        opts.sourcepos = self.sourcepos;
        opts.escaped_char_spans = self.escaped_char_spans;
        opts.ignore_empty_links = self.ignore_empty_links;
        opts.gfm_quirks = self.gfm_quirks;
        opts.prefer_fenced = self.prefer_fenced;
        opts.figure_with_caption = self.figure_with_caption;
        opts.tasklist_classes = self.tasklist_classes;
        opts.ol_width = self.ol_width;
        opts.experimental_minimize_commonmark = self.experimental_minimize_commonmark;
    }
}

#[pymethods]
impl PyRenderOptions {
    #[new]
    #[pyo3(signature = (
        hardbreaks=None,
        github_pre_lang=None,
        full_info_string=None,
        width=None,
        unsafe_=None,
        escape=None,
        list_style=None,
        sourcepos=None,
        escaped_char_spans=None,
        ignore_empty_links=None,
        gfm_quirks=None,
        prefer_fenced=None,
        figure_with_caption=None,
        tasklist_classes=None,
        ol_width=None,
        experimental_minimize_commonmark=None,
    ))]
    pub fn new(
        hardbreaks: Option<bool>,
        github_pre_lang: Option<bool>,
        full_info_string: Option<bool>,
        width: Option<usize>,
        unsafe_: Option<bool>,
        escape: Option<bool>,
        list_style: Option<PyListStyleType>,
        sourcepos: Option<bool>,
        escaped_char_spans: Option<bool>,
        ignore_empty_links: Option<bool>,
        gfm_quirks: Option<bool>,
        prefer_fenced: Option<bool>,
        figure_with_caption: Option<bool>,
        tasklist_classes: Option<bool>,
        ol_width: Option<usize>,
        experimental_minimize_commonmark: Option<bool>,
    ) -> Self {
        let defaults = ComrakRenderOptions::default();
        Self {
            hardbreaks: hardbreaks.unwrap_or(defaults.hardbreaks),
            github_pre_lang: github_pre_lang.unwrap_or(defaults.github_pre_lang),
            full_info_string: full_info_string.unwrap_or(defaults.full_info_string),
            width: width.unwrap_or(defaults.width),
            r#unsafe: unsafe_.unwrap_or(defaults.r#unsafe),
            escape: escape.unwrap_or(defaults.escape),
            list_style: list_style.unwrap_or(match defaults.list_style {
                comrak_lib::options::ListStyleType::Dash => PyListStyleType::Dash,
                comrak_lib::options::ListStyleType::Plus => PyListStyleType::Plus,
                comrak_lib::options::ListStyleType::Star => PyListStyleType::Star,
            }),
            sourcepos: sourcepos.unwrap_or(defaults.sourcepos),
            escaped_char_spans: escaped_char_spans.unwrap_or(defaults.escaped_char_spans),
            ignore_empty_links: ignore_empty_links.unwrap_or(defaults.ignore_empty_links),
            gfm_quirks: gfm_quirks.unwrap_or(defaults.gfm_quirks),
            prefer_fenced: prefer_fenced.unwrap_or(defaults.prefer_fenced),
            figure_with_caption: figure_with_caption.unwrap_or(defaults.figure_with_caption),
            tasklist_classes: tasklist_classes.unwrap_or(defaults.tasklist_classes),
            ol_width: ol_width.unwrap_or(defaults.ol_width),
            experimental_minimize_commonmark: experimental_minimize_commonmark
                .unwrap_or(defaults.experimental_minimize_commonmark),
        }
    }
}
