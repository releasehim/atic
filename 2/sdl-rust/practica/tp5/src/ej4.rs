use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::fecha::Fecha;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GeneroLibro {
    Novela,
    Infantil,
    Tecnico,
    Otros,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Libro {
    pub isbn: String,
    pub titulo: String,
    pub autor: String,
    pub num_paginas: u32,
    pub genero: GeneroLibro,
}

impl Libro {
    pub fn new(isbn: String, titulo: String, autor: String, num_paginas: u32, genero: GeneroLibro) -> Self {
        Self {
            isbn,
            titulo,
            autor,
            num_paginas,
            genero,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cliente {
    pub nombre: String,
    pub telefono: String,
    pub email: String,
}

impl Cliente {
    pub fn new(nombre: String, telefono: String, email: String) -> Self {
        Self {
            nombre,
            telefono,
            email,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EstadoPrestamo {
    Devuelto,
    EnPrestamo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Prestamo {
    pub libro: Libro,
    pub cliente: Cliente,
    pub fecha_vencimiento: Fecha,
    pub fecha_devolucion: Option<Fecha>,
    pub estado: EstadoPrestamo,
}

impl Prestamo {
    pub fn new(libro: Libro, cliente: Cliente, fecha_vencimiento: Fecha) -> Self {
        Self {
            libro,
            cliente,
            fecha_vencimiento,
            fecha_devolucion: None,
            estado: EstadoPrestamo::EnPrestamo,
        }
    }
}

pub struct Biblioteca {
    pub nombre: String,
    pub direccion: String,
    pub file_path_copias: String,
    pub file_path_prestamos: String,
}

impl Biblioteca {
    pub fn new(nombre: String, direccion: String, file_path_copias: String, file_path_prestamos: String) -> Self {
        if File::open(&file_path_copias).is_err() {
            let mut f = File::create(&file_path_copias).unwrap();
            f.write_all(b"{}").unwrap();
        }
        if File::open(&file_path_prestamos).is_err() {
            let mut f = File::create(&file_path_prestamos).unwrap();
            f.write_all(b"[]").unwrap();
        }
        Self {
            nombre,
            direccion,
            file_path_copias,
            file_path_prestamos,
        }
    }

    pub fn leer_copias(&self) -> HashMap<String, u32> {
        let mut f = match File::open(&self.file_path_copias) {
            Ok(file) => file,
            Err(_) => return HashMap::new(),
        };
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();
        serde_json::from_str(&buf).unwrap_or_else(|_| HashMap::new())
    }

    pub fn guardar_copias(&self, copias: &HashMap<String, u32>) {
        let data = serde_json::to_string(copias).unwrap();
        let mut f = File::create(&self.file_path_copias).unwrap();
        f.write_all(data.as_bytes()).unwrap();
    }

    pub fn leer_prestamos(&self) -> Vec<Prestamo> {
        let mut f = match File::open(&self.file_path_prestamos) {
            Ok(file) => file,
            Err(_) => return Vec::new(),
        };
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();
        serde_json::from_str(&buf).unwrap_or_else(|_| Vec::new())
    }

    pub fn guardar_prestamos(&self, prestamos: &Vec<Prestamo>) {
        let data = serde_json::to_string(prestamos).unwrap();
        let mut f = File::create(&self.file_path_prestamos).unwrap();
        f.write_all(data.as_bytes()).unwrap();
    }

    pub fn obtener_cantidad_copias(&self, libro: &Libro) -> u32 {
        let copias = self.leer_copias();
        match copias.get(&libro.isbn) {
            Some(&cant) => cant,
            None => 0,
        }
    }

    pub fn registrar_copias(&mut self, libro: &Libro, cantidad: u32) {
        let mut copias = self.leer_copias();
        copias.insert(libro.isbn.to_string(), cantidad);
        self.guardar_copias(&copias);
    }

    pub fn decrementar_cantidad_copias(&mut self, libro: &Libro) {
        let mut copias = self.leer_copias();
        if let Some(cant) = copias.get_mut(&libro.isbn) {
            if *cant > 0 {
                *cant -= 1;
            }
        }
        self.guardar_copias(&copias);
    }

    pub fn incrementar_cantidad_copias(&mut self, libro: &Libro) {
        let mut copias = self.leer_copias();
        let cant = match copias.get(&libro.isbn) {
            Some(&c) => c,
            None => 0,
        };
        copias.insert(libro.isbn.to_string(), cant + 1);
        self.guardar_copias(&copias);
    }

    pub fn contar_prestamos_cliente(&self, cliente: &Cliente) -> u32 {
        let prestamos = self.leer_prestamos();
        let mut count = 0;
        for p in &prestamos {
            if p.cliente == *cliente && p.estado == EstadoPrestamo::EnPrestamo {
                count += 1;
            }
        }
        count
    }

    pub fn realizar_prestamo(&mut self, libro: Libro, cliente: Cliente, fecha_vencimiento: Fecha) -> bool {
        let prestamos_actuales = self.contar_prestamos_cliente(&cliente);
        if prestamos_actuales >= 5 {
            return false;
        }
        let copias_disponibles = self.obtener_cantidad_copias(&libro);
        if copias_disponibles == 0 {
            return false;
        }
        self.decrementar_cantidad_copias(&libro);
        let mut prestamos = self.leer_prestamos();
        prestamos.push(Prestamo::new(libro, cliente, fecha_vencimiento));
        self.guardar_prestamos(&prestamos);
        true
    }

    pub fn ver_prestamos_a_vencer(&self, dias: u32, fecha_actual: Fecha) -> Vec<Prestamo> {
        let prestamos = self.leer_prestamos();
        let mut res = Vec::new();
        let mut fecha_limite = fecha_actual;
        fecha_limite.sumar_dias(dias);
        for p in prestamos {
            if p.estado == EstadoPrestamo::EnPrestamo {
                let no_vencido = !fecha_actual.es_mayor(&p.fecha_vencimiento);
                let dentro_limite = !p.fecha_vencimiento.es_mayor(&fecha_limite);
                if no_vencido && dentro_limite {
                    res.push(p);
                }
            }
        }
        res
    }

    pub fn ver_prestamos_vencidos(&self, fecha_actual: Fecha) -> Vec<Prestamo> {
        let prestamos = self.leer_prestamos();
        let mut res = Vec::new();
        for p in prestamos {
            if p.estado == EstadoPrestamo::EnPrestamo && fecha_actual.es_mayor(&p.fecha_vencimiento) {
                res.push(p);
            }
        }
        res
    }

    pub fn buscar_prestamo(&self, libro: &Libro, cliente: &Cliente) -> Option<Prestamo> {
        let prestamos = self.leer_prestamos();
        for p in prestamos {
            if p.libro == *libro && p.cliente == *cliente {
                return Some(p);
            }
        }
        None
    }

    pub fn devolver_libro(&mut self, libro: &Libro, cliente: &Cliente, fecha_devolucion: Fecha) -> bool {
        let mut prestamos = self.leer_prestamos();
        let mut devuelto = false;
        for p in &mut prestamos {
            if p.libro == *libro && p.cliente == *cliente && p.estado == EstadoPrestamo::EnPrestamo {
                p.estado = EstadoPrestamo::Devuelto;
                p.fecha_devolucion = Some(fecha_devolucion);
                devuelto = true;
                break;
            }
        }
        if devuelto {
            self.guardar_prestamos(&prestamos);
            self.incrementar_cantidad_copias(libro);
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
    fn test_biblioteca() {
        let file_path_copias = "src/test_copias.json".to_string();
        let file_path_prestamos = "src/test_prestamos.json".to_string();
        let _ = std::fs::remove_file(&file_path_copias);
        let _ = std::fs::remove_file(&file_path_prestamos);

        let mut b = Biblioteca::new("Biblio".to_string(), "Calle 7".to_string(), file_path_copias.clone(), file_path_prestamos.clone());
        let l = Libro::new("123".to_string(), "Rust Book".to_string(), "Author".to_string(), 300, GeneroLibro::Tecnico);
        let c = Cliente::new("Ana".to_string(), "654321".to_string(), "ana@email.com".to_string());

        b.registrar_copias(&l, 2);
        assert_eq!(b.obtener_cantidad_copias(&l), 2);

        let f_venc = Fecha::new(10, 7, 2026);
        assert_eq!(b.realizar_prestamo(l.clone(), c.clone(), f_venc), true);
        assert_eq!(b.obtener_cantidad_copias(&l), 1);
        assert_eq!(b.contar_prestamos_cliente(&c), 1);

        let f_act = Fecha::new(5, 7, 2026);
        let vencen = b.ver_prestamos_a_vencer(10, f_act);
        assert_eq!(vencen.len(), 1);

        let f_vencida = Fecha::new(15, 7, 2026);
        let vencidos = b.ver_prestamos_vencidos(f_vencida);
        assert_eq!(vencidos.len(), 1);

        assert_eq!(b.buscar_prestamo(&l, &c).is_some(), true);

        assert_eq!(b.devolver_libro(&l, &c, f_act), true);
        assert_eq!(b.obtener_cantidad_copias(&l), 2);
        assert_eq!(b.contar_prestamos_cliente(&c), 0);
        assert_eq!(b.devolver_libro(&l, &c, f_act), false);

        let _ = std::fs::remove_file(&file_path_copias);
        let _ = std::fs::remove_file(&file_path_prestamos);
    }
}
