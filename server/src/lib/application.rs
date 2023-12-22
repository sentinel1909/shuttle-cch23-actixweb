// src/lib/application.rs

// dependencies
use axum::{
    routing::{get, post},
    Router,
};
use day1_endpoints::calibrate_ids;
use day4_endpoints::calculate_total_strength;
use day11_endpoints::static_file_get;
use day12_endpoints::{timekeeper_get, timekeeper_post};
use day13_endpoints::select_20231213;
use domain::AppState;
use minus1_endpoints::{error, root};
use shuttle_persist::PersistInstance;
use shuttle_runtime::CustomError;
use sqlx::{Executor, PgPool};

// function to spin up the axum server and routes
pub async fn start_axum_service(
    persist: PersistInstance,
    pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    // configure the database
    pool.execute(include_str!("schema.sql"))
        .await
        .map_err(CustomError::new)?;

    // set up the application state with the persist storage and a database pool
    let state = AppState { persist, pool };

    // configure the routes and add state
    let router = Router::new()
        .route("/", get(root))
        .route("/-1/error", get(error))
        .route("/1/*nums", get(calibrate_ids))
        .route("/4/strength", post(calculate_total_strength))
        .route("/11/assets/decoration.png", get(static_file_get))
        .route("/12/save/:packet_id", post(timekeeper_post))
        .route("/12/load/:packet_id", get(timekeeper_get))
        .route("/13/sql", get(select_20231213))
        .with_state(state);

    // spin up
    Ok(router.into())
}
