from books.models import Book
from django.db import models


class MetadataSource(models.TextChoices):
    GOODREADS = "goodreads", "Goodreads"


class ExternalBookID(models.Model):
    book = models.ForeignKey(
        Book, on_delete=models.CASCADE, related_name="external_ids"
    )
    source = models.CharField(
        max_length=20,
        choices=MetadataSource.choices,
    )
    external_id = models.CharField(max_length=200)

    class Meta:
        constraints = [
            models.UniqueConstraint(
                fields=["source", "external_id"],
                name="unique_external_book_source_id",
            )
        ]
        indexes = [models.Index(fields=["source", "external_id"])]
