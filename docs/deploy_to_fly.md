# GoTrue on Fly.io

### Create app instances

```sh
# Create Fly.io app
fly apps create wasta --org personal

# Create volume for the data.
fly postgres create --org personal --name wasta-db --region sjc --password $(openssl rand -hex 8)
```

### Launch and deploy

```sh
# Attach Postgres database
fly postgres attach wasta-db -a wasta

# Load secrets from dotenv file then initialize deployment
fly secrets set $(cat .env.production | xargs -I %s echo %s)
fly secrets list

# Deploy the app
fly deploy --remote-only
```

### Setup custom domain

Point DNS A Record to the assigned IP address.
Or, if using subdomain you can point `wasta.fly.dev` CNAME record.

```sh
# Allocate IPs and setup custom domain (optional)
fly ips allocate-v4 -a wasta
fly ips allocate-v6 -a wasta
fly certs create auth.example.com -a wasta
```