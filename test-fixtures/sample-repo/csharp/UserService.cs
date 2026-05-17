using Apex.Data;

namespace Apex.Service;

public interface IUserReader
{
    string? FindUser(string id);
}

public class UserService : IUserReader
{
    private readonly UserRepository repository;

    public UserService(UserRepository repository)
    {
        this.repository = repository;
    }

    public string? FindUser(string id)
    {
        return repository.Find(id);
    }
}

