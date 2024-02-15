mod paplay;
mod stt;

use crate::paplay::paplay;
use glib::Continue;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};
use single_instance::SingleInstance;
use stt::nerd_dictation::NerdDictation;

#[derive(Debug)]
pub struct App {
    pub button: Button,
}

impl App {
    fn new(app: &Application, nerd_dictation: NerdDictation) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Clavier Vocale")
            .border_width(10)
            .default_width(300)
            .default_height(70)
            .build();

        window.set_keep_above(true);

        let button = Button::with_label("Clavier Vocale");
        button.connect_clicked(move |_button| {
            changer_position(&nerd_dictation);
        });
        window.add(&button);

        window.show_all();

        let copy_button = button.clone();

        glib::timeout_add_local(1000, move || {
            if nerd_dictation.is_running() {
                if nerd_dictation.is_suspended() {
                    copy_button.set_label("reprendre");
                } else {
                    copy_button.set_label("stop");
                }
            } else {
                copy_button.set_label("start");
            }

            Continue(true)
        });

        Self { button }
    }
}

fn changer_position(nerd_dictation: &NerdDictation) {
    if nerd_dictation.is_running() {
        println!("is running");
        if nerd_dictation.is_suspended() {
            println!("is suspended");
            nerd_dictation.resume();
            paplay("/usr/share/sounds/LinuxMint/stereo/dialog-question.ogg");
        } else {
            println!("is not suspended");
            nerd_dictation.suspend();
        }
    } else {
        println!("not is running");
        nerd_dictation.start();
        paplay("/usr/share/sounds/LinuxMint/stereo/dialog-question.ogg");
    }
}

fn main() {
    let nerd_dictation = NerdDictation::new();
    changer_position(&nerd_dictation);
    let instance = SingleInstance::new("clavier-vocale").unwrap();
    if !instance.is_single() {
        println!("une instance est déjà en route");
        return;
    }

    let application = Application::builder()
        .application_id("com.example.stt")
        .build();

    application.connect_activate(move |app| {
        let _app = App::new(app, nerd_dictation);
    });

    application.run();
}
