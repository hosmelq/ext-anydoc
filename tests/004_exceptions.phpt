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

$mixedPdf = base64_decode('JVBERi0xLjQKMSAwIG9iago8PCAvVHlwZSAvQ2F0YWxvZyAvUGFnZXMgMiAwIFIgPj4KZW5kb2JqCjIgMCBvYmoKPDwgL1R5cGUgL1BhZ2VzIC9LaWRzIFs1IDAgUiA3IDAgUl0gL0NvdW50IDIgPj4KZW5kb2JqCjMgMCBvYmoKPDwgL1R5cGUgL0ZvbnQgL1N1YnR5cGUgL1R5cGUxIC9CYXNlRm9udCAvSGVsdmV0aWNhID4+CmVuZG9iago0IDAgb2JqCjw8IC9UeXBlIC9YT2JqZWN0IC9TdWJ0eXBlIC9JbWFnZSAvV2lkdGggMSAvSGVpZ2h0IDEgL0NvbG9yU3BhY2UgL0RldmljZUdyYXkgL0JpdHNQZXJDb21wb25lbnQgOCAvTGVuZ3RoIDEgPj4Kc3RyZWFtCoAKZW5kc3RyZWFtCmVuZG9iago1IDAgb2JqCjw8IC9UeXBlIC9QYWdlIC9QYXJlbnQgMiAwIFIgL01lZGlhQm94IFswIDAgNjEyIDc5Ml0gL1Jlc291cmNlcyA8PCAvRm9udCA8PCAvRjEgMyAwIFIgPj4gL1hPYmplY3QgPDwgL0ltMSA0IDAgUiA+PiA+PiAvQ29udGVudHMgNiAwIFIgPj4KZW5kb2JqCjYgMCBvYmoKPDwgL0xlbmd0aCA1MyA+PgpzdHJlYW0KQlQgL0YxIDI0IFRmIDcyIDcwMCBUZCAoVGV4dCBvbiB0aGUgZmlyc3QgcGFnZSkgVGogRVQKZW5kc3RyZWFtCmVuZG9iago3IDAgb2JqCjw8IC9UeXBlIC9QYWdlIC9QYXJlbnQgMiAwIFIgL01lZGlhQm94IFswIDAgNjEyIDc5Ml0gL1Jlc291cmNlcyA8PCAvRm9udCA8PCAvRjEgMyAwIFIgPj4gL1hPYmplY3QgPDwgL0ltMSA0IDAgUiA+PiA+PiAvQ29udGVudHMgOCAwIFIgPj4KZW5kb2JqCjggMCBvYmoKPDwgL0xlbmd0aCAzMiA+PgpzdHJlYW0KcSA0NjggMCAwIDY0OCA3MiA3MiBjbSAvSW0xIERvIFEKZW5kc3RyZWFtCmVuZG9iagp4cmVmCjAgOQowMDAwMDAwMDAwIDY1NTM1IGYgCjAwMDAwMDAwMDkgMDAwMDAgbiAKMDAwMDAwMDA1OCAwMDAwMCBuIAowMDAwMDAwMTIxIDAwMDAwIG4gCjAwMDAwMDAxOTEgMDAwMDAgbiAKMDAwMDAwMDMzNSAwMDAwMCBuIAowMDAwMDAwNDg3IDAwMDAwIG4gCjAwMDAwMDA1OTAgMDAwMDAgbiAKMDAwMDAwMDc0MiAwMDAwMCBuIAp0cmFpbGVyCjw8IC9TaXplIDkgL1Jvb3QgMSAwIFIgPj4Kc3RhcnR4cmVmCjgyNAolJUVPRgo=', true);

try {
    anydoc_to_markdown_bytes($mixedPdf, 'pdf');
} catch (Anydoc\Exception\NeedsOcrException $exception) {
    echo $exception::class, ':', $exception::ERROR_CODE, "\n";
    echo 'page-count:', $exception->pageCount, "\n";
    echo 'pages:', implode(',', $exception->pages), "\n";
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
Anydoc\Exception\NeedsOcrException:needsOcr
page-count:2
pages:2
Exception:unknown anydoc format: unknown
