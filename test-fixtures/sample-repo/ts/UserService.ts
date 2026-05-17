import { UserRepository } from "./UserRepository";
import { TokenValidator } from "./api/TokenValidator";

function Service(): ClassDecorator {
  return () => undefined;
}

@Service()
export class UserService {
  constructor(
    private readonly repository: UserRepository,
    private readonly validator: TokenValidator
  ) {}

  canReadUser(token: string): boolean {
    return this.validator.isValid(token) && this.repository.findById("current") !== undefined;
  }
}

