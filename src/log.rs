use btui::Terminal;
use btui::{
    effects::*,
    print::{fg, sp},
};
use chrono::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

/// the type of log
pub enum LogType {
    Warning,
    Error,
    Log,
}

/// A logger that logs to stderr and/or a file
pub struct Logger {
    term: Option<Terminal>,
    file: Option<File>,
}

impl Logger {
    /// create a new logger
    pub fn new() -> Logger {
        Logger {
            term: None,
            file: None,
        }
    }

    /// set the log file to log to for the logger
    /// # Returns
    /// A result that is an error variant if the file opening process fails
    pub fn set_logfile(&mut self, fname: &str) -> Result<(), std::io::Error> {
        self.file = match OpenOptions::new().append(true).create(true).open(fname) {
            Ok(n) => Some(n),
            Err(e) => {
                return Err(e);
            }
        };
        Ok(())
    }

    /// set the terminal to log to for a logger
    pub fn set_term(&mut self, term: Terminal) {
        self.term = Some(term);
    }

    /// log a message with a specific type
    pub fn log<T: std::fmt::Display>(&mut self, msg: T, msgtype: LogType) {
        match msgtype {
            LogType::Warning => {
                let cur_time: DateTime<Local> = Local::now();
                if let Some(n) = &self.term {
                    match n.eprintln(format!(
                        "[{}]: {}Warning: {}{}{}",
                        cur_time,
                        fg(Color::Yellow),
                        fg(Color::White),
                        msg,
                        sp(Special::Reset)
                    )) {
                        _ => (),
                    }
                }
                if let Some(n) = &mut self.file {
                    let _ = n.write(
                        format!("[{}]: Warning: {}\n", cur_time, msg)
                            .as_str()
                            .as_bytes(),
                    );
                    let _ = n.flush();
                }
            }
            LogType::Error => {
                let cur_time: DateTime<Local> = Local::now();
                if let Some(n) = &self.term {
                    match n.eprintln(format!(
                        "[{}]: {}Error: {}{}{}",
                        cur_time,
                        fg(Color::Red),
                        fg(Color::White),
                        msg,
                        sp(Special::Reset)
                    )) {
                        _ => (),
                    }
                }
                if let Some(n) = &mut self.file {
                    let _ = n.write(
                        format!("[{}]: Error: {}\n", cur_time, msg)
                            .as_str()
                            .as_bytes(),
                    );
                    let _ = n.flush();
                }
            }
            LogType::Log => {
                let cur_time: DateTime<Local> = Local::now();
                if let Some(n) = &self.term {
                    match n.eprintln(format!("[{}]: {}", cur_time, msg)) {
                        _ => (),
                    }
                }
                if let Some(n) = &mut self.file {
                    let _ = n.write(format!("[{}]: {}\n", cur_time, msg).as_str().as_bytes());
                    let _ = n.flush();
                }
            }
        }
    }
}
