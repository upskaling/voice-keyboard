use psutil::process::{processes, Process, ProcessError};
use std::process::Command;

// nerd-dictation
// begin               Begin dictation.
// end                 End dictation.
// cancel              Cancel dictation.
// suspend             Suspend the dictation process.
// resume              Resume the dictation process.

#[derive(Debug, Copy, Clone)]
pub struct NerdDictation {}

/** récupérer le pid du processus */
fn get_pid(process: Result<Process, ProcessError>, process_name: &str) -> Option<u32> {
    let p = match process {
        Ok(p) => p,
        Err(_) => return None,
    };

    if p.name().unwrap_or_else(|_| "".to_string()) == process_name {
        return Some(p.pid());
    }

    None
}

/** récupérer les processus en pause */
fn get_pid_paused(process: Result<Process, ProcessError>, process_name: &str) -> Option<u32> {
    let p = match process {
        Ok(p) => p,
        Err(_) => return None,
    };

    if p.name().unwrap_or_else(|_| "".to_string()) == process_name
        && p.status().unwrap().to_string() == "T"
    {
        return Some(p.pid());
    }

    None
}

#[allow(dead_code)]
impl NerdDictation {
    pub fn new() -> Self {
        NerdDictation {}
    }

    /** tester si le programme a déjà été lancée */
    pub fn is_running(&self) -> bool {
        let processes = processes().unwrap().into_iter();

        !processes
            .filter_map(|p| get_pid(p, "nerd-dictation"))
            .collect::<Vec<u32>>()
            .is_empty()
    }

    /** tester si le programme est suspendu */
    pub fn is_suspended(&self) -> bool {
        let processes = processes().unwrap().into_iter();
        !processes
            .filter_map(|p| get_pid_paused(p, "nerd-dictation"))
            .collect::<Vec<u32>>()
            .is_empty()
    }

    /** démarrer le programme */
    pub fn start(&self) {
        if !self.is_running() {
            Command::new("nerd-dictation").arg("begin").spawn().unwrap();
        }
    }

    /** arrêter le programme */
    pub fn stop(&self) {
        if self.is_running() {
            Command::new("nerd-dictation")
                .arg("end")
                .output()
                .expect("nerd-dictation end failed");
        }
    }

    /** suspendre le programme */
    pub fn suspend(&self) {
        if self.is_running() {
            Command::new("nerd-dictation")
                .arg("suspend")
                .output()
                .expect("nerd-dictation suspend failed");
        }
    }

    /** reprendre le programme */
    pub fn resume(&self) {
        if self.is_running() {
            Command::new("nerd-dictation")
                .arg("resume")
                .output()
                .expect("nerd-dictation resume failed");
        }
    }
}
