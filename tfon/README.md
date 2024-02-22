This is a bitmap font parsing / conversion library.  It supports a few font
formats:

Format   | Import | Export |
---------|--------|--------|
[bdf]    | ✔️      | ❌     |
`.tfon`  | ✔️      | ✔️      |
`.ifnt`  | ✔️      | ✔️      |
`.ifntx` | ✔️      | ❌     |

## `.tfon` Format

Any text editor can be used to create fonts in this format.  There are two
parts to the file: a **header** and a list of **characters**.

The header contains 4 key/value pairs:

- **font_name**: Name of font (up to 64 characters)
- **font_number**: Integer number, between 1 and 255
- **char_spacing**: Horizontal pixel spacing between characters
- **line_spacing**: Vertical pixel spacing between lines

```text
font_name: Example font
font_number: 6
char_spacing: 1
line_spacing: 3
```

The character list can have between 1 and 255 values.  Each character has a
**ch** definition and a **pixel grid**:

- **ch**: Character number, followed by its _symbol_
- **Pixel grid** is a rectangular section of **.** (off) / **@** (on) of the
  character bitmap.  All characters in a font must have the same height.

```text
ch: 52 4
...@.
..@@.
.@.@.
@..@.
@@@@@
...@.
...@.
```


[bdf]: https://en.wikipedia.org/wiki/Glyph_Bitmap_Distribution_Format
