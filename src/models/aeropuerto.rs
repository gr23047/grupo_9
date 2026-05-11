use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Aeropuerto {
    pub id_aeropuerto: i32,
    pub codigo_iata: String,
    pub nombre: String,
    pub ciudad: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuevoAeropuerto {
    pub codigo_iata: String,
    pub nombre: String,
    pub ciudad: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActualizarAeropuerto {
pub codigo_iata: String,
pub nombre: String,
pub ciudad: String,
}

