use std::process::Command;

enum Status {OK=0, ProgramNotInstalled=1, StopFailed=2, WaitFailed=3, StatusFailed=4, CountryFailed=5, CityFailed=6, NoArguments=7, ReachEnd=8}

fn main() -> Result<(), i32> {
    // Command | modifier | modifier | Result<Child> | Child | Result<ExitStatus> | ExitStatus | bool
    if !Command::new("pacman").arg("-Qq").arg("cyberghostvpn").spawn().expect("cyberghostvpn is not installed").wait().expect("check wait failed").success() {return Err(Status::ProgramNotInstalled as i32)}
    
    let mut vpn = Command::new("cyberghostvpn");
    let mut arg_vec: Vec<String> = std::env::args().collect();

    for ar in 0..arg_vec.len() {
        match ar {
            0 => {},
            1 => {
                if arg_vec[ar] == "stop" {if vpn.arg("--stop").spawn().expect("stop failed").wait().expect("stop wait failed").success() {return Ok(());} else {return Err(Status::StopFailed as i32);}}
                else if arg_vec[ar] == "help" {if vpn.arg("--help").spawn().expect("help failed").wait().expect("help wait failed").success() {return Ok(());} else {return Err(Status::WaitFailed as i32);}}
                else if arg_vec[ar] == "status" {if vpn.arg("--status").spawn().expect("status failed").wait().expect("status wait failed").success() {return Ok(());} else {return Err(Status::StatusFailed as i32);}}
                else if arg_vec[ar] == "connect" {vpn.arg("--traffic").arg("--connect").arg("--country-code");}
            },
            2 => {
                if arg_vec[ar].len() == 2 {
                    vpn.arg(&arg_vec[ar]);
                    if ar+1 == arg_vec.len() {if vpn.spawn().expect("connect country failed").wait().expect("connect country wait failed").success() {return Ok(());} else {return Err(Status::CountryFailed as i32);}}
                }
            } ,
            3 => {
                if vpn.arg("--city").arg(format!("'{}'", arg_vec[ar])).spawn().expect("commect city failed").wait().expect("connct city wait failed").success() {return Ok(());} else {return Err(Status::CityFailed as i32);};
            },
            _ => {return Err(Status::NoArguments as i32);}
        }
    }

    Err(Status::ReachEnd as i32)
}
