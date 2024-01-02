
# Lista de tareas en rust!
Esta es mi primera CLI escrita en el lenguaje rust, puede que tenga bugs pero espero mejorar y aprender más sobre este hermoso lenguaje para seguir creando cosas con el :D


## Uso y Ejemplos

* Primero:
> Clonar el repositorio
```
git clone https://github.com/Arnollcvc/rust-task-list.git
cd rust-task-list
```
* Segundo:
> Instalar las dependencias con cargo y compilar
```
cargo check
cargo run
# con esto ya estarías ejecutando el programa
```

* Actualmente hay 10 opciones

```
cargo run
...
  0  | menu
  1  | crear tarea
  2  | ver tareas
  3  | marcar completado
  4  | marcar incompleto
  5  | quitar tarea
  6  | marca todas como completado
  7  | marca todas como incompleto
  00 | borrar tareas

```
si, en esta lista solo hay 9 y eso es por qué el numero 10 es el "exit" que es una opción "global" con la que puedes salir o cancelar opciones tomadas.

* Ejemplo:

> Aquí estoy intentando añadir una tarea pero me arrepentí.
```
cargo run
...
[]:: 1 // crear/añadir tarea
Descripción de la tarea: exit
Cancelado...
[]::
```
#
