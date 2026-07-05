#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ColorAuto {
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro,
}

#[derive(Debug, PartialEq)]
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

pub struct ConcesionarioAuto {
    pub nombre: String,
    pub direccion: String,
    pub capacidad_maxima: usize,
    pub autos: Vec<Auto>,
}

impl ConcesionarioAuto {
    pub fn new(nombre: String, direccion: String, capacidad_maxima: usize) -> Self {
        Self {
            nombre,
            direccion,
            capacidad_maxima,
            autos: Vec::new(),
        }
    }

    pub fn agregar_auto(&mut self, auto: Auto) -> bool {
        if self.autos.len() < self.capacidad_maxima {
            self.autos.push(auto);
            true
        } else {
            false
        }
    }

    pub fn eliminar_auto(&mut self, auto: &Auto) -> bool {
        let mut index = None;
        let mut i = 0;
        while i < self.autos.len() {
            if self.autos[i] == *auto {
                index = Some(i);
                break;
            }
            i += 1;
        }
        if let Some(idx) = index {
            self.autos.remove(idx);
            true
        } else {
            false
        }
    }

    pub fn buscar_auto(&self, auto: &Auto) -> Option<&Auto> {
        for a in &self.autos {
            if a == auto {
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
        let mut c = ConcesionarioAuto::new("Mio".to_string(), "Calle 4".to_string(), 2);
        let a1 = Auto::new("BMW".to_string(), "M3".to_string(), 2020, 10000.0, ColorAuto::Rojo);
        let a2 = Auto::new("Fiat".to_string(), "Uno".to_string(), 1998, 2000.0, ColorAuto::Verde);
        let a3 = Auto::new("Ford".to_string(), "Focus".to_string(), 2015, 5000.0, ColorAuto::Negro);

        assert_eq!(a1.calcular_precio(), 14000.0); 
        assert_eq!(a2.calcular_precio(), 1700.0); 

        assert_eq!(c.agregar_auto(a1), true);
        assert_eq!(c.agregar_auto(a2), true);
        assert_eq!(c.agregar_auto(a3), false);

        let test_a1 = Auto::new("BMW".to_string(), "M3".to_string(), 2020, 10000.0, ColorAuto::Rojo);
        assert_eq!(c.buscar_auto(&test_a1).is_some(), true);
        assert_eq!(c.eliminar_auto(&test_a1), true);
        assert_eq!(c.buscar_auto(&test_a1), None);
    }
}
