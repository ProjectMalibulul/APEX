export interface IUserRepository {
  findByEmail(email: string): User | undefined;
}

export interface User {
  id: string;
  email: string;
}

