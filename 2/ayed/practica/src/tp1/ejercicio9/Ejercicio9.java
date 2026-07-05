package tp1.ejercicio9;

public class Ejercicio9 {
    public static void main(String[] args) {
        String[] testStrings = {
            "{{( ) [ ( ) ] }}", // Balanceado
            "( [ ) ]",          // No balanceado
            "()",                // Balanceado
            "[()",               // No balanceado
            "())",               // No balanceado
            "{[()]}",            // Balanceado
            "",                  // Balanceado
            "abc",               // Balanceado (no tiene paréntesis, por ende balanceado)
            "([{}])"             // Balanceado
        };

        System.out.println("=== Probando Balanceo de Caracteres ===");
        for (String test : testStrings) {
            boolean res = TestBalanceo.esBalanceado(test);
            System.out.printf("String: \"%-20s\" -> Balanceado? %s\n", test, res);
        }
    }
}
