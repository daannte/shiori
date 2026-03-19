from datetime import datetime
import json
from bs4 import BeautifulSoup
import requests
from metadata.providers.base import MetadataProvider
from metadata.serializers import MetadataBookSerializer


class GoodreadsProvider(MetadataProvider):
    name = "Goodreads"
    URL: str = "https://www.goodreads.com/book/show/"

    def search(self, title: str) -> dict | None:
        r = requests.get(f"{self.URL}{title}")
        r.raise_for_status()

        soup = BeautifulSoup(r.text, "html.parser")
        script_tag = soup.find("script", id="__NEXT_DATA__")
        if not script_tag or not script_tag.string:
            return None

        data = json.loads(script_tag.string)
        apollo_state = data["props"]["pageProps"]["apolloState"]
        book_info = self._get_book_info(apollo_state)
        if not book_info:
            return None

        print(json.dumps(apollo_state, indent=4))

        book_dict = {
            "title": book_info.get("title", ""),
            "authors": self._extract_authors(apollo_state, book_info),
            "description": book_info.get("description", ""),
            "cover_image_url": book_info.get("imageUrl"),
            "asin": None,
            "isbn": None,
            "isbn13": None,
            "pages": None,
            "publication_time": None,
            "publisher": None,
            "language": None,
        }

        details = book_info.get("details", {})
        if details:
            ts = details.get("publicationTime")
            publication_time = datetime.fromtimestamp(ts / 1000) if ts else None
            lang = details.get("language") or {}

            book_dict.update(
                asin=details.get("asin"),
                isbn=details.get("isbn"),
                isbn13=details.get("isbn13"),
                pages=details.get("numPages"),
                publication_time=publication_time,
                publisher=details.get("publisher"),
                language=lang.get("name"),
            )

        serializer = MetadataBookSerializer(book_dict)
        print(json.dumps(serializer.data, indent=4))
        return serializer.data

    def _get_book_info(self, apollo_state: dict[str, dict]) -> dict | None:
        for key, val in apollo_state.items():
            if key.lower().startswith("book:"):
                return val
        return None

    def _get_primary_author_ref(self, book_info: dict) -> str | None:
        edge = book_info.get("primaryContributorEdge")
        if not edge:
            return None
        node = edge.get("node")
        if not node:
            return None
        return node.get("__ref")

    def _resolve_author_key(self, ref: str | None, apollo_state: dict) -> str | None:
        if not ref:
            return None
        if ref in apollo_state:
            return ref
        return None

    def _extract_authors(
        self, apollo_state: dict[str, dict], book_info: dict
    ) -> list[dict]:
        authors = []

        primary_ref = self._get_primary_author_ref(book_info)
        primary_key = self._resolve_author_key(primary_ref, apollo_state)

        if primary_key:
            author_details = apollo_state.get(primary_key)
            if author_details:
                authors.append(
                    {
                        "name": author_details.get("name", ""),
                        "bio": author_details.get("description") or "",
                    }
                )

        for edge in book_info.get("secondaryContributorEdges", []):
            if edge.get("role", "").lower() != "author":
                continue

            node_ref = edge.get("node", {}).get("__ref")
            key = self._resolve_author_key(node_ref, apollo_state)
            if not key:
                continue

            author_details = apollo_state.get(key)
            if not author_details or not author_details.get("name"):
                continue

            authors.append(
                {
                    "name": author_details.get("name", ""),
                    "bio": author_details.get("description") or "",
                    "is_primary": False,
                }
            )

        return authors
