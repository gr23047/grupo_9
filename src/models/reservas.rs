use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, PartialEq)]
pub struct Reserva {
    pub reserva_id: i32,
    pub vuelo_id: i32,
    pub pasajero_id: i32,
    pub asiento: String,
    pub precio_boleto: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NuevaReserva {
    pub vuelo_id: i32,
    pub pasajero_id: i32,
    pub asiento: String,
    pub precio_boleto: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActualizarReserva {
    pub vuelo_id: i32,
    pub pasajero_id: i32,
    pub asiento: String,
    pub precio_boleto: f64,
}