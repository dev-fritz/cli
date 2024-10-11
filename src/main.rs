mod commands;
mod errors;
mod services;

use commands::handle_commands;

fn main() {
    handle_commands();
}
