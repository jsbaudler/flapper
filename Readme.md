# Flapper
*The dolphin bird*

## Summary
Flapper is a small tool to expose variables for dev environments with mocking as main purpose.

- It can expose environment variables under a configurable path prefix.
- It can expose whole databases under a configurable path prefix.

## How to use

### Environment variables
It must be configured via environment variables. Due to some limitations which require to allow non-conform ENV_VARS, the environment variables have to be set in a specific way where ```<ACTIVE_FLAG>_<NAME>="<internal_name>"```. The convention for that ENV strings may look weird, but it is made in that way to allow non-conform env vars set as keys.

You can furthermore configure the a prefix under which the JSON is exposed via ```PATH_PREFIX```. The prefix defaults to ```"/"```.

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
      PATH_PREFIX: "/flipper/"
      O_VARIABLE_1: "bird"
      X_VARIABLE_2: "dolphin"
      O_VARIABLE_3: "bird.dolphin"
    ports:
      - "8080:8080"
```

leads to

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

### Database
Flapper exposes all **SQLite3** databases which it finds under the subfolder `dbs/` under the path prefix `/db/raw/`.

The data is exposed as JSON and independent of it's datatype as string in the following format:

```JSON
{
  "<Database_Path>":
  {
    "<Table>":
    {
      "<Column>": ["<Data1>","<Data2>"]
    },
    "<Table>":
    {
      "<Column>": ["<Data1>","<Data2>"],
      "<Column>": ["<Data1>","<Data2>"]
    }
  }
}
```