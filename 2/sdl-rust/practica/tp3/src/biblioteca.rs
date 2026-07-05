use std::collections::HashMap;
use crate::fecha::Fecha;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GeneroLibro {
    Novela,
    Infantil,
    Tecnico,
    Otros,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EstadoPrestamo {
    Devuelto,
    EnPrestamo,
}

#[derive(Debug, PartialEq)]
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
    pub copias_disposicion: HashMap<String, u32>,
    pub prestamos_efectuados: Vec<Prestamo>,
}

impl Biblioteca {
    pub fn new(nombre: String, direccion: String) -> Self {
        Self {
            nombre,
            direccion,
            copias_disposicion: HashMap::new(),
            prestamos_efectuados: Vec::new(),
        }
    }

    pub fn obtener_cantidad_copias(&self, libro: &Libro) -> u32 {
        match self.copias_disposicion.get(&libro.isbn) {
            Some(&cant) => cant,
            None => 0,
        }
    }

    pub fn registrar_copias(&mut self, libro: &Libro, cantidad: u32) {
        self.copias_disposicion.insert(libro.isbn.to_string(), cantidad);
    }

    pub fn decrementar_cantidad_copias(&mut self, libro: &Libro) {
        if let Some(cant) = self.copias_disposicion.get_mut(&libro.isbn) {
            if *cant > 0 {
                *cant -= 1;
            }
        }
    }

    pub fn incrementar_cantidad_copias(&mut self, libro: &Libro) {
        let cant = self.obtener_cantidad_copias(libro);
        self.copias_disposicion.insert(libro.isbn.to_string(), cant + 1);
    }

    pub fn contar_prestamos_cliente(&self, cliente: &Cliente) -> u32 {
        let mut count = 0;
        for p in &self.prestamos_efectuados {
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
        self.prestamos_efectuados.push(Prestamo::new(libro, cliente, fecha_vencimiento));
        true
    }

    pub fn ver_prestamos_a_vencer(&self, dias: u32, fecha_actual: Fecha) -> Vec<&Prestamo> {
        let mut res = Vec::new();
        let mut fecha_limite = fecha_actual;
        fecha_limite.sumar_dias(dias);
        for p in &self.prestamos_efectuados {
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

    pub fn ver_prestamos_vencidos(&self, fecha_actual: Fecha) -> Vec<&Prestamo> {
        let mut res = Vec::new();
        for p in &self.prestamos_efectuados {
            if p.estado == EstadoPrestamo::EnPrestamo && fecha_actual.es_mayor(&p.fecha_vencimiento) {
                res.push(p);
            }
        }
        res
    }

    pub fn buscar_prestamo(&self, libro: &Libro, cliente: &Cliente) -> Option<&Prestamo> {
        for p in &self.prestamos_efectuados {
            if p.libro == *libro && p.cliente == *cliente {
                return Some(p);
            }
        }
        None
    }

    pub fn devolver_libro(&mut self, libro: &Libro, cliente: &Cliente, fecha_devolucion: Fecha) -> bool {
        for p in &mut self.prestamos_efectuados {
            if p.libro == *libro && p.cliente == *cliente && p.estado == EstadoPrestamo::EnPrestamo {
                p.estado = EstadoPrestamo::Devuelto;
                p.fecha_devolucion = Some(fecha_devolucion);
                self.incrementar_cantidad_copias(libro);
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biblioteca() {
        let mut b = Biblioteca::new("Biblio".to_string(), "Calle 7".to_string());
        let l = Libro::new("123".to_string(), "Rust Book".to_string(), "Author".to_string(), 300, GeneroLibro::Tecnico);
        let c = Cliente::new("Ana".to_string(), "654321".to_string(), "ana@email.com".to_string());

        b.registrar_copias(&l, 2);
        assert_eq!(b.obtener_cantidad_copias(&l), 2);

        let f_venc = Fecha::new(10, 7, 2026);
        let l_for_prestamo = Libro::new("123".to_string(), "Rust Book".to_string(), "Author".to_string(), 300, GeneroLibro::Tecnico);
        let c_for_prestamo = Cliente::new("Ana".to_string(), "654321".to_string(), "ana@email.com".to_string());
        assert_eq!(b.realizar_prestamo(l_for_prestamo, c_for_prestamo, f_venc), true);
        assert_eq!(b.obtener_cantidad_copias(&l), 1);
        assert_eq!(b.contar_prestamos_cliente(&c), 1);

        let f_act = Fecha::new(5, 7, 2026);
        let vencen = b.ver_prestamos_a_vencer(10, f_act);
        assert_eq!(vencen.len(), 1);

        let f_vencida = Fecha::new(15, 7, 2026);
        let vencidos = b.ver_prestamos_vencidos(f_vencida);
        assert_eq!(vencidos.len(), 1);

        assert_eq!(b.devolver_libro(&l, &c, f_act), true);
        assert_eq!(b.obtener_cantidad_copias(&l), 2);
        assert_eq!(b.contar_prestamos_cliente(&c), 0);
    }
}
