# ext-anydoc

Native PHP bindings for [Firecrawl anydoc](https://github.com/firecrawl/anydoc)
to convert Word, PowerPoint, Excel, OpenDocument, RTF, EPUB, CSV, and PDF files
to GitHub-Flavored Markdown.

Starting with v0.2.0, extension releases match the anydoc version they embed.

This community project is not affiliated with, endorsed by, or maintained by
Firecrawl.

## Requirements

- PHP 8.4+.
- PIE 1.4+.

## Installation

Install the extension with [PIE](https://php.github.io/pie/):

```bash
pie install hosmelq/ext-anydoc
```

Prebuilt binaries are available for PHP 8.4 and 8.5 on Linux glibc x64 and
ARM64, macOS Intel and Apple Silicon, and Windows x64. Linux and Windows include
NTS and ZTS builds; macOS includes NTS builds. Other Unix targets build from
source and require matching PHP development headers and Rust 1.88+.

## Usage

```php
<?php

$markdown = anydoc_to_markdown('report.docx');

$bytes = file_get_contents('report.docx');
$fromBytes = anydoc_to_markdown_bytes($bytes);

$csv = file_get_contents('data.csv');
$fromCsv = anydoc_to_markdown_bytes($csv, 'csv');

$document = anydoc_to_document($bytes);
```

Conversions are synchronous. CSV bytes require an explicit `csv` format.
The document model is unavailable for PDF; use `anydoc_to_markdown_bytes()`.

## Supported formats

| Format | Extensions |
| --- | --- |
| CSV | `.csv` |
| EPUB | `.epub` |
| Excel | `.xls`, `.xlsb`, `.xlsm`, `.xlsx` |
| OpenDocument | `.odp`, `.ods`, `.odt` |
| PDF | `.pdf` |
| PowerPoint | `.pot`, `.pps`, `.ppsm`, `.ppsx`, `.ppt`, `.pptm`, `.pptx` |
| Rich Text Format | `.rtf` |
| Word | `.doc`, `.docm`, `.docx` |

## API

- `anydoc_format_from_bytes(string $bytes): ?string`
- `anydoc_format_from_extension(string $extension): ?string`
- `anydoc_format_from_path(string $path): ?string`
- `anydoc_to_document(string $bytes, ?string $format = null): Anydoc\Document`
- `anydoc_to_markdown(string $path): string`
- `anydoc_to_markdown_bytes(string $bytes, ?string $format = null): string`

`Anydoc\Document` exposes readonly `assets`, `blocks`, and `notes`. Embedded
asset data is binary-safe. See [`stubs/anydoc.stub.php`](stubs/anydoc.stub.php)
for the complete model API.

## Errors

Catch `Anydoc\Exception\ConvertException` for conversion failures. Its concrete
types are `EncryptedException`, `IoException`, `MalformedException`,
`MissingPartException`, `ResourceLimitException`, and `UnsupportedException`.
Each concrete conversion exception exposes `ERROR_CODE`; some also expose
`detail`, `limit`, or `part`. Rust panics surface as `PanicException`.

## License

Released under the [MIT License](LICENSE.md).
