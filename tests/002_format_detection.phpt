--TEST--
Anydoc detects and normalizes document formats
--EXTENSIONS--
anydoc
--FILE--
<?php

echo json_encode([
    'bytes' => anydoc_format_from_bytes("%PDF-1.7\n"),
    'extension' => anydoc_format_from_extension('.pptm'),
    'path' => anydoc_format_from_path('report.xls'),
    'unknown' => anydoc_format_from_extension('.unknown'),
], JSON_THROW_ON_ERROR), "\n";
?>
--EXPECT--
{"bytes":"pdf","extension":"pptx","path":"xlsx","unknown":null}
