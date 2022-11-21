# WIK-DPS-TP03

## Requirements

- OS : Linux (everything was made on Manjaro)
- Docker

## How it works ?

- First, clone the repo 
    - `git clone https://github.com/zHatsuharu/WIK-DPS-TP03.git`

- Go in `tp-03-docker-compose`
    - `cd tp-03-docker-compose`

- Run the docker with :
    - `docker compose up --build`

## Result
Open your browser and go to `localhost:8081/ping`

You can curl it with :
```bash
curl -v localhost:8081/ping
```