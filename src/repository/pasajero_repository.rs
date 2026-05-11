use sqlx::{PgPool, Row};
use crate::models::pasajero::{ ActualizarPasajero, NuevoPasajero, Pasajero};

pub struct PasajeroRepository {
    pool: PgPool,
}

impl PasajeroRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    

    pub async fn obtener_pasajeros(&self)->sqlx:: Result<Vec<Pasajero>>{
        let filas = sqlx::query("SELECT id_pasajero, nombre, pasaporte, nacionalidad FROM pasajeros")
            .fetch_all(&self.pool)
            .await?;
        let pasajeros = filas.into_iter().map(|fila| 
            Pasajero {
            id_pasajero: fila.get("id_pasajero"),
            nombre: fila.get("nombre"),
            pasaporte: fila.get("pasaporte"),
            nacionalidad: fila.get("nacionalidad"),
        }).collect();
        Ok(pasajeros)
    }

    pub async fn obtener_pasajero_por_id(&self, id_pasajero: i32) -> sqlx::Result<Pasajero> {
        let fila = sqlx::query("SELECT id_pasajero, nombre, pasaporte, nacionalidad FROM pasajeros WHERE id_pasajero = $1")
            .bind(id_pasajero)
            .fetch_one(&self.pool)
            .await?;
        let pasajero=Pasajero {
           id_pasajero: fila.get("id_pasajero"),
           nombre: fila.get("nombre"),
            pasaporte: fila.get("pasaporte"),
           nacionalidad: fila.get("nacionalidad"),
        };
        Ok(pasajero)
    }

    pub async fn crear_pasajero(&self, nuevo_pasajero: NuevoPasajero) -> sqlx::Result<Pasajero> {
        let fila = sqlx::query("INSERT INTO pasajeros (nombre, pasaporte, nacionalidad) VALUES ($1, $2, $3) RETURNING id_pasajero, nombre, pasaporte, nacionalidad")
            .bind(nuevo_pasajero.nombre)
            .bind(nuevo_pasajero.pasaporte)
            .bind(nuevo_pasajero.nacionalidad)
            .fetch_one(&self.pool)
            .await?;
        let pasajero=Pasajero {
           id_pasajero: fila.get("id_pasajero"),
           nombre: fila.get("nombre"),
            pasaporte: fila.get("pasaporte"),
           nacionalidad: fila.get("nacionalidad"),
        };

        Ok(pasajero)
    }

    pub async fn actualizar_pasajero(&self, id_pasajero: i32, pasajero_actualizado: ActualizarPasajero) -> sqlx::Result<Pasajero> {
        let fila = sqlx::query("UPDATE pasajeros SET nombre = $1, pasaporte = $2, nacionalidad = $3 WHERE id_pasajero = $4 RETURNING id_pasajero, nombre, pasaporte, nacionalidad")
            .bind(pasajero_actualizado.nombre)
            .bind(pasajero_actualizado.pasaporte)
            .bind(pasajero_actualizado.nacionalidad)
            .bind(id_pasajero)
            .fetch_one(&self.pool)
            .await?;
        let pasajero=Pasajero {
           id_pasajero: fila.get("id_pasajero"),
           nombre: fila.get("nombre"),
            pasaporte: fila.get("pasaporte"),
           nacionalidad: fila.get("nacionalidad"),
        };
        Ok(pasajero)
    }

    pub async fn eliminar_pasajero(&self, id_pasajero: i32) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM pasajeros WHERE id_pasajero = $1")
            .bind(id_pasajero)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

}
    
