use crate::models::aeropuerto::{ActualizarAeropuerto, Aeropuerto, NuevoAeropuerto};
use crate::repository::aeropuerto_repository::AeropuertoRepository;
use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::PgPool;

pub async fn obtener_aeropuertos(
    State(pool): State<PgPool>,
) -> Json<Vec<crate::models::aeropuerto::Aeropuerto>> {
    let aeropuertos = AeropuertoRepository::new(pool);
    match aeropuertos.obtener_aeropuertos().await {
        Ok(aeropuertos) => Json(aeropuertos),
        Err(_) => Json(vec![]),
    }
}

pub async fn obtener_aeropuerto_por_id(
    State(pool): State<PgPool>,
    Path(id_aeropuerto): Path<i32>,
) -> Json<Aeropuerto> {
    let repo = AeropuertoRepository::new(pool);
    match repo.obtener_aeropuerto_por_id(id_aeropuerto).await {
        Ok(repo) => Json(repo),
        Err(_) => Json(Aeropuerto {
            id_aeropuerto: 0,
            codigo_iata: "404".to_string(),
            nombre: format!(
                "Error: Aeropuerto con id {} no encontrado", id_aeropuerto
            ),
            ciudad: format!(
                "Error: Ciudad del aeropuerto {} no encontrada",
                id_aeropuerto
            ),
        }),
    }
}

pub async fn crear_aeropuerto(
    State(pool): State<PgPool>,
    Json(nuevo_aeropuerto): Json<NuevoAeropuerto>,
) -> Json<crate::models::aeropuerto::Aeropuerto> {
    let aeropuertos = AeropuertoRepository::new(pool);
    match aeropuertos.crear_aeropuerto(nuevo_aeropuerto).await {
        Ok(aeropuerto) => Json(aeropuerto),
        Err(_) => Json(Aeropuerto {
            id_aeropuerto: 0,
            codigo_iata: "Error".to_string(),
            nombre: "Error: No se pudo crear el aeropuerto".to_string(),
            ciudad: "Error: Ciudad no registrada".to_string(),
        }),
    }
}

pub async fn actualizar_aeropuerto(
    State(pool): State<PgPool>,
    Json(aeropuerto_actualizado): Json<crate::models::aeropuerto::Aeropuerto>,
) -> Json<crate::models::aeropuerto::Aeropuerto> {
    let aeropuertos = AeropuertoRepository::new(pool);
    let id_aeropuerto = aeropuerto_actualizado.id_aeropuerto;
    let nuevo_aeropuerto = ActualizarAeropuerto {
        codigo_iata: aeropuerto_actualizado.codigo_iata,
        nombre: aeropuerto_actualizado.nombre,
        ciudad: aeropuerto_actualizado.ciudad,
    };
    match aeropuertos
        .actualizar_aeropuerto(id_aeropuerto, nuevo_aeropuerto)
        .await
    {
        Ok(aeropuerto) => Json(aeropuerto),
        Err(_) => Json(Aeropuerto {
            id_aeropuerto: 0,
            codigo_iata: "Error".to_string(),
            nombre: format!(
                "Error: Aeropuerto con id {} no pudo actualizarse",
                id_aeropuerto
            ),
            ciudad: format!(
                "Error: Ciudad del aeropuerto {} no actualizada",
                id_aeropuerto
            ),
        }),
    }
}

pub async fn actualizar_aeropuerto_por_id(
    State(pool): State<PgPool>,
    Path(id_aeropuerto): Path<i32>,
    Json(aeropuerto_actualizado): Json<ActualizarAeropuerto>,
) -> Json<crate::models::aeropuerto::Aeropuerto> {
    let aeropuertos = AeropuertoRepository::new(pool);
    let nuevo_aeropuerto = ActualizarAeropuerto {
        codigo_iata: aeropuerto_actualizado.codigo_iata,
        nombre: aeropuerto_actualizado.nombre,
        ciudad: aeropuerto_actualizado.ciudad,
    };
    match aeropuertos
        .actualizar_aeropuerto(id_aeropuerto, nuevo_aeropuerto)
        .await
    {
        Ok(aeropuerto) => Json(aeropuerto),
        Err(_) => Json(Aeropuerto {
            id_aeropuerto: 0,
            codigo_iata: "Error".to_string(),
            nombre: format!(
                "Error: Aeropuerto con id {} no pudo actualizarse",
                id_aeropuerto
            ),
            ciudad: format!(
                "Error: Ciudad del aeropuerto {} no actualizada",
                id_aeropuerto
            ),
        }),
    }
}

pub async fn eliminar_aeropuerto(
    State(pool): State<PgPool>,
    Json(aeropuerto): Json<crate::models::aeropuerto::Aeropuerto>,
) -> Json<bool> {
    let aeropuertos = AeropuertoRepository::new(pool);
    let id_aeropuerto = aeropuerto.id_aeropuerto;
    match aeropuertos.eliminar_aeropuerto(id_aeropuerto).await {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}

pub async fn eliminar_aeropuerto_por_id(
    State(pool): State<PgPool>,
    Path(id_aeropuerto): Path<i32>,
) -> Json<bool> {
    let aeropuertos = AeropuertoRepository::new(pool);
    match aeropuertos.eliminar_aeropuerto(id_aeropuerto).await {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}
