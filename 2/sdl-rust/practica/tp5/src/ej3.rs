use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;
use serde::{Serialize, Deserialize};
use crate::fecha::Fecha;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TipoAnimal {
    Perro,
    Gato,
    Caballo,
    Otros,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dueno {
    pub nombre: String,
    pub direccion: String,
    pub telefono: String,
}

impl Dueno {
    pub fn new(nombre: String, direccion: String, telefono: String) -> Self {
        Self {
            nombre,
            direccion,
            telefono,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mascota {
    pub nombre: String,
    pub edad: u32,
    pub tipo: TipoAnimal,
    pub dueno: Dueno,
}

impl Mascota {
    pub fn new(nombre: String, edad: u32, tipo: TipoAnimal, dueno: Dueno) -> Self {
        Self {
            nombre,
            edad,
            tipo,
            dueno,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Atencion {
    pub mascota: Mascota,
    pub diagnostico: String,
    pub tratamiento: String,
    pub proxima_visita: Option<Fecha>,
}

impl Atencion {
    pub fn new(mascota: Mascota, diagnostico: String, tratamiento: String, proxima_visita: Option<Fecha>) -> Self {
        Self {
            mascota,
            diagnostico,
            tratamiento,
            proxima_visita,
        }
    }
}

pub struct Veterinaria {
    pub nombre: String,
    pub direccion: String,
    pub id: u32,
    pub cola_atencion: VecDeque<Mascota>,
    pub file_path_atenciones: String,
}

impl Veterinaria {
    pub fn new(nombre: String, direccion: String, id: u32, file_path_atenciones: String) -> Self {
        if File::open(&file_path_atenciones).is_err() {
            let mut f = File::create(&file_path_atenciones).unwrap();
            f.write_all(b"[]").unwrap();
        }
        Self {
            nombre,
            direccion,
            id,
            cola_atencion: VecDeque::new(),
            file_path_atenciones,
        }
    }

    pub fn leer_atenciones(&self) -> Vec<Atencion> {
        let mut f = match File::open(&self.file_path_atenciones) {
            Ok(file) => file,
            Err(_) => return Vec::new(),
        };
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();
        serde_json::from_str(&buf).unwrap_or_else(|_| Vec::new())
    }

    pub fn guardar_atenciones(&self, atenciones: &Vec<Atencion>) {
        let data = serde_json::to_string(atenciones).unwrap();
        let mut f = File::create(&self.file_path_atenciones).unwrap();
        f.write_all(data.as_bytes()).unwrap();
    }

    pub fn agregar_mascota(&mut self, mascota: Mascota) {
        self.cola_atencion.push_back(mascota);
    }

    pub fn agregar_mascota_prioridad(&mut self, mascota: Mascota) {
        self.cola_atencion.push_front(mascota);
    }

    pub fn atender_proxima_mascota(&mut self) -> Option<Mascota> {
        self.cola_atencion.pop_front()
    }

    pub fn eliminar_mascota_cola(&mut self, nombre_mascota: &str, nombre_dueno: &str) -> Option<Mascota> {
        let mut nueva_cola = VecDeque::new();
        let mut mascota_eliminada = None;
        while let Some(mascota) = self.cola_atencion.pop_front() {
            if mascota.nombre == nombre_mascota && mascota.dueno.nombre == nombre_dueno {
                mascota_eliminada = Some(mascota);
            } else {
                nueva_cola.push_back(mascota);
            }
        }
        self.cola_atencion = nueva_cola;
        mascota_eliminada
    }

    pub fn registrar_atencion(&mut self, mascota: Mascota, diagnostico: String, tratamiento: String, proxima_visita: Option<Fecha>) {
        let mut atenciones = self.leer_atenciones();
        atenciones.push(Atencion::new(mascota, diagnostico, tratamiento, proxima_visita));
        self.guardar_atenciones(&atenciones);
    }

    pub fn buscar_atencion(&self, nombre_mascota: &str, nombre_dueno: &str, telefono: &str) -> Option<Atencion> {
        let atenciones = self.leer_atenciones();
        for a in atenciones {
            if a.mascota.nombre == nombre_mascota && a.mascota.dueno.nombre == nombre_dueno && a.mascota.dueno.telefono == telefono {
                return Some(a);
            }
        }
        None
    }

    pub fn modificar_diagnostico(&mut self, nombre_mascota: &str, nombre_dueno: &str, nuevo_diagnostico: String) -> bool {
        let mut atenciones = self.leer_atenciones();
        let mut modificado = false;
        for a in &mut atenciones {
            if a.mascota.nombre == nombre_mascota && a.mascota.dueno.nombre == nombre_dueno {
                a.diagnostico = nuevo_diagnostico.clone();
                modificado = true;
            }
        }
        if modificado {
            self.guardar_atenciones(&atenciones);
        }
        modificado
    }

    pub fn modificar_proxima_visita(&mut self, nombre_mascota: &str, nombre_dueno: &str, nueva_fecha: Option<Fecha>) -> bool {
        let mut atenciones = self.leer_atenciones();
        let mut modificado = false;
        for a in &mut atenciones {
            if a.mascota.nombre == nombre_mascota && a.mascota.dueno.nombre == nombre_dueno {
                a.proxima_visita = nueva_fecha;
                modificado = true;
            }
        }
        if modificado {
            self.guardar_atenciones(&atenciones);
        }
        modificado
    }

    pub fn eliminar_atencion(&mut self, nombre_mascota: &str, nombre_dueno: &str) -> bool {
        let mut atenciones = self.leer_atenciones();
        let mut index = None;
        let mut i = 0;
        while i < atenciones.len() {
            if atenciones[i].mascota.nombre == nombre_mascota && atenciones[i].mascota.dueno.nombre == nombre_dueno {
                index = Some(i);
                break;
            }
            i += 1;
        }
        if let Some(idx) = index {
            atenciones.remove(idx);
            self.guardar_atenciones(&atenciones);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_veterinaria() {
        let file_path = "src/test_atenciones.json".to_string();
        let _ = std::fs::remove_file(&file_path);

        let mut v = Veterinaria::new("Vet1".to_string(), "Calle 5".to_string(), 1, file_path.clone());
        let m1 = Mascota::new("Fido".to_string(), 3, TipoAnimal::Perro, Dueno::new("Carlos".to_string(), "Calle 6".to_string(), "123456".to_string()));
        let m2 = Mascota::new("Michi".to_string(), 2, TipoAnimal::Gato, Dueno::new("Carlos".to_string(), "Calle 6".to_string(), "123456".to_string()));

        v.agregar_mascota(m1.clone());
        v.agregar_mascota_prioridad(m2.clone());

        assert_eq!(v.cola_atencion[0].nombre, "Michi");

        let prox = v.atender_proxima_mascota().unwrap();
        assert_eq!(prox.nombre, "Michi");

        v.registrar_atencion(m1.clone(), "Sano".to_string(), "Ninguno".to_string(), None);
        assert_eq!(v.buscar_atencion("Fido", "Carlos", "123456").is_some(), true);
        assert_eq!(v.buscar_atencion("Fido", "Inexistente", "123456").is_none(), true);

        assert!(v.modificar_diagnostico("Fido", "Carlos", "Resfriado".to_string()));
        let at = v.buscar_atencion("Fido", "Carlos", "123456").unwrap();
        assert_eq!(at.diagnostico, "Resfriado");
        assert!(!v.modificar_diagnostico("Inexistente", "Carlos", "Resfriado".to_string()));

        let f = Fecha::new(10, 10, 2026);
        assert!(v.modificar_proxima_visita("Fido", "Carlos", Some(f)));
        let at2 = v.buscar_atencion("Fido", "Carlos", "123456").unwrap();
        assert_eq!(at2.proxima_visita, Some(f));
        assert!(!v.modificar_proxima_visita("Inexistente", "Carlos", Some(f)));

        assert!(v.eliminar_mascota_cola("Fido", "Carlos").is_some());
        assert!(v.eliminar_mascota_cola("Inexistente", "Carlos").is_none());

        assert!(v.eliminar_atencion("Fido", "Carlos"));
        assert_eq!(v.buscar_atencion("Fido", "Carlos", "123456").is_none(), true);
        assert!(!v.eliminar_atencion("Fido", "Carlos"));

        let _ = std::fs::remove_file(&file_path);
    }
}
