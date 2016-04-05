extern crate notify_rust;
extern crate ini;

use ini::Ini;
use notify_rust::Notification;
use notify_rust::server::NotificationServer;
use std::process::Command;
use std::env;

fn run_command(notifier:&String, notification:&Notification) {
    let ref summary = notification.summary;
    let ref body = notification.body;
    Command::new(notifier).arg(summary).arg(body).status().unwrap_or_else(|e| {
        panic!("Fail: {}", e)
    });
}

fn main() {
    let mut server = NotificationServer::new();
    let mut config_file_path = env::home_dir().unwrap();
    config_file_path.push(".config/over-here/over-here.conf");

    let conf = Ini::load_from_file(config_file_path.to_str().unwrap()).unwrap();
    let section = conf.section(Some("Notifier".to_owned())).unwrap();
    let notifier_exec = section.get("executable").unwrap();

    server.start( |notification|
                   run_command(notifier_exec, notification)
    );
}
