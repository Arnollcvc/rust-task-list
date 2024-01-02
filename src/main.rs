use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{self, prelude::*};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    completed: bool,
}

fn get_tasks(file_path: &str) -> Vec<Task> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .expect("Error al abrir/crear el archivo");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let tasks: Vec<Task> = if contents.is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&contents).expect("Error al deserializar el JSON")
    };
    tasks
}

fn add_task(file_path: &str, task: Task) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .expect("Error al abrir/crear el archivo");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut tasks: Vec<Task> = if contents.is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&contents).expect("Error al deserializar el JSON")
    };
    let new_task = task;
    tasks.push(new_task);

    let serialize = serde_json::to_string(&tasks).expect("Error la serializar a JSON");
    file.seek(std::io::SeekFrom::Start(0))
        .expect("Error al buscar al inicio del archivo");
    file.set_len(0).expect("Error la truncar el archivo");
    file.write_all(serialize.as_bytes())
        .expect("Error al escribir en el archivo");
    println!("Tarea añadida!");
}

fn input(msg: &str) -> String {
    print!("{}", &msg);
    io::stdout().flush().unwrap();
    let mut content = String::new();
    io::stdin().read_line(&mut content).unwrap();
    content.trim().to_string()
}

fn print_menu() {
    println!(
        "
    #-----~~Lista de Tareas~~-----#
        0  | menu
        1  | crear tarea
        2  | ver tareas
        3  | marcar completado
        4  | marcar incompleto
        5  | quitar tarea
        6  | marca todas como completado
        7  | marca todas como incompleto
        00 | borrar tareas
    "
    );
}

fn print_tasks(tasks: Vec<Task>) {
    let mut indice: usize = 0;
    let max: usize = tasks.len();
    let max_head_len: usize = 100;
    let max_desc_len: usize = 70;
    let header_char = "─";
    let header: String = {
        let head = header_char.repeat(max_head_len);
        head.to_string()
    };
    for task in tasks {
        indice += 1;
        if indice == 1 {
            println!("╭{}╮", header);
        }
        let indice_str = format!("{}", indice);
        let desc: String = task.description.to_string();
        let completed: String = if task.completed { "Si".to_string() } else { "No".to_string() };
        let wrap_desc: String = if desc.len() > max_desc_len {
            let l_part = &task.description[0..max_desc_len];
            let r_part = &task.description[max_desc_len..];
            let r = format!("{}..({})", l_part, r_part.chars().count());
            r.to_string()
        } else {
            task.description.to_string()
        };
        let fmt_id: String = {
            let id_str_len = indice_str.len();
            let white_space = "\u{0020}".repeat(max_head_len-id_str_len-6);
            let r = format!("{}│", white_space);
            r
        };
        let fmt_desc: String = {
            let white_space = "\u{0020}".repeat(max_head_len-wrap_desc.chars().count()-9);
            let r = format!("{}│", white_space);
            r
        };
        let fmt_complete: String = {
            let white_space = "\u{0020}".repeat(max_head_len-completed.len()-14);
            let r = format!("{}│", white_space);
            r
        };
        println!("│  ID: {}{}", indice, fmt_id);
        println!("│  Tarea: {}{}", wrap_desc, fmt_desc);
        println!("│  Completado: {}{}", completed, fmt_complete);
        if max == indice {
            println!("╰{}╯", header);
        } else {
            println!("├{}┤", header);
        }
    }
}

fn complete_task(file_path: &str, task_id: usize, complete: bool) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .expect("Error al abrir/crear el archivo");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut tasks: Vec<Task> = if contents.is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&contents).expect("Error al deserializar el JSON")
    };
    if task_id > tasks.len() {
        println!("No existe esa cantidad de tareas!");
    } else if task_id < 1 {
        println!("No puedes acceder con numeros negativos.");
    } else {
        if complete && tasks[task_id-1].completed {
            println!("La tarea '{}' ya está completada!", task_id);
        } else if !complete && !tasks[task_id-1].completed {
            println!("Que intentas hacer? ya está puesto como no completado...");
        } else {
            tasks[task_id-1].completed = complete;
            let serialize = serde_json::to_string(&tasks).expect("Error la serializar a JSON");
            file.seek(std::io::SeekFrom::Start(0)).expect("Error al buscar al inicio del archivo");
            file.set_len(0).expect("Error la truncar el archivo");
            file.write_all(serialize.as_bytes()).expect("Error al escribir en el archivo");
            println!("Tarea completada!");
        }
    }
}

fn remove_task(file_path: &str, task_id: usize) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .expect("Error al abrir/crear el archivo");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut tasks: Vec<Task> = if contents.is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&contents).expect("Error al deserializar el JSON")
    };
    if task_id > tasks.len() {
        println!("No hay esa cantidad de Tareas!");
    } else if task_id < 1 {
        println!("No acceder con numeros negativos.");
    } else if tasks.len() == 0 {
        println!("No hay tareas!");
    } else {
        tasks.remove(task_id-1);
        let serialize = serde_json::to_string(&tasks).expect("Error la serializar a JSON");
        file.seek(std::io::SeekFrom::Start(0)).expect("Error al buscar al inicio del archivo");
        file.set_len(0).expect("Error la truncar el archivo");
        file.write_all(serialize.as_bytes()).expect("Error al escribir en el archivo");
        println!("Listo!!\n¿Quieres eliminar otra?");
    }
}

fn toggle_all_tasks_complete_or_incomplete(file_path: &str, complete: bool) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .expect("Error al abrir/crear el archivo");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let tasks: Vec<Task> = if contents.is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&contents).expect("Error al deserializar el JSON")
    };
    if tasks.len() == 0 {
        println!("No hay tareas!");
    } else {
        let mut new_task_list: Vec<Task> = Vec::new();
        for mut t in tasks {
            t.completed = complete;
            new_task_list.push(t);
        }
        let serialize = serde_json::to_string(&new_task_list).expect("Error la serializar a JSON");
        file.seek(std::io::SeekFrom::Start(0)).expect("Error al buscar al inicio del archivo");
        file.set_len(0).expect("Error la truncar el archivo");
        file.write_all(serialize.as_bytes()).expect("Error al escribir en el archivo");
        let msg = if complete { "completas" } else { "incompletas" };
        println!("Todas las tareas han sido marcadas como {}!", msg);
    }
}

fn delete_all_tasks(file_path: &str) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .expect("Error al abrir/crear el archivo");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let tasks: Vec<Task> = if contents.is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&contents).expect("Error al deserializar el JSON")
    };
    if tasks.len() == 0 {
        println!("No hay tareas!");
    } else {
        file.seek(std::io::SeekFrom::Start(0)).expect("Error al buscar al inicio del archivo");
        file.set_len(0).expect("Error la truncar el archivo");
        file.write_all("".as_bytes()).expect("Error al escribir en el archivo");
        println!("Listo!");
    }
}

fn main() {
    const FILE_PATH: &str = "tareas.json";
    print_menu();
    loop {
        let option = input("[]:: ");
        if option.to_lowercase() == "exit" {
            println!("Saliendo...");
            break;
        } else if option == "menu" || option == "0" {
            print_menu();
        } else if option == "1" {
            loop {
                let desc = input("Descripción de la tarea: ");
                if desc.trim().len() == 0  {
                    println!("No puedes dejar este espacio en blanco.");
                    continue;
                } else if desc.trim().to_lowercase() == "exit" {
                    println!("Cancelado...");
                    break;
                }
                add_task(
                    FILE_PATH,
                    Task {
                        description: desc.trim().to_string(),
                        completed: false,
                    },
                );
                println!("¿Quieres añadir otra?");
            }
        } else if option == "2" {
            let tasks = get_tasks(FILE_PATH);
            if tasks.len() == 0 {
                println!("No hay tareas!!");
                continue;
            }
            print_tasks(tasks);
        } else if option == "3" || option == "4" {
            loop {
                let complete_state: bool = if option == "3" { true } else { false };
                let tasks = get_tasks(FILE_PATH);
                if tasks.len() == 0  {
                    println!("No hay tareas...");
                    break;
                }
                let task_id = input("Ingresa el ID de la tarea: ");
                if task_id.trim().len() == 0 {
                    println!("No puedes dejar este espacio en blanco. (exit para salir)");
                    continue;
                } else if task_id.trim().to_lowercase() == "exit" {
                    println!("Cancelado.");
                    break;
                } else {
                    let t_id = task_id.parse::<usize>().unwrap_or_else(|_|  0);
                    if t_id < 1 {
                        println!("Id no valido, no pueden ser numeros negativos o '0'");
                        continue;
                    } else if t_id > tasks.len() {
                        println!("No hay esa cantidad de Tareas..!");
                        continue;
                    }
                    complete_task(FILE_PATH, t_id, complete_state);
                    break;
                }
            }
        } else if option == "5" {
            loop {
                let tasks = get_tasks(FILE_PATH);
                if tasks.len() == 0 {
                    println!("No hay tareas!");
                    break;
                }
                let task_id = input("Ingresa el Id para eliminar la tarea: ");
                if task_id.trim().len() == 0 {
                    println!("No pudes dejar este espacio en blanco. (exit para salir)");
                } else if task_id.trim().to_lowercase() == "exit" {
                    println!("Cancelado.");
                    break;
                } else {
                    let t_id: usize = task_id.parse::<usize>().unwrap_or_else(|_| 0);
                    if t_id < 1 {
                        println!("Id no valido, no pueden ser numeros negativos o '0'");
                        continue;
                    } else if t_id > tasks.len() {
                        println!("No hay esa cantidad de tareas..!");
                        continue;
                    }
                    remove_task(FILE_PATH, t_id);
                }
            }
        } else if option == "6"  || option == "7" {
            let complete: bool = if option == "6" { true } else { false };
            toggle_all_tasks_complete_or_incomplete(FILE_PATH, complete);
        } else if option.trim() == "00" {
            delete_all_tasks(FILE_PATH);
        }
    }
}
