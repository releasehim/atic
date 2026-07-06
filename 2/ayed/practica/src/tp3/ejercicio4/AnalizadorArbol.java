package tp3.ejercicio4;

import tp3.GeneralTree;
import tp1.ejercicio8.Queue;

public class AnalizadorArbol {

    public double devolverMaximoPromedio(GeneralTree<AreaEmpresa> arbol) {
        if (arbol == null || arbol.isEmpty()) {
            return 0.0;
        }
        Queue<GeneralTree<AreaEmpresa>> cola = new Queue<>();
        cola.enqueue(arbol);
        double maxPromedio = -1.0;

        while (!cola.isEmpty()) {
            int nodosNivel = cola.size();
            int sumaNivel = 0;
            for (int i = 0; i < nodosNivel; i++) {
                GeneralTree<AreaEmpresa> actual = cola.dequeue();
                sumaNivel += actual.getData().getTardanza();
                for (GeneralTree<AreaEmpresa> child : actual.getChildren()) {
                    cola.enqueue(child);
                }
            }
            double promedioNivel = (double) sumaNivel / nodosNivel;
            maxPromedio = Math.max(maxPromedio, promedioNivel);
        }
        return maxPromedio;
    }
}
