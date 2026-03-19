from abc import ABC, abstractmethod


class MetadataProvider(ABC):
    name: str = "base"

    @abstractmethod
    def search(self, title: str) -> dict | None:
        pass
