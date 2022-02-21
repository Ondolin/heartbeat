use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use rocket::response::Redirect;
use rocket::response::status::NotFound;
use rocket::State;

use crate::service::{Notifier, ServiceInfo};

#[post("/report/<user>/<service_id>?<timeout>")]
pub fn report(user: String, service_id: String, timeout: Option<u64>, last_heartbeat: &State<Arc<Mutex<HashMap<String, ServiceInfo>>>>) {

    let mut hash_map = last_heartbeat.lock().unwrap();

    let service_name = format!("{}/{}", user, service_id);

    let service_info = hash_map.entry(service_name.clone()).or_insert_with(|| {
        let info = ServiceInfo::new(service_name, Notifier::new(), timeout);
        info.notify_registered();
        info
    });

    eprintln!("Got request from {}", service_info.name);

    // report that the service is online again 🎉
    if service_info.is_offline {
        service_info.notify_online();
    }

    service_info.last_heartbeat = Instant::now();
    service_info.is_offline = false;
}

#[get("/report/<user>/<service_id>")]
pub fn is_service_online(user: String, service_id: String, last_heartbeat: &State<Arc<Mutex<HashMap<String, ServiceInfo>>>>) -> Result<String, NotFound<String>> {

    let hash_map = last_heartbeat.lock().unwrap();

    return match hash_map.get(&format!("{}/{}", user, service_id)) {
        None => {
            Err(NotFound(format!("{}/{} is not registered", user, service_id)))
        },
        Some(service) => {
            if service.is_offline {
                Err(NotFound(format!("{}/{} is not offline", user, service_id)))
            } else {
                Ok(format!("Service {}/{} is online!", user, service_id))
            }
        }
    };

}

#[get("/online")]
pub fn online() -> &'static str {
    "I'm online!"
}

#[get("/")]
pub fn redirect_to_repo() -> Redirect {
    Redirect::to("https://github.com/ondolin/heartbeat")
}