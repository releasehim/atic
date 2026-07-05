package tp1.ejercicio8;

import java.util.LinkedList;
import java.util.List;

public class Queue<T> extends Sequence<T> {
    // Definido como protected para permitir que DoubleEndedQueue acceda a la lista
    protected List<T> data;

    public Queue() {
        this.data = new LinkedList<>();
    }

    public void enqueue(T dato) {
        data.add(dato);
    }

    public T dequeue() {
        if (isEmpty()) {
            throw new RuntimeException("La cola está vacía");
        }
        return data.remove(0);
    }

    public T head() {
        if (isEmpty()) {
            throw new RuntimeException("La cola está vacía");
        }
        return data.get(0);
    }

    @Override
    public boolean isEmpty() {
        return data.isEmpty();
    }

    @Override
    public int size() {
        return data.size();
    }

    @Override
    public String toString() {
        return data.toString();
    }
}
