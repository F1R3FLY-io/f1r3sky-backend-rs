# Developer Environment Setup Guide

#### This document provides detailed instructions to set up and run the developer environment.

---

## Prerequisites

Ensure the following tools and dependencies are installed before beginning:

1. **nvm**: Use `nvm install` in each project directory to set the Node.js version.
1. **direnv**: Manage environment variables for your shell.
1. **cargo**: Rust's package manager and build tool.
1. **pnpm**: A high-performance JavaScript package manager.
1. **sbt**: Scala's interactive build tool.
1. **bnfc**: Converter for Backus-Naur Form syntax.
1. **jflex**: Lexical analyzer generator for Java.

---

## Other documentation sources

1. [paul_brain_dump.md](https://github.com/F1R3FLY-io/f1r3fly/blob/main/docs/paul_brain_dump.md): Documentation for [rnode](https://github.com/F1R3FLY-io/f1r3fly) implementation.

---

## Configuration

Configuration environment variables are specified in the `.env` files. You should have the `.env` file located in the main directory.
The required variables include:

```
AWS_ACCESS_KEY_ID=                          minioadmin
AWS_ENDPOINT=                               http://localhost:9000
AWS_SECRET_ACCESS_KEY=                      minioadmin
PDS_DEV_MODE=                               true
PDS_DID_PLC_URL=                            http://localhost:2582
PDS_INVITE_REQUIRED=                        false
PDS_JWT_KEY_K256_PRIVATE_KEY_HEX=           <your_secret>
PDS_PLC_ROTATION_KEY_K256_PRIVATE_KEY_HEX=  <your_secret>
PDS_REPO_SIGNING_KEY_K256_PRIVATE_KEY_HEX=  <your_secret>
PDS_SERVICE_DID=                            did:web:localhost
PDS_SERVICE_HANDLE_DOMAINS=                 .test
ROCKET_ADDRESS=                             0.0.0.0

DATABASE_URL=                               postgresql://postgres@localhost:5434
PDS_BSKY_APP_VIEW_DID=                      # Retrieve this value in Step 4
PDS_BSKY_APP_VIEW_URL=                      http://localhost:2584
ROCKET_PORT=                                2583
PDS_PORT=                                   2583
```

Replace `<your_secret>` with the appropriate secret values where required.

---

## Steps to Set Up and Run the Developer Environment

### 1. Build the RChain Node Image

Navigate to the [rchain/f1r3fly](https://github.com/F1R3FLY-io/f1r3fly/) directory and execute the following:

```
sbt ';compile ;project node ;Docker/publishLocal'
```

---

### 2. Start the Frontend

Navigate to the [f1r3sky](https://github.com/F1R3FLY-io/f1r3sky) directory and run:

```
yarn run web
```

---

### 3. Run Node.js based Backend (Backend 1) for appview and plc registry from there because rsky does not implement them.

Navigate to [f1r3sky-backend1](https://github.com/F1R3FLY-io/f1r3sky-backend-ts) and execute:

```
make deps && make build && ENABLE_PDS=0 make run-dev-env-logged
```

This command starts the Legacy Backend, which generates the **DID** value required for Backend 2.

---

### 4. Retrieve the DID Value

Check the logs of **Backend 1** for the `Bsky Appview DID`. Copy this value for use in the next steps.

---

### 5. Set the DID Value for Backend 2

1. Navigate to [f1r3sky-backend2](https://github.com/F1R3FLY-io/f1r3sky-backend-rs).
2. Open the `.env` file.
3. Paste the copied **DID** value into `PDS_BSKY_APP_VIEW_DID` as follows:

```
PDS_BSKY_APP_VIEW_DID=<your_did_value>
```

---

### 6. Start Dockers with All Services

Inside [f1r3sky-backend2](https://github.com/F1R3FLY-io/f1r3sky-backend-rs), run the following command to build and start all the required services:

```
docker compose -f docker/docker-compose.yaml up --build --force-recreate --wait
```

---

### 7. Start the Rust Backend

1. Once inside [f1r3sky-backend2](https://github.com/F1R3FLY-io/f1r3sky-backend-rs), navigate to `rsky-pos`:

```
cd rsky-pos
```

2. Run the Rust project:

```
cargo run
```

---

## Notes and Common Issues

* Ensure all required dependencies are installed before starting the setup process.
* If the environment variables are missing or incorrect, services may fail to start.
* Confirm that the **DID** value from **Backend 1** is correctly set in the `.env` file for **Backend 2**.
* Don't forget to start the Rust backend after starting the Docker services.
* Backend 2 docker services and the Rust backend must be turned off while running the Legacy Backend 1.
