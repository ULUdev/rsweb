# rsweb
[![pipeline status](https://gitlab.sokoll.com/moritz/rsweb/badges/main/pipeline.svg)](https://gitlab.sokoll.com/moritz/rsweb/-/commits/main)
## a web server and creation library for multithreaded web servers

## Installation
`rsweb` has a docker image `uludev/rsweb:latest` which will run an `x86_64` executable.
It expects a configuration file to be in `/etc/rsweb/rsweb.config.toml` and it will log to `/var/log/rsweb/latest.log`.

## Configuration
An example configuration looks like this:
```toml
[http]
port = 8080
ip = "127.0.0.1"
threads = 10
logfile = "log.txt"
[http.resources]
root = "."
index = "/test.html"
aliases = ["/test:/test.html"]
routes = ["/route:/index.html"]
```

## SSL
`rsweb` has an SSL implementation. An example configuration using SSL looks like this:
```toml
# optional addition of an http server for compatibility reasons
[http]
port = 8080
ip = "127.0.0.1"
threads = 1
logfile = "alternate_log.txt"
[http.resources]
root = "."
routes = ["/*:https://localhost:4343"]

[ssl.resources]
root = "."
aliases = ["/test:/test.html", "/:/test.html"]
[ssl]
port = 4343
ip = "127.0.0.1"
threads = 10
logfile = "log.txt"
private_key = "privkey.pem"
certificate_chain = "certs.pem"
```
