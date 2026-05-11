use sqlx::{PgPool, Row};
use crate::models::avion::{Avion, nuevo_avion, actualizar_avion};

pub struct AvionRepository {
    pool: PgPool,
}

impl AvionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn obtener_aviones(&self) -> sqlx::Result<Vec<Avion>> {
        let filas = sqlx::query("SELECT id_avion, modelo, capacidad_pasajeros, fabricante FROM aviones")
            .fetch_all(&self.pool)
            .await?;

        let aviones = filas.into_iter().map(|fila| Avion {
            id_avion: fila.get("id_avion"),
            modelo: fila.get("modelo"),
            capacidad_pasajeros: fila.get("capacidad_pasajeros"),
            fabricante: fila.get("fabricante"),
        }).collect();
        Ok(aviones)
    }

    pub async fn obtener_avion_por_id(&self, id_avion: i32) -> sqlx::Result<Avion> {
        let fila = sqlx::query("SELECT id_avion, modelo, capacidad_pasajeros, fabricante FROM aviones WHERE id_avion = $1")
            .bind(id_avion)
            .fetch_one(&self.pool)
            .await?;

        Ok(Avion {
            id_avion: fila.get("id_avion"),
            modelo: fila.get("modelo"),
            capacidad_pasajeros: fila.get("capacidad_pasajeros"),
            fabricante: fila.get("fabricante"),
        })
    }   
    
    pub async fn crear_avion(&self, nuevo_avion: nuevo_avion) -> sqlx::Result<Avion> {
        let fila = sqlx::query("INSERT INTO aviones (modelo, capacidad_pasajeros, fabricante) VALUES ($1, $2, $3) RETURNING id_avion")
            .bind(&nuevo_avion.modelo)
            .bind(nuevo_avion.capacidad_pasajeros)
            .bind(&nuevo_avion.fabricante)
            .fetch_one(&self.pool)
            .await?;

        let id_avion: i32 = fila.get("id_avion");
        Ok(Avion {
            id_avion,
            modelo: nuevo_avion.modelo,
            capacidad_pasajeros: nuevo_avion.capacidad_pasajeros,
            fabricante: nuevo_avion.fabricante,
        })
    }

    pub async fn actualizar_avion(&self, id_avion: i32, avion_actualizado: actualizar_avion) -> sqlx::Result<Avion> {
        let fila = sqlx::query("UPDATE aviones SET modelo = $1, capacidad_pasajeros = $2, fabricante = $3 WHERE id_avion = $4 RETURNING id_avion, modelo, capacidad_pasajeros, fabricante")
            .bind(&avion_actualizado.modelo)
            .bind(avion_actualizado.capacidad_pasajeros)
            .bind(&avion_actualizado.fabricante)
            .bind(id_avion)
            .fetch_one(&self.pool)
            .await?;

        let avion = Avion {
            id_avion: fila.get("id_avion"),
            modelo: fila.get("modelo"),
            capacidad_pasajeros: fila.get("capacidad_pasajeros"),
            fabricante: fila.get("fabricante"),
        };
        Ok(avion)
    }
    
    pub async fn eliminar_avion(&self, id_avion: i32) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM aviones WHERE id_avion = $1")
            .bind(id_avion)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn eliminar_avion_por_id(&self, id_avion: i32) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM aviones WHERE id_avion = $1")
            .bind(id_avion)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn actualizar_avion_por_id(&self, id_avion: i32, avion_actualizado: actualizar_avion) -> sqlx::Result<Avion> {
        let fila = sqlx::query("UPDATE aviones SET modelo = $1, capacidad_pasajeros = $2, fabricante = $3 WHERE id_avion = $4 RETURNING id_avion")
            .bind(&avion_actualizado.modelo)
            .bind(avion_actualizado.capacidad_pasajeros)
            .bind(&avion_actualizado.fabricante)
            .bind(id_avion)
            .fetch_one(&self.pool)
            .await?;

        let id_avion: i32 = fila.get("id_avion");
        Ok(Avion {
            id_avion,
            modelo: avion_actualizado.modelo,
            capacidad_pasajeros: avion_actualizado.capacidad_pasajeros,
            fabricante: avion_actualizado.fabricante,
        })
    }



}