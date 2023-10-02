//// 2023/10/02 // 13:19 //

// Operador de control de flujo match

// Rust posee un oeprador de control de flujo extremadamente útil llamado match
// que nos va a permitir comparar un valor con una serie de patrones y luego
// ejecutar código según el patrón que cincida.

// Los patrones pueden estar formados por valores literales, nombres de variables
// o comodines entre otros.

// #[derive(Debug)]

// enum Mes {
//     Enero,
//     Febrero,
//     Marzo,
//     Abril,
//     Mayo,
//     Junio,
//     Julio,
//     Agosto,
//     Septiembre,
//     Octubre,
//     Noviembre,
//     Diciembre,
// }

// enum Tiempo {
//     Segundo,
//     Minuto,
//     Hora,
//     Dia(Mes),
// }

// fn valor_en_segundos(tiempo: Tiempo) -> u32 {
//     match tiempo {
//         Tiempo::Segundo => {
//             println!("Un segundo no es nada");
//             1
//         },
//         Tiempo::Minuto => 60,
//         Tiempo::Hora => 3600,
//         Tiempo::Dia(mes) => {
//             println!("Ese día corresponde al mes {:?}", mes);
//             86400
//         },
//     }
// }


fn incrementar_uno(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}


fn una_jugada_sin_tirar() {}
fn vuelve_a_tirar() {}
fn avanza(numero_celdas: u8) {}
fn vuelve_a_salida() {}

fn main() {

    // let dia_enero = Tiempo::Dia(Mes::Enero);
    // let tiempo_segundos = valor_en_segundos(dia_enero);


    let cinco = Some(5);
    let seis = incrementar_uno(cinco);
    let nada = incrementar_uno(None);

    
    let dado = 5;

    match dado {
        1 => una_jugada_sin_tirar(),
        6 => vuelve_a_tirar(),
        // other => avanza(other),
        _ => vuelve_a_salida(),

    }


}






