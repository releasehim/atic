#[derive(Debug, PartialEq)]
pub enum TipoTriangulo {
    Equilatero,
    Isosceles,
    Escaleno,
}

pub struct Triangulo {
    lado1: f64,
    lado2: f64,
    lado3: f64,
}

impl Triangulo {
    pub fn new(lado1: f64, lado2: f64, lado3: f64) -> Self {
        Self { lado1, lado2, lado3 }
    }

    pub fn determinar_tipo(&self) -> TipoTriangulo {
        if self.lado1 == self.lado2 && self.lado2 == self.lado3 {
            TipoTriangulo::Equilatero
        } else if self.lado1 == self.lado2 || self.lado2 == self.lado3 || self.lado1 == self.lado3 {
            TipoTriangulo::Isosceles
        } else {
            TipoTriangulo::Escaleno
        }
    }

    pub fn calcular_area(&self) -> f64 {
        let s = self.calcular_perimetro() / 2.0;
        let diff1 = s - self.lado1;
        let diff2 = s - self.lado2;
        let diff3 = s - self.lado3;
        (s * diff1 * diff2 * diff3).sqrt()
    }

    pub fn calcular_perimetro(&self) -> f64 {
        self.lado1 + self.lado2 + self.lado3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangulo() {
        let t1 = Triangulo::new(5.0, 5.0, 5.0);
        assert_eq!(t1.determinar_tipo(), TipoTriangulo::Equilatero);
        assert_eq!(t1.calcular_perimetro(), 15.0);

        let t2 = Triangulo::new(5.0, 5.0, 3.0);
        assert_eq!(t2.determinar_tipo(), TipoTriangulo::Isosceles);

        let t3 = Triangulo::new(3.0, 4.0, 5.0);
        assert_eq!(t3.determinar_tipo(), TipoTriangulo::Escaleno);
        assert_eq!(t3.calcular_area(), 6.0);
    }
}
