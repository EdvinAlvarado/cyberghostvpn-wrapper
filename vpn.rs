use std::process::Command;

enum Status {ProgramNotInstalled=1, CommandFailed=2, CountryFailed=3, InputFailed=4}

fn main() -> Result<(), i32> {
    // Command | modifier | modifier | Result<Child> | Child | Result<ExitStatus> | ExitStatus | bool
    if !Command::new("pacman").arg("-Qq").arg("cyberghostvpn").spawn().expect("cyberghostvpn is not installed").wait().expect("check wait failed").success() {return Err(Status::ProgramNotInstalled as i32);}
    
    let mut vpn = Command::new("cyberghostvpn");
    let arg_vec: Vec<String> = std::env::args().collect();

    for (i, ar) in arg_vec.iter().enumerate() {
        match i {
            0 => {},
            1 => {
                match ar.as_str() {
                    "stop"      => {vpn.arg("--stop");},
                    "help"      => {vpn.arg("--stop");},
                    "status"    => {vpn.arg("--status");},
                    "connect"   => {vpn.arg("--traffic").arg("--connect").arg("--country-code");},
                    _           => return Err(Status::InputFailed as i32),
                }
            },
            2 => {if ar.len() == 2 {vpn.arg(&ar);} else {return Err(Status::CountryFailed as i32);}},
            3 => {vpn.arg("--city").arg(format!("'{}'", ar));},
            _ => {return Err(Status::InputFailed  as i32);}
        }
    }
    if vpn.spawn().expect("process failed").wait().expect("Wait error").success() {return Ok(());} else {return Err(Status::CommandFailed as i32);}
}
