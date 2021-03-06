extern crate notify_rust;
extern crate ini;
extern crate nix;
extern crate dirs;

use ini::Ini;
use notify_rust::Notification;
use notify_rust::server::NotificationServer;
use std::process::Command;
use std::env;
use nix::unistd::daemon;

fn run_command(notifier:&String, notification:&Notification) {
    let ref summary = notification.summary;
    let ref body = notification.body;
    Command::new(notifier).arg(summary).arg(body).status().unwrap_or_else(|e| {
        panic!("Error: {}", e)
    });
}

fn ensure_display() {
    match env::var("DISPLAY") {
        Ok(_) => (),
        Err(_) => panic!("No display available")
    }
}

fn main() {
    ensure_display();

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let daemonized = daemon(false, true);
        assert!(daemonized.is_ok());
    }

    let mut server = NotificationServer::new();

    let home_dir = dirs::home_dir().unwrap();
    let config_file_path = format!("{}/.config/over-here/over-here.conf", home_dir.to_str().unwrap());

    let conf = Ini::load_from_file(config_file_path).expect(
        "No config file found");

    let section = conf.section(Some("Notifier".to_owned())).expect(
        "No section `Notifier` in config file");

    let notifier_exec = section.get("executable").expect(
        "No option `executable` in section `Notifier`");

    server.start( |notification|
                   run_command(notifier_exec, notification)
    );
}
