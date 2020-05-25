//! Molt Image Commands
use crate::MoltPixel;
use molt::check_args;
use molt::molt_err;
use molt::molt_ok;
use molt::types::*;
use molt::Interp;

/// Installs the Molt image commands into the interpreter.
pub fn install(interp: &mut Interp) {
    interp.add_command("pixel", cmd_pixel);
}

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
