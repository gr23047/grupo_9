use sqlx::{PgPool, Error};
use crate::models::reservas::{Reserva, NuevaReserva,ActualizarReserva};
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;

pub struct ReservaRepository {
    pool: PgPool,
}

impl ReservaRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn crear_reserva(&self, nueva_reserva: NuevaReserva) -> Result<Reserva, Error> {
        let reserva = sqlx::query_as::<_, Reserva>(
            r#"
            INSERT INTO reservas (id_vuelo, id_pasajero, asiento, precio_boleto)
            VALUES ($1, $2, $3, $4)
            RETURNING id_reserva, id_vuelo, id_pasajero, asiento, precio_boleto
            "#,
        )
        .bind(nueva_reserva.id_vuelo)
        .bind(nueva_reserva.id_pasajero)
        .bind(nueva_reserva.asiento)
        .bind(nueva_reserva.precio_boleto)
        .fetch_one(&self.pool)
        .await?;

        Ok(reserva)
    }

    pub async fn obtener_reservas(&self) -> Result<Vec<Reserva>, Error> {
        let reservas = sqlx::query_as::<_, Reserva>(
            r#"
            SELECT id_reserva, id_vuelo, id_pasajero, asiento, precio_boleto
            FROM reservas
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(reservas)
    }

    pub async fn obtener_reserva_por_id(&self, id_reserva: i32) -> Result<Reserva, Error> {
        let reserva = sqlx::query_as::<_, Reserva>(
            r#"
            SELECT id_reserva, id_vuelo, id_pasajero, asiento, precio_boleto
            FROM reservas
            WHERE id_reserva = $1
            "#,
        )
        .bind(id_reserva)
        .fetch_one(&self.pool)
        .await?;

        Ok(reserva)
    }


    pub async fn actualizar_reserva(&self, id_reserva: i32, actualizar_reserva: ActualizarReserva) -> Result<Reserva, Error> {
        let reserva = sqlx::query_as::<_, Reserva>(
            r#"
            UPDATE reservas
            SET id_vuelo = $1, id_pasajero = $2, asiento = $3, precio_boleto = $4
            WHERE id_reserva = $5
            RETURNING id_reserva, id_vuelo, id_pasajero, asiento, precio_boleto
            "#,
        )
        .bind(actualizar_reserva.id_vuelo)
        .bind(actualizar_reserva.id_pasajero)
        .bind(actualizar_reserva.asiento)
        .bind(actualizar_reserva.precio_boleto)
        .bind(id_reserva)
        .fetch_one(&self.pool)
        .await?;
        Ok(reserva)
    }

    pub async fn actualizar_reserva_por_id(&self, id_reserva: i32, actualizar_reserva: ActualizarReserva) -> Result<Reserva, Error> {
        let reserva = sqlx::query_as::<_, Reserva>(
            r#"
            UPDATE reservas
            SET id_vuelo = $1, id_pasajero = $2, asiento = $3, precio_boleto = $4
            WHERE id_reserva = $5
            RETURNING id_reserva, id_vuelo, id_pasajero, asiento, precio_boleto
            "#,
        )
        .bind(actualizar_reserva.id_vuelo)
        .bind(actualizar_reserva.id_pasajero)
        .bind(actualizar_reserva.asiento)
        .bind(actualizar_reserva.precio_boleto)
        .bind(id_reserva)
        .fetch_one(&self.pool)
        .await?;
        Ok(reserva)
    }

    pub async fn eliminar_reserva(&self, id_reserva: i32) -> Result<(), Error> {
        sqlx::query(
            r#"
            DELETE FROM reservas
            WHERE id_reserva = $1
            "#,
        )
        .bind(id_reserva)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn eliminar_reserva_por_id(&self, id_reserva: i32) -> Result<(), Error> {
        sqlx::query(
            r#"
            DELETE FROM reservas
            WHERE id_reserva = $1
            "#,
        )
        .bind(id_reserva)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}