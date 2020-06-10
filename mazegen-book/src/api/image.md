# image -- Image Object

An image object is a Molt object that wraps an `RgbaImage` from the
[**image** crate](https://github.com/image-rs/image) and allows it to be manipulated in
various ways.  The [**image** constructor](#constructor) creates
instances of the image [object command](#object-command).

## Pixels

Pixels are represented as Molt pixel values; see the [**pixel**](pixel.md) command for more
details.

## Constructor

**Syntax: image *name* *width* *height***

The **image** command creates an image object, a Molt binding to a Rust `RgbaImage` struct.  The
new image object is a Molt command called *name*; the command provides access to the newly created
image, which will have the given width and height in pixels.  Returns the *name*.

```tcl
$ image myimage 32 32
myimage
$ myimage width
32
$ myimage height
32
$
```

## Object Command

**Syntax: *image subcommand* ?*args*...?**

The *image* object command has the following subcommands.

| Subcommand                          | Description |
| ----------------------------------- | ----------- |
| [*image* clear](#image-clear)       | Clears an image to a given color |
| [*image* height](#image-height)      | An image's height in pixels |
| [*image* put](#image-put)            | Sets a pixel in the image |
| [*image* save](#image-save)          | Saves the image to disk |
| [*image* width](#image-width)        | An image's width in pixels |

### *image* clear
---
**Syntax: *image* clear ?*fill*?**

Clears the image's pixels by setting them to the given *fill* color, which must be a
[**pixel**](pixel.md) value.  The *fill* defaults to white, `#FFFFFF`.

```tcl
$image clear #000000    ;# Clear to black
$image clear #000000.00 ;# Clear to transparent
```

### *image* height
---
**Syntax: *image* height**

Returns an image's height in pixels.

### *image* put
---
**Syntax: *image* put *x y* ?*pixel*?**

Sets the pixel at the given (*x*,*y*) coordinates to the given *pixel* value, which defaults to
black, `#000000`.

```tcl
$image put 10 15 #0000FF   ;# Set pixel at 10,15 to blue
```

### *image* save
---
**Syntax: *image* save *filename***

Attempts to save the the image to the given file.  The image type is determined by the
[**image** crate](https://github.com/image-rs/image) from the file's file type.

### *image* width
---
**Syntax: *image* width**

Returns an image's width in pixels.
