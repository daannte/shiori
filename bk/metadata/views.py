from http import HTTPMethod
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
        methods=[HTTPMethod.GET],
        serializer_class=MetadataSearchSerializer,
    )
    def search(self, request):
        serializer = self.get_serializer(data=request.query_params)
        serializer.is_valid(raise_exception=True)

        data = serializer.validated_data
        source = data["source"]
        external_id = data["external_id"]
        provider = PROVIDERS[source]

        book_data = provider.search(external_id)
        if book_data is None:
            raise NotFound(f"No book found for external ID {external_id} from {source}")

        return Response(book_data, status=status.HTTP_200_OK)
