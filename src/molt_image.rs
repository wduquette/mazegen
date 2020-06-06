//! Molt Image Commands
use crate::MoltPixel;
use image::ImageBuffer;
use image::RgbaImage;
use molt::check_args;
use molt::molt_err;
use molt::molt_ok;
use molt::types::*;
use molt::Interp;

/// Installs the Molt image commands into the interpreter.
pub fn install(interp: &mut Interp) {
    interp.add_command("image", cmd_image);
    interp.add_command("pixel", cmd_pixel);
}

/// Image constructor: creates a new grid called "name" with a specified width and height
pub fn cmd_image(interp: &mut Interp, _: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(1, argv, 4, 4, "name width height")?;

    let name = argv[1].as_str();
    let width = argv[2].as_int()?;
    let height = argv[3].as_int()?;

    if width < 1 || height < 1 {
        return molt_err!(
            "expected an of size at least 1x1, got {}x{}",
            width,
            height
        );
    }

    let image: RgbaImage = ImageBuffer::new(width as u32, height as u32);

    make_image_object(interp, name, image);
    molt_ok!(name)
}

/// Makes a Molt object command for the given Grid with the given name.
pub fn make_image_object(interp: &mut Interp, name: &str, image: RgbaImage) {
    let ctx = interp.save_context(image);
    interp.add_context_command(name, obj_image, ctx);
}

fn obj_image(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    interp.call_subcommand(ctx, argv, 1, &OBJ_IMAGE_SUBCOMMANDS)
}

const OBJ_IMAGE_SUBCOMMANDS: [Subcommand; 4] = [
    Subcommand("clear", obj_image_clear),
    Subcommand("height", obj_image_height),
    Subcommand("save", obj_image_save),
    Subcommand("width", obj_image_width),
];

// Clears the image to a given pixel.
fn obj_image_clear(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 2, 3, "fill")?;
    let image = interp.context::<RgbaImage>(ctx);

    let pixel: MoltPixel = if argv.len() == 3 {
        MoltPixel::from_molt(&argv[2])?
    } else {
        MoltPixel::rgb(255,255,255) // White
    };

    for x in 0..image.width() {
        for y in 0..image.height() {
            image.put_pixel(x, y, pixel.ipixel())
        }
    }
    
    molt_ok!()
}

// Gets the height of the image, in pixels.
fn obj_image_height(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 2, 2, "")?;
    let image = interp.context::<RgbaImage>(ctx);
    molt_ok!(image.height() as MoltInt)
}

// Saves the content of the image to disk.
fn obj_image_save(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 3, 3, "filename")?;
    let image = interp.context::<RgbaImage>(ctx);
    let filename = argv[2].as_str();

    match image.save(filename) {
        Ok(_) => molt_ok!(),
        // TODO: Can do a better job of describing the error.
        Err(_) => molt_err!("error saving grid image"),
    }
}

// Gets the width of the image, in pixels.
fn obj_image_width(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 2, 2, "")?;
    let image = interp.context::<RgbaImage>(ctx);
    molt_ok!(image.width() as MoltInt)
}

//----------------------------------------------------------------------------
// Pixel Command
//
// TODO: Should be in its own file?

fn cmd_pixel(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    interp.call_subcommand(ctx, argv, 1, &PIXEL_SUBCOMMANDS)
}

const PIXEL_SUBCOMMANDS: [Subcommand; 5] = [
    Subcommand("from", cmd_pixel_from),
    Subcommand("red", cmd_pixel_red),
    Subcommand("green", cmd_pixel_green),
    Subcommand("blue", cmd_pixel_blue),
    Subcommand("alpha", cmd_pixel_alpha),
];

// pixel from *r g b* ?*a*?
//
// Constructs a pixel from components.
fn cmd_pixel_from(_: &mut Interp, _: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 5, 6, "r g b ?a?")?;

    let r = get_unsigned_byte(&argv[2])?;
    let g = get_unsigned_byte(&argv[3])?;
    let b = get_unsigned_byte(&argv[4])?;

    let a = if argv.len() == 6 {
        get_unsigned_byte(&argv[5])?
    } else {
        255
    };

    molt_ok!(Value::from_other(MoltPixel::rgba(r, g, b, a)))
}

// pixel red *pixel*
//
// Gets a pixel's red component.
fn cmd_pixel_red(_: &mut Interp, _: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 3, 3, "pixel")?;

    let pixel = MoltPixel::from_molt(&argv[2])?;

    molt_ok!(pixel.red() as MoltInt)
}

// pixel green *pixel*
//
// Gets a pixel's green component.
fn cmd_pixel_green(_: &mut Interp, _: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 3, 3, "pixel")?;

    let pixel = MoltPixel::from_molt(&argv[2])?;

    molt_ok!(pixel.green() as MoltInt)
}

// pixel blue *pixel*
//
// Gets a pixel's blue component.
fn cmd_pixel_blue(_: &mut Interp, _: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 3, 3, "pixel")?;

    let pixel = MoltPixel::from_molt(&argv[2])?;

    molt_ok!(pixel.blue() as MoltInt)
}

// pixel alpha *pixel*
//
// Gets a pixel's alpha component.
fn cmd_pixel_alpha(_: &mut Interp, _: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 3, 3, "pixel")?;

    let pixel = MoltPixel::from_molt(&argv[2])?;

    molt_ok!(pixel.alpha() as MoltInt)
}

fn get_unsigned_byte(arg: &Value) -> Result<u8, Exception> {
    let num = arg.as_int()?;

    if num >= 0 && num < 256 {
        Ok(num as u8)
    } else {
        molt_err!("expected unsigned byte, got \"{}\"", num)
    }
}
