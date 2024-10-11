use crate::errors::ServiceError;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write, Read};
use std::process::{Command, Output};
use std::env;

use crossterm::{execute, style::{Color, Print, ResetColor, SetForegroundColor}, cursor::MoveTo};
use crossterm::terminal::{Clear, ClearType};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
struct Service {
    id: usize,
    name: String,
    start_command: Option<String>,
    stop_command: Option<String>,
    restart_command: Option<String>,
}

fn get_user_name() -> String {
    if cfg!(target_os = "windows") {
        env::var("USERNAME").expect("Failed to get username")
    } else {
        env::var("USER").expect("Failed to get username")
    }
}

fn get_path() -> String {
    let user = get_user_name();
    let path = format!("/home/{user}/.cli/services.json");
    
    path.to_string()
}

fn create_dir() {
    let user = get_user_name();
    let path = format!("/home/{user}/.cli");
    std::fs::create_dir_all(path).expect("Failed to create directory");
}

fn create_json_file() -> Result<File, std::io::Error> {
    create_dir();
    File::create(get_path())
}

fn read_services_from_json() -> Result<Vec<Service>, ServiceError> {
    let path = get_path();

    match File::open(&path) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).map_err(ServiceError::Io)?;

            let services: Vec<Service> = serde_json::from_str(&contents).map_err(ServiceError::SerdeJson)?;
            Ok(services)
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
            println!("Nenhum arquivo de serviços encontrado. Parece que você ainda não adicionou nenhum serviço.");
            Ok(vec![])
        }
        Err(e) => {
            println!("Ocorreu um erro ao tentar acessar o arquivo de serviços: {}", e);
            Err(ServiceError::Io(e))
        }
    }
}

fn write_service_to_json(service: Service) {
    let mut services = read_services_from_json().expect("Failed to read services");
    services.push(service);
    
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(get_path())
        .unwrap_or_else(|_| create_json_file().expect("Failed to create file"));
    
    let mut buffer = BufWriter::new(file);
    let json_data = serde_json::to_string_pretty(&services).expect("Failed to serialize services");
    
    buffer.write_all(json_data.as_bytes()).expect("Failed to write data to JSON file");
    buffer.flush().expect("Failed to flush buffer");
}

fn reorganize_ids() {
    let mut services = read_services_from_json().expect("Failed to read services");
    
    for (index, service) in services.iter_mut().enumerate() {
        service.id = index + 1;
    }
    
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(get_path())
        .unwrap_or_else(|_| create_json_file().expect("Failed to create file"));
    
    let mut buffer = BufWriter::new(file);
    let json_data = serde_json::to_string_pretty(&services).expect("Failed to serialize services");
    
    buffer.write_all(json_data.as_bytes()).expect("Failed to write data to JSON file");
    buffer.flush().expect("Failed to flush buffer");
}

pub fn add_service(
    name: String, 
    start_command: Option<String>, 
    stop_command: Option<String>, 
    restart_command: Option<String>
) {
    let services = read_services_from_json().expect("Failed to read services");
    
    let service = Service {
        id: services.len() + 1,
        name,
        start_command,
        stop_command,
        restart_command
    };
    
    write_service_to_json(service);
    println!("Service added.");
}

pub fn list_services() {
    let services = read_services_from_json().expect("Failed to read services");

    if services.is_empty() {
        println!("No services found.");
        return;
    }
    execute!(std::io::stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();

    for service in services {
        let mut max_len = service.name.len();

        let start_command = service.start_command.clone().unwrap_or_else(|| "N/A".to_string());
        let stop_command = service.stop_command.clone().unwrap_or_else(|| "N/A".to_string());
        let restart_command = service.restart_command.clone().unwrap_or_else(|| "N/A".to_string());

        max_len = max_len.max(start_command.len() + 15);
        max_len = max_len.max(stop_command.len() + 14);
        max_len = max_len.max(restart_command.len() + 17);

        let width = max_len + 10;
        let padding = 2;

        execute!(
            std::io::stdout(),
            SetForegroundColor(Color::DarkGrey),
            Print(format!("┌{}┐\n", "─".repeat(width - 2))),
            ResetColor
        ).unwrap();

        execute!(
            std::io::stdout(),
            SetForegroundColor(Color::Yellow),
            Print(format!("│{:padding$}ID: {:<width$}│\n", "", service.id, width = width - padding - 6, padding = padding)),
            SetForegroundColor(Color::Cyan),
            Print(format!("│{:padding$}Name: {:<width$}│\n", "", service.name, width = width - padding - 8, padding = padding)),
            SetForegroundColor(Color::Green),
            Print(format!("│{:padding$}Start Command: {:<width$}│\n", "", start_command, width = width - padding - 17, padding = padding)),
            SetForegroundColor(Color::Red),
            Print(format!("│{:padding$}Stop Command: {:<width$}│\n", "", stop_command, width = width - padding - 16, padding = padding)),
            SetForegroundColor(Color::Magenta),
            Print(format!("│{:padding$}Restart Command: {:<width$}│\n", "", restart_command, width = width - padding - 19, padding = padding)),
            ResetColor
        ).unwrap();

        execute!(
            std::io::stdout(),
            SetForegroundColor(Color::DarkGrey),
            Print(format!("└{}┘\n", "─".repeat(width - 2))),
            ResetColor
        ).unwrap();
    }
}

pub fn edit_service_in_json(
    id: usize,
    name: Option<String>,
    start_command: Option<String>, 
    stop_command: Option<String>, 
    restart_command: Option<String>
) {
    let mut services = read_services_from_json().expect("Failed to read services");
    
    if let Some(service) = services.iter_mut().find(|s| s.id == id) {
        if let Some(new_name) = name {
            service.name = new_name;
        }
        if start_command != None { service.start_command = start_command; };
        if stop_command != None { service.stop_command = stop_command; };
        if restart_command != None { service.restart_command = restart_command; };  
    }
    
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(get_path())
        .expect("Failed to open JSON file");
    
    let mut buffer = BufWriter::new(file);
    let json_data = serde_json::to_string_pretty(&services).expect("Failed to serialize services");
    
    buffer.write_all(json_data.as_bytes()).expect("Failed to write data to JSON file");
    buffer.flush().expect("Failed to flush buffer");
    
    println!("Service edited.");
}

pub fn remove_service(id: usize) {
    let mut services = read_services_from_json().expect("Failed to read services");
    
    services.retain(|s| s.id != id);
    
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(get_path())
        .expect("Failed to open JSON file");
    
    let mut buffer = BufWriter::new(file);
    let json_data = serde_json::to_string_pretty(&services).expect("Failed to serialize services");
    
    buffer.write_all(json_data.as_bytes()).expect("Failed to write data to JSON file");
    buffer.flush().expect("Failed to flush buffer");
    reorganize_ids();
    
    println!("Service removed.");
}

pub fn execute(id: usize, command_type: i8) {
    let services = read_services_from_json().expect("Failed to read services");
    if services.is_empty() {
        println!("Nenhum serviço encontrado. Crie um serviço antes de executar comandos.");
        return;
    }
    let service = services.iter().find(|s| s.id == id).expect("Servidor não encontrado");
    let output: Output;
    
    match command_type {
        1 => {
            if let Some(start_command) = &service.start_command {
                println!("Iniciando serviço '{}' com comando: '{}'", service.name, start_command);
                output = Command::new("sh")
                    .arg("-c")
                    .arg(start_command)
                    .output()
                    .expect("Falha ao executar o comando");
            } else {
                println!("Comando de início não implementado.");
                return;
            }
        },
        2 => {
            if let Some(stop_command) = &service.stop_command {
                println!("Parando serviço '{}' com comando: '{}'", service.name, stop_command);
                output = Command::new("sh")
                    .arg("-c")
                    .arg(stop_command)
                    .output()
                    .expect("Falha ao executar o comando");
            } else {
                println!("Comando de parada não implementado.");
                return;
            }
        },
        3 => {
            if let Some(restart_command) = &service.restart_command {
                println!("Reiniciando serviço '{}' com comando: '{}'", service.name, restart_command);
                output = Command::new("sh")
                    .arg("-c")
                    .arg(restart_command)
                    .output()
                    .expect("Falha ao executar o comando");
            } else {
                println!("Comando de reinício não implementado.");
                return;
            }
        },
        _ => {
            output = Command::new("sh")
                .arg("-c")
                .arg("echo 'Comando não reconhecido.'")
                .output()
                .expect("Falha ao executar o comando");
        }
    }

    if output.status.success() {
        println!("Comando executado com sucesso.\n {}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Erro na execução do comando: {}", String::from_utf8_lossy(&output.stderr));
    }
}