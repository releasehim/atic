use std::fs::File;
use std::io::prelude::*;
use std::fmt::Display;
use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum ColorAuto {
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Auto {
    pub marca: String,
    pub modelo: String,
    pub anio: u32,
    pub precio_bruto: f64,
    pub color: ColorAuto,
}

impl Auto {
    pub fn new(marca: String, modelo: String, anio: u32, precio_bruto: f64, color: ColorAuto) -> Self {
        Self {
            marca,
            modelo,
            anio,
            precio_bruto,
            color,
        }
    }

    pub fn calcular_precio(&self) -> f64 {
        let mut precio = self.precio_bruto;
        let es_primario = match self.color {
            ColorAuto::Rojo | ColorAuto::Amarillo | ColorAuto::Azul => true,
            _ => false,
        };
        if es_primario {
            precio += self.precio_bruto * 0.25;
        } else {
            precio -= self.precio_bruto * 0.10;
        }
        if self.marca == "BMW" {
            precio += self.precio_bruto * 0.15;
        }
        if self.anio < 2000 {
            precio -= self.precio_bruto * 0.05;
        }
        precio
    }
}

#[derive(Debug)]
pub struct LimiteExcedidoError(pub String);

impl Display for LimiteExcedidoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct ConcesionarioAuto {
    pub nombre: String,
    pub direccion: String,
    pub capacidad_maxima: usize,
    pub file_path: String,
}

impl ConcesionarioAuto {
    pub fn new(nombre: String, direccion: String, capacidad_maxima: usize, file_path: String) -> Self {
        if File::open(&file_path).is_err() {
            let mut f = File::create(&file_path).unwrap();
            f.write_all(b"[]").unwrap();
        }
        Self {
            nombre,
            direccion,
            capacidad_maxima,
            file_path,
        }
    }

    pub fn leer_autos(&self) -> Vec<Auto> {
        let mut f = match File::open(&self.file_path) {
            Ok(file) => file,
            Err(_) => return Vec::new(),
        };
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();
        serde_json::from_str(&buf).unwrap_or_else(|_| Vec::new())
    }

    pub fn guardar_autos(&self, autos: &Vec<Auto>) {
        let data = serde_json::to_string(autos).unwrap();
        let mut f = File::create(&self.file_path).unwrap();
        f.write_all(data.as_bytes()).unwrap();
    }

    pub fn agregar_auto(&mut self, auto: Auto) -> Result<(), LimiteExcedidoError> {
        let mut autos = self.leer_autos();
        if autos.len() >= self.capacidad_maxima {
            return Err(LimiteExcedidoError(format!(
                "No se puede agregar el auto. Capacidad máxima de {} superada.",
                self.capacidad_maxima
            )));
        }
        autos.push(auto);
        self.guardar_autos(&autos);
        Ok(())
    }

    pub fn eliminar_auto(&mut self, auto: &Auto) -> bool {
        let mut autos = self.leer_autos();
        let mut index = None;
        let mut i = 0;
        while i < autos.len() {
            if autos[i] == *auto {
                index = Some(i);
                break;
            }
            i += 1;
        }
        if let Some(idx) = index {
            autos.remove(idx);
            self.guardar_autos(&autos);
            true
        } else {
            false
        }
    }

    pub fn buscar_auto(&self, auto: &Auto) -> Option<Auto> {
        let autos = self.leer_autos();
        for a in autos {
            if a == *auto {
                return Some(a);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concesionario() {
        let file_path = "src/test_concesionario.json".to_string();
        let _ = std::fs::remove_file(&file_path);

        let mut c = ConcesionarioAuto::new("Mio".to_string(), "Calle 4".to_string(), 2, file_path.clone());
        let a1 = Auto::new("BMW".to_string(), "M3".to_string(), 2020, 10000.0, ColorAuto::Rojo);
        let a2 = Auto::new("Fiat".to_string(), "Uno".to_string(), 1998, 2000.0, ColorAuto::Verde);
        let a3 = Auto::new("Ford".to_string(), "Focus".to_string(), 2015, 5000.0, ColorAuto::Negro);

        assert_eq!(a1.calcular_precio(), 14000.0); 
        assert_eq!(a2.calcular_precio(), 1700.0); 

        assert!(c.agregar_auto(a1.clone()).is_ok());
        assert!(c.agregar_auto(a2.clone()).is_ok());

        let err = c.agregar_auto(a3.clone());
        assert!(err.is_err());
        assert_eq!(err.unwrap_err().0, "No se puede agregar el auto. Capacidad máxima de 2 superada.");

        assert_eq!(c.buscar_auto(&a1), Some(a1.clone()));
        assert!(c.eliminar_auto(&a1));
        assert_eq!(c.buscar_auto(&a1), None);
        assert!(!c.eliminar_auto(&a1));

        let _ = std::fs::remove_file(&file_path);
    }
}
