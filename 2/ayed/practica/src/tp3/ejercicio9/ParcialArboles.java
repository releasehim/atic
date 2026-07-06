package tp3.ejercicio9;

import tp3.GeneralTree;

public class ParcialArboles {

    public static boolean esDeSeleccion(GeneralTree<Integer> arbol) {
        if (arbol == null || arbol.isEmpty()) {
            return true;
        }
        if (arbol.isLeaf()) {
            return true;
        }

        int min = Integer.MAX_VALUE;
        for (GeneralTree<Integer> child : arbol.getChildren()) {
            if (child.getData() != null) {
                min = Math.min(min, child.getData());
            }
        }

        if (arbol.getData() == null || arbol.getData() != min) {
            return false;
        }

        for (GeneralTree<Integer> child : arbol.getChildren()) {
            if (!esDeSeleccion(child)) {
                return false;
            }
        }

        return true;
    }
}
