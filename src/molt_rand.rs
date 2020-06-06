//! Molt "rand" Command
use molt::check_args;
use molt::molt_err;
use molt::molt_ok;
use molt::types::*;
use molt::Interp;
use rand::{thread_rng, Rng};

/// Installs the Molt "rand" command into the interpreter.
pub fn install(interp: &mut Interp) {
    interp.add_command("rand", cmd_rand);
}

// Random number generation.  Uses rand::thread_rng().
// TODO: Should save thread_rng and reuse it?
fn cmd_rand(interp: &mut Interp, ctx: ContextID, argv: &[Value]) -> MoltResult {
    interp.call_subcommand(ctx, argv, 1, &RAND_SUBCOMMANDS)
}

const RAND_SUBCOMMANDS: [Subcommand; 3] = [
    Subcommand("bool", cmd_rand_bool),
    Subcommand("range", cmd_rand_range),
    Subcommand("sample", cmd_rand_sample),
];

// rand bool ?*prob*?
//
// Returns `1` or `0`, with `1` having the given probability, which defaults to 0.5.
fn cmd_rand_bool(_: &mut Interp, _: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 2, 3, "?prob?")?;

    let prob = if argv.len() == 3 {
        argv[2].as_float()?
    } else {
        0.5
    };

    if prob <= 0.0 || prob >= 1.0 {
        molt_err!("expected probability between 0.0 and 1.0, got \"{}\"", prob)
    } else {
        molt_ok!(thread_rng().gen_bool(prob))
    }
}

// rand range ?*start*? *end*
//
// Generates a random integer in the range [*start*, *end).  If not given,
// *start* defaults to 0.
fn cmd_rand_range(_: &mut Interp, _: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 3, 4, "?start? end")?;

    let (start, end) = if argv.len() == 3 {
        (0, argv[2].as_int()?)
    } else {
        (argv[2].as_int()?, argv[3].as_int()?)
    };

    let val: MoltInt = thread_rng().gen_range(start, end);

    molt_ok!(val)
}

// rand range *list...*
//
// Makes a random selection from the list, which may be passed as a single argument
// or as multiple arguments.
fn cmd_rand_sample(_: &mut Interp, _: ContextID, argv: &[Value]) -> MoltResult {
    // Correct number of arguments?
    check_args(2, argv, 3, 0, "list...")?;

    if argv.len() != 3 {
        sample_from(&argv[2..])
    } else {
        sample_from(&*argv[2].as_list()?)
    }
}

fn sample_from(list: &[Value]) -> MoltResult {
    if list.is_empty() {
        molt_ok!()
    } else if list.len() == 1 {
        molt_ok!(list[0].clone())
    } else {
        let i: usize = thread_rng().gen_range(0, list.len());
        molt_ok!(list[i].clone())
    }
}
