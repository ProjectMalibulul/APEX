from abc import ABC


class BaseService(ABC):
    def is_ready(self) -> bool:
        return True

