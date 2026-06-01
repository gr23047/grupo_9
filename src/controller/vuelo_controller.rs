use axum::{routing::{delete, get, post, put}, Router};
use sqlx::PgPool;
use crate::service::vuelo_service::{
    obtener_vuelos , 
    obtener_vuelo_por_id,
    crear_vuelo,
    actualizar_vuelo, 
    actualizar_vuelo_por_id, 
    eliminar_vuelo, 
    eliminar_vuelo_por_id
};

pub fn vuelo_router(pool: PgPool) -> Router{
    Router::new()
        .route("/api/vuelos", get(obtener_vuelos)
                                .post(crear_vuelo)
                                .put(actualizar_vuelo)
                                .delete(eliminar_vuelo))
        .route("/api/vuelos/{id_vuelo}", get(obtener_vuelo_por_id)
                                .put(actualizar_vuelo_por_id)
                                .delete(eliminar_vuelo_por_id))
        .with_state(pool)
}