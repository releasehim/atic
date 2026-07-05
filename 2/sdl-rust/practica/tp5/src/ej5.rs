use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::fecha::Fecha;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TipoSuscripcion {
    Basic,
    Classic,
    Super,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MedioPago {
    Efectivo,
    MercadoPago(String),
    TarjetaCredito(String, String),
    TransferenciaBancaria(String),
    Cripto(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Suscripcion {
    pub tipo: TipoSuscripcion,
    pub costo_mensual: f64,
    pub meses: u32,
    pub fecha_inicio: Fecha,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Usuario {
    pub nombre: String,
    pub suscripcion: Option<Suscripcion>,
    pub medio_pago: MedioPago,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistorialContrato {
    pub tipo_suscripcion: TipoSuscripcion,
    pub medio_pago: MedioPago,
}

pub struct StreamingRust {
    pub file_path_usuarios: String,
    pub file_path_historial: String,
}

impl StreamingRust {
    pub fn new(file_path_usuarios: String, file_path_historial: String) -> Self {
        if File::open(&file_path_usuarios).is_err() {
            let mut f = File::create(&file_path_usuarios).unwrap();
            f.write_all(b"[]").unwrap();
        }
        if File::open(&file_path_historial).is_err() {
            let mut f = File::create(&file_path_historial).unwrap();
            f.write_all(b"[]").unwrap();
        }
        Self {
            file_path_usuarios,
            file_path_historial,
        }
    }

    pub fn leer_usuarios(&self) -> Vec<Usuario> {
        let mut f = match File::open(&self.file_path_usuarios) {
            Ok(file) => file,
            Err(_) => return Vec::new(),
        };
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();
        serde_json::from_str(&buf).unwrap_or_else(|_| Vec::new())
    }

    pub fn guardar_usuarios(&self, usuarios: &Vec<Usuario>) {
        let data = serde_json::to_string(usuarios).unwrap();
        let mut f = File::create(&self.file_path_usuarios).unwrap();
        f.write_all(data.as_bytes()).unwrap();
    }

    pub fn leer_historial(&self) -> Vec<HistorialContrato> {
        let mut f = match File::open(&self.file_path_historial) {
            Ok(file) => file,
            Err(_) => return Vec::new(),
        };
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();
        serde_json::from_str(&buf).unwrap_or_else(|_| Vec::new())
    }

    pub fn guardar_historial(&self, historial: &Vec<HistorialContrato>) {
        let data = serde_json::to_string(historial).unwrap();
        let mut f = File::create(&self.file_path_historial).unwrap();
        f.write_all(data.as_bytes()).unwrap();
    }

    pub fn crear_usuario(&mut self, nombre: String, tipo_sub: Option<TipoSuscripcion>, medio: MedioPago, fecha: Fecha) {
        let mut usuarios = self.leer_usuarios();
        let mut historial = self.leer_historial();
        let sub = match tipo_sub {
            Some(t) => {
                historial.push(HistorialContrato {
                    tipo_suscripcion: t,
                    medio_pago: medio.clone(),
                });
                let costo = match t {
                    TipoSuscripcion::Basic => 100.0,
                    TipoSuscripcion::Classic => 200.0,
                    TipoSuscripcion::Super => 300.0,
                };
                Some(Suscripcion {
                    tipo: t,
                    costo_mensual: costo,
                    meses: 1,
                    fecha_inicio: fecha,
                })
            }
            None => None,
        };
        usuarios.push(Usuario {
            nombre,
            suscripcion: sub,
            medio_pago: medio,
        });
        self.guardar_usuarios(&usuarios);
        self.guardar_historial(&historial);
    }

    pub fn upgrade_suscripcion(&mut self, index: usize) {
        let mut usuarios = self.leer_usuarios();
        if index >= usuarios.len() {
            return;
        }
        let u = &mut usuarios[index];
        if let Some(ref mut s) = u.suscripcion {
            let nuevo_tipo = match s.tipo {
                TipoSuscripcion::Basic => Some(TipoSuscripcion::Classic),
                TipoSuscripcion::Classic => Some(TipoSuscripcion::Super),
                TipoSuscripcion::Super => None,
            };
            if let Some(t) = nuevo_tipo {
                s.tipo = t;
                s.costo_mensual = match t {
                    TipoSuscripcion::Basic => 100.0,
                    TipoSuscripcion::Classic => 200.0,
                    TipoSuscripcion::Super => 300.0,
                };
                let mut historial = self.leer_historial();
                historial.push(HistorialContrato {
                    tipo_suscripcion: t,
                    medio_pago: u.medio_pago.clone(),
                });
                self.guardar_historial(&historial);
            }
        }
        self.guardar_usuarios(&usuarios);
    }

    pub fn downgrade_suscripcion(&mut self, index: usize) {
        let mut usuarios = self.leer_usuarios();
        if index >= usuarios.len() {
            return;
        }
        let u = &mut usuarios[index];
        let mut cancelar = false;
        if let Some(ref mut s) = u.suscripcion {
            let nuevo_tipo = match s.tipo {
                TipoSuscripcion::Super => Some(TipoSuscripcion::Classic),
                TipoSuscripcion::Classic => Some(TipoSuscripcion::Basic),
                TipoSuscripcion::Basic => {
                    cancelar = true;
                    None
                }
            };
            if let Some(t) = nuevo_tipo {
                s.tipo = t;
                s.costo_mensual = match t {
                    TipoSuscripcion::Basic => 100.0,
                    TipoSuscripcion::Classic => 200.0,
                    TipoSuscripcion::Super => 300.0,
                };
                let mut historial = self.leer_historial();
                historial.push(HistorialContrato {
                    tipo_suscripcion: t,
                    medio_pago: u.medio_pago.clone(),
                });
                self.guardar_historial(&historial);
            }
        }
        if cancelar {
            u.suscripcion = None;
        }
        self.guardar_usuarios(&usuarios);
    }

    pub fn cancelar_suscripcion(&mut self, index: usize) {
        let mut usuarios = self.leer_usuarios();
        if index < usuarios.len() {
            usuarios[index].suscripcion = None;
            self.guardar_usuarios(&usuarios);
        }
    }

    pub fn nombre_medio(mp: &MedioPago) -> &'static str {
        match mp {
            MedioPago::Efectivo => "Efectivo",
            MedioPago::MercadoPago(_) => "MercadoPago",
            MedioPago::TarjetaCredito(_, _) => "TarjetaCredito",
            MedioPago::TransferenciaBancaria(_) => "TransferenciaBancaria",
            MedioPago::Cripto(_) => "Cripto",
        }
    }

    pub fn medio_pago_mas_utilizado_activas(&self) -> Option<&'static str> {
        let usuarios = self.leer_usuarios();
        let mut counts = HashMap::new();
        for u in usuarios.iter().filter(|u| u.suscripcion.is_some()) {
            let name = Self::nombre_medio(&u.medio_pago);
            *counts.entry(name).or_insert(0) += 1;
        }
        let mut max_name = None;
        let mut max_count = 0;
        for (name, count) in counts {
            if count > max_count {
                max_count = count;
                max_name = Some(name);
            }
        }
        max_name
    }

    pub fn suscripcion_mas_contratada_activas(&self) -> Option<TipoSuscripcion> {
        let usuarios = self.leer_usuarios();
        let mut counts = HashMap::new();
        for u in usuarios.iter().filter_map(|u| u.suscripcion.as_ref()) {
            *counts.entry(u.tipo).or_insert(0) += 1;
        }
        let mut max_tipo = None;
        let mut max_count = 0;
        for (tipo, count) in counts {
            if count > max_count {
                max_count = count;
                max_tipo = Some(tipo);
            }
        }
        max_tipo
    }

    pub fn medio_pago_mas_utilizado_historico(&self) -> Option<&'static str> {
        let historial = self.leer_historial();
        let mut counts = HashMap::new();
        for h in &historial {
            let name = Self::nombre_medio(&h.medio_pago);
            *counts.entry(name).or_insert(0) += 1;
        }
        let mut max_name = None;
        let mut max_count = 0;
        for (name, count) in counts {
            if count > max_count {
                max_count = count;
                max_name = Some(name);
            }
        }
        max_name
    }

    pub fn suscripcion_mas_contratada_historico(&self) -> Option<TipoSuscripcion> {
        let historial = self.leer_historial();
        let mut counts = HashMap::new();
        for h in &historial {
            *counts.entry(h.tipo_suscripcion).or_insert(0) += 1;
        }
        let mut max_tipo = None;
        let mut max_count = 0;
        for (tipo, count) in counts {
            if count > max_count {
                max_count = count;
                max_tipo = Some(tipo);
            }
        }
        max_tipo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_streaming_rust() {
        let file_path_usuarios = "src/test_usuarios.json".to_string();
        let file_path_historial = "src/test_historial.json".to_string();
        let _ = std::fs::remove_file(&file_path_usuarios);
        let _ = std::fs::remove_file(&file_path_historial);

        let mut platform = StreamingRust::new(file_path_usuarios.clone(), file_path_historial.clone());
        let f = Fecha::new(1, 1, 2026);

        platform.crear_usuario("User1".to_string(), Some(TipoSuscripcion::Basic), MedioPago::Efectivo, f);
        platform.crear_usuario("User2".to_string(), Some(TipoSuscripcion::Classic), MedioPago::Cripto("wallet_addr".to_string()), f);
        platform.crear_usuario("User3".to_string(), Some(TipoSuscripcion::Basic), MedioPago::Cripto("wallet_addr2".to_string()), f);
        platform.crear_usuario("User4".to_string(), Some(TipoSuscripcion::Basic), MedioPago::Cripto("wallet_addr3".to_string()), f);

        assert_eq!(platform.medio_pago_mas_utilizado_activas(), Some("Cripto"));
        assert_eq!(platform.suscripcion_mas_contratada_activas(), Some(TipoSuscripcion::Basic));

        platform.upgrade_suscripcion(0); 
        assert_eq!(platform.leer_usuarios()[0].suscripcion.as_ref().unwrap().tipo, TipoSuscripcion::Classic);

        platform.downgrade_suscripcion(2); 
        assert!(platform.leer_usuarios()[2].suscripcion.is_none());

        assert_eq!(platform.medio_pago_mas_utilizado_historico(), Some("Cripto"));
        assert_eq!(platform.suscripcion_mas_contratada_historico(), Some(TipoSuscripcion::Basic));

        platform.cancelar_suscripcion(1);
        assert!(platform.leer_usuarios()[1].suscripcion.is_none());

        // invalid indices check
        platform.upgrade_suscripcion(99);
        platform.downgrade_suscripcion(99);
        platform.cancelar_suscripcion(99);

        // check user created without subscription
        platform.crear_usuario("UserNoSub".to_string(), None, MedioPago::Efectivo, f);

        let _ = std::fs::remove_file(&file_path_usuarios);
        let _ = std::fs::remove_file(&file_path_historial);
    }
}
