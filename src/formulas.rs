use std::io;

pub fn ingreso_datos()-> String {
    let mut dato = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut dato).unwrap();
    return dato
    
}

pub fn razon(total:f32, sintoma: f32) -> i32 {
    let frecuencia = (sintoma / total) * 10.0;
    return frecuencia.round() as i32
}

pub fn cambio_a_numero(texto: &str) -> f32 {
    let numero: f32 = match texto.trim().parse(){
        Ok(numero) => numero,
        Err(_) =>  return 0.0
    };
    return numero
}
pub fn cambio_a_entero(texto: &str) -> i32 {
    let numero: i32 = match texto.trim().parse(){
        Ok(numero) => numero,
        Err(_) =>  {
            println!("No valido");
            return 0
        }
    };
    return numero
}

pub fn sacar_porcentaje(total: f32, numero: f32) -> f32 {
    let porcentaje = (numero / total) * 100.0;
    return porcentaje
}

pub fn encontrar_mayor(arreglo: [f32;8]) -> f32{
    let mut i = 0;
    let mut max = 0.0;
    while i < arreglo.len() {
        for numero in arreglo{
            if numero > max{
                max = numero;
            } 
        }
        i = i + 1;
    }

    return max


}

pub fn ingreso_numeros()-> i32 {
    loop {
        let mut numero = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut numero).unwrap();
        let numero: i32 = match numero.trim().parse() {
            Ok(numero) => numero,
            Err(_) => {
                println!("No valido");
                continue;
            },
        };
        return numero
    }

}   
pub fn ingreso_numeros_u32()-> u32 {
    loop {
        let mut numero = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut numero).unwrap();
        let numero: u32 = match numero.trim().parse() {
            Ok(numero) => numero,
            Err(_) => {
                println!("No valido");
                continue;
            },
        };
        return numero
    }

}   
pub fn pregunta(palabra: String) -> () {
    println!("Ingrese {} (1=no, 2=si)", palabra);
    
}
