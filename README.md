# rsweb
## a web server and creation library for multithreaded web servers

## Installation
`rsweb` has a docker image `uludev/rsweb:latest` which will run an `x86_64` executable.
It expects a configuration file to be in `/etc/rsweb/rsweb.config.toml` and it will log to `/var/log/rsweb/latest.log`.

## Configuration
An example configuration looks like this:
```toml
threads = 10
port = 8080
ip = "127.0.0.1"
threads = 10
logfile = "log.txt"
[resources]
root = "."
index = "/test.html"
aliases = ["/test:/test.html"]
```

## SSL
`rsweb` has an SSL implementation. An example configuration using SSL looks like this:
```toml
port = 8080
ip = "127.0.0.1"
threads = 10
logfile = "log.txt"
[resources]
root = "."
index = "/test.html"
aliases = ["/test:/test.html"]
[ssl]
private_key = "privkey.pem"
certificate_chain = "certs.pem"
```
