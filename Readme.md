# Flapper
*The dolphin bird*

## Summary
Flapper is a small tool to expose set environment variables in a distinct way for dev environments. It exposes a list of configurables as JSON under a configurable path.

## How to use
It must be configured via environment variables. Due to some limitations which require to allow non-conform ENV_VARS, the environment variables have to be set in a specific way where ```<ACTIVE_FLAG>_<NAME>="<internal_name>"```. The convention for that ENV strings may look weird, but it is made in that way to allow non-conform env vars set as keys.

You can furthermore configure the a prefix under which the JSON is exposed via ```PATH_PREFIX```. The prefix defaults to ```"/"```.

## Example
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
      PATH_PREFIX: "/flipper/"
      O_VARIABLE_1: "bird"
      X_VARIABLE_2: "dolphin"
      O_VARIABLE_3: "bird.dolphin"
    ports:
      - "8080:8080"
```
this leads to
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
