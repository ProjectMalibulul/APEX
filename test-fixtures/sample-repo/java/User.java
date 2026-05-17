import jakarta.persistence.Entity;
import jakarta.persistence.ManyToOne;

@Entity
public class User {
    private final String id;

    @ManyToOne
    private final Role role;

    public User(String id, Role role) {
        this.id = id;
        this.role = role;
    }
}

