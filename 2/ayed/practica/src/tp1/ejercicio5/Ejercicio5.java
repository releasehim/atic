package tp1.ejercicio5;

public class Ejercicio5 {

    // Variables estáticas para el inciso c
    public static int[] arregloEntrada;
    public static int maxResult;
    public static int minResult;
    public static double promedioResult;

    // a. Devuelve lo pedido por el mecanismo de retorno ("return")
    public static Valores calcularA(int[] arr) {
        if (arr == null || arr.length == 0) {
            return null;
        }
        Valores v = new Valores();
        int max = arr[0];
        int min = arr[0];
        int suma = 0;
        for (int val : arr) {
            if (val > max) max = val;
            if (val < min) min = val;
            suma += val;
        }
        v.setMax(max);
        v.setMin(min);
        v.setPromedio((double) suma / arr.length);
        return v;
    }

    // b. Devuelve lo pedido interactuando con algún parámetro (no arreglo)
    public static void calcularB(int[] arr, Valores res) {
        if (arr == null || arr.length == 0 || res == null) {
            return;
        }
        int max = arr[0];
        int min = arr[0];
        int suma = 0;
        for (int val : arr) {
            if (val > max) max = val;
            if (val < min) min = val;
            suma += val;
        }
        res.setMax(max);
        res.setMin(min);
        res.setPromedio((double) suma / arr.length);
    }

    // c. Devuelve lo pedido sin usar parámetros ni la sentencia "return"
    public static void calcularC() {
        if (arregloEntrada == null || arregloEntrada.length == 0) {
            return;
        }
        int max = arregloEntrada[0];
        int min = arregloEntrada[0];
        int suma = 0;
        for (int val : arregloEntrada) {
            if (val > max) max = val;
            if (val < min) min = val;
            suma += val;
        }
        maxResult = max;
        minResult = min;
        promedioResult = (double) suma / arregloEntrada.length;
    }

    public static void main(String[] args) {
        int[] datos = {10, 20, 5, 40, 15};

        System.out.println("Arreglo de entrada: [10, 20, 5, 40, 15]");

        // Probar inciso a
        Valores resA = calcularA(datos);
        System.out.println("Inciso A (return): " + resA);

        // Probar inciso b
        Valores resB = new Valores();
        calcularB(datos, resB);
        System.out.println("Inciso B (parámetro): " + resB);

        // Probar inciso c
        arregloEntrada = datos;
        calcularC();
        System.out.println("Inciso C (variables estáticas): Max: " + maxResult + 
                           ", Min: " + minResult + ", Promedio: " + promedioResult);
    }
}
