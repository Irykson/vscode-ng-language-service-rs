// TODO assert
fn find_argument(argv: &Vec<&str>, arg_name: &str) -> Option<String> {
    let argument = argv.into_iter().find(|&arg| arg == arg_name);
    match argument {
        Some(&arg) => Some(String::from(arg)),
        None => Option::None,
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
    logFile: Option<String>,
    // logVerbosity: Option<String>,
    // ngProbeLocations: Vec<String>,
    // tsProbeLocations: Vec<String>,
}

pub fn parse_command_line(argv: Vec<&str>) -> CommandLineOptions {
    CommandLineOptions {
        help: has_argument(&argv, "--help"),
        ivy: has_argument(&argv, "--experimental-ivy"),
        logFile: find_argument(&argv, "--logFile"),
    }
}
