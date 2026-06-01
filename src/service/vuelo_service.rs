use crate::models::vuelo::{ActualizarVuelo, NuevoVuelo, Vuelo};
use crate::repository::vuelo_repository::VueloRepository;

use axum::{
    Json,
    extract::{Path, State},
};

use sqlx::PgPool;

pub async fn obtener_vuelos(State(pool): State<PgPool>) -> Json<Vec<crate::models::vuelo::Vuelo>> {
    let vuelos = VueloRepository::new(pool);
    match vuelos.obtener_vuelos().await {
        Ok(vuelos) => Json(vuelos),
        Err(_) => Json(vec![]),
    }
}

pub async fn obtener_vuelo_por_id(
    State(pool): State<PgPool>,
    Path(id_vuelo): Path<i32>,
) -> Json<Vuelo> {
    let repo = VueloRepository::new(pool);
    match repo.obtener_vuelo_por_id(id_vuelo).await {
        Ok(repo) => Json(repo),
        Err(_) => Json(Vuelo {
            id_vuelo: 0,
            id_avion: 0,
            numero_vuelo: "404".to_string(),
            id_aeropuerto_origen: 0,
            id_aeropuerto_destino: 0,
        }),
    }
}

pub async fn crear_vuelo(
    State(pool): State<PgPool>,
    Json(nuevo_vuelo): Json<NuevoVuelo>,
) -> Json<crate::models::vuelo::Vuelo> {
    let vuelos = VueloRepository::new(pool);
    match vuelos.crear_vuelo(nuevo_vuelo).await {
        Ok(vuelo) => Json(vuelo),
        Err(_) => Json(Vuelo {
            id_vuelo: 0,
            id_avion: 0,
            numero_vuelo: "Error".to_string(),
            id_aeropuerto_origen: 0,
            id_aeropuerto_destino: 0,
        }),
    }
}

pub async fn actualizar_vuelo(
    State(pool): State<PgPool>,
    Json(vuelo_actualizado): Json<ActualizarVuelo>,
) -> Json<crate::models::vuelo::Vuelo> {
    let vuelos = VueloRepository::new(pool);
    let id_vuelo = vuelo_actualizado.numero_vuelo.parse::<i32>().unwrap();
    match vuelos
        .actualizar_vuelo(id_vuelo, vuelo_actualizado)
        .await
    {
        Ok(vuelo) => Json(vuelo),
        Err(_) => Json(Vuelo {
            id_vuelo: 0,
            id_avion: 0,
            numero_vuelo: "Error".to_string(),
            id_aeropuerto_origen: 0,
            id_aeropuerto_destino: 0,
        }),
    }
}

pub async fn actualizar_vuelo_por_id(
    State(pool): State<PgPool>,
    Path(id_vuelo): Path<i32>,
    Json(vuelo_actualizado): Json<ActualizarVuelo>,
) -> Json<crate::models::vuelo::Vuelo> {
    let vuelos = VueloRepository::new(pool);
    match vuelos
        .actualizar_vuelo(id_vuelo, vuelo_actualizado)
        .await
    {
        Ok(vuelo) => Json(vuelo),
        Err(_) => Json(Vuelo {
            id_vuelo: 0,
            id_avion: 0,
            numero_vuelo: "Error".to_string(),
            id_aeropuerto_origen: 0,
            id_aeropuerto_destino: 0,
        }),
    }
}

pub async fn eliminar_vuelo(
    State(pool): State<PgPool>,
    Path(id_vuelo): Path<i32>,
) -> Json<String> {
    let vuelos = VueloRepository::new(pool);
    match vuelos.eliminar_vuelo(id_vuelo).await {
        Ok(_) => Json(format!("Vuelo con id {} eliminado exitosamente", id_vuelo)),
        Err(_) => Json(format!("Error: Vuelo con id {} no pudo ser eliminado", id_vuelo)),
    }
}

pub async fn eliminar_vuelo_por_id(
    State(pool): State<PgPool>,
    Path(id_vuelo): Path<i32>,
) -> Json<String> {
    let vuelos = VueloRepository::new(pool);
    match vuelos.eliminar_vuelo(id_vuelo).await {
        Ok(_) => Json(format!("Vuelo con id {} eliminado exitosamente", id_vuelo)),
        Err(_) => Json(format!("Error: Vuelo con id {} no pudo ser eliminado", id_vuelo)),
    }
}
