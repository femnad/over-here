extern crate notify_rust;
extern crate ini;
extern crate nix;

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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let daemonized = daemon(false, true);
        assert!(daemonized.is_ok());
    }

    let mut server = NotificationServer::new();
    let mut config_file_path = env::home_dir().unwrap();
    config_file_path.push(".config/over-here/over-here.conf");

    let conf = Ini::load_from_file(
        config_file_path.to_str().unwrap()).expect(
        "No config file found");

    let section = conf.section(Some("Notifier".to_owned())).expect(
        "No section `Notifier` in config file");

    let notifier_exec = section.get("executable").expect(
        "No option `executable` in section `Notifier`");

    server.start( |notification|
                   run_command(notifier_exec, notification)
    );
}
