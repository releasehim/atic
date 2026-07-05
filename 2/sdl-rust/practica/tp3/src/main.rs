pub mod persona;
pub mod rectangulo;
pub mod fecha;
pub mod triangulo;
pub mod producto;
pub mod estudiante;
pub mod concesionario;
pub mod playlist;
pub mod veterinaria;
pub mod biblioteca;

fn main() {
    let p = persona::Persona::new("Juan".to_string(), 30, Some("Calle 123".to_string()));
    println!("{}", p.to_string());

    let r = rectangulo::Rectangulo::new(10.0, 5.0);
    println!("Rectangulo area: {}", r.calcular_area());

    let f = fecha::Fecha::new(28, 2, 2024);
    println!("Fecha bisiesta: {}", f.es_bisiesto());

    let t = triangulo::Triangulo::new(3.0, 4.0, 5.0);
    println!("Triangulo perimetro: {}", t.calcular_perimetro());

    let prod = producto::Producto::new("Laptop".to_string(), 1000.0, 1);
    println!("Producto precio: {}", prod.calcular_precio_total(Some(21.0), None));

    let e = estudiante::Examen::new("Matematica".to_string(), 9.0);
    let est = estudiante::Estudiante::new("Maria".to_string(), 101, vec![e]);
    println!("Estudiante promedio: {:?}", est.obtener_promedio());

    let a = concesionario::Auto::new("BMW".to_string(), "M3".to_string(), 2020, 10000.0, concesionario::ColorAuto::Rojo);
    println!("Auto precio: {}", a.calcular_precio());

    let mut play = playlist::Playlist::new("Rock".to_string());
    let c = playlist::Cancion::new("Song".to_string(), "Artist".to_string(), playlist::GeneroCancion::Rock);
    play.agregar_cancion(c);
    println!("Playlist canciones: {}", play.canciones.len());

    let vet = veterinaria::Veterinaria::new("Vet".to_string(), "Calle 5".to_string(), 1);
    println!("Veterinaria: {}", vet.nombre);

    let bib = biblioteca::Biblioteca::new("Bib".to_string(), "Calle 7".to_string());
    println!("Biblioteca: {}", bib.nombre);
}
