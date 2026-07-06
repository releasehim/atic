package tp3.ejercicio6;

import tp3.GeneralTree;

public class Ejercicio6 {
    public static void main(String[] args) {
        System.out.println("=== Probando Ejercicio 6: RedDeAguaPotable ===");

        // Construir la estructura del ejemplo:
        //                      [A]
        //           ___________/ | \___________
        //          /            |              \
        //        [B]           [C]           [D]           [E]
        //                     /   \        /  |  \  \  \
        //                   [F]   [G]     [H][I][J][K][P]
        //                          |            / \
        //                         [L]         [M] [N]

        GeneralTree<Character> nB = new GeneralTree<>('B');
        GeneralTree<Character> nE = new GeneralTree<>('E');

        // Subárbol de C
        GeneralTree<Character> nF = new GeneralTree<>('F');
        GeneralTree<Character> nL = new GeneralTree<>('L');
        GeneralTree<Character> nG = new GeneralTree<>('G');
        nG.addChild(nL);
        GeneralTree<Character> nC = new GeneralTree<>('C');
        nC.addChild(nF);
        nC.addChild(nG);

        // Subárbol de D
        GeneralTree<Character> nH = new GeneralTree<>('H');
        GeneralTree<Character> nI = new GeneralTree<>('I');
        GeneralTree<Character> nK = new GeneralTree<>('K');
        GeneralTree<Character> nP = new GeneralTree<>('P');
        
        GeneralTree<Character> nM = new GeneralTree<>('M');
        GeneralTree<Character> nN = new GeneralTree<>('N');
        GeneralTree<Character> nJ = new GeneralTree<>('J');
        nJ.addChild(nM);
        nJ.addChild(nN);

        GeneralTree<Character> nD = new GeneralTree<>('D');
        nD.addChild(nH);
        nD.addChild(nI);
        nD.addChild(nJ);
        nD.addChild(nK);
        nD.addChild(nP);

        // Raíz A
        GeneralTree<Character> raiz = new GeneralTree<>('A');
        raiz.addChild(nB);
        raiz.addChild(nC);
        raiz.addChild(nD);
        raiz.addChild(nE);

        RedDeAguaPotable red = new RedDeAguaPotable(raiz);
        double minCaudal = red.minimoCaudal(1000.0);

        System.out.println("Caudal mínimo recibido por una casa (esperado: 25.0): " + minCaudal);
    }
}
