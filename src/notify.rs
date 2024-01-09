use notify_rust::Notification;

pub fn notify(message: &str) {
    Notification::new()
        .summary("stt-clavier")
        .body(message)
        .timeout(1)
        .show()
        .unwrap();
}
