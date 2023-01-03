# GoTrue on Fly.io

### Create app instances

```sh
# Create Fly.io app
fly apps create trusty --org personal

# Create POstgres database instance.
fly postgres create --org personal --name trusty-db --region sjc --password $(openssl rand -hex 8)
```

### Launch and deploy

```sh
# Attach Postgres database
fly postgres attach trusty-db -a trusty

# Load secrets from dotenv file then initialize deployment
fly secrets set $(cat .env.production | xargs -I %s echo %s)
fly secrets list

# Deploy the app
fly deploy -e PRIMARY_REGION=sjc --remote-only
```

### Setup custom domain

Point DNS A Record to the assigned IP address.
Or, if using subdomain you can point `trusty.fly.dev` CNAME record.

```sh
# Allocate IPs and setup custom domain (optional)
fly ips allocate-v4 -a trusty
fly ips allocate-v6 -a trusty
fly certs create auth.example.com -a trusty
```
