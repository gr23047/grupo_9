use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use rust_decimal::Decimal;

#[derive(Debug,Clone,Serialize, Deserialize, FromRow,PartialEq)]
pub struct Reserva {
    pub id_reserva: i32,
    pub id_vuelo: i32,
    pub id_pasajero: i32,
    pub asiento: String,
    pub precio_boleto: Decimal,
}

#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct NuevaReserva {
    pub id_vuelo: i32,
    pub id_pasajero: i32,
    pub asiento: String,
    pub precio_boleto: Decimal,
}

#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct ActualizarReserva {
    pub id_vuelo: i32,
    pub id_pasajero: i32,
    pub asiento: String,
    pub precio_boleto: Decimal,
}