use axum::{
    routing::{get, post, put, delete},
    Router
};
use sqlx::PgPool;
use crate::service::avion_service::{
    obtener_aviones, 
    obtener_avion_por_id,
    crear_avion, 
    actualizar_avion, 
    actualizar_avion_por_id, 
    eliminar_avion, 
    eliminar_avion_por_id};

    pub fn avion_router(pool: PgPool) -> Router {
        Router::new()
            .route("/api/aviones", get(obtener_aviones))
            .route("/api/aviones/{id_avion}", get(obtener_avion_por_id))
            .route("/api/aviones", post(crear_avion))
            .route("/api/aviones", put(actualizar_avion))
            .route("/api/aviones/{id_avion}", put(actualizar_avion_por_id))
            .route("/api/aviones", delete(eliminar_avion))
            .route("/api/aviones/{id_avion}", delete(eliminar_avion_por_id))
            .with_state(pool)
    }
            

