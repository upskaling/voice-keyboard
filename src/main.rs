mod cli;
mod notify;
mod stt;

use notify::notify;
use stt::nerd_dictation::NerdDictation;

fn main() {
    let nerd_dictation = NerdDictation::new();

    if nerd_dictation.is_running() {
        if nerd_dictation.is_suspended() {
            println!("resume");
            notify("resume");
            nerd_dictation.resume();
        } else {
            println!("suspend");
            notify("suspend");
            nerd_dictation.suspend();
        }
    } else {
        println!("start");
        notify("start");
        nerd_dictation.start();
    }
}
