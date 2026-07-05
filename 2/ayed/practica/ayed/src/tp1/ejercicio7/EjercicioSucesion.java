package tp1.ejercicio7;

import java.util.ArrayList;
import java.util.List;

public class EjercicioSucesion {
    public List<Integer> calcularSucesion(int n) {
        List<Integer> list;
        if (n == 1) {
            list = new ArrayList<>();
            list.add(1);
            return list;
        }
        int next = (n % 2 == 0) ? (n / 2) : (3 * n + 1);
        list = calcularSucesion(next);
        list.add(0, n);
        return list;
    }
}
