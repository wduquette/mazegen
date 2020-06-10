# pixel -- Pixel Type

**Syntax: pixel *subcommand* ?*args*...?**

The **pixel** command provides access to the **image** crate's `Rgba` pixel type: red/green/blue
plus the alpha channel.  This is useful for creating pixel art.

| Subcommand                   | Description |
| ---------------------------- | ----------- |
| [pixel from](#pixel-from)    | Constructs a pixel from components |
| [pixel red](#pixel-red)      | Extracts the red component from a pixel |
| [pixel green](#pixel-green)  | Extracts the green component from a pixel |
| [pixel blue](#pixel-blue)    | Extracts the blue component from a pixel |
| [pixel alpha](#pixel-alpha)  | Extracts the alpha component from a pixel |

## Pixel Representation

The string representation of a pixel is a standard web RGB hex string, one byte per color, with
an optional suffix for the alpha channel: "#*rrggbb*?.*aa*?".  Some examples:

* `#000000` black, alpha is 255
* `#00ff00` green, alpha is 255
* `#ffffff` white, alpha is 255
* `#ffffff.ff` white, alpha is 255
* `#ffffff.00` white, alpha is 0 (fully transparent)
* `#ffffff.80` white, alpha is 128 (half transparent)

The hex digits are case-insensitive.

Note: the "." is a little odd; but it simplifies the logic if we allow large pixel components
in the future.  Without the ".",  "#0123456789AB" could be interpreted as three RGB values,
"0123", "4567", and "89AB", or three RGB values and an alpha, "012", "345", "678", "9AB".

## pixel from
---
**Syntax: pixel from *r g b* ?*a*?**

Constructs a pixel value given its red, green, blue, and alpha components as integers from
0 to 255.  If omitted, the alpha component defaults to 255.  Returns the pixel value.

## pixel red
---
**Syntax: pixel red *pixel***

Extracts the red component from a pixel as an integer.

## pixel green
---
**Syntax: pixel green *pixel***

Extracts the green component from a pixel as an integer.

## pixel blue
---
**Syntax: pixel blue *pixel***

Extracts the blue component from a pixel as an integer.

## pixel alpha
---
**Syntax: pixel alpha *pixel***

Extracts the alpha component from a pixel as an integer.
