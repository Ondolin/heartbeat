#[macro_use] extern crate rocket;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{env, thread};
use std::thread::sleep;
use std::time::Duration;
use crate::service::ServiceInfo;
use crate::routes::*;

mod routes;
mod service;

#[launch]
fn rocket() -> _ {

    dotenv::dotenv().ok();

    let hash_map: HashMap<String, ServiceInfo> = HashMap::new();
    let services = Arc::new(Mutex::new(hash_map));

    let services_clone = services.clone();

    // monitoring thread
    thread::spawn(move || loop {
        sleep(Duration::from_secs(env::var("POLL_RATE").unwrap().parse::<u64>().unwrap()));

        services_clone.lock().unwrap().retain(|_, value|{
            
            let service_info = value;

            if service_info.is_timeout() && !service_info.is_offline {
                service_info.make_dead();
            }

            true
        });


    });

    rocket::build().manage(services).mount("/", routes![report, online, redirect_to_repo, is_service_online])
}
