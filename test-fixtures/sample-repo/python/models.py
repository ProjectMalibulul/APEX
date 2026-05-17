class models:
    class Model:
        pass

    @staticmethod
    def ForeignKey(target: str, on_delete: str) -> str:
        return f"{target}:{on_delete}"


class Author(models.Model):
    name = "anonymous"


class Article(models.Model):
    author = models.ForeignKey("Author", on_delete="CASCADE")

