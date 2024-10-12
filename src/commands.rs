use clap::{Parser, Subcommand};

use crate::services::*;

#[derive(Parser)]
#[command(name = "Services Control - CLI", version = "0.0.4", author = "Dev Fritz <fritzhenrique.dev@gmail.com>")]
#[command(about = "
Services Control - CLI
    
    This CLI works with a JSON file to store services and their commands.
    Is created a hidden folder to save the json with the services commands.
    The commands are executed in the terminal through the id that is saved in the file.")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Start a service.")]
    Start{
        #[arg(short, long, help = "Service id.", default_value = None)]
        id: Option<usize>,
        
        #[arg(short, long, help = "Service Name", default_value = None)]
        name: Option<String>,
    },
    
    #[command(about = "Stop a service.")]
    Stop{
        #[arg(short, long, help = "Service id", default_value = None)]
        id: Option<usize>,
        
        #[arg(short, long, help = "Service Name", default_value = None)]
        name: Option<String>,
    },
    
    #[command(about = "Restart a service.")]
    Restart{
        #[arg(short, long, help = "Service id", default_value = None)]
        id: Option<usize>,
        
        #[arg(short, long, help = "Service Name", default_value = None)]
        name: Option<String>,
    },
    
    #[command(about = "List all save services.")]
    List,
    
    #[command(about = "Adiciona a new service.")]
    Add{
        #[arg(short, long, help = "Service name.")]
        name: String,
        
        #[arg(long="start", help = "Command to start execute a service.", default_value = None)]
        start_commands: Option<String>,

        #[arg(long="stop", help = "Command to stop execute a service.", default_value = None)]
        stop_commands: Option<String>,

        #[arg(long="restart", help = "Command to restart execute a service.", default_value = None)]
        restart_commands: Option<String>,
    },
    
    #[command(about = "Remove a service by id and reorganize ids..", aliases=["rm"])]
    Remove{
        #[arg(short, long, help = "Service id", default_value = None)]
        id: Option<usize>,
        
        #[arg(short, long, help = "Service Name", default_value = None)]
        name: Option<String>,
    },
    
    #[command(about = "Edit a service.")]
    Edit{
        #[arg(short, long, help = "Service id")]
        id: usize,
        
        #[arg(short, long, help = "Service name.", default_value = None)]
        name: Option<String>,
        
        #[arg(long="start", help = "Command to start execute a service.", default_value = None)]
        start_commands: Option<String>,

        #[arg(long="stop", help = "Command to stop execute a service.", default_value = None)]
        stop_commands: Option<String>,

        #[arg(long="restart", help = "Command to restart execute a service.", default_value = None)]
        restart_commands: Option<String>,
    },
}

pub fn handle_commands() {
    let cli = Cli::parse();

    match cli.command {
        
        Commands::Start{id, name} => {
            execute(id, name, 1);
        },
        
        Commands::Stop{id, name} => {
            execute(id, name, 2);
        },
        
        Commands::Restart{id, name} => {
            execute(id, name, 3);
        },
        
        Commands::List => {
            list_services();
        },
        
        Commands::Add{name, start_commands, stop_commands, restart_commands} => {
            add_service(name, start_commands, stop_commands, restart_commands);
        },
        
        Commands::Remove{id, name} => {
            remove_service(id, name);
        },
        
        Commands::Edit{id, name, start_commands, stop_commands, restart_commands} => {
            edit_service_in_json(id, name, start_commands, stop_commands, restart_commands);        
        },

    }
}
