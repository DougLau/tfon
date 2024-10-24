This is a bitmap font parsing / conversion library.  It supports a few font
formats:

Format   | Import | Export |
---------|--------|--------|
[bdf]    | ✔️      | ❌     |
`.tfon`  | ✔️      | ✔️      |
`.ifnt`  | ✔️      | ✔️      |
`.ifntx` | ✔️      | ❌     |

## `.tfon` Format

Fonts in this format can be created with any text editor.  There are two
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

- **ch**: Character number, between 1 and 255, followed by its _symbol_
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

### Symbols (ASCII)

`X` | `0X` | `1X` | `2X` | `3X` | `4X` | `5X` | `6X` | `7X`
----|------|------|------|------|------|------|------|-----
`0` | NUL  | DLE  | SP   | 0    | @    | P    | \`   | p
`1` | SOH  | DC1  | !    | 1    | A    | Q    | a    | q
`2` | STX  | DC2  | \"   | 2    | B    | R    | b    | r
`3` | ETX  | DC3  | #    | 3    | C    | S    | c    | s
`4` | EOT  | DC4  | $    | 4    | D    | T    | d    | t
`5` | ENQ  | NAK  | %    | 5    | E    | U    | e    | u
`6` | ACK  | SYN  | &    | 6    | F    | V    | f    | v
`7` | BEL  | ETB  | '    | 7    | G    | W    | g    | w
`8` | BS   | CAN  | (    | 8    | H    | X    | h    | x
`9` | HT   | EM   | )    | 9    | I    | Y    | i    | y
`A` | LF   | SUB  | *    | :    | J    | Z    | j    | z
`B` | VT   | ESC  | +    | ;    | K    | \[   | k    | {
`C` | FF   | FS   | ,    | <    | L    | \\   | l    | \|
`D` | CR   | GS   | -    | =    | M    | \]   | m    | }
`E` | SO   | RS   | .    | >    | N    | ^    | n    | ~
`F` | SI   | US   | /    | ?    | O    | _    | o    | DEL

### Symbols (Latin-1)

`X` | `8X` | `9X` | `AX` | `BX` | `CX` | `DX` | `EX` | `FX`
----|------|------|------|------|------|------|------|-----
`0` | PAD  | DCS  | NBSP | °    | À    | Ð    | à    | ð
`1` | HOP  | PU1  | ¡    | ±    | Á    | Ñ    | á    | ñ
`2` | BPH  | PU2  | ¢    | ²    | Â    | Ò    | â    | ò
`3` | NBH  | STS  | £    | ³    | Ã    | Ó    | ã    | ó
`4` | IND  | CCH  | ¤    | ´    | Ä    | Ô    | ä    | ô
`5` | NEL  | MW   | ¥    | µ    | Å    | Õ    | å    | õ
`6` | SSA  | SPA  | ¦    | ¶    | Æ    | Ö    | æ    | ö
`7` | ESA  | EPA  | §    | ·    | Ç    | ×    | ç    | ÷
`8` | HTS  | SOS  | ¨    | ¸    | È    | Ø    | è    | ø
`9` | HTJ  | SGCI | ©    | ¹    | É    | Ù    | é    | ù
`A` | LTS  | SCI  | ª    | º    | Ê    | Ú    | ê    | ú
`B` | PLD  | CSI  | «    | »    | Ë    | Û    | ë    | û
`C` | PLU  | ST   | ¬    | ¼    | Ì    | Ü    | ì    | ü
`D` | RI   | OSC  | SHY  | ½    | Í    | Ý    | í    | ý
`E` | SS2  | PM   | ®    | ¾    | Î    | Þ    | î    | þ
`F` | SS3  | APC  | ¯    | ¿    | Ï    | ß    | ï    | ÿ


[bdf]: https://en.wikipedia.org/wiki/Glyph_Bitmap_Distribution_Format
