package tp1.ejercicio5;

public class Valores {
    private int max;
    private int min;
    private double promedio;

    public int getMax() {
        return max;
    }

    public void setMax(int max) {
        this.max = max;
    }

    public int getMin() {
        return min;
    }

    public void setMin(int min) {
        this.min = min;
    }

    public double getPromedio() {
        return promedio;
    }

    public void setPromedio(double promedio) {
        this.promedio = promedio;
    }

    @Override
    public String toString() {
        return "Max: " + max + ", Min: " + min + ", Promedio: " + promedio;
    }
}
