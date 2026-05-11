use axum::{extract::{{Path, State}}, Json};
use sqlx::PgPool;
use crate::models::avion::{Avion, nuevo_avion, actualizar_avion};
use crate::repository::avion_repository::AvionRepository;

pub async fn obtener_aviones(State(pool): State<PgPool>) -> Json<Vec<Avion>> {
    let aviones =  AvionRepository::new(pool);
    match aviones.obtener_aviones().await {
        Ok(aviones) => Json(aviones),
        Err(_) => Json(vec![]),
    }
}

pub async fn obtener_avion_por_id(State(pool): State<PgPool>, Path(id_avion): Path<i32>) -> Json<Avion> {
    let aviones = AvionRepository::new(pool);
    match aviones.obtener_avion_por_id(id_avion).await {
        Ok(avion) => Json(avion),
        Err(_) => Json(Avion {
            id_avion,
            modelo: "Avión no encontrado".to_string(),
            capacidad_pasajeros: 0,
            fabricante: "Avión no encontrado".to_string(),
        }),
    }
}

pub async fn crear_avion(State(pool): State<PgPool>, Json(nuevo_avion): Json<nuevo_avion>) -> Json<Avion> {
    let aviones = AvionRepository::new(pool);
    match aviones.crear_avion(nuevo_avion).await {
        Ok(avion) => Json(avion),
        Err(_) => Json(Avion {
            id_avion: 0,
            modelo: "Error al crear avión".to_string(),
            capacidad_pasajeros: 0,
            fabricante: "Error al crear avión".to_string(),
        }),
    }
}

pub async fn actualizar_avion(State(pool): State<PgPool>, Json(avion_actualizado): Json<crate::models::avion::Avion>) -> 
Json<crate::models::avion::Avion> {
    let aviones = AvionRepository::new(pool);
    let id_avion = avion_actualizado.id_avion;
    let nuevo_avion = actualizar_avion {
        modelo: avion_actualizado.modelo,
        capacidad_pasajeros: avion_actualizado.capacidad_pasajeros,
        fabricante: avion_actualizado.fabricante,
    };
    match aviones.actualizar_avion(id_avion, nuevo_avion).await {
        Ok(avion) => Json(avion),
        Err(_) => Json(Avion {
            id_avion,
            modelo: "Error al actualizar avión".to_string(),
            capacidad_pasajeros: 0,
            fabricante: "Error al actualizar avión".to_string(),
        }),
    }
}

pub async fn actualizar_avion_por_id(State(pool): State<PgPool>, Path(id_avion): Path<i32>, Json(avion_actualizado): Json<actualizar_avion>) -> Json<Avion> {
    let aviones = AvionRepository::new(pool);
    match aviones.actualizar_avion_por_id(id_avion, avion_actualizado).await {
        Ok(avion) => Json(avion),
        Err(_) => Json(Avion {
            id_avion,
            modelo: "Error al actualizar avión".to_string(),
            capacidad_pasajeros: 0,
            fabricante: "Error al actualizar avión".to_string(),
        }),
    }
}

pub async fn eliminar_avion(State(pool): State<PgPool>, Json(avion): Json<crate::models::avion::Avion>) -> Json<bool> {
    let aviones = AvionRepository::new(pool);
    let id_avion = avion.id_avion;
    match aviones.eliminar_avion(id_avion).await {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}

pub async fn eliminar_avion_por_id(State(pool): State<PgPool>, Path(id_avion): Path<i32>) -> Json<bool> {
    let aviones = AvionRepository::new(pool);
    match aviones.eliminar_avion_por_id(id_avion).await {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}




    