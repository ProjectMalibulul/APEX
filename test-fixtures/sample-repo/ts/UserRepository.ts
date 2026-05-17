import { BaseRepository } from "./BaseRepository";
import { IUserRepository, User } from "./IUserRepository";

export class UserRepository extends BaseRepository<User> implements IUserRepository {
  findByEmail(email: string): User | undefined {
    return [...this.items.values()].find((user) => user.email === email);
  }
}

