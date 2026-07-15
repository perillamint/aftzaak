use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect,
};
use uuid::Uuid;

use crate::AppState;
use crate::entity::item;
use crate::error::{AppError, AppResult};
use crate::types::api::item::{Item, ItemListQuery, ItemListResponse, ItemPatch};

pub fn get_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create).get(list))
        .route("/{id}", get(get_one).patch(update).delete(delete))
}

async fn create(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ItemPatch>,
) -> AppResult<(StatusCode, Json<Item>)> {
    let now = Utc::now().fixed_offset();
    let model = item::ActiveModel {
        id: ActiveValue::Set(Uuid::now_v7()),
        title: ActiveValue::Set(
            req.title
                .ok_or_else(|| AppError::BadRequest("Empty title".to_string()))?,
        ),
        mime_type: ActiveValue::Set(
            req.mime_type
                .ok_or_else(|| AppError::BadRequest("MIME not specified".to_string()))?,
        ),
        // TODO: Fetch it from file database
        size_bytes: ActiveValue::Set(req.size_bytes.unwrap_or(0)),
        // TODO: Fetch it from file database
        checksum: ActiveValue::Set(req.checksum.unwrap_or(None)),
        // TODO: Fetch it from file database
        storage_uri: ActiveValue::Set(req.storage_uri.unwrap_or("".to_string())),
        metadata: ActiveValue::Set(req.metadata.unwrap_or(None)),
        created_at: ActiveValue::Set(now),
        updated_at: ActiveValue::Set(now),
        deleted_at: ActiveValue::Set(None),
    };
    let model = model.insert(&state.db).await?;
    Ok((StatusCode::CREATED, Json(model.into())))
}

async fn list(
    State(state): State<Arc<AppState>>,
    Query(q): Query<ItemListQuery>,
) -> AppResult<Json<ItemListResponse>> {
    let limit = q.limit.unwrap_or(20).min(100);
    let offset = q.offset.unwrap_or(0);

    let total = item::Entity::find()
        .filter(item::Column::DeletedAt.is_null())
        .count(&state.db)
        .await?;

    let items = item::Entity::find()
        .filter(item::Column::DeletedAt.is_null())
        .order_by_desc(item::Column::CreatedAt)
        .offset(offset)
        .limit(limit)
        .all(&state.db)
        .await?
        .into_iter()
        .map(Item::from)
        .collect();

    Ok(Json(ItemListResponse { items, total }))
}

async fn get_one(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Item>> {
    let model = item::Entity::find_by_id(id)
        .filter(item::Column::DeletedAt.is_null())
        .one(&state.db)
        .await?
        .ok_or(AppError::ItemNotFound)?;
    Ok(Json(model.into()))
}

async fn update(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<ItemPatch>,
) -> AppResult<Json<Item>> {
    let model = item::Entity::find_by_id(id)
        .filter(item::Column::DeletedAt.is_null())
        .one(&state.db)
        .await?
        .ok_or(AppError::ItemNotFound)?;

    let mut active: item::ActiveModel = model.into();
    let now = Utc::now().fixed_offset();

    if let Some(v) = req.title {
        active.title = ActiveValue::Set(v);
    }
    if let Some(v) = req.mime_type {
        active.mime_type = ActiveValue::Set(v);
    }
    if let Some(v) = req.size_bytes {
        active.size_bytes = ActiveValue::Set(v);
    }
    if let Some(v) = req.checksum {
        active.checksum = ActiveValue::Set(v);
    }
    if let Some(v) = req.storage_uri {
        active.storage_uri = ActiveValue::Set(v);
    }
    if let Some(v) = req.metadata {
        active.metadata = ActiveValue::Set(v);
    }
    active.updated_at = ActiveValue::Set(now);

    let model = active.update(&state.db).await?;
    Ok(Json(model.into()))
}

async fn delete(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> AppResult<StatusCode> {
    let model = item::Entity::find_by_id(id)
        .filter(item::Column::DeletedAt.is_null())
        .one(&state.db)
        .await?
        .ok_or(AppError::ItemNotFound)?;

    let mut active: item::ActiveModel = model.into();
    let now = Utc::now().fixed_offset();
    active.deleted_at = ActiveValue::Set(Some(now));
    active.updated_at = ActiveValue::Set(now);
    active.update(&state.db).await?;

    Ok(StatusCode::NO_CONTENT)
}
