package tp1.ejercicio7;

import java.util.ArrayList;
import java.util.Iterator;
import java.util.LinkedList;
import java.util.List;
import tp1.ejercicio3.Estudiante;

public class Ejercicio7 {

    // 7.d: Demostración de copia y modificación de lista de estudiantes
    public static void probarCopiaYModificacion() {
        System.out.println("--- Ejercicio 7.d: Copia y Modificación ---");
        // 1. Crear lista con 3 estudiantes
        List<Estudiante> original = new ArrayList<>();
        
        Estudiante e1 = new Estudiante();
        e1.setNombre("Ana"); e1.setApellido("Gomez"); e1.setComision("1A");
        
        Estudiante e2 = new Estudiante();
        e2.setNombre("Pedro"); e2.setApellido("Perez"); e2.setComision("1B");
        
        Estudiante e3 = new Estudiante();
        e3.setNombre("Maria"); e3.setApellido("Diaz"); e3.setComision("1A");

        original.add(e1);
        original.add(e2);
        original.add(e3);

        // 2. Copia de la lista (Copia superficial)
        List<Estudiante> copia = new ArrayList<>(original);

        // 3. Imprimir ambas listas antes de modificar
        System.out.println("Antes de modificar:");
        System.out.println("Original: ");
        for (Estudiante est : original) System.out.println("  " + est.tusDatos());
        System.out.println("Copia: ");
        for (Estudiante est : copia) System.out.println("  " + est.tusDatos());

        // 4. Modificar un dato del estudiante pedro en la lista original
        original.get(1).setComision("3C");

        // 5. Imprimir de nuevo
        System.out.println("\nDespués de modificar la comisión de Pedro a '3C' en la lista original:");
        System.out.println("Original: ");
        for (Estudiante est : original) System.out.println("  " + est.tusDatos());
        System.out.println("Copia: ");
        for (Estudiante est : copia) System.out.println("  " + est.tusDatos());
    }

    // 7.e: Agregar estudiante sin repetir
    public static void agregarEstudianteSinRepetir(List<Estudiante> lista, Estudiante nuevo) {
        if (!lista.contains(nuevo)) {
            lista.add(nuevo);
            System.out.println("Estudiante " + nuevo.getNombre() + " agregado con éxito.");
        } else {
            System.out.println("El estudiante " + nuevo.getNombre() + " ya existe en la lista. No se agregó.");
        }
    }

    // 7.f: Determinar si es capicúa
    public boolean esCapicua(ArrayList<Integer> lista) {
        if (lista == null || lista.isEmpty()) return true;
        int i = 0;
        int j = lista.size() - 1;
        while (i < j) {
            if (!lista.get(i).equals(lista.get(j))) {
                return false;
            }
            i++;
            j--;
        }
        return true;
    }

    // 7.h: Invertir ArrayList recursivamente
    public void invertirArrayList(ArrayList<Integer> lista) {
        invertirArrayListHelper(lista, 0, lista.size() - 1);
    }

    private void invertirArrayListHelper(ArrayList<Integer> lista, int inicio, int fin) {
        if (inicio >= fin) {
            return;
        }
        Integer temp = lista.get(inicio);
        lista.set(inicio, lista.get(fin));
        lista.set(fin, temp);
        invertirArrayListHelper(lista, inicio + 1, fin - 1);
    }

    // 7.i: Sumar LinkedList recursivamente
    public int sumarLinkedList(List<Integer> lista) {
        if (lista == null) return 0;
        return sumarLinkedListHelper(lista.iterator());
    }

    private int sumarLinkedListHelper(Iterator<Integer> it) {
        if (!it.hasNext()) {
            return 0;
        }
        return it.next() + sumarLinkedListHelper(it);
    }

    // 7.j: Combinar dos listas ordenadas en una nueva lista ordenada
    public ArrayList<Integer> combinarOrdenado(ArrayList<Integer> lista1, ArrayList<Integer> lista2) {
        ArrayList<Integer> resultado = new ArrayList<>();
        int i = 0;
        int j = 0;
        while (i < lista1.size() && j < lista2.size()) {
            if (lista1.get(i) <= lista2.get(j)) {
                resultado.add(lista1.get(i));
                i++;
            } else {
                resultado.add(lista2.get(j));
                j++;
            }
        }
        while (i < lista1.size()) {
            resultado.add(lista1.get(i));
            i++;
        }
        while (j < lista2.size()) {
            resultado.add(lista2.get(j));
            j++;
        }
        return resultado;
    }

    public static void main(String[] args) {
        Ejercicio7 ej = new Ejercicio7();

        // Probar 7.d
        probarCopiaYModificacion();

        // Probar 7.e
        System.out.println("\n--- Ejercicio 7.e: Agregar sin repetir ---");
        List<Estudiante> estudiantes = new ArrayList<>();
        Estudiante est1 = new Estudiante();
        est1.setNombre("Lucas"); est1.setApellido("Rios"); est1.setEmail("lucas@mail.com");
        
        Estudiante est2 = new Estudiante();
        est2.setNombre("Sofia"); est2.setApellido("Paz"); est2.setEmail("sofia@mail.com");

        Estudiante estRepetido = new Estudiante(); // Igual a est1 lógicamente
        estRepetido.setNombre("Lucas"); estRepetido.setApellido("Rios"); estRepetido.setEmail("lucas@mail.com");

        agregarEstudianteSinRepetir(estudiantes, est1);
        agregarEstudianteSinRepetir(estudiantes, est2);
        agregarEstudianteSinRepetir(estudiantes, estRepetido); // Debería indicar que ya existe

        // Probar 7.f: esCapicua
        System.out.println("\n--- Ejercicio 7.f: esCapicua ---");
        ArrayList<Integer> capicuaList1 = new ArrayList<>();
        capicuaList1.add(2); capicuaList1.add(5); capicuaList1.add(2);
        ArrayList<Integer> capicuaList2 = new ArrayList<>();
        capicuaList2.add(4); capicuaList2.add(5); capicuaList2.add(6); capicuaList2.add(3); capicuaList2.add(4);
        
        System.out.println("[2, 5, 2] es capicúa? " + ej.esCapicua(capicuaList1));
        System.out.println("[4, 5, 6, 3, 4] es capicúa? " + ej.esCapicua(capicuaList2));

        // Probar 7.g: Sucesión Collatz
        System.out.println("\n--- Ejercicio 7.g: Sucesión Collatz (n=6) ---");
        EjercicioSucesion sucesion = new EjercicioSucesion();
        List<Integer> listSucesion = sucesion.calcularSucesion(6);
        System.out.println("Sucesión para n=6: " + listSucesion);

        // Probar 7.h: Invertir ArrayList recursivo
        System.out.println("\n--- Ejercicio 7.h: Invertir ArrayList ---");
        ArrayList<Integer> listToInvert = new ArrayList<>();
        listToInvert.add(1); listToInvert.add(2); listToInvert.add(3); listToInvert.add(4); listToInvert.add(5);
        System.out.println("Original: " + listToInvert);
        ej.invertirArrayList(listToInvert);
        System.out.println("Invertido: " + listToInvert);

        // Probar 7.i: Sumar LinkedList recursivo
        System.out.println("\n--- Ejercicio 7.i: Sumar LinkedList ---");
        LinkedList<Integer> listToSum = new LinkedList<>();
        listToSum.add(10); listToSum.add(20); listToSum.add(30); listToSum.add(40);
        System.out.println("Elementos de la lista: " + listToSum);
        System.out.println("Suma recursiva: " + ej.sumarLinkedList(listToSum));

        // Probar 7.j: Combinar Ordenado
        System.out.println("\n--- Ejercicio 7.j: Combinar Ordenado ---");
        ArrayList<Integer> sorted1 = new ArrayList<>();
        sorted1.add(1); sorted1.add(3); sorted1.add(5); sorted1.add(7);
        ArrayList<Integer> sorted2 = new ArrayList<>();
        sorted2.add(2); sorted2.add(4); sorted2.add(6); sorted2.add(8); sorted2.add(10);
        System.out.println("Lista 1: " + sorted1);
        System.out.println("Lista 2: " + sorted2);
        System.out.println("Combinada y ordenada: " + ej.combinarOrdenado(sorted1, sorted2));
    }
}
