# Flapper
*The dolphin bird*

## Summary
Flapper is a small tool to expose variables for dev environments with mocking as main purpose.

- It can expose environment variables under a configurable path prefix.
- It can expose a version file under a configurable path prefix.

## Configuration
Flapper is configured completely with environment variables. The configuration options existing are:
- ```FLAPPER_VERSION``` - for setting a version 
- ```VERSION_FILE``` - for pointing to a json file which should be exposed as version 
- ```VERSION_PREFIX``` - the path under which the version is exposed 
- ```ENV_VAR_PREFIX``` - the path under which the env var list is exposed 
- ```O_``` and ```X_```-prefixed variables for the env var list 

## How to use
The server exposes two HTTP endpoints:

1. Configurable via ```VERSION_PREFIX``` which returns a JSON object containing the application version and additional version information stored in a file. If the file is not found, a message is printed to the console.
2. Configurable via ```ENV_VAR_PREFIX``` which returns a JSON array of configuration objects, where each object contains an environment variable name and a boolean value indicating whether the variable name starts with "O_" or "X_".

The code starts the server on 0.0.0.0:8080 and binds the two configured endpoints. The publish_envvars function filters the environment variables based on their names and creates Config structs for each of them. The version function reads the version information from a JSON file and combines it with the Flapper version. The version file path and Flapper version are retrieved from the VERSION_FILE and FLAPPER_VERSION environment variables, respectively.

### Environment variables
It must be configured via environment variables. Due to some limitations which require to allow non-conform ENV_VARS, the environment variables have to be set in a specific way where ```<ACTIVE_FLAG>_<NAME>="<internal_name>"```. The convention for that ENV strings may look weird, but it is made in that way to allow non-conform env vars set as keys.

You can furthermore configure the a prefix under which the JSON is exposed via ```PATH_PREFIX```. The prefix defaults to ```"/"```.

For running it locally you have to have cargo installed.

After installing it you can run the server via `cargo run`. With `cargo build --release` you can compile a executable for your operating system.

**Example**
You can run a local deployment of the server to check out it's behaviour. To do this you need to have a docker compatible container engine and docker-compose installed.

You can run the local environment via:
```docker-compose -f local.yaml up```

You can modify the ```local.yaml``` to your needs.
```bash
version: '3'

services:
  flapper:
    build:
      context: .
      dockerfile: Dockerfile
    image: flapper_local
    container_name: flapper_local
    environment:
      FLAPPER_VERSION: "docker-dev (not set)"
      VERSION_FILE: "example.json"
      VERSION_PREFIX: "/version/"
      ENV_VAR_PREFIX: "/flipper/"
      O_VARIABLE_1: "bird"
      X_VARIABLE_2: "dolphin"
      O_VARIABLE_3: "bird.dolphin"
    ports:
      - "8080:8080"
    volumes:
      - ${PWD}/example.json:/example.json
```

creates:

`localhost:8080/flipper`
```JSON
[
  {
    "name": "bird",
    "enabled": "true"
  },
  {
    "name": "dolphin",
    "enabled": "false"
  },
  {
    "name": "bird.dolphin",
    "enabled": "true"
  }
]
```

`localhost:8080/version`
```JSON
{
  "flapper_version":"0.0.0-dev (not set)",
  "build_date":"2023/12/24",
  "program_version":"v1.0.0"
}
```
