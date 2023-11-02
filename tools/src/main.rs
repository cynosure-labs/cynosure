const HELP: &str = "\
Cynosure Tools

USAGE:
  tools [COMMAND] [OPTIONS] [ARGS]

FLAGS:
  -h, --help            Prints help information

COMMAND:
  run                   Runs a command
  run-wasm              Runs a wasm command
  test                  Tests a command

OPTIONS:
  --bin                 Binary to run
  --example             Example to run
";

fn main() {
  let args = match parse_args() {
      Ok(v) => v,
      Err(e) => {
          eprintln!("Error: {}.", e);
          std::process::exit(1);
      }
  };
  println!("{:#?}", args);
}

fn parse_args() -> Result<(), pico_args::Error> {
  let mut pargs = pico_args::Arguments::from_env();

  if pargs.contains(["-h", "--help"]) {
    print!("{}", HELP);
    std::process::exit(101);
  }

  // It's up to the caller what to do with the remaining arguments.
  let remaining = pargs.finish();
  if !remaining.is_empty() {
      eprintln!("Warning: unused arguments left: {:?}.", remaining);
  }

  Ok({})
}
