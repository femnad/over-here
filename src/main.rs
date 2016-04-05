extern crate notify_rust;

use notify_rust::Notification;
use notify_rust::server::NotificationServer;
use std::process::Command;

fn run_command(notification:&Notification) {
    let ref summary = notification.summary;
    let ref body = notification.body;
    Command::new("dzenotify").arg(summary).arg(body).status().unwrap_or_else(|e| {
        panic!("Fail: {}", e)
    });
}

fn main() {
    let mut server = NotificationServer::new();
    server.start( |notification|
                   run_command(notification)
    );
}
