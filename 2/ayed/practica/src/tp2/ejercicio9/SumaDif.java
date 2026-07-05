package tp2.ejercicio9;

public class SumaDif {
    private int suma;
    private int diferencia;

    public SumaDif(int suma, int diferencia) {
        this.suma = suma;
        this.diferencia = diferencia;
    }

    public int getSuma() {
        return suma;
    }

    public int getDiferencia() {
        return diferencia;
    }

    @Override
    public String toString() {
        return "[" + suma + "|" + diferencia + "]";
    }
}
