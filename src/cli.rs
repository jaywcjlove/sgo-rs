use clap::{Arg, Command, ArgMatches};

pub fn get_matches() -> ArgMatches {
  Command::new("Static File Server")
      .version("1.0")
      .about("Serves static files")
      .arg(
          Arg::new("dir")
            .short('d')
            .long("dir")
            .value_name("DIRECTORY")
            .help("Sets the directory to serve files from")
            .default_value("./static"),
      )
      .arg(
          Arg::new("port")
            .short('p')
            .long("port")
            .value_name("PORT")
            .help("Sets the port number to listen on")
            .default_value("3030"),
      )
      .arg(
          Arg::new("no-request-logging")
            .long("no-request-logging")
            .value_name("LOGGING")
            .help("Do not log any request information to the console")
            .action(clap::ArgAction::SetTrue), // Define as a flag that sets the value to false
      )
      .get_matches()
}