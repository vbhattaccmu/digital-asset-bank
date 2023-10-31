//! Methods defining handlers for API endpoints.

use serde_derive::{Deserialize, Serialize};
use std::{convert::Infallible, sync::Arc};
use warp::{self, http, Filter};

use crate::db::{self, Transaction, User};
use crate::error_codes::Error as ServiceAPIError;

/// Index Route (GET /).
pub(crate) fn index_route(
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    async fn index_page_handler() -> Result<impl warp::Reply, Infallible> {
        let body = "Tocos Service alive.".to_string();
        Ok(http::Response::builder().body(body))
    }

    warp::path!().and(warp::get()).and_then(index_page_handler)
}

//////////////////////////////////
// Handlers for Service Endpoints
//////////////////////////////////

pub(crate) fn transactions(
    db: Arc<db::Database>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    // POST /transactions
    pub async fn post_tx(
        tx: Transaction,
        db: Arc<db::Database>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let query_response = db.post_tx(tx).await.map_err(|e| warp::reject::custom(e))?;

        Ok(warp::reply::json(&query_response))
    }

    // GET /transactions
    pub async fn get_transactions(
        limit: Limit,
        db: Arc<db::Database>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let window = limit.limit.unwrap_or(MAX_WINDOW_SIZE);
        if window == 0 || window > MAX_WINDOW_SIZE {
            return Err(warp::reject::custom(ServiceAPIError::WindowLimitExceeded));
        }

        let tx_response = db
            .get_tx(window as usize)
            .await
            .map_err(|e| warp::reject::custom(e))?;

        let query_resposne = serde_json::to_string(&tx_response)
            .map_err(|_| warp::reject::custom(ServiceAPIError::SerializationFailure))?;

        Ok(http::Response::builder()
            .status(http::StatusCode::OK)
            .body(query_resposne))
    }

    let get_tx_route = |db: Arc<db::Database>| {
        warp::get()
            .and(warp::path("transactions"))
            .and(warp::query::<Limit>())
            .and(warp::path::end())
            .and_then(move |limit| get_transactions(limit, Arc::clone(&db)))
    };

    let post_tx_route = |db: Arc<db::Database>| {
        warp::path!("transactions")
            .and(warp::post())
            .and(warp::body::content_length_limit(10 * 1024 * 1024))
            .and(warp::body::json())
            .and(warp::path::end())
            .and_then(move |tx| post_tx(tx, Arc::clone(&db)))
    };

    get_tx_route(db.clone()).or(post_tx_route(db.clone()))
}

pub(crate) fn accounts(
    db: Arc<db::Database>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    // POST /users
    pub async fn create_account(
        user: User,
        db: Arc<db::Database>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let query_response = db
            .create_account(user)
            .await
            .map_err(|e| warp::reject::custom(e))?;

        Ok(warp::reply::json(&query_response))
    }

    // GET /users/id
    pub async fn get_account(
        id: Option<u64>,
        db: Arc<db::Database>,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let user = db
            .get_account_info(id)
            .await
            .map_err(|e| warp::reject::custom(e))?;

        Ok(warp::reply::json(&user))
    }

    let get_account_route = |db: Arc<db::Database>| {
        warp::path!("users" / u64)
            .and(warp::get())
            .and(warp::path::end())
            .and_then(move |id| get_account(Some(id), Arc::clone(&db)))
    };

    let get_accounts_route = |db: Arc<db::Database>| {
        warp::path!("users")
            .and(warp::get())
            .and(warp::path::end())
            .and_then(move || get_account(None, Arc::clone(&db)))
    };

    let post_account_route = |db: Arc<db::Database>| {
        warp::path!("users")
            .and(warp::post())
            .and(warp::body::content_length_limit(10 * 1024 * 1024))
            .and(warp::body::json())
            .and(warp::path::end())
            .and_then(move |user| create_account(user, Arc::clone(&db)))
    };

    get_account_route(db.clone())
        .or(post_account_route(db.clone()))
        .or(get_accounts_route(db))
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Limit {
    pub limit: Option<u64>,
}

// MAX_WINDOW_SIZE denotes the maximum allowable number of entities that can be fetched
// from DB in one endpoint call.
const MAX_WINDOW_SIZE: u64 = 25;
