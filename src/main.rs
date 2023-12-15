use std::fs::OpenOptions;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;
use chrono::{DateTime, Duration, Utc};

mod formulas;

//Estructura
#[derive(Debug)]
struct Treatment {
    owner: String,
    contact: String,
    pet_name: String,
    pet_age: u32,
    pet_class: String,
    pet_gender: String,
    treatment_category: String,
    notes: String,
    price: i32,
    date: DateTime<Utc>
}

//Agregar nueva entrada
fn create(path: &Path){
    //abro archivo o creo uno nuevo.
    let mut file = OpenOptions::new()
    .write(true)
    .append(false)
    .create(true)
    .open(path)
    .unwrap();

    
    //Evaluo si la cabecera esta escrita
    let header = "OWNER,CONTACT,PET_NAME,PET_AGE,PET_CLASS,TREATMENT_CATEGORY,NOTES,PRICE,DATE".to_string();
    let mut header_present = false;
    let reader = BufReader::new(&file);
    for line in reader.lines() {
        println!("hola");
        if let Ok(actual) = line {
            if actual.trim() == header {
                header_present = true;
            } 
            
        }
        break;
        
        // let mut linea = line.expect("Error")
        // if linea.trim() == header {
        //     header_present = true;
        //     break;
        // }
    }


    if !header_present {
        if let Err(e) = writeln!(&file, "{}", header)
        {
            println!("No se pudo escribir: {}", e);
        }
    }
      
    println!("Nombre cliente: ");
    let owner = formulas::ingreso_datos();
    println!("Contacto: ");
    let contact = formulas::ingreso_datos();

    println!("=======================");
    
    println!("Nombre mascota: ");
    let pet_name = formulas::ingreso_datos();
    println!("Edad mascota: ");
    let pet_age = formulas::ingreso_numeros_u32();
    println!("Tipo de mascota: ");
    let pet_class = formulas::ingreso_datos();
    println!("Ingrese genero de la mascota: ");
    let pet_gender = formulas::ingreso_datos();
    
    println!("=======================");

    println!("Categoria del tratamiento");
    let category = formulas::ingreso_datos();
    println!("Precio");
    let price = formulas::ingreso_numeros();
    println!("Notas:");
    let notes = formulas::ingreso_datos();
    
    println!("=======================");

    let mut treatment = Treatment {
        owner: owner,
        contact: contact,
        pet_name: pet_name,
        pet_age: pet_age,
        pet_class: pet_class,
        pet_gender:pet_gender,
        treatment_category: category,
        price: price,
        notes: notes,
        date: Utc::now()        
    };
    
    let text = format!("{},{},{},{},{},{},{},{},{},{}", treatment.owner, treatment.contact, treatment.pet_name, treatment.pet_age, treatment.pet_class, treatment.pet_gender, treatment.treatment_category, treatment.price, treatment.notes, treatment.date );

    if let Err(e) = writeln!(file, "{}", text.replace('\n',""))
        {
            println!("No se pudo escribir: {}", e);
        }
    

}

//leer entrada
fn read() {
    
}

//editar entrada
fn update() {

}

//eliminar entrada
fn delete() {

}


fn main() {
    print!("Bienvenido");
    let file_path = Path::new("data.csv");
    let mut new_entry = create(file_path);
}
