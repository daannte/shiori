from library.models import Book
from django.db import models


class MetadataSource(models.Model):
    name = models.CharField(max_length=50)


class ExternalBookID(models.Model):
    book = models.ForeignKey(Book, on_delete=models.CASCADE)
    source = models.ForeignKey(MetadataSource, on_delete=models.CASCADE)
    external_id = models.CharField(max_length=200)
