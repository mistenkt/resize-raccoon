use miow::pipe::NamedPipeBuilder;
use regex::Regex;
use std::io::Read;
use std::sync::{Arc, Mutex};

use crate::debug_log;
use crate::operations::profile::Profile;
use crate::operations::window_manager;

pub fn listener(profiles: Arc<Mutex<Vec<Profile>>>) {
    let pipe_name = r"\\.\pipe\resize-raccoon";
    let command_regex = Regex::new(r#"("[^"]+"|\S+)"#).unwrap(); // Regex for splitting arguments

    loop {
        // Create a named pipe server
        let mut server = NamedPipeBuilder::new(pipe_name)
            .inbound(true)
            .outbound(true)
            .first(true)
            .create()
            .expect("Unable to create named pipe");

        debug_log!("Named pipe server waiting for connection...");

        // Wait for a client to connect
        server.connect().expect("Failed to wait for client");

        let mut buffer = [0; 256];
        let bytes_read = server.read(&mut buffer).expect("Failed to read from pipe");
        let command_string = String::from_utf8_lossy(&buffer[..bytes_read]);

        // Split the command string into command and arguments
        let tokens: Vec<String> = command_regex
            .captures_iter(&command_string)
            .map(|cap| {
                cap[1].to_string().trim_matches('"').to_string() // Convert &str to String
            })
            .collect();

        if tokens.is_empty() {
            continue; // Handle empty command or invalid input
        }

        let command = &tokens[0];
        let args = &tokens[1..];

        debug_log!("Received command: {}", command);
        debug_log!("Received arguments: {:?}", args);

        if command == "apply-profile" {
            let profiles_guard = profiles.lock().unwrap();
            let current_profiles = profiles_guard.clone();

            drop(profiles_guard);
            let profile_name = &args[0].to_lowercase();

            if let Some(profile) = current_profiles
                .iter()
                .find(|p| p.name.to_lowercase() == *profile_name)
            {
                let _ = window_manager::apply_profile(&profile, None);
            } else {
                debug_log!("Profile not found: {}", profile_name)
            }
        }
    }
}
