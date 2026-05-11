use sqlx::{PgPool, Row};
use crate::models::aeropuerto::{Aeropuerto, NuevoAeropuerto, ActualizarAeropuerto};

pub struct AeropuertoRepository {
    pool: PgPool,
}

impl AeropuertoRepository{
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn obtener_aeropuertos(&self) ->sqlx::Result<Vec<Aeropuerto>> {
        let filas = sqlx::query("SELECT id_aeropuerto, codigo_iata, nombre, ciudad FROM aeropuertos")
            .fetch_all(&self.pool)
            .await?;
        
        let aeropuertos = filas.into_iter().map(|fila| Aeropuerto{
            id_aeropuerto: fila.get("id_aeropuerto"),
            codigo_iata: fila.get("codigo_iata"),
            nombre: fila.get("nombre"),
            ciudad: fila.get("ciudad"),
        }).collect();
        Ok(aeropuertos)
    }

    pub async fn obtener_aeropuerto_por_id(&self, id_aeropuerto: i32) -> sqlx::Result<Aeropuerto> {
        let fila = sqlx::query("SELECT id_aeropuerto, codigo_iata, nombre, ciudad FROM aeropuertos WHERE id_aeropuerto = $1")
            .bind(id_aeropuerto)
            .fetch_one(&self.pool)
            .await?;
        let aeropuerto=Aeropuerto {
            id_aeropuerto: fila.get("id_aeropuerto"),
            codigo_iata: fila.get("codigo_iata"),
            nombre: fila.get("nombre"),
            ciudad: fila.get("ciudad"),
        };
        Ok(aeropuerto)
    }
    


    pub async fn crear_aeropuerto(&self, nuevo_aeropuerto: NuevoAeropuerto) -> sqlx::Result<Aeropuerto> {
        let fila = sqlx::query("INSERT INTO aeropuertos (codigo_iata, nombre, ciudad) VALUES ($1, $2, $3) RETURNING id_aeropuerto, codigo_iata, nombre, ciudad")
            .bind(&nuevo_aeropuerto.codigo_iata)
            .bind(&nuevo_aeropuerto.nombre)
            .bind(&nuevo_aeropuerto.ciudad)
            .fetch_one(&self.pool)
            .await?;
        let aeropuerto = Aeropuerto {
            id_aeropuerto: fila.get("id_aeropuerto"),
            codigo_iata: fila.get("codigo_iata"),
            nombre: fila.get("nombre"),
            ciudad: fila.get("ciudad"),
        };
        Ok(aeropuerto)
    }

    pub async fn actualizar_aeropuerto(&self, id_aeropuerto: i32, aeropuerto_actualizado: ActualizarAeropuerto) -> sqlx::Result<Aeropuerto> {
        let fila = sqlx::query("UPDATE aeropuertos SET codigo_iata = $1, nombre = $2, ciudad = $3 WHERE id_aeropuerto = $4 RETURNING id_aeropuerto, codigo_iata, nombre, ciudad")
            .bind(&aeropuerto_actualizado.codigo_iata)
            .bind(&aeropuerto_actualizado.nombre)
            .bind(&aeropuerto_actualizado.ciudad)
            .bind(id_aeropuerto)
            .fetch_one(&self.pool)
            .await?;

        let aeropuerto = Aeropuerto {
            id_aeropuerto: fila.get("id_aeropuerto"),
            codigo_iata: fila.get("codigo_iata"),
            nombre: fila.get("nombre"),
            ciudad: fila.get("ciudad"),
        };
        Ok(aeropuerto)
    }

    pub async fn eliminar_aeropuerto(&self, id_aeropuerto: i32) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM aeropuertos WHERE id_aeropuerto = $1")
            .bind(id_aeropuerto)
            .execute(&self.pool)
            .await?;
        Ok(())    
        }
    }