# SheepsTor
Utility for updating static websites manually or by responding to GitHub webhook events.

## Configuration
Sheepstor is configured from two places:

1. One environment variables containing a secret to be used by Github webhook. This will need to be set and exported in the runtime environment.
2. One config file (see `./config/config_SAMPLE.yaml` for annotated example)

Sheepstor currently support two possible content processors (set in the `content_processor` property in the config file):
- Hugo (invokes Hugo to compile the site)
- None (in which case it will just copy the sources verbatim)

## Debugging
Run any of the commands below with an additional flag `--debug`


## Run as web service to respond to GitHub webhook events
```bash
./sheepstor server --config=<CONFIG_FILE_PATH> --port <PORT_NUMBER>
```

## Run as command line utility to manually update site(s)
### Update single website
```bash
./sheepstor update --config=<CONFIG_FILE_PATH> --sites=<ID_OF_SITE_FROM_CONFIG>
```

### Update multiple websites (comma separated)
```bash
./sheepstor update --config=<CONFIG_FILE_PATH> --sites=<ID_OF_SITE_FROM_CONFIG>,<ID_OF_ANOTHER_SITE_FROM_CONFIG>
```

### Update all websites
(also useful for InitContainer to set up web service if deployed in Kubernetes)
```bash
./sheepstor update --config=<CONFIG_FILE_PATH> --sites=all
```
