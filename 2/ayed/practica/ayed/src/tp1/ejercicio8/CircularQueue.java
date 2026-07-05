package tp1.ejercicio8;

public class CircularQueue<T> extends Queue<T> {

    public T shift() {
        if (isEmpty()) {
            throw new RuntimeException("La cola está vacía");
        }
        T temp = dequeue();
        enqueue(temp);
        return temp;
    }
}
