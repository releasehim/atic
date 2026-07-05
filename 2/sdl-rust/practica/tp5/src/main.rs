pub mod fecha;
pub mod ej1;
pub mod ej2;
pub mod ej3;
pub mod ej4;
pub mod ej5;
pub mod ej6;

fn main() {
    let file_path = "src/concesionario_main.json".to_string();
    let _ = std::fs::remove_file(&file_path);
    let c = ej1::ConcesionarioAuto::new("Mio".to_string(), "Calle 4".to_string(), 2, file_path.clone());
    println!("Concesionario capacity: {}", c.capacidad_maxima);
    let _ = std::fs::remove_file(&file_path);

    let play_file = "src/playlist_main.json".to_string();
    let _ = std::fs::remove_file(&play_file);
    let p = ej2::Playlist::new("Rock".to_string(), play_file.clone());
    println!("Playlist name: {}", p.nombre);
    let _ = std::fs::remove_file(&play_file);

    let vet_file = "src/vet_main.json".to_string();
    let _ = std::fs::remove_file(&vet_file);
    let v = ej3::Veterinaria::new("Vet".to_string(), "Calle 5".to_string(), 1, vet_file.clone());
    println!("Vet name: {}", v.nombre);
    let _ = std::fs::remove_file(&vet_file);

    let copias_file = "src/copias_main.json".to_string();
    let prestamos_file = "src/prestamos_main.json".to_string();
    let _ = std::fs::remove_file(&copias_file);
    let _ = std::fs::remove_file(&prestamos_file);
    let b = ej4::Biblioteca::new("Bib".to_string(), "Calle 7".to_string(), copias_file.clone(), prestamos_file.clone());
    println!("Biblioteca name: {}", b.nombre);
    let _ = std::fs::remove_file(&copias_file);
    let _ = std::fs::remove_file(&prestamos_file);

    let users_file = "src/users_main.json".to_string();
    let hist_file = "src/historial_main.json".to_string();
    let _ = std::fs::remove_file(&users_file);
    let _ = std::fs::remove_file(&hist_file);
    let s = ej5::StreamingRust::new(users_file.clone(), hist_file.clone());
    println!("Streaming users: {}", s.leer_usuarios().len());
    let _ = std::fs::remove_file(&users_file);
    let _ = std::fs::remove_file(&hist_file);

    let xyz_users = "src/xyz_users_main.json".to_string();
    let xyz_trans = "src/xyz_trans_main.json".to_string();
    let _ = std::fs::remove_file(&xyz_users);
    let _ = std::fs::remove_file(&xyz_trans);
    let xyz = ej6::XYZPlataforma::new(xyz_users.clone(), xyz_trans.clone());
    println!("XYZ users: {}", xyz.leer_usuarios().len());
    let _ = std::fs::remove_file(&xyz_users);
    let _ = std::fs::remove_file(&xyz_trans);
}
