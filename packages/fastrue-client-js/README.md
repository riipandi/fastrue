# @fastrue/client-js

[![npm version](https://img.shields.io/npm/v/@fastrue/client-js)](https://www.npmjs.com/package/@fastrue/client-js)
[![npm downloads](https://img.shields.io/npm/dm/@fastrue/client-js)](https://www.npmjs.com/package/@fastrue/client-js)
[![License](https://img.shields.io/github/license/riipandi/fastrue)](https://github.com/riipandi/fastrue/blob/main/LICENSE)

This is a JavaScript client library for the Fastrue API. It lets you create and authenticate
users and is a building block for constructing the UI for signups, password recovery, login
and logout.

Visit the [documentation](https://fastrue.netlify.app) page for more detailed information.

## Installation

```bash
# Install with npm
npm install @fastrue/client-js

# Install with yarn
yarn add @fastrue/client-js

# Install with pnpm
pnpm add @fastrue/client-js
```

## Usage

### Create Fastrue instance

```typescript
import Fastrue from '@fastrue/client-js'

/**
 * Instantiate the Fastrue auth client with configuration
 */
const auth = new Fastrue({
  // The absolute path of the Fastrue endpoint
  apiUrl: 'https://<your_domain_name>/auth',
  audience: 'webapp',
  setCookie: false,
})
```

### Authentication examples

```typescript
auth
  .signup(email, password)
  .then((resp) => console.log('Confirmation email sent', resp))
  .catch((error) => console.log("It's an error", error))
```

## License

This project is open-sourced software licensed under the [MIT license](https://github.com/riipandi/fastrue/blob/main/LICENSE).

Copyrights in this project are retained by their [contributors](https://github.com/riipandi/fastrue/network/dependencies).
