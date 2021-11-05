use std::process::Command;

enum Status {ProgramNotInstalled=1, CommandFailed=2, CountryFailed=3, InputFailed=4}

impl Status {
    fn as_str(self) -> &'static str {
        match self {
            Self::ProgramNotInstalled   => "Program Not Installed",
            Self::CommandFailed         => "Command Failed",
            Self::CountryFailed         => "Counry Failed",
            Self::InputFailed           => "Input Failed",
        }
    }
}

fn main() -> Result<(), &'static str> {
    // Command | modifier | modifier | Result<Child> | Child | Result<ExitStatus> | ExitStatus | bool
    if !Command::new("pacman")
        .arg("-Qq")
        .arg("cyberghostvpn")
        .spawn()
        .expect("cyberghostvpn is not installed")
        .wait()
        .expect("check wait failed")
        .success() {
            return Err(Status::ProgramNotInstalled.as_str());
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
                    "connect"   => {vpn.arg("--traffic").arg("--connect").arg("--country-code");},
                    _           => return Err(Status::InputFailed.as_str()),
                }
            },
            2 => {if ar.len() == 2 {vpn.arg(&ar);} else {return Err(Status::CountryFailed.as_str());}},
            3 => {vpn.arg("--city").arg(format!("'{}'", ar));},
            _ => {return Err(Status::InputFailed.as_str());}
        }
    }

    if vpn.spawn()
        .expect("process failed")
        .wait()
        .expect("Wait error")
        .success() {
            return Ok(());} else {return Err(Status::CommandFailed.as_str());
        }
}
