# open-grip-board
A grip strenght measuring board

## Developer setup

To start the development server spin up the docker compose in the root folder.

```bash
docker compose up -d
```

### Upate entities

```bash
sea-orm-cli generate entity --output-dir ./entity/src -l
```