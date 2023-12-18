use axum::{Router, routing::get, extract::{State, Path}, Json};
use serde_json::{json, Value};
use tracing::debug;

use crate::{models::ModelManager, services::food_service, web::error::{Error, Result}};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/ingredients/food/:food_name", get(get_foods_by_name))
        .route("/api/ingredients/measure/:food_id", get(get_measures_by_food))
        .route("/api/ingredients/macros/:food_id/:measure_id", get(get_macros_by_food))
        .with_state(mm)
}

async fn get_foods_by_name(
    State(mm): State<ModelManager>,
    Path(food_name): Path<String>
) -> Result<Json<Value>> {
    debug!("{:<12} - get food", "HANDLER");

    let foods = food_service::get_foods_by_name(&mm, food_name).await?;
    let res = Json(json!({ "results": foods }));
    
    Ok(res)
}

async fn get_measures_by_food(
    State(mm): State<ModelManager>,
    Path(food_id): Path<i32>,
) -> Result<Json<Value>> {
    debug!("{:<12} - get measure", "HANDLER");

    let measures = food_service::get_measures_by_food(&mm, food_id).await?;
    let res = Json(json!({ "results": measures }));
    
    Ok(res)
}

async fn get_macros_by_food(
    State(mm): State<ModelManager>,
    Path((food_id, measure_id)): Path<(i32, i32)>,
) -> Result<Json<Value>> {
    debug!("{:<12} - get macros", "HANDLER");

    let macros = food_service::get_macros_by_food(&mm, food_id, measure_id).await?;
    let res = Json(json!({ "results": macros }));
    
    Ok(res)
}
