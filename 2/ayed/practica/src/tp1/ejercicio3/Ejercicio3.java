package tp1.ejercicio3;

public class Ejercicio3 {
    public static void main(String[] args) {
        // Crear arreglo con 2 objetos Estudiante
        Estudiante[] estudiantes = new Estudiante[2];
        
        estudiantes[0] = new Estudiante();
        estudiantes[0].setNombre("Juan");
        estudiantes[0].setApellido("Perez");
        estudiantes[0].setComision("1A");
        estudiantes[0].setEmail("juan.perez@alumnos.info.unlp.edu.ar");
        estudiantes[0].setDireccion("Calle 1 y 60");

        estudiantes[1] = new Estudiante();
        estudiantes[1].setNombre("Maria");
        estudiantes[1].setApellido("Lopez");
        estudiantes[1].setComision("2B");
        estudiantes[1].setEmail("maria.lopez@alumnos.info.unlp.edu.ar");
        estudiantes[1].setDireccion("Calle 7 y 50");

        // Crear arreglo con 3 objetos Profesor
        Profesor[] profesores = new Profesor[3];

        profesores[0] = new Profesor();
        profesores[0].setNombre("Carlos");
        profesores[0].setApellido("Gomez");
        profesores[0].setEmail("carlos.gomez@info.unlp.edu.ar");
        profesores[0].setCatedra("Algoritmos y Estructuras de Datos");
        profesores[0].setFacultad("Informatica");

        profesores[1] = new Profesor();
        profesores[1].setNombre("Ana");
        profesores[1].setApellido("Martinez");
        profesores[1].setEmail("ana.martinez@info.unlp.edu.ar");
        profesores[1].setCatedra("Funda de Programacion");
        profesores[1].setFacultad("Informatica");

        profesores[2] = new Profesor();
        profesores[2].setNombre("Luis");
        profesores[2].setApellido("Rodriguez");
        profesores[2].setEmail("luis.rodriguez@info.unlp.edu.ar");
        profesores[2].setCatedra("Sistemas Operativos");
        profesores[2].setFacultad("Informatica");

        // Recorrer e imprimir estudiantes
        System.out.println("--- Lista de Estudiantes ---");
        for (Estudiante est : estudiantes) {
            System.out.println(est.tusDatos());
        }

        // Recorrer e imprimir profesores
        System.out.println("\n--- Lista de Profesores ---");
        for (Profesor prof : profesores) {
            System.out.println(prof.tusDatos());
        }
    }
}
