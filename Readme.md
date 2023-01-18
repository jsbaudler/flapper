# Flapper
*The dolphin bird*

## Summary
Flapper is a small tool to expose set environment variables in a distinct way for dev environments. It exposes a list of configurables as JSON under /.

## How to use
It must be configured via environment variables. Due to some limitations which require to allow non-conform ENV_VARS, the environment variables have to be set in a specific way where ```<ACTIVE_FLAG>_<NAME>="<internal_name>"```.

e.g.
```bash
O_VARIABLE_1 = "bird"
X_VARIABLE_2 = "dolphin"
O_VARIABLE_3 = "bird.dolphin"
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
