pub struct Persona {
    nombre: String,
    edad: u32,
    direccion: Option<String>,
}

impl Persona {
    pub fn new(nombre: String, edad: u32, direccion: Option<String>) -> Self {
        Self {
            nombre,
            edad,
            direccion,
        }
    }

    pub fn to_string(&self) -> String {
        let dir_str = match &self.direccion {
            Some(dir) => dir.to_string(),
            None => "No registrada".to_string(),
        };
        let mut res = "Nombre: ".to_string();
        res += &self.nombre;
        res += ", Edad: ";
        res += &self.edad.to_string();
        res += ", Direccion: ";
        res += &dir_str;
        res
    }

    pub fn obtener_edad(&self) -> u32 {
        self.edad
    }

    pub fn actualizar_direccion(&mut self, nueva_direccion: Option<String>) {
        self.direccion = nueva_direccion;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_persona() {
        let mut p = Persona::new("Juan".to_string(), 30, Some("Calle 123".to_string()));
        assert_eq!(p.obtener_edad(), 30);
        assert_eq!(p.to_string(), "Nombre: Juan, Edad: 30, Direccion: Calle 123");
        p.actualizar_direccion(None);
        assert_eq!(p.to_string(), "Nombre: Juan, Edad: 30, Direccion: No registrada");
    }
}
