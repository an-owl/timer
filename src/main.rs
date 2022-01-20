use std::process::exit;
use timer::*;

fn main() {

    let time = get_time();
    let state = if let Ok(matches) = parse_opts(){
        InternalState{
            time: std::time::Duration::from_secs(time),
            suppress_notifications: matches.opt_present("s"),
            no_stdout: matches.opt_present("q"),
        }
    } else {
        return
    };

    timer_core(state);

}


/// parses options returns [getopts::Matches]
fn parse_opts() -> Result<getopts::Matches,()> {
    use getopts::{Occur,HasArg};
    static USAGE_BRIEF: &str =
        "USAGE: timer [OPTIONS] [TIMES] \n\
Times are formatted as [0-9]t there t is a duration suffix\n\
Supported suffixes are s (Seconds), m (Minutes), h (Hours), d(Days)\n\
All given times will be added together (i.e. 1m 17s = 77s)";

    let mut opts = getopts::Options::new();
    opts.opt(
        "q",
        "quiet",
        "Suppress writing to stdout",
        "",
        HasArg::No,
        Occur::Optional,
    );
    opts.opt(
        "s",
        "suppress-notifiy",
        "Suppress desktop notifications",
        "",
        HasArg::No,
        Occur::Optional,
    );
    opts.opt(
        "h",
        "help",
        "Prints a nice help message",
        "",
        HasArg::No,
        Occur::Optional,
    );
    let parse = opts.parse(std::env::args());
    return if parse.is_err() {
        println!("{}", opts.usage(USAGE_BRIEF));
        Err(())
    } else if parse.clone().unwrap().opt_present("h"){
        // I'd do these both in one if statement but i cant guarantee that
        // an unwrap() will always be called after is_err()
        // and i cant get around that without just being worse than this
        println!("{}", opts.usage(USAGE_BRIEF));
        Err(())
    } else {

        Ok(parse.unwrap())
    }

}


/// gets time to run from opts excludes all args beginning with "-"
fn get_time() -> u64 {
    let mut total_time = 0;

    for o in std::env::args().skip(1){

        if o.starts_with("-"){ continue }
        let time = parse_time(&*o);

        if time.is_err(){
            exit(1)
        } else {
            total_time += time.unwrap();

        }
    }
    total_time
}

/// calculate time in seconds from time symbol
///
/// Input is formatted as `[DURATION][SUFFIX]`
///
/// returns `Err(())` if improperly formatted
fn parse_time(symbol: &str) -> Result<u64,()> {
    static SUFFIXES: [TimeMultiplier;4] = [
        TimeMultiplier('s',1),
        TimeMultiplier('m',60),
        TimeMultiplier('h',60*60),
        TimeMultiplier('d',60*60*24)
    ];


    if let Ok(mut num) = symbol[..symbol.len()-1].parse::<u64>() {
        let mut found_suffix = false;

        for t in &SUFFIXES {
            if t.0 == symbol.chars().last().unwrap() {
                num *= t.1;
                found_suffix = true;
                break
            }
        }

        if !found_suffix {
            return Err(())
        }
        Ok(num)
    } else {
        eprintln!("Failed to parse {} invalid argument", symbol);
        return Err(())
    }
}

/// struct for parsing times
/// `Self.0` is for the suffix character
/// `Self.1` is for its multiplier
struct TimeMultiplier(char,u64);