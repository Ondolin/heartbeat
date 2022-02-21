#[macro_use] extern crate rocket;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{env, thread};
use std::thread::sleep;
use std::time::{Duration, Instant};
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::response::status::NotFound;

use rocket::State;

struct ServiceInfo {
    last_heartbeat: Instant,
    is_offline: bool,
    timeout: Duration,
}

#[post("/report/<user>/<service_id>?<timeout>")]
fn report(user: String, service_id: String, timeout: Option<u64>, last_heartbeat: &State<Arc<Mutex<HashMap<String, ServiceInfo>>>>) {
    let default_timeout: Duration = Duration::from_secs(env::var("DEFAULT_TIMEOUT").unwrap().parse::<u64>().unwrap());

    let mut hash_map = last_heartbeat.lock().unwrap();

    if !hash_map.contains_key(&format!("{}/{}", user, service_id)) {
        let notifier = pling::Telegram::from_env().unwrap();
        let message = format!("{}/{} has registered!", user, service_id);
        notifier.send_sync(&*message).unwrap();
    }

    let service_info = hash_map.entry(format!("{}/{}", user, service_id)).or_insert(ServiceInfo {
        last_heartbeat: Instant::now(),
        is_offline: false,
        timeout: match timeout {
            Some(timeout) => Duration::from_secs(timeout),
            None => default_timeout,
        },
    });

    eprintln!("Got request from {}/{}", user, service_id);

    // report that the service is online again 🎉
    if service_info.is_offline {
        let notifier = pling::Telegram::from_env().unwrap();
        let message = format!("{}/{} is back online!", user, service_id);
        notifier.send_sync(&*message).unwrap();
    }

    service_info.last_heartbeat = Instant::now();
    service_info.is_offline = false;
}

#[get("/report/<user>/<service_id>")]
fn is_service_online(user: String, service_id: String, last_heartbeat: &State<Arc<Mutex<HashMap<String, ServiceInfo>>>>) -> Result<String, NotFound<String>> {

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
fn online() -> &'static str {
    "I'm online!"
}

#[get("/")]
fn redirect_to_repo() -> Redirect {
    Redirect::to("https://github.com/ondolin/heartbeat")
}

#[launch]
fn rocket() -> _ {

    dotenv::dotenv().ok();

    let hash_map: HashMap<String, ServiceInfo> = HashMap::new();
    let services = Arc::new(Mutex::new(hash_map));

    let services_clone = services.clone();
    let services_rocket_clone = services.clone();

    // monitoring thread
    thread::spawn(move || loop {
        sleep(Duration::from_secs(env::var("POLL_RATE").unwrap().parse::<u64>().unwrap()));

        let notifiers = pling::Telegram::from_env().unwrap();

        services_clone.lock().unwrap().retain(|key, value|{

            let service_name = key.to_string();
            let service_info = value;

            if service_info.last_heartbeat.elapsed() > service_info.timeout && !service_info.is_offline {
                service_info.is_offline = true;
                notifiers.send_sync(&*format!("{} is dead", service_name)).unwrap();
            }

            true
        });


    });

    rocket::build().manage(services_rocket_clone).mount("/", routes![report, online, redirect_to_repo, is_service_online])
}
