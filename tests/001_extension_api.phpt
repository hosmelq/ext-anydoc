--TEST--
anydoc exposes its public PHP API
--EXTENSIONS--
anydoc
--FILE--
<?php

$functions = [
    'anydoc_format_from_bytes',
    'anydoc_format_from_extension',
    'anydoc_format_from_path',
    'anydoc_to_document',
    'anydoc_to_markdown',
    'anydoc_to_markdown_bytes',
];

echo extension_loaded('anydoc') ? "loaded\n" : "missing\n";
echo is_string(phpversion('anydoc')) ? "versioned\n" : "unversioned\n";

foreach ($functions as $function) {
    echo $function, ':', function_exists($function) ? "yes\n" : "no\n";
}

echo (new ReflectionClass(Anydoc\Document::class))->isReadOnly() ? "document:readonly\n" : "document:mutable\n";
echo (new ReflectionClass(Anydoc\Block::class))->isAbstract() ? "block:abstract\n" : "block:concrete\n";
echo is_subclass_of(Anydoc\Checkbox::class, Anydoc\Inline::class) ? "checkbox:inline\n" : "checkbox:invalid\n";
echo is_subclass_of(
    Anydoc\Exception\MalformedException::class,
    Anydoc\Exception\ConvertException::class,
) ? "exceptions:typed\n" : "exceptions:untyped\n";
echo (new ReflectionClass(Anydoc\ListItem::class))->hasProperty('checked') ? "list-item:legacy\n" : "list-item:current\n";
echo is_subclass_of(Anydoc\MathBlock::class, Anydoc\Block::class) ? "math-block:block\n" : "math-block:invalid\n";
echo is_subclass_of(Anydoc\MathInline::class, Anydoc\Inline::class) ? "math-inline:inline\n" : "math-inline:invalid\n";
?>
--EXPECT--
loaded
versioned
anydoc_format_from_bytes:yes
anydoc_format_from_extension:yes
anydoc_format_from_path:yes
anydoc_to_document:yes
anydoc_to_markdown:yes
anydoc_to_markdown_bytes:yes
document:readonly
block:abstract
checkbox:inline
exceptions:typed
list-item:current
math-block:block
math-inline:inline
