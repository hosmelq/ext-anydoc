<?php

declare(strict_types=1);

if ($argc !== 3) {
    fwrite(STDERR, "Usage: php model.php /path/to/anydoc /path/to/oracle.json\n");
    exit(2);
}

$upstream = rtrim($argv[1], DIRECTORY_SEPARATOR);
$fixtures = $upstream.'/tests/fixtures';
$oracleJson = file_get_contents($argv[2]);

if (! is_dir($fixtures) || $oracleJson === false) {
    throw new RuntimeException('The fixture root or model oracle is unavailable.');
}

$oracle = json_decode($oracleJson, true, flags: JSON_THROW_ON_ERROR);
$actual = [];
$iterator = new RecursiveIteratorIterator(new RecursiveDirectoryIterator(
    $fixtures,
    FilesystemIterator::SKIP_DOTS,
));

foreach ($iterator as $file) {
    if (! $file->isFile()) {
        continue;
    }

    $relative = str_replace(DIRECTORY_SEPARATOR, '/', substr($file->getPathname(), strlen($fixtures) + 1));
    $bytes = file_get_contents($file->getPathname());

    if ($bytes === false) {
        throw new RuntimeException("Unable to read fixture: $relative");
    }

    try {
        $format = strtolower($file->getExtension()) === 'csv' ? 'csv' : null;
        $actual[$relative] = ['document' => canonicalDocument(anydoc_to_document($bytes, $format))];
    } catch (Anydoc\Exception\ConvertException $exception) {
        $actual[$relative] = canonicalError($exception);
    }
}
ksort($actual);

if (count($actual) !== 66 || count($oracle) !== 66) {
    throw new RuntimeException(sprintf(
        'Unexpected model corpus size: PHP %d, Python %d',
        count($actual),
        count($oracle),
    ));
}

foreach ($oracle as $relative => $expected) {
    if (! array_key_exists($relative, $actual)) {
        throw new RuntimeException("$relative: missing PHP model result");
    }

    if ($actual[$relative] != $expected) {
        throw new RuntimeException(sprintf(
            "%s: PHP model differs from official Python binding\nExpected: %s\nActual: %s",
            $relative,
            json_encode($expected, JSON_THROW_ON_ERROR | JSON_UNESCAPED_SLASHES),
            json_encode($actual[$relative], JSON_THROW_ON_ERROR | JSON_UNESCAPED_SLASHES),
        ));
    }
}

$errors = count(array_filter($actual, static fn (array $result): bool => isset($result['error'])));
echo sprintf(
    "Anydoc model parity passed: %d fixtures (%d documents, %d errors).\n",
    count($actual),
    count($actual) - $errors,
    $errors,
);

function canonicalDocument(Anydoc\Document $document): array
{
    return [
        'blocks' => array_map(canonicalBlock(...), $document->blocks),
        'notes' => array_map(canonicalNote(...), $document->notes),
        'assets' => array_map(canonicalAsset(...), $document->assets),
    ];
}

function canonicalBlock(Anydoc\Block $block): array
{
    return match (true) {
        $block instanceof Anydoc\Heading => [
            'type' => 'heading',
            'level' => $block->level,
            'anchor' => $block->anchor,
            'content' => array_map(canonicalInline(...), $block->content),
        ],
        $block instanceof Anydoc\Paragraph => [
            'type' => 'paragraph',
            'content' => array_map(canonicalInline(...), $block->content),
        ],
        $block instanceof Anydoc\BlockList => [
            'type' => 'list',
            'list' => canonicalList($block->list),
        ],
        $block instanceof Anydoc\BlockTable => [
            'type' => 'table',
            'table' => canonicalTable($block->table),
        ],
        $block instanceof Anydoc\BlockQuote => [
            'type' => 'block_quote',
            'blocks' => array_map(canonicalBlock(...), $block->blocks),
        ],
        $block instanceof Anydoc\CodeBlock => [
            'type' => 'code_block',
            'lang' => $block->lang,
            'text' => $block->text,
        ],
        $block instanceof Anydoc\Rule => ['type' => 'rule'],
        default => throw new RuntimeException('Unknown PHP block class: '.$block::class),
    };
}

function canonicalInline(Anydoc\Inline $inline): array
{
    return match (true) {
        $inline instanceof Anydoc\Text => [
            'type' => 'text',
            'text' => $inline->text,
            'style' => canonicalStyle($inline->style),
        ],
        $inline instanceof Anydoc\Link => [
            'type' => 'link',
            'content' => array_map(canonicalInline(...), $inline->content),
            'target' => canonicalLinkTarget($inline->target),
        ],
        $inline instanceof Anydoc\Image => [
            'type' => 'image',
            'alt' => $inline->alt,
            'source' => canonicalImageSource($inline->source),
        ],
        $inline instanceof Anydoc\Anchor => ['type' => 'anchor', 'anchor' => $inline->anchor],
        $inline instanceof Anydoc\NoteReference => ['type' => 'note_ref', 'note_id' => $inline->noteId],
        $inline instanceof Anydoc\LineBreak => ['type' => 'line_break'],
        default => throw new RuntimeException('Unknown PHP inline class: '.$inline::class),
    };
}

function canonicalStyle(Anydoc\Style $style): array
{
    return [
        'bold' => $style->bold,
        'italic' => $style->italic,
        'strike' => $style->strike,
        'code' => $style->code,
    ];
}

function canonicalLinkTarget(Anydoc\LinkTarget $target): array
{
    return match (true) {
        $target instanceof Anydoc\ExternalLink => ['type' => 'external', 'value' => $target->value],
        $target instanceof Anydoc\RelativeLink => ['type' => 'relative', 'value' => $target->value],
        $target instanceof Anydoc\AnchorLink => ['type' => 'anchor', 'value' => $target->value],
        default => throw new RuntimeException('Unknown PHP link target class: '.$target::class),
    };
}

function canonicalImageSource(Anydoc\ImageSource $source): array
{
    return match (true) {
        $source instanceof Anydoc\ExternalImage => ['type' => 'external', 'url' => $source->url],
        $source instanceof Anydoc\AssetImage => ['type' => 'asset', 'asset_id' => $source->assetId],
        $source instanceof Anydoc\UnavailableImage => ['type' => 'unavailable'],
        default => throw new RuntimeException('Unknown PHP image source class: '.$source::class),
    };
}

function canonicalList(Anydoc\DocumentList $list): array
{
    $markers = [
        'bullet' => 'bullet',
        'decimal' => 'decimal',
        'lowerAlpha' => 'lower_alpha',
        'upperAlpha' => 'upper_alpha',
        'lowerRoman' => 'lower_roman',
        'upperRoman' => 'upper_roman',
    ];

    return [
        'marker' => $markers[$list->marker],
        'start' => $list->start,
        'items' => array_map(canonicalListItem(...), $list->items),
    ];
}

function canonicalListItem(Anydoc\ListItem $item): array
{
    return [
        'blocks' => array_map(canonicalBlock(...), $item->blocks),
        'checked' => $item->checked,
        'marker_label' => $item->markerLabel,
    ];
}

function canonicalTable(Anydoc\Table $table): array
{
    return [
        'grid' => array_map(
            static fn (array $row): array => array_map(canonicalCellSlot(...), $row),
            $table->grid,
        ),
        'header_rows' => $table->headerRows,
        'kind' => $table->kind,
    ];
}

function canonicalCellSlot(Anydoc\CellSlot $slot): array
{
    return match (true) {
        $slot instanceof Anydoc\OriginCell => ['type' => 'origin', 'cell' => canonicalCell($slot->cell)],
        $slot instanceof Anydoc\CoveredCell => [
            'type' => 'covered',
            'origin_row' => $slot->originRow,
            'origin_col' => $slot->originCol,
        ],
        default => throw new RuntimeException('Unknown PHP cell slot class: '.$slot::class),
    };
}

function canonicalCell(Anydoc\Cell $cell): array
{
    return [
        'blocks' => array_map(canonicalBlock(...), $cell->blocks),
        'col_span' => $cell->colSpan,
        'row_span' => $cell->rowSpan,
    ];
}

function canonicalNote(Anydoc\Note $note): array
{
    return [
        'id' => $note->id,
        'kind' => $note->kind,
        'blocks' => array_map(canonicalBlock(...), $note->blocks),
    ];
}

function canonicalAsset(Anydoc\Asset $asset): array
{
    return [
        'id' => $asset->id,
        'media_type' => $asset->mediaType,
        'origin_part' => $asset->originPart,
        'data' => base64_encode($asset->data),
    ];
}

function canonicalError(Anydoc\Exception\ConvertException $exception): array
{
    $result = [
        'error' => match (true) {
            $exception instanceof Anydoc\Exception\UnsupportedException => 'unsupported',
            $exception instanceof Anydoc\Exception\MalformedException => 'malformed',
            $exception instanceof Anydoc\Exception\EncryptedException => 'encrypted',
            $exception instanceof Anydoc\Exception\ResourceLimitException => 'resourceLimit',
            $exception instanceof Anydoc\Exception\MissingPartException => 'missingPart',
            $exception instanceof Anydoc\Exception\IoException => 'io',
            default => throw new RuntimeException('Unknown PHP conversion exception: '.$exception::class),
        },
        'message' => $exception->getMessage(),
    ];

    if ($exception instanceof Anydoc\Exception\MalformedException
        || $exception instanceof Anydoc\Exception\MissingPartException) {
        $result['part'] = $exception->part;
    }
    if ($exception instanceof Anydoc\Exception\ResourceLimitException) {
        $result['limit'] = $exception->limit;
    }

    return $result;
}
