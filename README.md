# SheepsTor
Utility for updating static websites (served via Hugo) from Github (responding to GitHub webhook events)

## Configuration
Sheepstor is configured with one config file `./config/config.yaml`

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
(also useful for InitContainer to set up web service)
```bash
./sheepstor update --config=<CONFIG_FILE_PATH> --sites=all
```
