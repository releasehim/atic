package tp3.ejercicio4;

import tp3.GeneralTree;

public class Ejercicio4 {
    public static void main(String[] args) {
        System.out.println("=== Probando Ejercicio 4: AnalizadorArbol ===");

        // Nivel 2
        GeneralTree<AreaEmpresa> nA = new GeneralTree<>(new AreaEmpresa("A", 4));
        GeneralTree<AreaEmpresa> nB = new GeneralTree<>(new AreaEmpresa("B", 7));
        GeneralTree<AreaEmpresa> nC = new GeneralTree<>(new AreaEmpresa("C", 5));
        GeneralTree<AreaEmpresa> nD = new GeneralTree<>(new AreaEmpresa("D", 6));
        GeneralTree<AreaEmpresa> nE = new GeneralTree<>(new AreaEmpresa("E", 10));
        GeneralTree<AreaEmpresa> nF = new GeneralTree<>(new AreaEmpresa("F", 18));
        GeneralTree<AreaEmpresa> nG = new GeneralTree<>(new AreaEmpresa("G", 9));
        GeneralTree<AreaEmpresa> nH = new GeneralTree<>(new AreaEmpresa("H", 12));
        GeneralTree<AreaEmpresa> nI = new GeneralTree<>(new AreaEmpresa("I", 19));

        // Nivel 1
        GeneralTree<AreaEmpresa> nJ = new GeneralTree<>(new AreaEmpresa("J", 13));
        nJ.addChild(nA);
        nJ.addChild(nB);
        nJ.addChild(nC);

        GeneralTree<AreaEmpresa> nK = new GeneralTree<>(new AreaEmpresa("K", 25));
        nK.addChild(nD);
        nK.addChild(nE);
        nK.addChild(nF);

        GeneralTree<AreaEmpresa> nL = new GeneralTree<>(new AreaEmpresa("L", 10));
        nL.addChild(nG);
        nL.addChild(nH);
        nL.addChild(nI);

        // Nivel 0 (Raíz)
        GeneralTree<AreaEmpresa> raiz = new GeneralTree<>(new AreaEmpresa("M", 14));
        raiz.addChild(nJ);
        raiz.addChild(nK);
        raiz.addChild(nL);

        AnalizadorArbol analizador = new AnalizadorArbol();
        double maxPromedio = analizador.devolverMaximoPromedio(raiz);

        System.out.println("Máximo promedio obtenido (esperado: 16.0): " + maxPromedio);
    }
}
