// Copyright (c) 2026 angrypig555
// SPDX-License-Identifier: MIT

use std::{env, io::{self, Error, ErrorKind::ConnectionAborted, Read, Write}, process::{Command, Stdio}};

fn install() -> std::io::Result<()> {
    print!("THIS WILL INSTALL AND CONFIGURE THE SERVER INCLUDING MODIFYING PASSWORDS AND OVERWRITING CONFIGURATIONS\nTHIS CANNOT BE UNDONE\nVERIFY THAT YOU ARE ON THE CORRECT SERVER\nContinue? [y/n]: ");
    let mut prompt_answer = String::new();
    io::stdout().flush()?;
    io::stdin().read_line(&mut prompt_answer)?;
    if prompt_answer.to_lowercase().trim() != "y" {
        return Err(Error::new(ConnectionAborted, "Installation has been aborted"))
    }
    println!("updating system...");
    let update_status = Command::new("dnf").arg("update").arg("-y").status()?;
    if !update_status.success() {
        panic!("Failed to update system");
    }
    println!("installing \"wordpress\" package");
    let wordpress_install_status = Command::new("dnf").arg("install").arg("-y").status()?;
    if !wordpress_install_status.success() {
        panic!("Failed to install wordpress");
    }
    println!("configuring mysql");
    let mut mysql_cmd = Command::new("mysql").arg("-u").arg("root").stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
    let sql_commands = r#"
    CREATE DATABASE wordpress ;
    CREATE USER 'wordpress' IDENTIFIED BY '
    "#;
    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("server management tool - smt");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("not enough arguments, see help for usage");
    }
    if args.len() > 2 {
        panic!("too many arguments, see help for usage");
    }
    if args[1] == "help" {
        println!("arguments for smt:\n help - displays the help menu\n install - installs the neccesary software (php, mysql, etc...)\n backup - dumps and backs up web and databases\n logs - compress all the logs into a downloadable file\n\nCopyright (c) 2026 angrypig555; Licensed under the MIT License")
    } else if args[1] == "install" {
        println!("install selected");
        install()?;
    } else if args[1] == "backup" {
        println!("backup selected");
    } else if args[1] == "logs" {
        println!("logs selected");
    }
    Ok(())
}
