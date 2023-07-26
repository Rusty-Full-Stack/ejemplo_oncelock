use std::sync::OnceLock;

struct ConexionStorage {
    sesion: String,
}

// OnceLock permite utiliza static para la definicion
// de las celdas
static CELL: OnceLock<ConexionStorage> = OnceLock::new();

fn main() {
    // Ahora inicializamos tal cual se hace con OnceCell
    // utiliza el mismo concepto de retornar una referencia &
    // inmutable, por lo que no puede modificarse directamente
    CELL.get_or_init(|| ConexionStorage {
        sesion: "0123456".to_string(),
    });

    // creamos uno de los hilos, el cual intentara inicializar el valor
    // de nuestro storage hasta en 9 ocasiones, pero debemos ver que siempre
    // imprime el valor con el cual fue inicilizado, cada intento se realizara
    // en un intervalo de 2 segundos
    let hilo = std::thread::spawn(|| {
        for i in 1..10 {
            let value: &ConexionStorage = CELL.get_or_init(|| ConexionStorage {
                sesion: "No deberia de imprimir este mensaje, sino Sesion Abierta".to_string(),
            });

            println!("Hilo Primario intento {} sesion: {}", i, value.sesion);

            std::thread::sleep(std::time::Duration::from_millis(2000));
        }
    });

    // Ahora en nuestro hilo principal, vamos a intentar realizar la incializacion
    // hasta en 4 intentos con un intervalo de 3 segundos por intento
    for i in 1..5 {
        let value: &ConexionStorage = CELL.get_or_init(|| ConexionStorage {
            sesion: "No deberia de imprimir este mensaje, sino Sesion Abierta".to_string(),
        });

        println!("Hilo secundario, intento {} sesion: {}", i, value.sesion);

        std::thread::sleep(std::time::Duration::from_millis(3000));
    }

    // Aca definimos al hilo principal que hay que esperar el que el thread
    // "hilo" finalice antes de terminar el programa
    hilo.join().unwrap();
}
