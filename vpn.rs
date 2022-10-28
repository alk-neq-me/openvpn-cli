#![allow(unused)]
use std::process::Command;
use std::env;
use std::fs;
use std::io::prelude::*;



#[derive(Debug)]
enum Error {
    NotFoundError,
}

struct OpenVpn {
    country: String,
    id: String,
    mode: String,
}

impl OpenVpn {
    fn display_list(&self, path_country: &String) -> Result<(), Error> {
        self.show_status();
        
        if !fs::metadata(path_country).is_ok() { Err(Error::NotFoundError) } else {
            let paths = fs::read_dir(path_country)
                .unwrap()
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .collect::<Vec<_>>();
            for i in paths {
                let file = i.display().to_string();
                let list: Vec<&str> = file
                    .split(".protonvpn")
                    .collect::<Vec<&str>>()[0]
                    .split(path_country)
                    .collect();
                println!("🌎  {:?}", list[1].replace("-free-", " "));
            }
            Ok(())
        }
    }
    
    fn show_status(&self) -> std::io::Result<()> {
        if self.mode == "List" {
            println!("{mode} mode is \x1b[92mActivated\x1b[0m!", mode=self.mode);
        } else {
            println!("{mode} mode is \x1b[92mActivated\x1b[0m!", mode=self.mode);
            println!("Country: {c}\nCode: {id}", c=self.country, id=self.id);
    
            write!(std::io::stdout(), "Press ENTER to continue...\n").unwrap();
            std::io::stdout().flush().unwrap();
    
            std::io::stdin().read(&mut [0u8]).unwrap();
        }
        Ok(())
    }
    
    fn status(&mut self, path_country: &String, path_auth: &String) -> Result<(), Error> {
        if self.id.len() == 1 { 
            self.id = "0".to_owned() + &self.id;
        }
        
        let country = format!("{path_country}{}-free-{}.protonvpn.net.udp.ovpn", self.country, self.id);
        
        self.show_status();
        
        if !fs::metadata(&country).is_ok() { 
            Err(Error::NotFoundError) 
        } else if !fs::metadata(path_auth).is_ok() { 
            Err(Error::NotFoundError) 
        } else {
            if cfg!(target_os="windows") {
                Command::new("cmd")
                    .args(["/C", "echo not supported!"])
                    .status()
                    .expect("failed to execute process.")
            } else {
                Command::new("sh")
                    .arg("-c")
                    .arg(format!(r#"
                        sudo openvpn --config {country} --auth-user-pass {path_auth}
                    "#))
                    .status()
                    .expect("failed to execute process.")
            };
            Ok(())
        }
    }
}


fn main() -> Result<(), Error> {
    let vpn: Vec<String> = env::args().collect();
    
    // Configure
    let path_config = String::from("./config/");
    let path_countries = String::from(format!("{}country/", path_config));
    let path_auth = String::from(format!("{}auth", path_config));
    
    // Authentication
    // let auth = fs::read_to_string(path_auth).expect("Should have been able to read the file.");
    
    // Options
    let cty_opt = &vpn.iter().position(|arg| arg == "-c");
    let idx_opt = &vpn.iter().position(|arg| arg == "-i");
    let lst_opt = &vpn.iter().position(|arg| arg == "--list");
    
    // Default
    let mut app = OpenVpn {
        country: String::from("jp"),
        id: String::from("1"),
        mode: String::from("Fast")
    };
    
    // Get Country or Not
    match cty_opt {
        Some(v) => {
            app.country = vpn[v+1].to_string();
            app.mode = String::from("Custom");
        },
        None => {}
    }
    
    // Get Index or Not
    match idx_opt {
        Some(v) => {
            app.id = vpn[v+1].to_string();
            app.mode = String::from("Custom");
        },
        None => {}
    }
    
    // List Avaliable Countries And Id(s)
    match lst_opt {
        Some(v) => {
            app.mode = String::from("List");
            OpenVpn::display_list(&app, &path_countries)
        },
        None => {
            app.status(&path_countries, &path_auth)
        }
    }
}
