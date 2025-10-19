fn main() {
    #[derive(Debug)]
    enum RolUsuario {
        Admin,
        Moderador,
        Usuario,
        Invitado,
    }
    struct Usuario {
        nombre: String,
        email: String,
        activo: bool,
        contador_inicio_sesion: u64,
        rol: RolUsuario,
    }
    impl Usuario {
        fn imprimir_rol(&self) {
            match self.rol {
                RolUsuario::Admin => {
                    println!("El usuario: {}, es un Administrador", &self.nombre);
                }
                RolUsuario::Usuario => {
                    println!("El usuario: {}, es un Usuario", &self.nombre);
                }
                RolUsuario::Moderador => {
                    println!("El usuario: {}, es un Moderador", &self.nombre);
                }
                RolUsuario::Invitado => {
                    println!("El usuario: {}, es un Invitado", &self.nombre);
                }
            }
        }
    }
    let usuario1 = Usuario {
        nombre: "Saul Muñoz".to_string(),
        email: "sam@gmail.com".to_string(),
        activo: true,
        contador_inicio_sesion: 1,
        rol: RolUsuario::Usuario,
    };

    let usuario2 = Usuario {
        nombre: "Saul Muñon".to_string(),
        email: "sam.pedreros@gmail.com".to_string(),
        activo: true,
        contador_inicio_sesion: 2,
        rol: RolUsuario::Admin,
    };
    let usuario3 = Usuario {
        nombre: "Saul Muñon".to_string(),
        email: "sam.pedreros@gmail.com".to_string(),
        activo: true,
        contador_inicio_sesion: 3,
        rol: RolUsuario::Invitado,
    };
    let usuario4 = Usuario {
        nombre: "Saul Muñon".to_string(),
        email: "sam.pedreros@gmail.com".to_string(),
        activo: true,
        contador_inicio_sesion: 4,
        rol: RolUsuario::Moderador,
    };

    usuario1.imprimir_rol();
    usuario2.imprimir_rol();
    usuario3.imprimir_rol();
    usuario4.imprimir_rol();
    println!("Nombre: {}", usuario1.nombre);
    println!("Email: {}", usuario1.email);
    println!("Estado: {}", usuario1.activo);
    println!("Inicio de Sesion: {}", usuario1.contador_inicio_sesion);
    println!("Rol: {:?}", usuario1.rol);
}
