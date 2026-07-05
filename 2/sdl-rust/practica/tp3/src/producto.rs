pub struct Producto {
    pub nombre: String,
    pub precio_bruto: f64,
    pub id: u32,
}

impl Producto {
    pub fn new(nombre: String, precio_bruto: f64, id: u32) -> Self {
        Self { nombre, precio_bruto, id }
    }

    pub fn calcular_impuestos(&self, porcentaje: Option<f64>) -> f64 {
        let p = match porcentaje {
            Some(val) => val,
            None => 0.0,
        };
        self.precio_bruto * (p / 100.0)
    }

    pub fn aplicar_descuento(&self, porcentaje: Option<f64>) -> f64 {
        let p = match porcentaje {
            Some(val) => val,
            None => 0.0,
        };
        self.precio_bruto * (p / 100.0)
    }

    pub fn calcular_precio_total(&self, impuesto: Option<f64>, descuento: Option<f64>) -> f64 {
        let imp = self.calcular_impuestos(impuesto);
        let desc = self.aplicar_descuento(descuento);
        self.precio_bruto + imp - desc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_producto() {
        let p = Producto::new("Laptop".to_string(), 1000.0, 1);
        assert_eq!(p.calcular_impuestos(Some(21.0)), 210.0);
        assert_eq!(p.aplicar_descuento(Some(10.0)), 100.0);
        assert_eq!(p.calcular_precio_total(Some(21.0), Some(10.0)), 1110.0);
        assert_eq!(p.calcular_precio_total(None, None), 1000.0);
    }
}
