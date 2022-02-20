# Heartbeat

This is a quite simple server to check if another server is still alive.

What is a heartbeat? [Wikipedia](https://en.wikipedia.org/wiki/Heartbeat_%28computing%29)

You can register your service and the heartbeat checker will send a telegram message if it does not get heartbeats.

## Installation

There are two ways to install it:

### Directly

1. clone the repository
2. run `cargo install --path .`
3. run `heartbeat`

### Docker

1. clone the repository (`docker pull ondolin/heartbeat`)
2. Start the container with:
```bash
docker run \
   -e TELEGRAM_TARGET_CHAT=<chat id> \
   -e TELEGRAM_BOT_TOKEN=<telegram token> \
   -e ROCKET_ADDRESS=0.0.0.0 \
   -e POLL_RATE=10 \
   -e DEFAULT_TIMEOUT=120 \
   -p <your desired port>:8000 \
   --name heartbeat \
   --rm -d \
   ondolin/heartbeat
```


## Usage

There are several routes to interact with the heartbeat server.

- `/online`: check if the heartbeat server is online
- `/report/<user>/<service_id>?<timeout>`:
    - `user`: the user to report
    - `service_id`: the service to report (to make is secure choose a random id)
    - `timeout`: the timeout in seconds, until the service is considers you offline

## Contribute

Feel free to contribute to this project by creating issues and pull requests. I am thankful for all your work.
