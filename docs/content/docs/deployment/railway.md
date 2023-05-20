+++
title = "Deploy to Render"
description = "How to deploy Fastrue to Render."
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 220
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = 'AdiDoks is a Zola theme helping you build modern documentation websites, which is a port of the Hugo theme <a href="https://github.com/h-enk/doks">Doks</a> for Zola.'
toc = true
top = false
+++

## Quick Start

[![Deploy on Railway](https://railway.app/button.svg)](https://railway.app/template/mlDOsy?referralCode=aris)

### Create app instances

```sh
# Create Fly.io app
fly apps create fastrue --org personal

# Create POstgres database instance.
fly postgres create --org personal --name fastrue-db --region sjc --password $(openssl rand -hex 8)
```

### Launch and deploy

```sh
# Attach Postgres database
fly postgres attach fastrue-db -a fastrue

# Load secrets from dotenv file then initialize deployment
fly secrets set $(cat .env.production | xargs -I %s echo %s)
fly secrets list

# Deploy the app
fly deploy -e PRIMARY_REGION=sjc --remote-only
```

### Setup custom domain

Point DNS A Record to the assigned IP address.
Or, if using subdomain you can point `fastrue.fly.dev` CNAME record.

```sh
# Allocate IPs and setup custom domain (optional)
fly ips allocate-v4 -a fastrue
fly ips allocate-v6 -a fastrue
fly certs create auth.example.com -a fastrue
```
