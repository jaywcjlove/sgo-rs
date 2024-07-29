use colored::Colorize;
use clap::{Arg, Command, ArgMatches};

pub fn get_matches() -> ArgMatches {
  Command::new("Static File Server")
      .version("1.0")
      .about(&format!(
          "{} - Static file serving and directory listing",
          "\nsgo".blue().bold()
      ))
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
            .short('L')
            .long("no-request-logging")
            .help("Do not log any request information to the console")
            .action(clap::ArgAction::SetTrue), // Define as a flag that sets the value to false
      )
      .arg(
          Arg::new("cors")
              .short('C')
              .long("cors")
              .help("Enable CORS, sets `Access-Control-Allow-Origin` to `*`")
              .action(clap::ArgAction::SetTrue),
      )
      .get_matches()
}