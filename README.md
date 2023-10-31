# Digital Asset Bank

A digital asset bank stores tokens for a user and allows transfer of tokens from one account to another. Simple logic!
Lets build it in Rust.

# Bank Demo

A demo is available [here](https://www.loom.com/share/ff10f4c7b02146fd9e4ca6a890d5f340?sid=2b0095b8-3783-4087-bd94-e39abeb8fbfe)

# Digital Asset Bank Backend

The digital asset bank backend should expose the following API endpoints to the user:

- `POST /users`: Create a new user with an initial balance.
- `GET /users/{id}`: Retrieve details of a user.
- `POST /transactions`: Perform a transaction between two users. The body of the request should include the ID of the sender, the ID of the receiver, the amount of Tacos to be transferred, and any other required information.

# Accessing the Bank API

## Prerequisites

- Users are required to install Docker and Docker Compose (v2) on their machines and to make sure that docker daemon is active. For installation instructions, refer to [Install Docker Engine on Ubuntu](https://docs.docker.com/engine/install/ubuntu/) and [Install the Compose plugin](https://docs.docker.com/compose/install/linux/).

- The Make utility should be pre-installed on Ubuntu operating system. If not, use the following command to install it manually:

```bash
sudo apt update && sudo apt install make
```

## Start the Servers

1. Navigate to the top level of the directory.

2. Run the following make command to start the service. This command also runs the test suite:

```bash
make start_service
```

In case you run into permission issues with docker daemon, you can use `sudo make start_service` instead.
The docker-compose uses the front end and backend images hosted in docker hub and they can be built from the
Dockerfiles provided in the following directories:-

- service_submission/Dockerfile -> rust backend (vbhattac453/rust_backend:latest)
- service_submission/front-end/Dockerfile -> front end (vbhattac453/frontend:latest)

3. Once `make start_service` has completed setting the services, the front end is visible in http://localhost:3000 and the tests are executed.

4. Clean up artifacts:

```bash
make service_clean_up
```

## Visualize the app

The front end is available on http://localhost:3000. You can create users, submit transactions and visualize current users and transactions!
You need at least 5 units to make a transaction from an account.

## You can run the Test Suite Separately

Follow these steps if you want to run the tests separately:

1. Install Rust using the following commands:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
```

2. Build the service binary from the top level of this directory:

```bash
cargo build
```

3. Launch all tests:

There are various ways to run the tests:

## To Run a Specific Module in the Suite

```bash
cargo test --package tests -- service_api_tests
```

## To Launch a Specific Test

```bash
cargo test --package tests -- service_api_tests::account::test_query_account_creation_success
```

Running all tests at once may consume significant resources. You can use the `--test-threads` flag to control the number of threads used:

```bash
cargo test -- --test-threads=3
```

Here are some additional design decisions behind this project:-

# Note

1. **Choice of Rust Language**: The decision to use Rust for the backend was made due to its strong memory safety guarantees and performance optimizations. Rust's ownership and borrowing system helps prevent programming errors and ensures a higher level of reliability in the codebase.

2. **Postgres Database Connection Pool**: The backend utilizes a Postgres database connection pool using the MOBC (Managed Objects Connection Builder) client library. This design choice was made to manage database connections and improve the application's overall performance and responsiveness.

3. **Containerization with Docker**: The application is containerized using Docker, which encapsulates the application, its dependencies, and environment configurations. This approach ensures consistency across different environments and simplifies deployment and scaling processes.

4. **Test-Driven Development (TDD)**: The codebase follows a Test-Driven Development approach, where tests are written before the implementation.

5. **API Documentation and Postman Collection**: The API endpoints are documented to provide clear instructions on their usage, expected inputs, and responses. Additionally, a Postman collection is provided to facilitate testing and interaction with the API endpoints.

6. **Error Handling and Logging**: Comprehensive error handling mechanisms are implemented to handle various scenarios, such as invalid inputs or server errors. Logging is used to capture important events using fern logger and provide insights into the application's behavior for debugging and monitoring purposes.

7. **A minimal frontend**: The frontend design focuses on providing an interface for creating users, performing transactions, and viewing user/transaction details. The front end is available on http://localhost:3000.
