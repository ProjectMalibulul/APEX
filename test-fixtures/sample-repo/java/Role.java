import jakarta.persistence.Entity;

@Entity
public class Role {
    private final String name;

    public Role(String name) {
        this.name = name;
    }
}

