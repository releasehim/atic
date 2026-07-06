package tp3.ejercicio10;

import java.util.ArrayList;
import java.util.List;
import tp3.GeneralTree;

public class ParcialArboles {

    public static List<Integer> resolver(GeneralTree<Integer> arbol) {
        List<Integer> caminoMax = new ArrayList<>();
        if (arbol != null && !arbol.isEmpty()) {
            List<Integer> caminoAct = new ArrayList<>();
            int[] scoreMax = {-1};
            resolverAux(arbol, 0, 0, caminoAct, caminoMax, scoreMax);
        }
        return caminoMax;
    }

    private static void resolverAux(GeneralTree<Integer> nodo, int nivel, int scoreAct, 
                                     List<Integer> caminoAct, List<Integer> caminoMax, int[] scoreMax) {
        int valor = nodo.getData();
        int scoreNodo = valor * nivel;
        int nuevoScore = scoreAct + scoreNodo;
        boolean agregado = false;

        if (valor == 1) {
            caminoAct.add(1);
            agregado = true;
        }

        if (nodo.isLeaf()) {
            if (nuevoScore > scoreMax[0]) {
                scoreMax[0] = nuevoScore;
                caminoMax.clear();
                caminoMax.addAll(caminoAct);
            }
        } else {
            for (GeneralTree<Integer> child : nodo.getChildren()) {
                resolverAux(child, nivel + 1, nuevoScore, caminoAct, caminoMax, scoreMax);
            }
        }

        if (agregado) {
            caminoAct.remove(caminoAct.size() - 1); // Backtracking
        }
    }
}
