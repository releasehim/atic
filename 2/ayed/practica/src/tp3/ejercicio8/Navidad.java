package tp3.ejercicio8;

import tp3.GeneralTree;

public class Navidad {
    private GeneralTree<Integer> abeto;

    public Navidad(GeneralTree<Integer> abeto) {
        this.abeto = abeto;
    }

    public String esAbetoNavidenio() {
        if (this.abeto == null || this.abeto.isEmpty()) {
            return "No";
        }
        return esAbetoAux(this.abeto) ? "Yes" : "No";
    }

    private boolean esAbetoAux(GeneralTree<Integer> arbol) {
        if (arbol.isLeaf()) {
            return true;
        }
        int countHojas = 0;
        for (GeneralTree<Integer> child : arbol.getChildren()) {
            if (child.isLeaf()) {
                countHojas++;
            }
        }
        if (countHojas < 3) {
            return false;
        }
        for (GeneralTree<Integer> child : arbol.getChildren()) {
            if (!esAbetoAux(child)) {
                return false;
            }
        }
        return true;
    }
}
