use std::fs::File;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GeneroCancion {
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub file_path: String,
}

impl Playlist {
    pub fn new(nombre: String, file_path: String) -> Self {
        if File::open(&file_path).is_err() {
            let mut f = File::create(&file_path).unwrap();
            f.write_all(b"[]").unwrap();
        }
        Self { nombre, file_path }
    }

    pub fn leer_canciones(&self) -> Vec<Cancion> {
        let mut f = match File::open(&self.file_path) {
            Ok(file) => file,
            Err(_) => return Vec::new(),
        };
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();
        serde_json::from_str(&buf).unwrap_or_else(|_| Vec::new())
    }

    pub fn guardar_canciones(&self, canciones: &Vec<Cancion>) {
        let data = serde_json::to_string(canciones).unwrap();
        let mut f = File::create(&self.file_path).unwrap();
        f.write_all(data.as_bytes()).unwrap();
    }

    pub fn agregar_cancion(&mut self, cancion: Cancion) {
        let mut canciones = self.leer_canciones();
        canciones.push(cancion);
        self.guardar_canciones(&canciones);
    }

    pub fn eliminar_cancion(&mut self, cancion: &Cancion) -> bool {
        let mut canciones = self.leer_canciones();
        let mut index = None;
        let mut i = 0;
        while i < canciones.len() {
            if canciones[i] == *cancion {
                index = Some(i);
                break;
            }
            i += 1;
        }
        if let Some(idx) = index {
            canciones.remove(idx);
            self.guardar_canciones(&canciones);
            true
        } else {
            false
        }
    }

    pub fn mover_cancion(&mut self, cancion: &Cancion, posicion: usize) -> bool {
        let mut canciones = self.leer_canciones();
        let mut index = None;
        let mut i = 0;
        while i < canciones.len() {
            if canciones[i] == *cancion {
                index = Some(i);
                break;
            }
            i += 1;
        }
        if let Some(idx) = index {
            let item = canciones.remove(idx);
            let target_pos = if posicion > canciones.len() {
                canciones.len()
            } else {
                posicion
            };
            canciones.insert(target_pos, item);
            self.guardar_canciones(&canciones);
            true
        } else {
            false
        }
    }

    pub fn buscar_cancion_por_nombre(&self, titulo: &str) -> Option<Cancion> {
        let canciones = self.leer_canciones();
        for c in canciones {
            if c.titulo == titulo {
                return Some(c);
            }
        }
        None
    }

    pub fn obtener_canciones_de_genero(&self, genero: GeneroCancion) -> Vec<Cancion> {
        let canciones = self.leer_canciones();
        let mut res = Vec::new();
        for c in canciones {
            if c.genero == genero {
                res.push(c);
            }
        }
        res
    }

    pub fn obtener_canciones_de_artista(&self, artista: &str) -> Vec<Cancion> {
        let canciones = self.leer_canciones();
        let mut res = Vec::new();
        for c in canciones {
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
        self.guardar_canciones(&Vec::new());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_playlist() {
        let file_path = "src/test_playlist.json".to_string();
        let _ = std::fs::remove_file(&file_path);

        let mut p = Playlist::new("Mi Lista".to_string(), file_path.clone());
        let c1 = Cancion::new("Song1".to_string(), "Artist1".to_string(), GeneroCancion::Rock);
        let c2 = Cancion::new("Song2".to_string(), "Artist2".to_string(), GeneroCancion::Pop);
        let c3 = Cancion::new("Song3".to_string(), "Artist1".to_string(), GeneroCancion::Rock);

        let test_c2 = Cancion::new("Song2".to_string(), "Artist2".to_string(), GeneroCancion::Pop);
        let test_c3 = Cancion::new("Song3".to_string(), "Artist1".to_string(), GeneroCancion::Rock);

        p.agregar_cancion(c1.clone());
        p.agregar_cancion(c2.clone());
        p.agregar_cancion(c3.clone());

        assert_eq!(p.buscar_cancion_por_nombre("Song2"), Some(test_c2.clone()));
        assert_eq!(p.obtener_canciones_de_genero(GeneroCancion::Rock), vec![c1.clone(), c3.clone()]);
        assert_eq!(p.obtener_canciones_de_artista("Artist1"), vec![c1.clone(), c3.clone()]);

        assert!(p.mover_cancion(&test_c3, 0));
        let canciones_pos = p.leer_canciones();
        assert_eq!(canciones_pos[0].titulo, "Song3");

        assert!(p.eliminar_cancion(&test_c2));
        assert_eq!(p.leer_canciones().len(), 2);
        assert!(!p.eliminar_cancion(&test_c2));
        assert!(!p.mover_cancion(&test_c2, 1));

        p.modificar_titulo("Nuevo Nombre".to_string());
        assert_eq!(p.nombre, "Nuevo Nombre");

        p.eliminar_todas_las_canciones();
        assert_eq!(p.leer_canciones().is_empty(), true);

        let _ = std::fs::remove_file(&file_path);
    }
}
