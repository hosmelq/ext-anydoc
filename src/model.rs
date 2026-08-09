use ext_php_rs::{
    binary::Binary,
    convert::IntoZval,
    exception::{PhpException, PhpResult},
    flags::ClassFlags,
    prelude::*,
    types::Zval,
};

use anydoc::model as core;

#[php_class]
#[php(name = "Anydoc\\Document")]
#[php(readonly)]
#[php(flags = ClassFlags::Final)]
#[derive(Clone)]
pub struct Document {
    blocks: Vec<core::Block>,
    notes: Vec<core::Note>,
    assets: Vec<core::Asset>,
}

#[php_impl]
impl Document {
    #[php(getter)]
    pub fn get_blocks(&self) -> PhpResult<Vec<Zval>> {
        blocks(self.blocks.clone())
    }

    #[php(getter)]
    pub fn get_notes(&self) -> PhpResult<Vec<Note>> {
        self.notes.clone().into_iter().map(note).collect()
    }

    #[php(getter)]
    pub fn get_assets(&self) -> PhpResult<Vec<Asset>> {
        self.assets.clone().into_iter().map(asset).collect()
    }
}

#[php_class]
#[php(name = "Anydoc\\Block")]
#[php(readonly)]
#[php(flags = ClassFlags::Abstract)]
pub struct Block;

#[php_class]
#[php(name = "Anydoc\\Heading")]
#[php(extends(Block))]
#[php(readonly)]
#[php(flags = ClassFlags::Final)]
#[derive(Clone)]
pub struct Heading {
    #[php(prop)]
    pub level: i64,
    #[php(prop)]
    pub anchor: Option<String>,
    content: Vec<core::Inline>,
}

#[php_impl]
impl Heading {
    #[php(getter)]
    pub fn get_content(&self) -> PhpResult<Vec<Zval>> {
        inlines(self.content.clone())
    }
}

#[php_class]
#[php(name = "Anydoc\\Paragraph")]
#[php(extends(Block))]
#[php(readonly)]
#[php(flags = ClassFlags::Final)]
#[derive(Clone)]
pub struct Paragraph {
    content: Vec<core::Inline>,
}

#[php_impl]
impl Paragraph {
    #[php(getter)]
    pub fn get_content(&self) -> PhpResult<Vec<Zval>> {
        inlines(self.content.clone())
    }
}

#[php_class]
#[php(name = "Anydoc\\BlockList")]
#[php(extends(Block))]
#[php(readonly)]
#[php(flags = ClassFlags::Final)]
#[derive(Clone)]
pub struct BlockList {
    list: core::List,
}

#[php_impl]
impl BlockList {
    #[php(getter)]
    pub fn get_list(&self) -> PhpResult<DocumentList> {
        document_list(self.list.clone())
    }
}

#[php_class]
#[php(name = "Anydoc\\BlockTable")]
#[php(extends(Block))]
#[php(readonly)]
#[php(flags = ClassFlags::Final)]
#[derive(Clone)]
pub struct BlockTable {
    table: core::Table,
}

#[php_impl]
impl BlockTable {
    #[php(getter)]
    pub fn get_table(&self) -> PhpResult<Table> {
        table(self.table.clone())
    }
}

#[php_class]
#[php(name = "Anydoc\\BlockQuote")]
#[php(extends(Block))]
#[php(readonly)]
#[php(flags = ClassFlags::Final)]
#[derive(Clone)]
pub struct BlockQuote {
    blocks: Vec<core::Block>,
}

#[php_impl]
impl BlockQuote {
    #[php(getter)]
    pub fn get_blocks(&self) -> PhpResult<Vec<Zval>> {
        blocks(self.blocks.clone())
    }
}

#[php_class]
#[php(name = "Anydoc\\CodeBlock")]
#[php(extends(Block))]
#[php(readonly)]
#[php(flags = ClassFlags::Final)]
#[derive(Clone)]
pub struct CodeBlock {
    #[php(prop)]
    pub lang: Option<String>,
    #[php(prop)]
    pub text: String,
}

#[php_class]
#[php(name = "Anydoc\\Rule")]
#[php(extends(Block))]
#[php(readonly)]
#[php(flags = ClassFlags::Final)]
#[derive(Clone)]
pub struct Rule;

#[php_class]
#[php(name = "Anydoc\\Inline")]
#[php(readonly)]
#[php(flags = ClassFlags::Abstract)]
pub struct Inline;

#[php_class]
#[php(name = "Anydoc\\Text")]
#[php(extends(Inline))]
#[php(readonly)]
#[php(flags = ClassFlags::Final)]
#[derive(Clone)]
pub struct TextInline {
    #[php(prop)]
    pub text: String,
    style: core::Style,
}

#[php_impl]
impl TextInline {
    #[php(getter)]
    pub fn get_style(&self) -> Style {
        self.style.into()
    }
}

#[php_class]
#[php(name = "Anydoc\\Link")]
#[php(extends(Inline))]
#[php(readonly)]
#[php(flags = ClassFlags::Final)]
#[derive(Clone)]
pub struct LinkInline {
    content: Vec<core::Inline>,
    target: core::LinkTarget,
}

#[php_impl]
impl LinkInline {
    #[php(getter)]
    pub fn get_content(&self) -> PhpResult<Vec<Zval>> {
        inlines(self.content.clone())
    }

    #[php(getter)]
    pub fn get_target(&self) -> PhpResult<Zval> {
        link_target(self.target.clone())
    }
}

#[php_class]
#[php(name = "Anydoc\\Image")]
#[php(extends(Inline))]
#[php(readonly)]
#[php(flags = ClassFlags::Final)]
#[derive(Clone)]
pub struct ImageInline {
    #[php(prop)]
    pub alt: String,
    source: core::ImageSource,
}

#[php_impl]
impl ImageInline {
    #[php(getter)]
    pub fn get_source(&self) -> PhpResult<Zval> {
        image_source(self.source.clone())
    }
}

#[php_class]
#[php(name = "Anydoc\\Anchor")]
#[php(extends(Inline))]
#[php(readonly)]
#[php(flags = ClassFlags::Final)]
#[derive(Clone)]
pub struct AnchorInline {
    #[php(prop)]
    pub anchor: String,
}

#[php_class]
#[php(name = "Anydoc\\NoteReference")]
#[php(extends(Inline))]
#[php(readonly)]
#[php(flags = ClassFlags::Final)]
#[derive(Clone)]
pub struct NoteReference {
    #[php(prop, name = "noteId")]
    pub note_id: String,
}

#[php_class]
#[php(name = "Anydoc\\LineBreak")]
#[php(extends(Inline))]
#[php(readonly)]
#[php(flags = ClassFlags::Final)]
#[derive(Clone)]
pub struct LineBreak;

#[php_class]
#[php(name = "Anydoc\\Style")]
#[php(readonly)]
#[php(flags = ClassFlags::Final)]
#[derive(Clone)]
pub struct Style {
    #[php(prop)]
    pub bold: bool,
    #[php(prop)]
    pub italic: bool,
    #[php(prop)]
    pub strike: bool,
    #[php(prop)]
    pub code: bool,
}

impl From<core::Style> for Style {
    fn from(style: core::Style) -> Self {
        Self {
            bold: style.bold,
            italic: style.italic,
            strike: style.strike,
            code: style.code,
        }
    }
}

#[php_class]
#[php(name = "Anydoc\\LinkTarget")]
#[php(readonly)]
#[php(flags = ClassFlags::Abstract)]
pub struct LinkTarget;

#[php_class]
#[php(name = "Anydoc\\ExternalLink")]
#[php(extends(LinkTarget))]
#[php(readonly)]
#[php(flags = ClassFlags::Final)]
#[derive(Clone)]
pub struct ExternalLink {
    #[php(prop)]
    pub value: String,
}

#[php_class]
#[php(name = "Anydoc\\RelativeLink")]
#[php(extends(LinkTarget))]
#[php(readonly)]
#[php(flags = ClassFlags::Final)]
#[derive(Clone)]
pub struct RelativeLink {
    #[php(prop)]
    pub value: String,
}

#[php_class]
#[php(name = "Anydoc\\AnchorLink")]
#[php(extends(LinkTarget))]
#[php(readonly)]
#[php(flags = ClassFlags::Final)]
#[derive(Clone)]
pub struct AnchorLink {
    #[php(prop)]
    pub value: String,
}

#[php_class]
#[php(name = "Anydoc\\ImageSource")]
#[php(readonly)]
#[php(flags = ClassFlags::Abstract)]
pub struct ImageSource;

#[php_class]
#[php(name = "Anydoc\\ExternalImage")]
#[php(extends(ImageSource))]
#[php(readonly)]
#[php(flags = ClassFlags::Final)]
#[derive(Clone)]
pub struct ExternalImage {
    #[php(prop)]
    pub url: String,
}

#[php_class]
#[php(name = "Anydoc\\AssetImage")]
#[php(extends(ImageSource))]
#[php(readonly)]
#[php(flags = ClassFlags::Final)]
#[derive(Clone)]
pub struct AssetImage {
    #[php(prop, name = "assetId")]
    pub asset_id: i64,
}

#[php_class]
#[php(name = "Anydoc\\UnavailableImage")]
#[php(extends(ImageSource))]
#[php(readonly)]
#[php(flags = ClassFlags::Final)]
#[derive(Clone)]
pub struct UnavailableImage;

#[php_class]
#[php(name = "Anydoc\\DocumentList")]
#[php(readonly)]
#[php(flags = ClassFlags::Final)]
#[derive(Clone)]
pub struct DocumentList {
    #[php(prop)]
    pub marker: String,
    #[php(prop)]
    pub start: i64,
    items: Vec<core::ListItem>,
}

#[php_impl]
impl DocumentList {
    #[php(getter)]
    pub fn get_items(&self) -> PhpResult<Vec<ListItem>> {
        self.items.clone().into_iter().map(list_item).collect()
    }
}

#[php_class]
#[php(name = "Anydoc\\ListItem")]
#[php(readonly)]
#[php(flags = ClassFlags::Final)]
#[derive(Clone)]
pub struct ListItem {
    blocks: Vec<core::Block>,
    #[php(prop)]
    pub checked: Option<bool>,
    #[php(prop, name = "markerLabel")]
    pub marker_label: Option<String>,
}

#[php_impl]
impl ListItem {
    #[php(getter)]
    pub fn get_blocks(&self) -> PhpResult<Vec<Zval>> {
        blocks(self.blocks.clone())
    }
}

#[php_class]
#[php(name = "Anydoc\\Table")]
#[php(readonly)]
#[php(flags = ClassFlags::Final)]
#[derive(Clone)]
pub struct Table {
    grid: Vec<Vec<core::CellSlot>>,
    #[php(prop, name = "headerRows")]
    pub header_rows: i64,
    #[php(prop)]
    pub kind: String,
}

#[php_impl]
impl Table {
    #[php(getter)]
    pub fn get_grid(&self) -> PhpResult<Vec<Vec<Zval>>> {
        self.grid
            .clone()
            .into_iter()
            .map(|row| row.into_iter().map(cell_slot).collect())
            .collect()
    }
}

#[php_class]
#[php(name = "Anydoc\\CellSlot")]
#[php(readonly)]
#[php(flags = ClassFlags::Abstract)]
pub struct CellSlot;

#[php_class]
#[php(name = "Anydoc\\OriginCell")]
#[php(extends(CellSlot))]
#[php(readonly)]
#[php(flags = ClassFlags::Final)]
#[derive(Clone)]
pub struct OriginCell {
    cell: core::Cell,
}

#[php_impl]
impl OriginCell {
    #[php(getter)]
    pub fn get_cell(&self) -> PhpResult<Cell> {
        cell(self.cell.clone())
    }
}

#[php_class]
#[php(name = "Anydoc\\CoveredCell")]
#[php(extends(CellSlot))]
#[php(readonly)]
#[php(flags = ClassFlags::Final)]
#[derive(Clone)]
pub struct CoveredCell {
    #[php(prop, name = "originRow")]
    pub origin_row: i64,
    #[php(prop, name = "originCol")]
    pub origin_col: i64,
}

#[php_class]
#[php(name = "Anydoc\\Cell")]
#[php(readonly)]
#[php(flags = ClassFlags::Final)]
#[derive(Clone)]
pub struct Cell {
    blocks: Vec<core::Block>,
    #[php(prop, name = "colSpan")]
    pub col_span: i64,
    #[php(prop, name = "rowSpan")]
    pub row_span: i64,
}

#[php_impl]
impl Cell {
    #[php(getter)]
    pub fn get_blocks(&self) -> PhpResult<Vec<Zval>> {
        blocks(self.blocks.clone())
    }
}

#[php_class]
#[php(name = "Anydoc\\Note")]
#[php(readonly)]
#[php(flags = ClassFlags::Final)]
#[derive(Clone)]
pub struct Note {
    #[php(prop)]
    pub id: String,
    #[php(prop)]
    pub kind: String,
    blocks: Vec<core::Block>,
}

#[php_impl]
impl Note {
    #[php(getter)]
    pub fn get_blocks(&self) -> PhpResult<Vec<Zval>> {
        blocks(self.blocks.clone())
    }
}

#[php_class]
#[php(name = "Anydoc\\Asset")]
#[php(readonly)]
#[php(flags = ClassFlags::Final)]
#[derive(Clone)]
pub struct Asset {
    #[php(prop)]
    pub id: i64,
    #[php(prop, name = "mediaType")]
    pub media_type: String,
    #[php(prop, name = "originPart")]
    pub origin_part: String,
    data: Vec<u8>,
}

#[php_impl]
impl Asset {
    #[php(getter)]
    pub fn get_data(&self) -> Binary<u8> {
        Binary::new(self.data.clone())
    }
}

pub fn convert(document: core::Document) -> PhpResult<Document> {
    Ok(Document {
        blocks: document.blocks,
        notes: document.notes,
        assets: document.assets,
    })
}

fn blocks(values: Vec<core::Block>) -> PhpResult<Vec<Zval>> {
    values.into_iter().map(block).collect()
}

fn block(value: core::Block) -> PhpResult<Zval> {
    match value {
        core::Block::Heading {
            level,
            anchor,
            content,
        } => object(Heading {
            level: i64::from(level),
            anchor,
            content,
        }),
        core::Block::Paragraph(content) => object(Paragraph { content }),
        core::Block::List(list) => object(BlockList { list }),
        core::Block::Table(table) => object(BlockTable { table }),
        core::Block::BlockQuote(blocks) => object(BlockQuote { blocks }),
        core::Block::CodeBlock { lang, text } => object(CodeBlock { lang, text }),
        core::Block::Rule => object(Rule),
    }
}

fn inlines(values: Vec<core::Inline>) -> PhpResult<Vec<Zval>> {
    values.into_iter().map(inline).collect()
}

fn inline(value: core::Inline) -> PhpResult<Zval> {
    match value {
        core::Inline::Text { text, style } => object(TextInline { text, style }),
        core::Inline::Link { content, target } => object(LinkInline { content, target }),
        core::Inline::Image { alt, source } => object(ImageInline { alt, source }),
        core::Inline::Anchor(anchor) => object(AnchorInline { anchor }),
        core::Inline::NoteRef(note_id) => object(NoteReference { note_id }),
        core::Inline::LineBreak => object(LineBreak),
    }
}

fn link_target(value: core::LinkTarget) -> PhpResult<Zval> {
    match value {
        core::LinkTarget::External(value) => object(ExternalLink { value }),
        core::LinkTarget::Relative(value) => object(RelativeLink { value }),
        core::LinkTarget::Anchor(value) => object(AnchorLink { value }),
    }
}

fn image_source(value: core::ImageSource) -> PhpResult<Zval> {
    match value {
        core::ImageSource::External(url) => object(ExternalImage { url }),
        core::ImageSource::Asset(id) => object(AssetImage {
            asset_id: usize_to_php(id.0, "asset id")?,
        }),
        core::ImageSource::Unavailable => object(UnavailableImage),
    }
}

fn document_list(value: core::List) -> PhpResult<DocumentList> {
    Ok(DocumentList {
        marker: match value.marker {
            core::MarkerKind::Bullet => "bullet",
            core::MarkerKind::Decimal => "decimal",
            core::MarkerKind::LowerAlpha => "lowerAlpha",
            core::MarkerKind::UpperAlpha => "upperAlpha",
            core::MarkerKind::LowerRoman => "lowerRoman",
            core::MarkerKind::UpperRoman => "upperRoman",
        }
        .into(),
        start: i64::try_from(value.start).map_err(|_| integer_overflow("list start"))?,
        items: value.items,
    })
}

fn list_item(value: core::ListItem) -> PhpResult<ListItem> {
    Ok(ListItem {
        blocks: value.blocks,
        checked: value.checked,
        marker_label: value.marker_label,
    })
}

fn table(value: core::Table) -> PhpResult<Table> {
    Ok(Table {
        grid: value.grid,
        header_rows: usize_to_php(value.header_rows, "table header row count")?,
        kind: match value.kind {
            core::TableKind::Data => "data",
            core::TableKind::Layout => "layout",
        }
        .into(),
    })
}

fn cell_slot(value: core::CellSlot) -> PhpResult<Zval> {
    match value {
        core::CellSlot::Origin(cell) => object(OriginCell { cell }),
        core::CellSlot::Covered {
            origin_row,
            origin_col,
        } => object(CoveredCell {
            origin_row: usize_to_php(origin_row, "covered cell origin row")?,
            origin_col: usize_to_php(origin_col, "covered cell origin column")?,
        }),
    }
}

fn cell(value: core::Cell) -> PhpResult<Cell> {
    Ok(Cell {
        blocks: value.blocks,
        col_span: i64::from(value.col_span),
        row_span: i64::from(value.row_span),
    })
}

fn note(value: core::Note) -> PhpResult<Note> {
    Ok(Note {
        id: value.id,
        kind: match value.kind {
            core::NoteKind::Footnote => "footnote",
            core::NoteKind::Endnote => "endnote",
        }
        .into(),
        blocks: value.blocks,
    })
}

fn asset(value: core::Asset) -> PhpResult<Asset> {
    Ok(Asset {
        id: usize_to_php(value.id.0, "asset id")?,
        media_type: value.media_type,
        origin_part: value.origin_part,
        data: value.bytes,
    })
}

fn object(value: impl IntoZval) -> PhpResult<Zval> {
    value.into_zval(false).map_err(|error| {
        PhpException::default(format!("failed to build Anydoc PHP model: {error}"))
    })
}

fn usize_to_php(value: usize, field: &str) -> PhpResult<i64> {
    i64::try_from(value).map_err(|_| integer_overflow(field))
}

fn integer_overflow(field: &str) -> PhpException {
    PhpException::default(format!("Anydoc {field} exceeds the PHP integer range"))
}
