use std::collections::VecDeque;
use crate::fecha::Fecha;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TipoAnimal {
    Perro,
    Gato,
    Caballo,
    Otros,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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
    pub atenciones_realizadas: Vec<Atencion>,
}

impl Veterinaria {
    pub fn new(nombre: String, direccion: String, id: u32) -> Self {
        Self {
            nombre,
            direccion,
            id,
            cola_atencion: VecDeque::new(),
            atenciones_realizadas: Vec::new(),
        }
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
        self.atenciones_realizadas.push(Atencion::new(mascota, diagnostico, tratamiento, proxima_visita));
    }

    pub fn buscar_atencion(&self, nombre_mascota: &str, nombre_dueno: &str, telefono: &str) -> Option<&Atencion> {
        for a in &self.atenciones_realizadas {
            if a.mascota.nombre == nombre_mascota && a.mascota.dueno.nombre == nombre_dueno && a.mascota.dueno.telefono == telefono {
                return Some(a);
            }
        }
        None
    }

    pub fn modificar_diagnostico(&mut self, nombre_mascota: &str, nombre_dueno: &str, nuevo_diagnostico: String) -> bool {
        for a in &mut self.atenciones_realizadas {
            if a.mascota.nombre == nombre_mascota && a.mascota.dueno.nombre == nombre_dueno {
                a.diagnostico = nuevo_diagnostico;
                return true;
            }
        }
        false
    }

    pub fn modificar_proxima_visita(&mut self, nombre_mascota: &str, nombre_dueno: &str, nueva_fecha: Option<Fecha>) -> bool {
        for a in &mut self.atenciones_realizadas {
            if a.mascota.nombre == nombre_mascota && a.mascota.dueno.nombre == nombre_dueno {
                a.proxima_visita = nueva_fecha;
                return true;
            }
        }
        false
    }

    pub fn eliminar_atencion(&mut self, nombre_mascota: &str, nombre_dueno: &str) -> bool {
        let mut index = None;
        let mut i = 0;
        while i < self.atenciones_realizadas.len() {
            if self.atenciones_realizadas[i].mascota.nombre == nombre_mascota && self.atenciones_realizadas[i].mascota.dueno.nombre == nombre_dueno {
                index = Some(i);
                break;
            }
            i += 1;
        }
        if let Some(idx) = index {
            self.atenciones_realizadas.remove(idx);
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
        let mut v = Veterinaria::new("Vet1".to_string(), "Calle 5".to_string(), 1);
        
        let m1 = Mascota::new("Fido".to_string(), 3, TipoAnimal::Perro, Dueno::new("Carlos".to_string(), "Calle 6".to_string(), "123456".to_string()));
        let m2 = Mascota::new("Michi".to_string(), 2, TipoAnimal::Gato, Dueno::new("Carlos".to_string(), "Calle 6".to_string(), "123456".to_string()));

        v.agregar_mascota(m1);
        v.agregar_mascota_prioridad(m2);

        assert_eq!(v.cola_atencion[0].nombre, "Michi");

        let prox = v.atender_proxima_mascota().unwrap();
        assert_eq!(prox.nombre, "Michi");

        let m1_for_atencion = Mascota::new("Fido".to_string(), 3, TipoAnimal::Perro, Dueno::new("Carlos".to_string(), "Calle 6".to_string(), "123456".to_string()));
        v.registrar_atencion(m1_for_atencion, "Sano".to_string(), "Ninguno".to_string(), None);
        assert_eq!(v.buscar_atencion("Fido", "Carlos", "123456").is_some(), true);

        v.modificar_diagnostico("Fido", "Carlos", "Resfriado".to_string());
        let at = v.buscar_atencion("Fido", "Carlos", "123456").unwrap();
        assert_eq!(at.diagnostico, "Resfriado");

        let f = Fecha::new(10, 10, 2026);
        v.modificar_proxima_visita("Fido", "Carlos", Some(f));
        let at2 = v.buscar_atencion("Fido", "Carlos", "123456").unwrap();
        assert_eq!(at2.proxima_visita, Some(f));

        v.eliminar_atencion("Fido", "Carlos");
        assert_eq!(v.buscar_atencion("Fido", "Carlos", "123456").is_none(), true);
    }
}
