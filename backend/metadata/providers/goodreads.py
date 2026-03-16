from datetime import datetime
from library.dataclasses import BookData, Details, Author
import json
from bs4 import BeautifulSoup
import requests
from metadata.providers.base import MetadataProvider


class GoodreadsProvider(MetadataProvider):
    name = "Goodreads"

    URL: str = "https://www.goodreads.com/book/show/"

    def search(self, title: str) -> BookData | None:
        r = requests.get(f"{self.URL}{title}")
        r.raise_for_status()

        soup = BeautifulSoup(r.text, "html.parser")
        script_tag = soup.find("script", id="__NEXT_DATA__")
        if not script_tag or not script_tag.string:
            return None

        data = json.loads(script_tag.string)
        apollo_state = data["props"]["pageProps"]["apolloState"]
        book_info = self._get_book_info(apollo_state)

        if book_info is None:
            return None

        book = BookData(
            title=book_info.get("title", ""),
            authors=[],
            description=book_info.get("description", ""),
            cover_url=book_info.get("imageUrl"),
            details=Details(),
        )

        self._fill_details(book, book_info)
        self._fill_authors(book, apollo_state)

        return book

    def _get_book_info(self, apollo_state: dict[str, dict]) -> dict | None:
        book_info = None
        for key in apollo_state:
            if "Book:kca" in key:
                book_info = apollo_state[key]
                break

        return book_info

    def _get_author_keys(self, apollo_state: dict[str, dict]) -> list[str]:
        author_keys = []
        for key in apollo_state:
            if "Contributor:kca" in key:
                author_keys.append(key)

        return author_keys

    def _fill_details(self, book: BookData, book_info: dict) -> None:
        book_details = book_info.get("details")
        if book_details is None:
            return

        ts = book_details.get("publicationTime")
        publication_time = datetime.fromtimestamp(ts / 1000) if ts else None

        lang = book_details.get("language") or {}

        details = Details(
            asin=book_details.get("asin"),
            isbn=book_details.get("isbn"),
            isbn13=book_details.get("isbn13"),
            pages=book_details.get("numPages"),
            publication_time=publication_time,
            publisher=book_details.get("publisher"),
            language=lang.get("name"),
        )

        book.details = details

    def _fill_authors(self, book: BookData, apollo_state: dict[str, dict]) -> None:
        author_keys = self._get_author_keys(apollo_state)
        for key in author_keys:
            author_details = apollo_state.get(key)
            if author_details is None:
                continue

            if author_details.get("name") is None:
                continue

            author = Author(
                name=author_details.get("name", ""),
                bio=author_details.get("description"),
            )

            book.authors.append(author)
