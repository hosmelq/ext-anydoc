--TEST--
Anydoc converts CSV bytes to Markdown and the document model
--EXTENSIONS--
anydoc
--FILE--
<?php

$csv = "Name,Age\nAda,36\n";

echo anydoc_to_markdown_bytes($csv, 'csv');

$document = anydoc_to_document($csv, 'csv');
$table = $document->blocks[0]->table;
$name = $table->grid[0][0]->cell->blocks[0]->content[0];
$age = $table->grid[1][1]->cell->blocks[0]->content[0];

echo $document::class, "\n";
echo count($document->blocks), ':', count($document->notes), ':', count($document->assets), "\n";
echo $table->kind, ':', $table->headerRows, ':', count($table->grid), "\n";
echo $name->text, ':', $age->text, "\n";
?>
--EXPECT--
| Name | Age |
| --- | --- |
| Ada | 36 |
Anydoc\Document
1:0:0
data:1:2
Name:36
