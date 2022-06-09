use std::process::Command;
use std::error;

#[derive(Debug)]
enum Status {
    ProgramNotInstalled,
    CommandFailed,
    CountryFailed,
    InputFailed
}

impl std::error::Error for Status {}
impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Status::ProgramNotInstalled => write!(f, "Program not Installed"),
            Status::CommandFailed => write!(f, "Command Failed"),
            Status::CountryFailed => write!(f, "Country Failed"),
            Status::InputFailed=> write!(f, "Input Failed"),
        }
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    // Command | modifier | modifier | Result<Child> | Child | Result<ExitStatus> | ExitStatus | bool
    if !Command::new("pacman").arg("-Qq").arg("cyberghostvpn").spawn().expect("cyberghostvpn is not installed")
        .wait().expect("check wait failed")
        .success() {
            return Err(Box::new(Status::ProgramNotInstalled));
    }
    
    let mut vpn = Command::new("cyberghostvpn");
    // let arg_vec: Vec<String> = std::env::args().collect();

    for (i, ar) in std::env::args().enumerate() {
        match i {
            0 => {},
            1 => {
                match ar.as_str() {
                    "stop"      => {vpn.arg("--stop");},
                    "help"      => {vpn.arg("--stop");},
                    "status"    => {vpn.arg("--status");},
                    "setup"     => {vpn.arg("--setup");},
                    "connect"   => {vpn.arg("--traffic").arg("--connect").arg("--country-code");},
                    _           => return Err(Box::new(Status::InputFailed)),
                }
            },
            2 => {if ar.len() == 2 {vpn.arg(&ar);} else {return Err(Box::new(Status::CountryFailed));}},
            3 => {vpn.arg("--city").arg(format!("'{}'", ar));},
            _ => {return Err(Box::new(Status::InputFailed));}
        }
    }

    if vpn.spawn().expect("process failed")
        .wait().expect("Wait error")
        .success() {
            return Ok(());} else {return Err(Box::new(Status::CommandFailed));
        }
}
