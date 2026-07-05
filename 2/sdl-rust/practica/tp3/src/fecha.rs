#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Fecha {
    pub dia: u32,
    pub mes: u32,
    pub anio: i32,
}

impl Fecha {
    pub fn new(dia: u32, mes: u32, anio: i32) -> Self {
        Self { dia, mes, anio }
    }

    pub fn es_bisiesto_anio(anio: i32) -> bool {
        (anio % 4 == 0 && anio % 100 != 0) || anio % 400 == 0
    }

    pub fn es_bisiesto(&self) -> bool {
        Self::es_bisiesto_anio(self.anio)
    }

    pub fn dias_en_mes(mes: u32, anio: i32) -> u32 {
        if mes == 1 || mes == 3 || mes == 5 || mes == 7 || mes == 8 || mes == 10 || mes == 12 {
            31
        } else if mes == 4 || mes == 6 || mes == 9 || mes == 11 {
            30
        } else if mes == 2 {
            if Self::es_bisiesto_anio(anio) {
                29
            } else {
                28
            }
        } else {
            0
        }
    }

    pub fn es_fecha_valida(&self) -> bool {
        if self.mes < 1 || self.mes > 12 {
            return false;
        }
        let max_dias = Self::dias_en_mes(self.mes, self.anio);
        self.dia >= 1 && self.dia <= max_dias
    }

    pub fn sumar_dias(&mut self, mut dias: u32) {
        if !self.es_fecha_valida() {
            return;
        }
        while dias > 0 {
            let max_dias = Self::dias_en_mes(self.mes, self.anio);
            let dias_disponibles = max_dias - self.dia;
            if dias <= dias_disponibles {
                self.dia += dias;
                dias = 0;
            } else {
                dias -= dias_disponibles + 1;
                self.dia = 1;
                if self.mes == 12 {
                    self.mes = 1;
                    self.anio += 1;
                } else {
                    self.mes += 1;
                }
            }
        }
    }

    pub fn restar_dias(&mut self, mut dias: u32) {
        if !self.es_fecha_valida() {
            return;
        }
        while dias > 0 {
            if dias < self.dia {
                self.dia -= dias;
                dias = 0;
            } else {
                dias -= self.dia;
                if self.mes == 1 {
                    self.mes = 12;
                    self.anio -= 1;
                } else {
                    self.mes -= 1;
                }
                self.dia = Self::dias_en_mes(self.mes, self.anio);
            }
        }
    }

    pub fn es_mayor(&self, una_fecha: &Fecha) -> bool {
        if self.anio != una_fecha.anio {
            return self.anio > una_fecha.anio;
        }
        if self.mes != una_fecha.mes {
            return self.mes > una_fecha.mes;
        }
        self.dia > una_fecha.dia
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fecha() {
        let mut f = Fecha::new(28, 2, 2024);
        assert_eq!(f.es_bisiesto(), true);
        assert_eq!(f.es_fecha_valida(), true);

        f.sumar_dias(1);
        assert_eq!(f.dia, 29);
        assert_eq!(f.mes, 2);
        assert_eq!(f.anio, 2024);

        f.sumar_dias(1);
        assert_eq!(f.dia, 1);
        assert_eq!(f.mes, 3);

        let mut f2 = Fecha::new(1, 3, 2024);
        f2.restar_dias(1);
        assert_eq!(f2.dia, 29);
        assert_eq!(f2.mes, 2);

        let f3 = Fecha::new(10, 10, 2024);
        let f4 = Fecha::new(9, 10, 2024);
        assert_eq!(f3.es_mayor(&f4), true);
        assert_eq!(f4.es_mayor(&f3), false);
    }
}
