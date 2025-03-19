## Prerequisites

Make sure the following tools and dependencies are installed on your system before starting work:

* **nvm**: Use `nvm install` in each project to set up the appropriate Node.js version.
* **nix**: Install nix by running:

```
curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install
```

* **direnv**: Environment variable manager for shell.
* **cargo**: Rust's package manager and build system.
* **pnpm**: A fast and efficient package manager for JavaScript.
* **sbt**: The interactive build tool for Scala.
* **bnfc**: The Backus-Naur Form Converter tool.
* **jflex**: A lexical analyzer generator for Java.

---

## Steps to Set Up and Run the Developer Environment

Follow the steps below to get the developer environment running:

1. **Build the RChain Node Image**
   Navigate to `[rchain/f1r3fly]` and run:

```
sbt ';compile ;project node ;Docker/publishLocal'
```

2. **Start the Backend**
   Navigate to `[f1r3sky]` and run:

```
yarn run web
```

3. **Run Legacy Backend (Backend 1)**
   Navigate to `[f1r3sky-backend1]` and execute the following commands:

```
make deps && make build && ENABLE_PDS=0 make run-dev-env-logged
```

This step starts the legacy backend to generate the DID value required for Backend 2.

4. **Retrieve the DID Value**
   Check the logs of **Backend 1** for the `Bsky Appview DID` value and copy it.
5. **Set the DID Value for Backend 2**
   Navigate to `[f1r3sky-backend2]` and paste the copied DID value into the following file:

```
docker/.env
```

6. **Start Backend 2**
   Still inside `[f1r3sky-backend2]`, run:

```
docker compose -f docker/docker-compose.yaml up --build --force-recreate --wait
```
