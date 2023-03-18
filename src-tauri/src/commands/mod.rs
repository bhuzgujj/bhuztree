pub mod errors;

use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;
use std::process::{Child, Command};
use error_stack::{IntoReport, Result, ResultExt};
use errors::InvalidCommandError;
use std::process::Stdio;

#[derive(Debug)]
pub struct CliCommand {
    pub command: String,
    pub flags: Vec<String>,
    stdout: Option<u8>,
}

impl CliCommand {
    pub fn new(command: String, flags: Vec<String>) -> Self {
        Self { command, flags, stdout: None }
    }

    fn exec(&mut self, repositories_locations: Vec<PathBuf>, args: Vec<&String>) -> Result<HashMap<String, Child>, InvalidCommandError> {
        let mut processes: HashMap<String, Child> = HashMap::new();

        for branch in &repositories_locations {
            processes.insert(branch.clone().to_str().unwrap().to_string(), Command::new("git")
                .args(&args)
                .current_dir(branch)
                .stdout(self.get_std_type()?)
                .spawn()
                .into_report()
                .change_context(InvalidCommandError { message: format!("git {:?} in path {:?} crash at function \"exec\"", args, branch) })
                .attach_printable_lazy(|| format!("Failed to execute command: {:?}", args))?,
            );
        }

        self.stdout = None;
        Ok(processes)
    }

    #[allow(dead_code)]
    pub fn execute(&mut self, repositories_locations: Vec<PathBuf>) -> Result<(), InvalidCommandError> {
        let cmd = self.command.clone();
        let mut command = vec![&cmd];
        let flags = self.flags.clone();
        flags.iter().for_each(|flag| command.push(flag));
        Self::print_branches(&mut command, &repositories_locations);

        self.stdout = Some(0);
        for (name, process) in &mut self.exec(repositories_locations, command)? {
            // TODO: handle status codes
            let _ = process.wait().into_report()
                .change_context(InvalidCommandError { message: format!("git {:?} in path {:?} crash at function \"execute\"", cmd, name) })
                .attach_printable_lazy(|| format!("Failed to wait for process"))?;
        }
        Ok(())
    }

    pub fn execute_catching_stdout(&mut self, repositories_locations: Vec<PathBuf>) -> Result<HashMap<String, String>, InvalidCommandError> {
        let binding = self.command.clone();
        let mut command = vec![&binding];
        let flags = self.flags.clone();
        flags.iter().for_each(|flag| command.push(flag));

        let mut stdout: HashMap<String, String> = HashMap::new();

        self.stdout = Some(1);
        for (paths, process) in &mut self.exec(repositories_locations, command.clone())? {
            // TODO: handle status codes
            let _ = &process.wait().into_report()
                .change_context(InvalidCommandError {
                    message: format!("git {:?} in path {:?} crash at function \"execute_catching_stdout\"", command, paths.clone())
                })
                .attach_printable_lazy(|| format!("Failed to wait for process"))?;
            if let Some(output) = process.stdout.take() {
                let output_as_bytes: Vec<u8> = output.bytes()
                    .filter(|item| { return item.is_ok(); })
                    .map(|item| { return item.unwrap(); })
                    .collect();
                stdout.insert(paths.clone(), String::from_utf8(output_as_bytes).unwrap());
            }
        }
        Ok(stdout)
    }

    #[allow(dead_code)]
    fn print_branches(command: &mut Vec<&String>, branches: &Vec<PathBuf>) {
        for branch in branches {
            let branche = branch.clone().to_str().unwrap().chars().skip(4).collect::<String>();
            println!("Executing command: {:?} in {:?}", command, branche);
        }
    }

    fn get_std_type(&mut self) -> Result<Stdio, InvalidCommandError> {
        if let Some(outy) = self.stdout {
            match outy {
                0 => Ok(Stdio::inherit()),
                1 => Ok(Stdio::piped()),
                _ => Err(InvalidCommandError { message: "Unknown stdout".to_string() }.into())
            }
        } else {
            Err(InvalidCommandError { message: "Error unpacking stdout".to_string() }.into())
        }
    }
}
