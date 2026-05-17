import { CacheStore } from "../infrastructure/CacheStore";

export class TokenValidator {
  constructor(private readonly cache: CacheStore) {}

  isValid(token: string): boolean {
    return this.cache.get(token) === "valid";
  }
}

