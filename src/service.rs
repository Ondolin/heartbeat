use std::env;
use std::time::{Duration, Instant};
use pling::Telegram;

pub struct ServiceInfo {
    pub last_heartbeat: Instant,
    pub is_offline: bool,
    pub timeout: Duration,
    pub name: String,
    notifier: Notifier,
}

impl ServiceInfo {

    pub fn new(name: String, notifier: Notifier, timeout: Option<u64>) -> ServiceInfo {
        ServiceInfo {
            last_heartbeat: Instant::now(),
            is_offline: false,
            timeout: match timeout {
                Some(t) => Duration::from_secs(t),
                None => Duration::from_secs(env::var("DEFAULT_TIMEOUT").unwrap().parse::<u64>().unwrap()),
            },
            notifier,
            name,
        }
    }

    pub fn is_timeout(&self) -> bool {
        self.last_heartbeat.elapsed() > self.timeout
    }

    pub fn make_dead(&mut self) {
        self.is_offline = true;
        self.notifier.send_message(format!("{} is dead", self.name));
    }

    pub fn notify_online(&self) {
        self.notifier.send_message(format!("{} is back online!", self.name));
    }

    pub fn notify_registered(&self) {
        self.notifier.send_message(format!("{} has registered!", self.name));
    }

}

pub struct Notifier {
    telegram: Option<Telegram>,
}

impl Notifier {
    fn send_message(&self, message: String) {
        if self.telegram.is_some() {
            self.telegram.as_ref().unwrap().send_sync(&*message).unwrap();
        }
    }

    // TODO implement this correctly
    pub fn new() -> Notifier {
        Notifier {
            telegram: Some(pling::Telegram::from_env().unwrap()),
        }
    }
}