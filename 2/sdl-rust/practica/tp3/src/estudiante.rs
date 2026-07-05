pub struct Examen {
    pub materia: String,
    pub nota: f64,
}

impl Examen {
    pub fn new(materia: String, nota: f64) -> Self {
        Self { materia, nota }
    }
}

pub struct Estudiante {
    pub nombre: String,
    pub id: u32,
    pub examenes: Vec<Examen>,
}

impl Estudiante {
    pub fn new(nombre: String, id: u32, examenes: Vec<Examen>) -> Self {
        Self { nombre, id, examenes }
    }

    pub fn obtener_promedio(&self) -> Option<f64> {
        if self.examenes.is_empty() {
            return None;
        }
        let mut suma = 0.0;
        for e in &self.examenes {
            suma += e.nota;
        }
        Some(suma / self.examenes.len() as f64)
    }

    pub fn obtener_calificacion_mas_alta(&self) -> Option<f64> {
        if self.examenes.is_empty() {
            return None;
        }
        let mut max = self.examenes[0].nota;
        for e in &self.examenes {
            if e.nota > max {
                max = e.nota;
            }
        }
        Some(max)
    }

    pub fn obtener_calificacion_mas_baja(&self) -> Option<f64> {
        if self.examenes.is_empty() {
            return None;
        }
        let mut min = self.examenes[0].nota;
        for e in &self.examenes {
            if e.nota < min {
                min = e.nota;
            }
        }
        Some(min)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estudiante() {
        let e1 = Examen::new("Matematica".to_string(), 8.0);
        let e2 = Examen::new("Fisica".to_string(), 6.0);
        let est = Estudiante::new("Maria".to_string(), 101, vec![e1, e2]);

        assert_eq!(est.obtener_promedio(), Some(7.0));
        assert_eq!(est.obtener_calificacion_mas_alta(), Some(8.0));
        assert_eq!(est.obtener_calificacion_mas_baja(), Some(6.0));

        let est_empty = Estudiante::new("Pedro".to_string(), 102, vec![]);
        assert_eq!(est_empty.obtener_promedio(), None);
    }
}
