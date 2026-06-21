use axum::{
    extract::{Path, State},
    response::Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{errors::ApiError, middleware::auth_middleware::AuthenticatedUser, state::AppState};
use finance_assistant_app::dto::item::*;

// ─── Item Categories ─────────────────────────────────────────────────────────

pub async fn create_category(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Json(req): Json<CreateItemCategoryRequest>,
) -> Result<Json<ItemCategoryResponse>, ApiError> {
    let res = state.item_service.create_category(req).await?;
    Ok(Json(res))
}

pub async fn get_category(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ItemCategoryResponse>, ApiError> {
    let res = state.item_service.get_category(id).await?;
    Ok(Json(res))
}

pub async fn update_category(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateItemCategoryRequest>,
) -> Result<Json<ItemCategoryResponse>, ApiError> {
    let res = state.item_service.update_category(id, req).await?;
    Ok(Json(res))
}

pub async fn delete_category(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    state.item_service.delete_category(id).await?;
    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn list_categories(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(company_id): Path<Uuid>,
) -> Result<Json<Vec<ItemCategoryResponse>>, ApiError> {
    let res = state.item_service.list_categories(company_id).await?;
    Ok(Json(res))
}

// ─── Items ───────────────────────────────────────────────────────────────────

pub async fn create_item(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Json(req): Json<CreateItemRequest>,
) -> Result<Json<ItemResponse>, ApiError> {
    let res = state.item_service.create_item(req).await?;
    Ok(Json(res))
}

pub async fn get_item(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ItemResponse>, ApiError> {
    let res = state.item_service.get_item(id).await?;
    Ok(Json(res))
}

pub async fn update_item(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateItemRequest>,
) -> Result<Json<ItemResponse>, ApiError> {
    let res = state.item_service.update_item(id, req).await?;
    Ok(Json(res))
}

pub async fn delete_item(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    state.item_service.delete_item(id).await?;
    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn list_items(
    State(state): State<Arc<AppState>>,
    _user: AuthenticatedUser,
    Path(company_id): Path<Uuid>,
) -> Result<Json<Vec<ItemResponse>>, ApiError> {
    let res = state.item_service.list_items(company_id).await?;
    Ok(Json(res))
}
