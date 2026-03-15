from dataclasses import dataclass, field
from datetime import datetime


@dataclass
class Details:
    asin: str | None = None
    isbn: str | None = None
    isbn13: str | None = None
    pages: int | None = None
    publication_time: datetime | None = None
    publisher: str | None = None
    language: str | None = None

    def __str__(self):
        pub_time = (
            self.publication_time.strftime("%Y-%m-%d")
            if self.publication_time
            else "N/A"
        )
        return (
            f"ASIN: {self.asin}, ISBN: {self.isbn}, ISBN13: {self.isbn13}, "
            f"Pages: {self.pages}, Published: {pub_time}, "
            f"Publisher: {self.publisher}, Language: {self.language}"
        )


@dataclass
class Author:
    name: str = ""
    bio: str | None = None

    def __str__(self):
        return self.name


@dataclass
class BookData:
    title: str
    authors: list[Author] = field(default_factory=list)
    description: str = ""
    cover_url: str | None = None
    details: Details | None = None

    def __str__(self):
        authors_str = ", ".join(str(a) for a in self.authors) if self.authors else "N/A"
        return (
            f"Title: {self.title}\n"
            f"Authors: {authors_str}\n"
            f"Description: {self.description or 'N/A'}\n"
            f"Cover URL: {self.cover_url or 'N/A'}\n"
            f"Details: {self.details or 'N/A'}"
        )
