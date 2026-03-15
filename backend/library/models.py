from django.db import models


class Author(models.Model):
    name = models.CharField(max_length=255)


class Book(models.Model):
    title = models.CharField(max_length=255)
    authors = models.ManyToManyField(Author)
    description = models.TextField(blank=True)
    language = models.CharField(max_length=10, blank=True)
    file_path = models.TextField()
    created_at = models.DateTimeField(auto_now_add=True)
