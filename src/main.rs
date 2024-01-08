mod cli;
mod stt;

use stt::nerd_dictation::NerdDictation;

fn main() {
    let nerd_dictation = NerdDictation::new();

    if nerd_dictation.is_running() {
        if nerd_dictation.is_suspended() {
            println!("resume");
            nerd_dictation.resume();
        } else {
            println!("suspend");
            nerd_dictation.suspend();
        }
    } else {
        println!("start");
        nerd_dictation.start();
    }
}
