package tp1.ejercicio3;

public class Estudiante {
    private String nombre;
    private String apellido;
    private String comision;
    private String email;
    private String direccion;

    // Getters and Setters
    public String getNombre() {
        return nombre;
    }

    public void setNombre(String nombre) {
        this.nombre = nombre;
    }

    public String getApellido() {
        return apellido;
    }

    public void setApellido(String apellido) {
        this.apellido = apellido;
    }

    public String getComision() {
        return comision;
    }

    public void setComision(String comision) {
        this.comision = comision;
    }

    public String getEmail() {
        return email;
    }

    public void setEmail(String email) {
        this.email = email;
    }

    public String getDireccion() {
        return direccion;
    }

    public void setDireccion(String direccion) {
        this.direccion = direccion;
    }

    public String tusDatos() {
        return "Estudiante: " + getNombre() + " " + getApellido() + 
               " | Comisión: " + getComision() + 
               " | Email: " + getEmail() + 
               " | Dirección: " + getDireccion();
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Estudiante est = (Estudiante) o;
        return (nombre != null ? nombre.equals(est.nombre) : est.nombre == null) &&
               (apellido != null ? apellido.equals(est.apellido) : est.apellido == null) &&
               (email != null ? email.equals(est.email) : est.email == null);
    }
}
