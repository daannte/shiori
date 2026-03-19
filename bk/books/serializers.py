from pathlib import Path

import requests
from django.core.files.base import ContentFile
from django.db import transaction
from django.utils.text import slugify
from rest_framework import serializers

from books.models import Author, Book
from metadata.models import ExternalBookID, MetadataSource


class AuthorSerializer(serializers.ModelSerializer):
    class Meta:
        model = Author
        fields = ["id", "name", "bio"]


class BookSerializer(serializers.ModelSerializer):
    authors = AuthorSerializer(many=True, read_only=True)
    epub_file_url = serializers.SerializerMethodField()
    cover_image_url = serializers.SerializerMethodField()

    class Meta:
        model = Book
        fields = [
            "id",
            "title",
            "authors",
            "description",
            "epub_file_url",
            "cover_image_url",
            "asin",
            "isbn",
            "isbn13",
            "pages",
            "publication_time",
            "publisher",
            "language",
            "currently_reading",
            "read",
            "type",
            "created_at",
        ]
        read_only_fields = ["id"]

    def get_epub_file_url(self, obj):
        return obj.epub_file.url if obj.epub_file else None

    def get_cover_image_url(self, obj):
        return obj.cover_image.url if obj.cover_image else None


class BookCreateSerializer(serializers.ModelSerializer):
    file = serializers.FileField(write_only=True)

    class Meta:
        model = Book
        fields = ["file"]

    def create(self, validated_data):
        uploaded_file = validated_data.get("file")

        original_name = Path(uploaded_file.name).stem
        title = slugify(original_name) or "untitled"

        book = Book.objects.create(title=title)  # ty:ignore[unresolved-attribute]
        book.epub_file.save("book.epub", uploaded_file, save=True)

        return book


class EnrichAuthorSerializer(serializers.Serializer):
    name = serializers.CharField()
    bio = serializers.CharField(allow_blank=True, required=False)


class EnrichBookSerializer(serializers.Serializer):
    title = serializers.CharField(required=False, allow_blank=True)
    description = serializers.CharField(required=False, allow_blank=True)
    cover_image_url = serializers.URLField(required=False, allow_blank=True)
    asin = serializers.CharField(required=False, allow_blank=True)
    isbn = serializers.CharField(required=False, allow_blank=True)
    isbn13 = serializers.CharField(required=False, allow_blank=True)
    pages = serializers.IntegerField(required=False)
    publication_time = serializers.DateTimeField(required=False)
    publisher = serializers.CharField(required=False, allow_blank=True)
    language = serializers.CharField(required=False, allow_blank=True)

    authors = EnrichAuthorSerializer(many=True, required=False, default=[])
    external_source = serializers.ChoiceField(choices=MetadataSource.choices)
    external_id = serializers.CharField()

    def update(self, instance: Book, validated_data: dict):
        with transaction.atomic():
            for field in [
                "title",
                "description",
                "asin",
                "isbn",
                "isbn13",
                "pages",
                "publication_time",
                "publisher",
                "language",
            ]:
                if field in validated_data:
                    setattr(instance, field, validated_data[field])

            cover_url = validated_data.get("cover_image_url")
            if cover_url:
                try:
                    resp = requests.get(cover_url)
                    resp.raise_for_status()
                    instance.cover_image.save(  # ty:ignore[unresolved-attribute]
                        "cover.jpg", ContentFile(resp.content), save=False
                    )
                except Exception as e:
                    print(f"Failed to download cover image: {e}")

            instance.save()

            authors_data = validated_data.get("authors", [])
            if authors_data:
                instance.authors.clear()  # ty:ignore[unresolved-attribute]
                for author_info in authors_data:
                    name = author_info.get("name")
                    if not name:
                        continue
                    bio = author_info.get("bio", "")
                    author, _ = Author.objects.get_or_create(name=name)  # ty:ignore[unresolved-attribute]
                    if bio and author.bio != bio:
                        author.bio = bio
                        author.save()
                    instance.authors.add(author)  # ty:ignore[unresolved-attribute]

            ExternalBookID.objects.get_or_create(  # ty:ignore[unresolved-attribute]
                book=instance,
                source=validated_data["external_source"],
                external_id=validated_data["external_id"],
            )

        return instance
