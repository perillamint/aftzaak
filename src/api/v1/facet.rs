use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::middleware::from_fn_with_state;
use axum::routing::{delete, get, patch, post};
use axum::{Json, Router};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue, EntityTrait, PaginatorTrait, QueryOrder, QuerySelect,
};
use uuid::Uuid;

use crate::AppState;
use crate::api::middleware::permission_filter::check_perm;
use crate::entity::{
    facet::ActiveModel as FacetActiveModel, facet::Column as FacetColumn,
    facet::Entity as FacetEntity,
};
use crate::error::{AppError, AppResult};
use crate::perms::Permission;
use crate::types::api::facet::{Facet, FacetPatch};
use crate::types::api::{ListQuery, ListResponse};
use crate::update_am;

pub fn get_router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/",
            get(list_facet).layer(from_fn_with_state(Permission::FacetRead, check_perm)),
        )
        .route(
            "/",
            post(create_facet).layer(from_fn_with_state(Permission::FacetWrite, check_perm)),
        )
        .route(
            "/{id}",
            get(get_facet).layer(from_fn_with_state(Permission::FacetRead, check_perm)),
        )
        .route(
            "/{id}",
            patch(update_facet).layer(from_fn_with_state(Permission::FacetWrite, check_perm)),
        )
        .route(
            "/{id}",
            delete(delete_facet).layer(from_fn_with_state(Permission::FacetDelete, check_perm)),
        )
}

async fn create_facet(
    State(state): State<Arc<AppState>>,
    Json(req): Json<FacetPatch>,
) -> AppResult<(StatusCode, Json<Facet>)> {
    let now = Utc::now().fixed_offset();
    let model = FacetActiveModel {
        id: ActiveValue::Set(Uuid::now_v7()),
        key: ActiveValue::Set(
            req.key
                .ok_or_else(|| AppError::BadRequest("Empty key".to_string()))?,
        ),
        display_name: ActiveValue::Set(
            req.display_name
                .ok_or_else(|| AppError::BadRequest("Empty display_name".to_string()))?,
        ),
        value_type: ActiveValue::Set(
            req.value_type
                .ok_or_else(|| AppError::BadRequest("Empty value_type".to_string()))?,
        ),
        is_multi_value: ActiveValue::Set(req.is_multi_value.unwrap_or(false)),
        sort_order: ActiveValue::Set(req.sort_order.unwrap_or(0)),
        created_at: ActiveValue::Set(now),
        updated_at: ActiveValue::Set(now),
    };
    let model = model.insert(&state.db).await?;
    Ok((StatusCode::CREATED, Json(model.into())))
}

async fn list_facet(
    State(state): State<Arc<AppState>>,
    Query(q): Query<ListQuery>,
) -> AppResult<Json<ListResponse<Facet>>> {
    let limit = q.limit.unwrap_or(20).min(100);
    let offset = q.offset.unwrap_or(0);

    let total = FacetEntity::find().count(&state.db).await?;

    let facets = FacetEntity::find()
        .order_by_asc(FacetColumn::SortOrder)
        .offset(offset)
        .limit(limit)
        .all(&state.db)
        .await?
        .into_iter()
        .map(Facet::from)
        .collect();

    Ok(Json(ListResponse {
        data: facets,
        total,
    }))
}

async fn get_facet(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Facet>> {
    let model = FacetEntity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("facet".to_string()))?;
    Ok(Json(model.into()))
}

async fn update_facet(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<FacetPatch>,
) -> AppResult<Json<Facet>> {
    let model = FacetEntity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(AppError::NotFound("facet".to_string()))?;

    let mut active: FacetActiveModel = model.into();
    let now = Utc::now().fixed_offset();

    update_am!(
        active,
        req,
        key,
        display_name,
        value_type,
        is_multi_value,
        sort_order,
    );
    active.updated_at = ActiveValue::Set(now);

    let model = active.update(&state.db).await?;
    Ok(Json(model.into()))
}

async fn delete_facet(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    FacetEntity::delete_by_id(id).exec(&state.db).await?;
    Ok(StatusCode::NO_CONTENT)
}
