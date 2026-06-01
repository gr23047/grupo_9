use axum::{extract::{Path, State}, Json};
use serde_json::json;
use sqlx::PgPool;
use crate::repository::reserva_repository::ReservaRepository;
use crate::models::reservas::{Reserva, NuevaReserva, ActualizarReserva};

pub async fn crear_reserva(
    State(pool): State<PgPool>,
    Json(nueva_reserva): Json<NuevaReserva>,
) -> Result<Json<Reserva>, String> {
    let repo = ReservaRepository::new(pool);
    match repo.crear_reserva(nueva_reserva).await {
        Ok(reserva) => Ok(Json(reserva)),
        Err(e) => Err(format!("Error al crear reserva: {}", e)),
    }
}

pub async fn obtener_reservas(State(pool): State<PgPool>) -> Result<Json<Vec<Reserva>>, String> {
    let repo = ReservaRepository::new(pool);
    match repo.obtener_reservas().await {
        Ok(reservas) => Ok(Json(reservas)),
        Err(e) => Err(format!("Error al obtener reservas: {}", e)),
    }
}

pub async fn obtener_reserva_por_id(
    State(pool): State<PgPool>,
    Path(id_reserva): Path<i32>,
) -> Result<Json<Reserva>, String> {
    let repo = ReservaRepository::new(pool);
    match repo.obtener_reserva_por_id(id_reserva).await {
        Ok(reserva) => Ok(Json(reserva)),
        Err(e) => Err(format!("Error al obtener reserva por ID: {}", e)),
    }
}

pub async fn actualizar_reserva_por_id(State(pool): State<PgPool>,   Path(id_reserva): Path<i32>,  Json(reserva_actualizada): Json<ActualizarReserva>,) ->Json<Reserva>{
    let repo = ReservaRepository::new(pool);
    match repo.actualizar_reserva_por_id(id_reserva, reserva_actualizada).await {
        Ok(reserva) => Json(reserva),
        Err(e) => Json(Reserva {
            id_reserva,
            id_vuelo: 0,
            id_pasajero: 0,
            asiento: "Error al actualizar reserva".to_string(),
            precio_boleto: rust_decimal::Decimal::new(0, 0),
        }),
    }
}

pub async fn actualizar_reserva(State(pool): State<PgPool>,Json(reserva_actualizada): Json<crate::models::reservas::Reserva>) -> Json<crate::models::reservas::Reserva> {
    let repo = ReservaRepository::new(pool);
    let id_reserva = reserva_actualizada.id_reserva;
    let nueva_reserva = ActualizarReserva {
        id_vuelo: reserva_actualizada.id_vuelo,
        id_pasajero: reserva_actualizada.id_pasajero,
        asiento: reserva_actualizada.asiento,
        precio_boleto: reserva_actualizada.precio_boleto,
    };
    match repo.actualizar_reserva_por_id(id_reserva, nueva_reserva).await {
        Ok(reserva) => Json(reserva),
        Err(e) => Json(Reserva {
            id_reserva,
            id_vuelo: 0,
            id_pasajero: 0,
            asiento: "Error al actualizar reserva".to_string(),
            precio_boleto: rust_decimal::Decimal::new(0, 0),
        }),
    }
}

pub async fn eliminar_reserva(State(pool): State<PgPool>, Json(Reserva): Json<crate::models::reservas::Reserva>) -> Json<bool>{
    let repo = ReservaRepository::new(pool);
    let id_reserva= Reserva.id_reserva;
    match repo.eliminar_reserva_por_id(id_reserva).await {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}

pub async fn eliminar_reserva_por_id(State(pool): State<PgPool>, Path(id_reserva): Path<i32>,) -> Json<bool> {
    let repo = ReservaRepository::new(pool);
    match repo.eliminar_reserva_por_id(id_reserva).await {
        Ok(_) => Json(false),
        Err(_) => Json(true ),
    }
}
