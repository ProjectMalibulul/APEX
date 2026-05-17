# Language support

Apex uses lightweight local recognizers. They are designed for fast architecture views, not compiler-grade semantic analysis.

| Language/file type | Extensions | Extracts |
|---|---|---|
| TypeScript / JavaScript | `.ts`, `.tsx`, `.js`, `.jsx` | classes, interfaces, imports, extends, implements |
| Python | `.py` | classes, base classes, Django-style model relations |
| Java | `.java` | classes, interfaces, Spring/JPA annotations, imports, implements |
| Go | `.go` | structs, interfaces, functions, imports |
| Rust | `.rs` | structs, enums, traits, impls, use imports |
| Kotlin | `.kt`, `.kts` | classes, interfaces, objects, imports, inheritance |
| C# | `.cs` | classes, interfaces, structs, records, using imports, inheritance |
| Prisma | `.prisma` | models and relations |
| SQL | `.sql` | tables and foreign-key references |
| Manifests | `.json`, `.toml`, `.yaml`, `.yml` | package/config context nodes |

Check your installed build:

```bash
cargo run -p apex-cli -- languages
```

