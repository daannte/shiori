from library.serializers import BookSerializer
from rest_framework.exceptions import NotFound
from rest_framework.response import Response
from metadata.providers import PROVIDERS
from rest_framework.decorators import action
from metadata.serializers import MetadataSearchSerializer
from rest_framework import viewsets, status


class MetadataView(viewsets.GenericViewSet):
    def get_serializer_class(self):
        return MetadataSearchSerializer

    @action(
        detail=False,
        methods=["post"],
        serializer_class=MetadataSearchSerializer,
    )
    def search(self, request):
        serializer = self.get_serializer(data=request.data)
        serializer.is_valid(raise_exception=True)

        data = serializer.validated_data
        source = data["source"]
        external_id = data["external_id"]
        provider = PROVIDERS[source]

        book_data = provider.search(external_id)
        if book_data is None:
            raise NotFound(f"No book found for external ID {external_id} from {source}")

        book_json = {
            "id": None,
            "title": book_data.title,
            "authors": [
                {"id": None, "name": a.name, "bio": a.bio} for a in book_data.authors
            ],
            "description": book_data.description,
            "asin": book_data.details.asin if book_data.details else None,
            "isbn": book_data.details.isbn if book_data.details else None,
            "isbn13": book_data.details.isbn13 if book_data.details else None,
            "pages": book_data.details.pages if book_data.details else None,
            "publication_time": book_data.details.publication_time
            if book_data.details
            else None,
            "publisher": book_data.details.publisher if book_data.details else None,
            "language": book_data.details.language if book_data.details else None,
            "currently_reading": False,
            "read": False,
            "type": None,
            "created_at": None,
        }

        serializer = BookSerializer(book_json)

        return Response(serializer.data, status=status.HTTP_200_OK)
