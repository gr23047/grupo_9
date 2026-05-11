use axum::{
     routing::{get, post, put, delete},
     Router,
};

use sqlx::PgPool;

use crate::service::pasajero_service::{
    obtener_pasajeros,
    obtener_pasajero_por_id,
    crear_pasajero,
    actualizar_pasajero,
    eliminar_pasajero,
    eliminar_pasajero_por_id,
    actualizar_pasajero_por_id,
};

pub fn pasajero_router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/pasajeros", get(obtener_pasajeros))
        .route("/api/pasajeros", post(crear_pasajero))
        .route("/api/pasajeros", put(actualizar_pasajero))
        .route("/api/pasajeros/{id_pasajero}", get(obtener_pasajero_por_id))
        .route("/api/pasajeros/{id_pasajero}", put(actualizar_pasajero_por_id))
        .route("/api/pasajeros/{id_pasajero}", delete(eliminar_pasajero_por_id))
        .route("/api/pasajeros", delete(eliminar_pasajero))
        .with_state(pool)
} 