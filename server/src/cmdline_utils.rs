// TODO assert
fn find_argument(argv: &Vec<&str>, arg_name: &str) -> Option<String> {
    let argument = argv.into_iter().find(|&&arg| arg == arg_name);
    match argument {
        Some(&arg) => Some(String::from(arg)),
        None => Option::None,
    }
}

fn parse_string_array(argv: &Vec<&str>, arg_name: &str) -> Vec<String> {
    let arg = find_argument(argv, arg_name);

    match arg {
        Some(a) => a.split(',').map(|e| String::from(e)).collect(),
        None => Vec::new(),
    }
}

fn has_argument(argv: &Vec<&str>, arg_name: &str) -> bool {
    argv.iter().any(|&arg| arg == arg_name)
}

pub struct CommandLineOptions {
    help: bool,
    /**
     * If true, use the Ivy version of Angular LS. For now this is only used for
     * development.
     */
    ivy: bool,
    log_file: Option<String>,
    log_verbosity: Option<String>,
    ng_probe_locations: Vec<String>,
    ts_probe_locations: Vec<String>,
}

pub fn parse_command_line(argv: &Vec<&str>) -> CommandLineOptions {
    CommandLineOptions {
        help: has_argument(argv, "--help"),
        ivy: has_argument(argv, "--experimental-ivy"),
        log_file: find_argument(argv, "--logFile"),
        log_verbosity: find_argument(argv, "--logVerbosity"),
        ng_probe_locations: parse_string_array(argv, "--ngProbeLocations"),
        ts_probe_locations: parse_string_array(argv, "--tsProbeLocations"),
    }
}

pub fn generate_help_message(argv: &Vec<&str>) -> String {
    format!(
        "Angular Language Service that implements the Language Server Protocol (LSP).
    
    Usage: {} {} [options]

    Options:
      --help: Prints help message.
      --logFile: Location to log messages. Logging is disabled if not provided.
      --logVerbosity: terse|normal|verbose|requestTime. See ts.server.LogLevel.
      --ngProbeLocations: Path of @angular/language-service. Required.
      --tsProbeLocations: Path of typescript. Required.
    Additional options supported by vscode-languageserver:
      --clientProcessId=<number>: Automatically kills the server if the client process dies.
      --node-ipc: Communicate using Node's IPC. This is the default.
      --stdio: Communicate over stdin/stdout.
      --socket=<number>: Communicate using Unix socket.
    ",
        argv[0], argv[1]
    )
}
