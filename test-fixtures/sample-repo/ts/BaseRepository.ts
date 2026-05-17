export abstract class BaseRepository<T> {
  protected readonly items = new Map<string, T>();

  findById(id: string): T | undefined {
    return this.items.get(id);
  }
}

