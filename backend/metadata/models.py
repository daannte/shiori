from library.models import Book
from django.db import models


class MetadataSource(models.TextChoices):
    GOODREADS = "goodreads", "Goodreads"


class ExternalBookID(models.Model):
    book = models.ForeignKey(Book, on_delete=models.CASCADE)
    source = models.CharField(
        max_length=20,
        choices=MetadataSource.choices,
    )
    external_id = models.CharField(max_length=200)
