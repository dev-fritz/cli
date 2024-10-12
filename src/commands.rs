use clap::{Parser, Subcommand};

use crate::services::*;

#[derive(Parser)]
#[command(name = "Services Control - CLI", version = "0.0.3", author = "Dev Fritz <fritzhenrique.dev@gmail.com>")]
#[command(about = "
Services Control - CLI
    
    Essa CLI funciona com um arquivo JSON para armazenar os serviços e seus comandos.
    É criado uma pasta oculta para salvar o json com os comandos dos serviços. 
    Os comandos são executados no terminal através do id que é salvo no arquivo.")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Inicia um serviço utilizando o comando salvo, se houver um.")]
    Start{
        #[arg(short, long, help = "Id do serviço.", default_value = None)]
        id: Option<usize>,
        
        #[arg(short, long, help = "Nome do Serviço", default_value = None)]
        name: Option<String>,
    },
    
    #[command(about = "Para um serviço utilizando o comando salvo, se houver um.")]
    Stop{
        #[arg(short, long, help = "Id do serviço.", default_value = None)]
        id: Option<usize>,
        
        #[arg(short, long, help = "Nome do Serviço", default_value = None)]
        name: Option<String>,
    },
    
    #[command(about = "Restart um serviço utilizando o comando salvo, se houver um.")]
    Restart{
        #[arg(short, long, help = "Id do serviço.", default_value = None)]
        id: Option<usize>,
        
        #[arg(short, long, help = "Nome do Serviço", default_value = None)]
        name: Option<String>,
    },
    
    #[command(about = "Lista todos os serviços salvos.")]
    List,
    
    #[command(about = "Adiciona um novo serviço.")]
    Add{
        #[arg(short, long, help = "Nome do serviço.")]
        name: String,
        
        #[arg(long="start", help = "Comando para execução do serviço.", default_value = None)]
        start_commands: Option<String>,

        #[arg(long="stop", help = "Comando para parar a execução do serviço.", default_value = None)]
        stop_commands: Option<String>,

        #[arg(long="restart", help = "Comando para reiniciar o serviço.", default_value = None)]
        restart_commands: Option<String>,
    },
    
    #[command(about = "Remove um serviço e reorganiza os ids dos serviços.", aliases=["rm"])]
    Remove{
        #[arg(short, long, help = "ID do serviço que será removido")]
        id: usize,
    },
    
    #[command(about = "Edita um serviço, salvando somente os campos enviados.")]
    Edit{
        #[arg(short, long, help = "Id do serviço.")]
        id: usize,
        
        #[arg(short, long, help = "Nome do serviço.", default_value = None)]
        name: Option<String>,
        
        #[arg(long="start", help = "Comando para execução do serviço.", default_value = None)]
        start_commands: Option<String>,

        #[arg(long="stop", help = "Comando para parar a execução do serviço.", default_value = None)]
        stop_commands: Option<String>,

        #[arg(long="restart", help = "Comando para reiniciar o serviço.", default_value = None)]
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
        
        Commands::Remove{id} => {
            remove_service(id);
        },
        
        Commands::Edit{id, name, start_commands, stop_commands, restart_commands} => {
            edit_service_in_json(id, name, start_commands, stop_commands, restart_commands);        
        },

    }
}
