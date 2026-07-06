package tp3.ejercicio6;

import tp3.GeneralTree;

public class RedDeAguaPotable {
    private GeneralTree<Character> estructura;

    public RedDeAguaPotable(GeneralTree<Character> estructura) {
        this.estructura = estructura;
    }

    public double minimoCaudal(double caudal) {
        if (this.estructura == null || this.estructura.isEmpty()) {
            return 0.0;
        }
        return minimoCaudalAux(this.estructura, caudal);
    }

    private double minimoCaudalAux(GeneralTree<Character> arbol, double caudal) {
        if (arbol.isLeaf()) {
            return caudal;
        }
        double min = Double.MAX_VALUE;
        double caudalHijo = caudal / arbol.getChildren().size();
        for (GeneralTree<Character> child : arbol.getChildren()) {
            min = Math.min(min, minimoCaudalAux(child, caudalHijo));
        }
        return min;
    }
}
