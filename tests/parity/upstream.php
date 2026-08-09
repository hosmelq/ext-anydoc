<?php

declare(strict_types=1);

if ($argc !== 2) {
    fwrite(STDERR, "Usage: php upstream.php /path/to/anydoc\n");
    exit(2);
}

$upstream = rtrim($argv[1], DIRECTORY_SEPARATOR);
$fixtures = $upstream.'/tests/fixtures';
$snapshots = $upstream.'/tests/snapshots';

if (! is_dir($fixtures) || ! is_dir($snapshots)) {
    throw new RuntimeException('The supplied path is not an Anydoc source checkout.');
}

$files = [];
$iterator = new RecursiveIteratorIterator(new RecursiveDirectoryIterator(
    $fixtures,
    FilesystemIterator::SKIP_DOTS,
));

foreach ($iterator as $file) {
    if ($file->isFile()) {
        $files[] = $file->getPathname();
    }
}
sort($files);

$snapshotCount = 0;
$abuseCount = 0;

foreach ($files as $path) {
    $relative = str_replace(DIRECTORY_SEPARATOR, '/', substr($path, strlen($fixtures) + 1));

    if (str_starts_with($relative, 'abuse/')) {
        try {
            anydoc_to_markdown($path);
            throw new RuntimeException("$relative: expected a resource-limit failure");
        } catch (Anydoc\Exception\ResourceLimitException) {
            $abuseCount++;
        }

        continue;
    }

    $snapshotPath = $snapshots.'/snapshots__'.str_replace('/', '__', $relative).'.snap';
    $snapshot = file_get_contents($snapshotPath);

    if ($snapshot === false) {
        throw new RuntimeException("$relative: missing upstream snapshot");
    }

    $parts = explode("---\n", $snapshot, 3);

    if (count($parts) !== 3) {
        throw new RuntimeException("$relative: invalid upstream snapshot header");
    }

    $expected = $parts[2];

    if (str_starts_with($expected, 'ERROR: ') && str_ends_with($expected, "\n")) {
        $expected = substr($expected, 0, -1);
    }

    try {
        $actual = anydoc_to_markdown($path);
    } catch (Anydoc\Exception\ConvertException $exception) {
        $actual = 'ERROR: '.$exception->getMessage();
    } catch (Throwable $throwable) {
        throw new RuntimeException(
            sprintf('%s: unexpected %s: %s', $relative, $throwable::class, $throwable->getMessage()),
            previous: $throwable,
        );
    }

    if ($actual !== $expected) {
        throw new RuntimeException(sprintf(
            '%s: Markdown differs from the upstream snapshot (expected %d bytes, got %d)',
            $relative,
            strlen($expected),
            strlen($actual),
        ));
    }

    $snapshotCount++;
}

$detectedFormats = [
    'csv' => null,
    'doc' => 'doc',
    'docx' => 'docx',
    'epub' => 'epub',
    'odp' => 'odp',
    'ods' => 'ods',
    'odt' => 'odt',
    'pdf' => 'pdf',
    'ppt' => 'ppt',
    'pptx' => 'pptx',
    'rtf' => 'rtf',
    'xls' => 'xlsx',
    'xlsx' => 'xlsx',
];
$detectionCount = 0;

foreach ($detectedFormats as $directory => $expectedFormat) {
    foreach (glob($fixtures.'/'.$directory.'/*') ?: [] as $path) {
        $bytes = file_get_contents($path);

        if ($bytes === false) {
            throw new RuntimeException("Unable to read fixture: $path");
        }

        $actualFormat = anydoc_format_from_bytes($bytes);

        if ($actualFormat !== $expectedFormat) {
            throw new RuntimeException(sprintf(
                '%s: expected format %s, got %s',
                basename($path),
                var_export($expectedFormat, true),
                var_export($actualFormat, true),
            ));
        }

        $detectionCount++;
    }
}

if ($snapshotCount !== 58 || $abuseCount !== 8 || $detectionCount !== 48) {
    throw new RuntimeException(sprintf(
        'Unexpected corpus size: %d snapshots, %d abuse fixtures, %d detection fixtures',
        $snapshotCount,
        $abuseCount,
        $detectionCount,
    ));
}

echo sprintf(
    "Anydoc parity passed: %d snapshots, %d abuse fixtures, %d detection fixtures.\n",
    $snapshotCount,
    $abuseCount,
    $detectionCount,
);
