"""Serialize the official Anydoc Python binding model for PHP parity tests."""

from __future__ import annotations

import argparse
import base64
import json
from pathlib import Path
from typing import Any

import anydoc


def style(value: anydoc.Style) -> dict[str, Any]:
    return {
        "bold": value.bold,
        "italic": value.italic,
        "strike": value.strike,
        "code": value.code,
    }


def link_target(value: anydoc.LinkTarget) -> dict[str, Any]:
    return {"type": value.kind, "value": value.value}


def image_source(value: anydoc.ImageSource) -> dict[str, Any]:
    result = {"type": value.kind}
    if value.kind == "external":
        result["url"] = value.url
    elif value.kind == "asset":
        result["asset_id"] = value.asset_id
    return result


def inline(value: anydoc.Inline) -> dict[str, Any]:
    if value.kind == "text":
        return {"type": "text", "text": value.text, "style": style(value.style)}
    if value.kind == "link":
        return {
            "type": "link",
            "content": [inline(item) for item in value.content],
            "target": link_target(value.target),
        }
    if value.kind == "image":
        return {"type": "image", "alt": value.alt, "source": image_source(value.source)}
    if value.kind == "anchor":
        return {"type": "anchor", "anchor": value.anchor}
    if value.kind == "note_ref":
        return {"type": "note_ref", "note_id": value.note_id}
    if value.kind == "line_break":
        return {"type": "line_break"}
    raise ValueError(f"unknown inline kind: {value.kind}")


def document_list(value: anydoc.List) -> dict[str, Any]:
    return {
        "marker": value.marker,
        "start": value.start,
        "items": [list_item(item) for item in value.items],
    }


def list_item(value: anydoc.ListItem) -> dict[str, Any]:
    return {
        "blocks": [block(item) for item in value.blocks],
        "checked": value.checked,
        "marker_label": value.marker_label,
    }


def cell(value: anydoc.Cell) -> dict[str, Any]:
    return {
        "blocks": [block(item) for item in value.blocks],
        "col_span": value.col_span,
        "row_span": value.row_span,
    }


def cell_slot(value: anydoc.CellSlot) -> dict[str, Any]:
    if value.kind == "origin":
        return {"type": "origin", "cell": cell(value.cell)}
    if value.kind == "covered":
        return {
            "type": "covered",
            "origin_row": value.origin_row,
            "origin_col": value.origin_col,
        }
    raise ValueError(f"unknown cell slot kind: {value.kind}")


def table(value: anydoc.Table) -> dict[str, Any]:
    return {
        "grid": [[cell_slot(slot) for slot in row] for row in value.grid],
        "header_rows": value.header_rows,
        "kind": value.kind,
    }


def block(value: anydoc.Block) -> dict[str, Any]:
    if value.kind == "heading":
        return {
            "type": "heading",
            "level": value.level,
            "anchor": value.anchor,
            "content": [inline(item) for item in value.content],
        }
    if value.kind == "paragraph":
        return {"type": "paragraph", "content": [inline(item) for item in value.content]}
    if value.kind == "list":
        return {"type": "list", "list": document_list(value.list)}
    if value.kind == "table":
        return {"type": "table", "table": table(value.table)}
    if value.kind == "block_quote":
        return {"type": "block_quote", "blocks": [block(item) for item in value.blocks]}
    if value.kind == "code_block":
        return {"type": "code_block", "lang": value.lang, "text": value.text}
    if value.kind == "rule":
        return {"type": "rule"}
    raise ValueError(f"unknown block kind: {value.kind}")


def note(value: anydoc.Note) -> dict[str, Any]:
    return {
        "id": value.id,
        "kind": value.kind,
        "blocks": [block(item) for item in value.blocks],
    }


def asset(value: anydoc.Asset) -> dict[str, Any]:
    return {
        "id": value.id,
        "media_type": value.media_type,
        "origin_part": value.origin_part,
        "data": base64.b64encode(value.data).decode("ascii"),
    }


def document(value: anydoc.Document) -> dict[str, Any]:
    return {
        "blocks": [block(item) for item in value.blocks],
        "notes": [note(item) for item in value.notes],
        "assets": [asset(item) for item in value.assets],
    }


def error(value: BaseException) -> dict[str, Any]:
    codes = {
        anydoc.UnsupportedError: "unsupported",
        anydoc.MalformedError: "malformed",
        anydoc.EncryptedError: "encrypted",
        anydoc.ResourceLimitError: "resourceLimit",
        anydoc.MissingPartError: "missingPart",
    }
    result: dict[str, Any] = {
        "error": next(code for error_type, code in codes.items() if isinstance(value, error_type)),
        "message": str(value),
    }
    if isinstance(value, (anydoc.MalformedError, anydoc.MissingPartError)):
        result["part"] = value.part
    if isinstance(value, anydoc.ResourceLimitError):
        result["limit"] = value.limit
    return result


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("upstream", type=Path)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()

    fixture_root = args.upstream / "tests" / "fixtures"
    output: dict[str, Any] = {}

    for path in sorted(item for item in fixture_root.rglob("*") if item.is_file()):
        relative = path.relative_to(fixture_root).as_posix()
        explicit_format = "csv" if path.suffix.lower() == ".csv" else None
        try:
            output[relative] = {"document": document(anydoc.to_document(path.read_bytes(), explicit_format))}
        except anydoc.ConvertError as exception:
            output[relative] = error(exception)

    args.output.write_text(json.dumps(output, sort_keys=True, separators=(",", ":")))


if __name__ == "__main__":
    main()
