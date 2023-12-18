use std::fs::OpenOptions;
use std::fs;
use std::io::{ prelude::*};
use std::path::Path;
use chrono::{DateTime, Local};

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
    date: DateTime<Local>
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
    
    
    if let Err(e) = writeln!(&file, "{}", header)
    {
        println!("No se pudo escribir: {}", e);
    }

}

fn add_line(path:&Path) {
    let mut file = OpenOptions::new()
    .write(true)
    .append(true)
    .create(false)
    .open(path)
    .unwrap();

    println!("Nombre cliente: ");
    let owner = formulas::ingreso_datos();
    println!("Contacto: ");
    let contact = formulas::ingreso_datos();

    println!("=======================");
    
    println!("Nombre mascota: ");
    let pet_name = formulas::ingreso_datos();
    println!("Edad mascota (en meses): ");
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
        date: Local::now()        
    };
    
    let text = format!("{},{},{},{},{},{},{},{},{},{}", treatment.owner, treatment.contact, treatment.pet_name, treatment.pet_age, treatment.pet_class, treatment.pet_gender, treatment.treatment_category, treatment.price, treatment.notes, treatment.date );

    if let Err(e) = writeln!(file, "{}", text.replace('\n',""))
        {
            println!("No se pudo escribir: {}", e);
        }
    
}

//leer entrada
fn read(path: &Path) {
    println!("Ingrese nombre de cliente (propietario)");
    let name_to_find = formulas::ingreso_datos();
    let contents = fs::read_to_string(path).expect("Error en la lectura del archivo"); 
    let mut line_number = 0;
    for line in contents.lines() {

        let fields: Vec<&str> = line.split(',').collect();
        if fields[0] == name_to_find.trim() {
            line_number = line_number + 1;
            println!("*************************************");
            println!("{}.-Nombre = {},\n contacto: {},\n nombre mascota: {},\n edad de la mascota: {} meses,\n tipo de mascota: {},\n genero: {},\n tipo de tratamiento: {},\n precio: {},\n notas: {},\n fecha: {}\n", line_number,fields[0],fields[1],fields[2],fields[3],fields[4],fields[5],fields[6],fields[7],fields[8],fields[9]);
        }
        
    }
    println!("Desea 1.editar    2.eliminar  3.salir");
    let answer = formulas::ingreso_numeros();
    if answer == 1 {
        let new_content = update(&name_to_find, path);
    }
    else if answer == 2{
        let new_content = delete(&name_to_find, path);
    }

        
    
}

//editar entrada
fn update(name: &str, path: &Path) {
    let mut file = OpenOptions::new()
    .write(true)
    .append(false)
    .create(false)
    .open(path)
    .unwrap();

    let contents = fs::read_to_string(&path).expect("Error en la lectura del archivo");
    let mut line_number = 0;
    println!("Escoja la entrada a modificar.");
    let option = formulas::ingreso_numeros();
    for line in contents.lines() {
        let mut fields: Vec<&str> = line.split(',').collect();
        println!("{}", fields[0]);
        if fields[0] == name.trim() {
            line_number = line_number + 1;
            if option == line_number {
                println!("*************************************");
                println!("Nombre = {},\n contacto {},\n nombre mascota: {},\n edad de la mascota: {} meses,\n tipo de mascota: {},\n genero: {},\n tipo de tratamiento: {},\n precio: {},\n notas: {},\n fecha: {}\n", fields[0],fields[1],fields[2],fields[3],fields[4],fields[5],fields[6],fields[7],fields[8],fields[9]);
                println!("Escoja al campo a editar.\n");
                println!("1.Nombre del propietario  2.contacto  3.nombre de la mascota\n 4. Edad de la mascota  5.Tipo de mascota\n 6.Genero de la mascota  7.tipo de tratamiento\n 8.Precio 9.Notas");
                let mut field_to_edit = loop {
                    let mut field_to_edit = formulas::ingreso_numeros();
                    if field_to_edit <= 9 && field_to_edit > 0 {
                        break field_to_edit - 1;
                    } else {
                        continue;
                    }

                };
                let field_to_edit = field_to_edit as usize;
                println!("Que deseas cambiar?");
                let new_data = formulas::ingreso_datos();
                fields[field_to_edit] = &new_data;
                let new_contents = format!("{},{},{},{},{},{},{},{},{},{}",fields[0],fields[1],fields[2],fields[3],fields[4],fields[5],fields[6],fields[7],fields[8],fields[9]);
                if let Err(e) = writeln!(file, "{}", new_contents.replace('\n',""))
                {
                    println!("No se pudo escribir: {}", e);
                }
                
                

            }
        }
    }
    
}

//eliminar entrada
fn delete(name: &str, path: &Path) {
    let mut file = OpenOptions::new()
    .write(true)
    .append(false)
    .create(false)
    .open(path)
    .unwrap();

    let contents = fs::read_to_string(&path).expect("Error en la lectura del archivo");
    let mut line_number = 0;
    println!("Escoja la entrada a eliminar.");
    let option = formulas::ingreso_numeros();
    for line in contents.lines() {
        let mut fields: Vec<&str> = line.split(',').collect();
        
        if fields[0] == name.trim() {
            line_number = line_number + 1;
            if option == line_number {
                println!("*************************************");
                println!("Nombre = {},\n contacto {},\n nombre mascota: {},\n edad de la mascota: {} meses,\n tipo de mascota: {},\n genero: {},\n tipo de tratamiento: {},\n precio: {},\n notas: {},\n fecha: {}\n", fields[0],fields[1],fields[2],fields[3],fields[4],fields[5],fields[6],fields[7],fields[8],fields[9]);
                println!("Seguro?");
                let answer = formulas::ingreso_datos();
                if answer == "si"{
                
                    if let Err(e) = writeln!(file, "")
                    {
                        println!("No se pudo escribir: {}", e);
                    }
                }
            }
        }
            

            
    
    }
}


fn main() {
    print!("Bienvenido\n");
    print!("Que desea hacer?\n");
    print!("1. Crear archivo  2. Buscar Cliente   3.Agregar entrada\n");
    
    let first_action = loop {
        let first_action = formulas::ingreso_numeros();
        if first_action <= 3 {
            break first_action;
        } else {
            continue;
        }
    };
    
    let file_path = Path::new("data.csv");
    if first_action == 1 {
        let mut new_entry = create(&file_path);
    }
    else if first_action == 2 {
        let mut find_pet = read(&file_path);
    }
    else {
        let new_entry = add_line(&file_path);
    }
    
    
}
