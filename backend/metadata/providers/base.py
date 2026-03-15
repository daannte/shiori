from library.dataclasses import BookData
from abc import ABC, abstractmethod


class MetadataProvider(ABC):
    name: str = "base"

    @abstractmethod
    def search(self, title: str) -> BookData | None:
        pass
