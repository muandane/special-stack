# mule

just a smoll cdn PoC in rust

## Configuration

there's only 3 opetional variables to set:

- `CDN_ROOT`: set a custom folder to store your cached files (default: `/data/content`)
- `DB_PATH`: set a custom folder to store your database (default: `/data/db`)
- `DEBUG`: true,false for level of verbosity of the application logs

## Running your application

The served files are accessible at `http://<client IP or localhost>:3000`, and the managment endpoint is accessible at `http://<client IP or localhost>:9001`.

You can perform a couple of operations on the api endpoint such as:

- Query the cached files list from the database:

```sh
curl http://${SERVER_IP}:9001/mappings
```

- Upload/cache a new file:

```sh
curl -X POST http://${SERVER_IP}:9001/cache -d "https://hampter.io/hampter.gif"
```

## Building and running your application

When you're ready, start your application by running:
`docker compose up --build`.

Your application will be available at <http://localhost:3000>.

### Deploying your application to the cloud

First, build your image, e.g.: `docker build -t myapp .`.
If your cloud uses a different CPU architecture than your development
machine (e.g., you are on a Mac M1 and your cloud provider is amd64),
you'll want to build the image for that platform, e.g.:
`docker build --platform=linux/amd64 -t myapp .`.

Then, push it to your registry, e.g. `docker push myregistry.com/myapp`.

Consult Docker's [getting started](https://docs.docker.com/go/get-started-sharing/)
docs for more detail on building and pushing.

### Docker compose example using traefik

```yaml
services:
  traefik:
    image: "traefik:latest"
    container_name: "traefik"
    restart: unless-stopped
    command:
    #- "--log.level=DEBUG"
    - "--api.insecure=true"
    - "--providers.docker=true"
    - "--providers.docker.network=traefik"
    - "--entrypoints.web.address=:80"
    - "--entrypoints.web.http.redirections.entrypoint.to=websecure"
    - "--entrypoints.web.http.redirections.entrypoint.permanent=true"
    - "--entrypoints.web.http.redirections.entryPoint.scheme=https"
    - "--entrypoints.websecure.address=:443"
    - "--entrypoints.mule_mgmt.address=:9001"
    - "--certificatesresolvers.myresolver.acme.dnschallenge=true"
    - "--certificatesresolvers.myresolver.acme.dnschallenge.provider=duckdns"
    - "--certificatesresolvers.myresolver.acme.email=me@gmail.com"
    - "--certificatesresolvers.myresolver.acme.storage=/letsencrypt/acme.json"
    ports:
    - "443:443"
    - "80:80"
    environment:
    - "DUCKDNS_TOKEN=xxxxx-xxx-xxxx-xxxx-xxxxx"
    volumes:
    - "./letsencrypt:/letsencrypt"
    - "/var/run/docker.sock:/var/run/docker.sock:ro"
  mule:
    container_name: mule
    restart: always
    image: muandane/mule:24.6.4-main
    volumes:
      - "./data/mule/content:/data/content"
      - "./data/mule/db:/data/db:rw"
    pid: host
    ports:
      - 3000:3000
      - 9001:9001
    labels:
      - "traefik.http.routers.mule.tls=true"
      - "traefik.http.routers.mule.service=mule"
      - "traefik.http.routers.mule.entrypoints=websecure"
      - "traefik.http.routers.mule.rule=Host(`cdn.xxx.duckdns.org`)"
      - "traefik.http.routers.mule.tls.certresolver=myresolver"
      - "traefik.http.routers.mule_http.entrypoints=web"
      - "traefik.http.middlewares.redirect-to-https.redirectscheme.scheme=https"
      - "traefik.http.routers.mule_http.rule=Host(`cdn.xxx.duckdns.org`)"
      - "traefik.http.services.mule.loadBalancer.server.port=3000"
      - "traefik.tcp.routers.route_mule_mgmt.rule=HostSNI(`*`)"
      - "traefik.tcp.routers.route_mule_mgmt.entryPoints=mule_mgmt"
      - "traefik.tcp.routers.route_mule_mgmt.service=service_mule_mgmt"
      - "traefik.tcp.services.service_mule_mgmt.loadbalancer.server.port=9001"
```

### References

* [Docker's Rust guide](https://docs.docker.com/language/rust/)
