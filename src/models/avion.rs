use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]

pub struct Avion {
    pub id_avion: i32,
    pub modelo: String,
    pub capacidad_pasajeros: i32,
    pub fabricante: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct nuevo_avion {
    pub modelo: String,
    pub capacidad_pasajeros: i32,
    pub fabricante: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct actualizar_avion {
    pub modelo: String,
    pub capacidad_pasajeros: i32,
    pub fabricante: String,
}