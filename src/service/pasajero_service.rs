use crate::models::pasajero::{ActualizarPasajero, NuevoPasajero, Pasajero};
use crate::repository::pasajero_repository::PasajeroRepository;
use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::PgPool;

pub async fn obtener_pasajeros(State(pool): State<PgPool>) -> Json<Vec<Pasajero>> {
    let pasajeros = PasajeroRepository::new(pool);
    match pasajeros.obtener_pasajeros().await {
        Ok(pasajeros) => Json(pasajeros),
        Err(_) => Json(vec![]),
    }
}

pub async fn obtener_pasajero_por_id(State(pool): State<PgPool>, Path(id_pasajero): Path<i32>) -> Json<Pasajero> {
    let repo = PasajeroRepository::new(pool);
    match repo.obtener_pasajero_por_id(id_pasajero).await {
        Ok(pasajero) => Json(pasajero),
        Err(_) => Json(Pasajero {
            id_pasajero: 0,
            nombre: "Error al obtener el pasajero".to_string(),
            pasaporte: "Error al obtener el pasaporte".to_string(),
            nacionalidad: "Error al obtener la nacionalidad".to_string(),
        }),
    }
}

pub async fn crear_pasajero(
    State(pool): State<PgPool>,
    Json(nuevo_pasajero): Json<NuevoPasajero>,
) -> Json<crate::models::pasajero::Pasajero> {
    let repo = PasajeroRepository::new(pool);
    match repo.crear_pasajero(nuevo_pasajero).await {
        Ok(pasajero) => Json(pasajero),
        Err(_) => Json(Pasajero {
            id_pasajero: 0,
            nombre: "Error al crear el pasajero".to_string(),
            pasaporte: "Error al crear el pasaporte".to_string(),
            nacionalidad: "Error al crear la nacionalidad".to_string(),
        }),
    }
}

pub async fn actualizar_pasajero(
    State(pool): State<PgPool>,
    Json(pasajero_actualizado): Json<crate::models::pasajero::Pasajero>,
) -> Json<crate::models::pasajero::Pasajero> {
    let repo = PasajeroRepository::new(pool);
    let id_pasajero = pasajero_actualizado.id_pasajero;
    let nuevo_pasajero = ActualizarPasajero {
        nombre: pasajero_actualizado.nombre,
        pasaporte: pasajero_actualizado.pasaporte,
        nacionalidad: pasajero_actualizado.nacionalidad,
    };
    match repo.actualizar_pasajero(id_pasajero, nuevo_pasajero).await {
        Ok(pasajero) => Json(pasajero),
        Err(_) => Json(Pasajero {
            id_pasajero: 0,
            nombre: "Error al actualizar el pais".to_string(),
            pasaporte: "Error al actualizar el pais".to_string(),
            nacionalidad: "Error al actualizar el pais".to_string(),
        }),
    }
}

pub async fn actualizar_pasajero_por_id(
    State(pool): State<PgPool>,
    Path(id_pasajero): Path<i32>,
    Json(pasajero_actualizado): Json<ActualizarPasajero>,
) -> Json<crate::models::pasajero::Pasajero> {
    let repo = PasajeroRepository::new(pool);
    let nuevo_pasajero = ActualizarPasajero {
        nombre: pasajero_actualizado.nombre,
        pasaporte: pasajero_actualizado.pasaporte,
        nacionalidad: pasajero_actualizado.nacionalidad,
    };

    match repo.actualizar_pasajero(id_pasajero, nuevo_pasajero).await {
        Ok(pasajero) => Json(pasajero),
        Err(_) => Json(Pasajero {
            id_pasajero: 0,
            nombre: "Error al actualizar el pais".to_string(),
            pasaporte: "Error al actualizar el pais".to_string(),
            nacionalidad: "Error al actualizar el pais".to_string(),
        }),
    }
}

pub async fn eliminar_pasajero(
    State(pool): State<PgPool>,
    Json(pasajero): Json<Pasajero>,
) -> Json<bool> {
    let repo = PasajeroRepository::new(pool);
    match repo.eliminar_pasajero(pasajero.id_pasajero).await {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}

pub async fn eliminar_pasajero_por_id(
    State(pool): State<PgPool>,
    Path(id_pasajero): Path<i32>,
) -> Json<bool> {
    let repo = PasajeroRepository::new(pool);
    match repo.eliminar_pasajero(id_pasajero).await {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}
