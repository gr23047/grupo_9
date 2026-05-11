use axum::{routing::{delete, get, post, put}, Router};
use sqlx::PgPool;
use crate::service::aeropuerto_service::{
    obtener_aeropuertos , 
    obtener_aeropuerto_por_id,
    crear_aeropuerto,
    actualizar_aeropuerto, 
    actualizar_aeropuerto_por_id, 
    eliminar_aeropuerto, 
    eliminar_aeropuerto_por_id
};

pub fn aeropuerto_router(pool: PgPool) -> Router{
    Router::new()
        .route("/api/aeropuertos", get(obtener_aeropuertos)
                                .post(crear_aeropuerto)
                                .put(actualizar_aeropuerto)
                                .delete(eliminar_aeropuerto))
        .route("/api/aeropuertos/{id_aeropuerto}", get(obtener_aeropuerto_por_id)
                                .put(actualizar_aeropuerto_por_id)
                                .delete(eliminar_aeropuerto_por_id))
        .with_state(pool)
}