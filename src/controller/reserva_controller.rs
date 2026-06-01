use axum::{
     routing::{get, post, put, delete},
     Router,
};

use sqlx::PgPool;
use crate::service::reserva_service::{crear_reserva, obtener_reservas, obtener_reserva_por_id, actualizar_reserva_por_id, actualizar_reserva, eliminar_reserva_por_id, eliminar_reserva};

pub fn reserva_router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/reservas", post(crear_reserva).get(obtener_reservas).put(actualizar_reserva))
        .route("/api/reservas/{id_reserva}", get(obtener_reserva_por_id).put(actualizar_reserva_por_id).delete(eliminar_reserva_por_id))
        .route("/api/reservas", delete(eliminar_reserva))
        .with_state(pool)
}