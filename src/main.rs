mod commands;
mod errors;
mod services;

use commands::handle_commands;
use services::generate_bash_completion;

fn main() {
    handle_commands();
    generate_bash_completion();
}
