#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GeneroCancion {
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros,
}

#[derive(Debug, PartialEq)]
pub struct Cancion {
    pub titulo: String,
    pub artista: String,
    pub genero: GeneroCancion,
}

impl Cancion {
    pub fn new(titulo: String, artista: String, genero: GeneroCancion) -> Self {
        Self {
            titulo,
            artista,
            genero,
        }
    }
}

pub struct Playlist {
    pub nombre: String,
    pub canciones: Vec<Cancion>,
}

impl Playlist {
    pub fn new(nombre: String) -> Self {
        Self {
            nombre,
            canciones: Vec::new(),
        }
    }

    pub fn agregar_cancion(&mut self, cancion: Cancion) {
        self.canciones.push(cancion);
    }

    pub fn eliminar_cancion(&mut self, cancion: &Cancion) -> bool {
        let mut index = None;
        let mut i = 0;
        while i < self.canciones.len() {
            if self.canciones[i] == *cancion {
                index = Some(i);
                break;
            }
            i += 1;
        }
        if let Some(idx) = index {
            self.canciones.remove(idx);
            true
        } else {
            false
        }
    }

    pub fn mover_cancion(&mut self, cancion: &Cancion, posicion: usize) -> bool {
        let mut index = None;
        let mut i = 0;
        while i < self.canciones.len() {
            if self.canciones[i] == *cancion {
                index = Some(i);
                break;
            }
            i += 1;
        }
        if let Some(idx) = index {
            let item = self.canciones.remove(idx);
            let target_pos = if posicion > self.canciones.len() {
                self.canciones.len()
            } else {
                posicion
            };
            self.canciones.insert(target_pos, item);
            true
        } else {
            false
        }
    }

    pub fn buscar_cancion_por_nombre(&self, titulo: &str) -> Option<&Cancion> {
        for c in &self.canciones {
            if c.titulo == titulo {
                return Some(c);
            }
        }
        None
    }

    pub fn obtener_canciones_de_genero(&self, genero: GeneroCancion) -> Vec<&Cancion> {
        let mut res = Vec::new();
        for c in &self.canciones {
            if c.genero == genero {
                res.push(c);
            }
        }
        res
    }

    pub fn obtener_canciones_de_artista(&self, artista: &str) -> Vec<&Cancion> {
        let mut res = Vec::new();
        for c in &self.canciones {
            if c.artista == artista {
                res.push(c);
            }
        }
        res
    }

    pub fn modificar_titulo(&mut self, nuevo_titulo: String) {
        self.nombre = nuevo_titulo;
    }

    pub fn eliminar_todas_las_canciones(&mut self) {
        self.canciones.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_playlist() {
        let mut p = Playlist::new("Mi Lista".to_string());
        let c1 = Cancion::new("Song1".to_string(), "Artist1".to_string(), GeneroCancion::Rock);
        let c2 = Cancion::new("Song2".to_string(), "Artist2".to_string(), GeneroCancion::Pop);
        let c3 = Cancion::new("Song3".to_string(), "Artist1".to_string(), GeneroCancion::Rock);

        let test_c2 = Cancion::new("Song2".to_string(), "Artist2".to_string(), GeneroCancion::Pop);
        let test_c3 = Cancion::new("Song3".to_string(), "Artist1".to_string(), GeneroCancion::Rock);

        p.agregar_cancion(c1);
        p.agregar_cancion(c2);
        p.agregar_cancion(c3);

        assert_eq!(p.buscar_cancion_por_nombre("Song2"), Some(&test_c2));
        assert_eq!(p.obtener_canciones_de_genero(GeneroCancion::Rock).len(), 2);
        assert_eq!(p.obtener_canciones_de_artista("Artist1").len(), 2);

        p.mover_cancion(&test_c3, 0);
        assert_eq!(p.canciones[0].titulo, "Song3");

        p.eliminar_cancion(&test_c2);
        assert_eq!(p.canciones.len(), 2);

        p.modificar_titulo("Nuevo Nombre".to_string());
        assert_eq!(p.nombre, "Nuevo Nombre");

        p.eliminar_todas_las_canciones();
        assert_eq!(p.canciones.is_empty(), true);
    }
}
