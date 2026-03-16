from rest_framework.response import Response
from library.models import Book
from library.serializers import BookUploadSerializer, BookSerializer
from rest_framework import viewsets, status


class BookViewSet(viewsets.ModelViewSet):
    def get_queryset(self):
        return Book.objects.all()  # ty:ignore[unresolved-attribute]

    def get_serializer_class(self):
        match self.action:
            case "create":
                return BookUploadSerializer
            case "list" | "retrieve":
                return BookSerializer
            case _:
                return super().get_serializer_class()

    def create(self, request, *args, **kwargs):
        serializer = self.get_serializer(data=request.data)
        serializer.is_valid(raise_exception=True)
        book = serializer.save()

        output = BookSerializer(book)
        return Response(output.data, status=status.HTTP_201_CREATED)
