package tp2.ejercicio9;

import tp2.BinaryTree;

public class ParcialArboles {

    public BinaryTree<?> sumAndDif(BinaryTree<Integer> arbol) {
        if (arbol == null || arbol.isEmpty()) {
            return new BinaryTree<>();
        }
        BinaryTree<SumaDif> nuevo = new BinaryTree<>();
        resolverSumAndDif(arbol, nuevo, 0, 0);
        return nuevo;
    }

    private void resolverSumAndDif(BinaryTree<Integer> original, BinaryTree<SumaDif> nuevo, int sumaPadre, int valorPadre) {
        if (original == null || original.isEmpty()) {
            return;
        }

        int sumaActual = sumaPadre + original.getData();
        int diferenciaActual = original.getData() - valorPadre;
        nuevo.setData(new SumaDif(sumaActual, diferenciaActual));

        if (original.hasLeftChild()) {
            BinaryTree<SumaDif> nuevoIzq = new BinaryTree<>();
            nuevo.addLeftChild(nuevoIzq);
            resolverSumAndDif(original.getLeftChild(), nuevoIzq, sumaActual, original.getData());
        }

        if (original.hasRightChild()) {
            BinaryTree<SumaDif> nuevoDer = new BinaryTree<>();
            nuevo.addRightChild(nuevoDer);
            resolverSumAndDif(original.getRightChild(), nuevoDer, sumaActual, original.getData());
        }
    }
}
