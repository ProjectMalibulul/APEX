import org.springframework.stereotype.Repository;

@Repository
public class UserRepository {
    public User find(String id) {
        Role role = new Role("reader");
        return new User(id, role);
    }
}

