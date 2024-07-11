# Implant for C2 Infrastructure

This project is part of a Command and Control (C2) infrastructure. The implant written in Rust is responsible for gathering system information (i.e. ), generating unique identifiers, and sending periodic beacons to the C2 server. The implant also receives and executes commands from the server.

## Features

- Gathers comprehensive system information.
- Generates and saves a unique UUID for the host.
- Sends periodic beacons to the C2 server.
- Receives and executes commands from the C2 server.

## Getting Started

### Prerequisites

- Rust (https://www.rust-lang.org/tools/install) and cargo (https://github.com/rust-lang/cargo)
- A running C2 server that can accept and respond to beacons.

### Installation

Clone the repository:

```shell
git clone https://github.com/bitsima/zoltraak.git
cd zoltraak/client
```

Build the project:

```shell
cargo build --release
```

### Deployment

After building the project, the binary can be found in the *target/release/* directory. You can deploy this binary to the target machines.

### Configuration

Ensure the C2 server URL is correctly set in the code before building and deploying the binary. The C2 URL is hardcoded in the main.rs file. **Modify the C2_URL constant to point to your C2 server.**

### Running the Implant

To run the implant, use the following command:

```shell
cargo run
```

## Project Overview

### src/beacon/

- mod.rs: Module declarations for the beacon component.
- sender.rs: Contains logic for creating and sending beacons to the C2 server.

### src/commands/

- mod.rs: Module declarations for the commands component.
- execute.rs: Contains logic for executing commands received from the C2 server.

### src/sysinfo/

- mod.rs: Module declarations for the sysinfo component.
- saver.rs: Contains logic for collecting system information and saving it to a file.
- uuid.rs: Contains logic for checking and generating UUIDs for system identification.

### src/utils/

- Placeholder for utility functions that might be needed across various modules.

### src/main.rs

- Entry point of the program. Initializes the system, sets up the beacon sending loop, and handles command execution.

### src/lib.rs

- Library root file. It exposes the necessary modules and functionalities to be used by the main application.
