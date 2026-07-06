package tp3.ejercicio11;

import tp3.GeneralTree;
import tp1.ejercicio8.Queue;

public class ParcialArboles {

    public static boolean resolver(GeneralTree<Integer> arbol) {
        if (arbol == null || arbol.isEmpty()) {
            return true;
        }
        Queue<GeneralTree<Integer>> cola = new Queue<>();
        cola.enqueue(arbol);
        int nivel = 0;

        while (!cola.isEmpty()) {
            int nodosNivel = cola.size();
            if (nodosNivel != nivel + 1) {
                return false;
            }
            for (int i = 0; i < nodosNivel; i++) {
                GeneralTree<Integer> actual = cola.dequeue();
                for (GeneralTree<Integer> child : actual.getChildren()) {
                    cola.enqueue(child);
                }
            }
            nivel++;
        }
        return true;
    }
}
