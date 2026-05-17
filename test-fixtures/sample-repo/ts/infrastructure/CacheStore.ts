export class CacheStore {
  private readonly values = new Map<string, string>();

  get(key: string): string | undefined {
    return this.values.get(key);
  }
}

