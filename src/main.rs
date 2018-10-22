extern crate ssh2;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::io::prelude::*;
use std::net::TcpStream;
use std::process;

#[macro_use]
extern crate clap;
use clap::App;
use ssh2::Session;

struct Command {
    command: String,
    session: Session,
}

struct CommandResult {
    command: String,
    output: String,
    exitcode: i32,
}

impl Command {
    // Execute the command in question
    // Return a CommandResult object
    fn run(self) -> CommandResult {
        let mut channel = self.session.channel_session().unwrap();
        channel.exec(&self.command).unwrap();
        let mut output = String::new();
        channel.read_to_string(&mut output).unwrap();
        let exitcode = channel.exit_status().unwrap();

        let res = CommandResult {
            command: self.command,
            output: output,
            exitcode: exitcode,
        };
        return res;
    }
}

fn main() {
    env_logger::init();

    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Setup variables
    let hostname = matches.value_of("HOSTNAME").unwrap();
    let port = matches.value_of("port").unwrap_or("22");
    let hostport = format!("{}:{}", hostname, port);
    let cmd = matches.value_of("COMMAND").unwrap();
    let user = matches.value_of("user").unwrap_or(env!("USER"));

    debug!(" Hostname: {}", hostname);
    debug!("     Port: {}", port);

    // Connect to sshd
    let tcp = TcpStream::connect(hostport).unwrap();
    let mut sess = Session::new().unwrap();
    sess.handshake(&tcp).unwrap();

    // Try to authenticate with the first identity in the agent.
    sess.userauth_agent(user).unwrap();

    let cmd = Command {
        command: cmd.to_string(),
        session: sess,
    };

    // Run the command and display output
    let result = cmd.run();
    debug!("  Command: {}", result.command);
    debug!("Exit Code: {}", result.exitcode);
    println!("{}", result.output);
    process::exit(result.exitcode)
}
