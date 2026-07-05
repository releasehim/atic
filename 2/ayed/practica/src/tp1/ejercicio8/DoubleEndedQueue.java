package tp1.ejercicio8;

public class DoubleEndedQueue<T> extends Queue<T> {

    public void enqueueFirst(T dato) {
        // Al ser data de tipo protected, podemos acceder directamente y agregar al principio
        data.add(0, dato);
    }
}
