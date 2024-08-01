use axum::extract::State;
use axum::routing::post;
use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
use serde_json::Value;
use uuid::Uuid;

use crate::hue::v2::{ResourceLink, ResourceType, Scene, V2Reply};
use crate::state::AppState;

async fn get_root(State(state): State<AppState>) -> impl IntoResponse {
    Json(V2Reply {
        data: state.get_resources().await,
        errors: vec![],
    })
}

async fn get_resource(
    State(state): State<AppState>,
    Path(rtype): Path<ResourceType>,
) -> impl IntoResponse {
    Json(V2Reply {
        data: state.get_resources_by_type(rtype).await,
        errors: vec![],
    })
}

async fn post_resource(
    State(_state): State<AppState>,
    Path(rtype): Path<ResourceType>,
    Json(req): Json<Value>,
) -> impl IntoResponse {
    log::info!("POST {rtype:?}: {req:?}");
}

async fn post_scene(
    State(state): State<AppState>,
    Json(req): Json<Value>,
) -> axum::response::Result<Json<V2Reply<ResourceLink>>> {
    log::info!("POST scene: {}", serde_json::to_string(&req).unwrap());
    let scn = serde_json::from_value::<Scene>(req);
    println!("{:?}", &scn);
    let scene = scn.unwrap();

    log::info!("POST scene: {scene:#?}");
    let link = state.res.lock().await.add_scene(scene);
    Ok(Json(V2Reply {
        data: vec![link],
        errors: vec![],
    }))
}

async fn get_resource_id(
    State(state): State<AppState>,
    Path((rtype, id)): Path<(ResourceType, Uuid)>,
) -> impl IntoResponse {
    if let Some(res) = state.get_resource(rtype, id).await {
        Json(V2Reply {
            data: vec![res],
            errors: vec![],
        })
    } else {
        Json(V2Reply {
            data: vec![],
            errors: vec!["glump".to_owned()],
        })
    }
}

async fn put_resource_id(
    State(_state): State<AppState>,
    Path((rtype, id)): Path<(ResourceType, Uuid)>,
    Json(req): Json<Value>,
) -> impl IntoResponse {
    log::info!("PUT {rtype:?}/{id}: {req:?}");
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_root))
        .route("/scene", post(post_scene))
        .route("/:resource", get(get_resource).post(post_resource))
        .route("/:resource/:id", get(get_resource_id).put(put_resource_id))
}
