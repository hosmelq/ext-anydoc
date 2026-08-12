--TEST--
anydoc exposes typed conversion failures
--EXTENSIONS--
anydoc
--FILE--
<?php

try {
    anydoc_to_markdown_bytes('not a document', 'docx');
} catch (Anydoc\Exception\MalformedException $exception) {
    echo $exception::class, "\n";
    echo $exception::ERROR_CODE, "\n";
    echo $exception->part === null ? "part:null\n" : "part:set\n";
    echo str_contains($exception->detail, 'not a readable zip archive') ? "detail:zip\n" : "detail:other\n";
}

try {
    anydoc_to_markdown(__DIR__.'/missing.docx');
} catch (Anydoc\Exception\IoException $exception) {
    echo $exception::class, ':', $exception::ERROR_CODE, "\n";
}

try {
    anydoc_to_markdown_bytes('x', 'unknown');
} catch (Exception $exception) {
    echo $exception::class, ':', $exception->getMessage(), "\n";
}
?>
--EXPECT--
Anydoc\Exception\MalformedException
malformed
part:null
detail:zip
Anydoc\Exception\IoException:io
Exception:unknown anydoc format: unknown
