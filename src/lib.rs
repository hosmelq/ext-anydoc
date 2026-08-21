#![cfg_attr(windows, feature(abi_vectorcall))]

use std::panic::{AssertUnwindSafe, catch_unwind};

use ext_php_rs::{binary_slice::BinarySlice, prelude::*};

mod exception;
mod model;

fn format_name(format: anydoc::Format) -> &'static str {
    match format {
        anydoc::Format::Csv => "csv",
        anydoc::Format::Doc => "doc",
        anydoc::Format::Docx => "docx",
        anydoc::Format::Epub => "epub",
        anydoc::Format::Excel => "xlsx",
        anydoc::Format::Odp => "odp",
        anydoc::Format::Ods => "ods",
        anydoc::Format::Odt => "odt",
        anydoc::Format::Pdf => "pdf",
        anydoc::Format::Ppt => "ppt",
        anydoc::Format::Pptx => "pptx",
        anydoc::Format::Rtf => "rtf",
    }
}

fn parse_format(format: &str) -> PhpResult<anydoc::Format> {
    anydoc::Format::from_extension(format.trim_start_matches('.'))
        .ok_or_else(|| PhpException::default(format!("unknown anydoc format: {format}")))
}

fn guard<T>(operation: impl FnOnce() -> Result<T, anydoc::ConvertError>) -> PhpResult<T> {
    match catch_unwind(AssertUnwindSafe(operation)) {
        Ok(result) => result.map_err(exception::convert),
        Err(_) => Err(exception::panic()),
    }
}

/// Detect a document format from its binary content.
#[php_function]
pub fn anydoc_format_from_bytes(bytes: BinarySlice<u8>) -> Option<String> {
    anydoc::Format::from_bytes(&bytes).map(|format| format_name(format).to_owned())
}

/// Resolve a document format from a file extension, with or without a leading dot.
#[php_function]
pub fn anydoc_format_from_extension(extension: &str) -> Option<String> {
    anydoc::Format::from_extension(extension.trim_start_matches('.'))
        .map(|format| format_name(format).to_owned())
}

/// Resolve a document format from a path or file name.
#[php_function]
pub fn anydoc_format_from_path(path: &str) -> Option<String> {
    anydoc::Format::from_path(std::path::Path::new(path))
        .map(|format| format_name(format).to_owned())
}

/// Convert a local document file to Markdown.
#[php_function]
pub fn anydoc_to_markdown(path: &str) -> PhpResult<String> {
    guard(|| anydoc::to_markdown(path))
}

/// Convert in-memory document bytes to Markdown.
#[php_function]
#[php(defaults(format = None))]
pub fn anydoc_to_markdown_bytes(
    bytes: BinarySlice<u8>,
    format: Option<String>,
) -> PhpResult<String> {
    let format = format.as_deref().map(parse_format).transpose()?;

    guard(|| anydoc::to_markdown_bytes(&bytes, format))
}

/// Parse in-memory document bytes into anydoc's information-preserving model.
#[php_function]
#[php(defaults(format = None))]
pub fn anydoc_to_document(
    bytes: BinarySlice<u8>,
    format: Option<String>,
) -> PhpResult<model::Document> {
    let format = format.as_deref().map(parse_format).transpose()?;
    let document = guard(|| anydoc::to_document(&bytes, format))?;

    model::convert(document)
}

/// Register the anydoc PHP extension.
#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .name("anydoc")
        .version(env!("CARGO_PKG_VERSION"))
        .class::<exception::ConvertException>()
        .class::<exception::UnsupportedException>()
        .class::<exception::MalformedException>()
        .class::<exception::EncryptedException>()
        .class::<exception::ResourceLimitException>()
        .class::<exception::MissingPartException>()
        .class::<exception::IoException>()
        .class::<exception::PanicException>()
        .class::<model::Document>()
        .class::<model::Block>()
        .class::<model::Heading>()
        .class::<model::Paragraph>()
        .class::<model::BlockList>()
        .class::<model::BlockTable>()
        .class::<model::BlockQuote>()
        .class::<model::CodeBlock>()
        .class::<model::Rule>()
        .class::<model::MathBlock>()
        .class::<model::Inline>()
        .class::<model::TextInline>()
        .class::<model::LinkInline>()
        .class::<model::ImageInline>()
        .class::<model::AnchorInline>()
        .class::<model::NoteReference>()
        .class::<model::LineBreak>()
        .class::<model::MathInline>()
        .class::<model::Checkbox>()
        .class::<model::Style>()
        .class::<model::LinkTarget>()
        .class::<model::ExternalLink>()
        .class::<model::RelativeLink>()
        .class::<model::AnchorLink>()
        .class::<model::ImageSource>()
        .class::<model::ExternalImage>()
        .class::<model::AssetImage>()
        .class::<model::UnavailableImage>()
        .class::<model::DocumentList>()
        .class::<model::ListItem>()
        .class::<model::Table>()
        .class::<model::CellSlot>()
        .class::<model::OriginCell>()
        .class::<model::CoveredCell>()
        .class::<model::Cell>()
        .class::<model::Note>()
        .class::<model::Asset>()
        .function(wrap_function!(anydoc_format_from_bytes))
        .function(wrap_function!(anydoc_format_from_extension))
        .function(wrap_function!(anydoc_format_from_path))
        .function(wrap_function!(anydoc_to_markdown))
        .function(wrap_function!(anydoc_to_markdown_bytes))
        .function(wrap_function!(anydoc_to_document))
}

#[cfg(test)]
mod tests {
    use super::format_name;

    #[test]
    fn maps_anydoc_formats_to_the_public_php_names() {
        let cases = [
            (anydoc::Format::Csv, "csv"),
            (anydoc::Format::Doc, "doc"),
            (anydoc::Format::Docx, "docx"),
            (anydoc::Format::Epub, "epub"),
            (anydoc::Format::Excel, "xlsx"),
            (anydoc::Format::Odp, "odp"),
            (anydoc::Format::Ods, "ods"),
            (anydoc::Format::Odt, "odt"),
            (anydoc::Format::Pdf, "pdf"),
            (anydoc::Format::Ppt, "ppt"),
            (anydoc::Format::Pptx, "pptx"),
            (anydoc::Format::Rtf, "rtf"),
        ];

        for (format, expected) in cases {
            assert_eq!(format_name(format), expected);
        }
    }
}
