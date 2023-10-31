/// `config` defines methods for parsing configurable attributes.
mod config;

/// `db` defines methods that are responsible for storing and retrieving data from DB.
mod db;

/// `error_codes` defines a set of numeric codes for different types of errors during client-side HTTP requests.
mod error_codes;

/// `routes` defines the set of HTTP endpoints for serving information related to accounts and transactions.
mod routes;

use anyhow::{Error, Result};
use clap::Parser;
use humantime::Timestamp;
use std::{sync::Arc, time::SystemTime};
use warp::{self, Filter};

#[tokio::main]
async fn main() -> Result<(), Error> {
    //////////////////////////////////
    // 1. Load system configuration.
    //////////////////////////////////

    let cli_args = config::CLIArguments::parse();
    let service_config = config::load_config(cli_args.config_path.as_str())
        .expect("Irrecoverable error: fail to load config.toml");

    let db_config = format!(
        "host={} user={} password={} dbname={}",
        service_config.db_host,
        service_config.db_user,
        service_config.db_user_pw,
        service_config.db_name
    );

    ///////////////////////////////
    // 2. Set up and begin logging.
    ///////////////////////////////

    std::fs::create_dir_all(&service_config.logs_dir)?;
    let proc_start_time = Timestamp::from(SystemTime::now());

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level_for("_", log::LevelFilter::Error)
        .level_for("warp", log::LevelFilter::Error)
        .level(log::LevelFilter::Info)
        .chain(fern::log_file(format!(
            "{}/{}.log",
            &service_config.logs_dir, proc_start_time
        ))?)
        .apply()?;

    ///////////////////////
    // 3. Open Service DB
    ///////////////////////

    let db = db::Database::open(cli_args.start_anew, db_config.as_str())
        .await
        .expect("Irrecoverable error: Failed to open database.");

    let db = Arc::new(db);
    let db_instance_accounts = Arc::clone(&db);
    let db_instance_transactions = Arc::clone(&db);

    ///////////////////////////////////
    // 4. Serve Users and Tx Endpoints
    ///////////////////////////////////

    let warp_serve = warp::serve(
        routes::index_route()
            .or(routes::transactions(Arc::clone(&db_instance_transactions)))
            .or(routes::accounts(Arc::clone(&db_instance_accounts)))
            .recover(error_codes::handle_rejection)
            .with(
                warp::cors()
                    .allow_any_origin()
                    .allow_headers(vec![
                        "content-type",
                        "User-Agent",
                        "Sec-Fetch-Mode",
                        "Referer",
                        "Origin",
                        "Access-Control-Request-Method",
                        "Access-Control-Request-Headers",
                    ])
                    .allow_methods(&[
                        warp::http::Method::GET,
                        warp::http::Method::POST,
                        warp::http::Method::OPTIONS,
                    ]),
            ),
    );

    let (_, server) = warp_serve.bind_with_graceful_shutdown(
        ([0, 0, 0, 0], service_config.port_number),
        async move {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to listen to shutdown signal");
        },
    );

    server.await;

    Ok(())
}
