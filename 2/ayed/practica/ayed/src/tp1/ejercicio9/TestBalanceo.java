package tp1.ejercicio9;

import java.util.Stack;

public class TestBalanceo {

    public static boolean esBalanceado(String s) {
        if (s == null) return false;
        Stack<Character> pila = new Stack<>();
        
        for (int i = 0; i < s.length(); i++) {
            char c = s.charAt(i);
            
            // Si es un caracter de apertura, apilar
            if (c == '(' || c == '[' || c == '{') {
                pila.push(c);
            } 
            // Si es un caracter de cierre, verificar
            else if (c == ')' || c == ']' || c == '}') {
                if (pila.isEmpty()) {
                    return false;
                }
                char tope = pila.pop();
                if (!esPar(tope, c)) {
                    return false;
                }
            }
        }
        
        return pila.isEmpty();
    }

    private static boolean esPar(char apertura, char cierre) {
        return (apertura == '(' && cierre == ')') ||
               (apertura == '[' && cierre == ']') ||
               (apertura == '{' && cierre == '}');
    }

    public static void main(String[] args) {
        String input;
        if (args.length > 0) {
            input = args[0];
            System.out.println("String a evaluar (pasado por argumento): \"" + input + "\"");
        } else {
            input = "{{( ) [ ( ) ] }}";
            System.out.println("No se pasaron argumentos. Usando valor de prueba por defecto: \"" + input + "\"");
        }
        
        if (esBalanceado(input)) {
            System.out.println("El String está balanceado.");
        } else {
            System.out.println("El String NO está balanceado.");
        }
    }
}
