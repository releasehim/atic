pub struct Rectangulo {
    longitud: f64,
    ancho: f64,
}

impl Rectangulo {
    pub fn new(longitud: f64, ancho: f64) -> Self {
        Self { longitud, ancho }
    }

    pub fn calcular_area(&self) -> f64 {
        self.longitud * self.ancho
    }

    pub fn calcular_perimetro(&self) -> f64 {
        (self.longitud + self.ancho) * 2.0
    }

    pub fn es_cuadrado(&self) -> bool {
        self.longitud == self.ancho
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangulo() {
        let r = Rectangulo::new(10.0, 5.0);
        assert_eq!(r.calcular_area(), 50.0);
        assert_eq!(r.calcular_perimetro(), 30.0);
        assert_eq!(r.es_cuadrado(), false);

        let sq = Rectangulo::new(5.0, 5.0);
        assert_eq!(sq.es_cuadrado(), true);
    }
}
