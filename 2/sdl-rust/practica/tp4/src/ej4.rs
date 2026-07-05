use std::collections::HashMap;
use crate::fecha::Fecha;

#[derive(Clone, PartialEq, Debug)]
pub struct Producto {
    pub nombre: String,
    pub categoria: String,
    pub precio_base: f64,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Vendedor {
    pub legajo: u32,
    pub antiguedad: u32,
    pub salario: f64,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Cliente {
    pub nombre: String,
    pub apellido: String,
    pub direccion: String,
    pub dni: String,
    pub email_newsletter: Option<String>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MedioPagoVenta {
    TarjetaCredito,
    TarjetaDebito,
    TransferenciaBancaria,
    Efectivo,
}

#[derive(Clone, Debug)]
pub struct Venta {
    pub fecha: Fecha,
    pub cliente: Cliente,
    pub vendedor: Vendedor,
    pub medio_pago: MedioPagoVenta,
    pub productos: Vec<(Producto, u32)>,
}

pub struct SistemaVentas {
    pub ventas: Vec<Venta>,
    pub descuentos_categoria: HashMap<String, f64>,
    pub descuento_newsletter: f64,
}

impl SistemaVentas {
    pub fn new(descuento_newsletter: f64) -> Self {
        Self {
            ventas: Vec::new(),
            descuentos_categoria: HashMap::new(),
            descuento_newsletter,
        }
    }

    pub fn registrar_descuento_categoria(&mut self, categoria: String, descuento: f64) {
        self.descuentos_categoria.insert(categoria, descuento);
    }

    pub fn registrar_venta(&mut self, venta: Venta) {
        self.ventas.push(venta);
    }

    pub fn calcular_precio_final_venta(&self, venta: &Venta) -> f64 {
        let mut total = 0.0;
        for (prod, cant) in &venta.productos {
            let desc_cat = match self.descuentos_categoria.get(&prod.categoria) {
                Some(&d) => d,
                None => 0.0,
            };
            let precio_unitario = prod.precio_base * (1.0 - desc_cat / 100.0);
            total += precio_unitario * (*cant as f64);
        }
        if venta.cliente.email_newsletter.is_some() {
            total = total * (1.0 - self.descuento_newsletter / 100.0);
        }
        total
    }

    pub fn reporte_ventas_por_categoria(&self) -> HashMap<String, f64> {
        let mut reporte = HashMap::new();
        for venta in &self.ventas {
            for (prod, cant) in &venta.productos {
                let desc_cat = match self.descuentos_categoria.get(&prod.categoria) {
                    Some(&d) => d,
                    None => 0.0,
                };
                let precio_total_producto = prod.precio_base * (1.0 - desc_cat / 100.0) * (*cant as f64);
                let precio_reporte = if venta.cliente.email_newsletter.is_some() {
                    precio_total_producto * (1.0 - self.descuento_newsletter / 100.0)
                } else {
                    precio_total_producto
                };
                *reporte.entry(prod.categoria.clone()).or_insert(0.0) += precio_reporte;
            }
        }
        reporte
    }

    pub fn reporte_ventas_por_vendedor(&self) -> HashMap<u32, f64> {
        let mut reporte = HashMap::new();
        for venta in &self.ventas {
            let total_venta = self.calcular_precio_final_venta(venta);
            *reporte.entry(venta.vendedor.legajo).or_insert(0.0) += total_venta;
        }
        reporte
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sistema_ventas() {
        let mut sistema = SistemaVentas::new(10.0);
        sistema.registrar_descuento_categoria("Electronica".to_string(), 15.0);

        let p1 = Producto { nombre: "Televisor".to_string(), categoria: "Electronica".to_string(), precio_base: 1000.0 };
        let p2 = Producto { nombre: "Remera".to_string(), categoria: "Ropa".to_string(), precio_base: 100.0 };

        let v1 = Vendedor { legajo: 101, antiguedad: 2, salario: 50000.0 };

        let c1 = Cliente {
            nombre: "Juan".to_string(),
            apellido: "Perez".to_string(),
            direccion: "Calle 1".to_string(),
            dni: "1234".to_string(),
            email_newsletter: Some("juan@news.com".to_string()),
        };

        let venta = Venta {
            fecha: Fecha::new(1, 1, 2026),
            cliente: c1,
            vendedor: v1,
            medio_pago: MedioPagoVenta::Efectivo,
            productos: vec![(p1, 1), (p2, 2)],
        };

        // Price:
        // p1: 1000 * 0.85 = 850
        // p2: 100 * 2 = 200
        // total: 1050
        // news: -10% -> 1050 * 0.9 = 945.0
        let precio_final = sistema.calcular_precio_final_venta(&venta);
        assert_eq!(precio_final, 945.0);

        sistema.registrar_venta(venta);

        let rep_cat = sistema.reporte_ventas_por_categoria();
        assert_eq!(rep_cat.get("Electronica"), Some(&(850.0 * 0.9)));
        assert_eq!(rep_cat.get("Ropa"), Some(&(200.0 * 0.9)));

        let rep_vend = sistema.reporte_ventas_por_vendedor();
        assert_eq!(rep_vend.get(&101), Some(&945.0));
    }
}
