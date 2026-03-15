from django.db import models


class Author(models.Model):
    name = models.CharField(max_length=255)
    bio = models.TextField(blank=True, null=True)


class BookSource(models.TextChoices):
    EPUB = "epub", "Epub"
    WISHLIST = "wishlist", "Wishlist"


class Book(models.Model):
    title = models.CharField(max_length=255)
    authors = models.ManyToManyField(Author)
    description = models.TextField(blank=True)
    file = models.TextField(blank=True, null=True)

    cover_url = models.TextField(blank=True, null=True)
    asin = models.CharField(max_length=20, blank=True, null=True)
    isbn = models.CharField(max_length=20, blank=True, null=True)
    isbn13 = models.CharField(max_length=20, blank=True, null=True)
    pages = models.IntegerField(blank=True, null=True)
    publication_time = models.DateTimeField(blank=True, null=True)
    publisher = models.CharField(max_length=255, blank=True, null=True)
    language = models.CharField(max_length=50, blank=True, null=True)

    currently_reading = models.BooleanField(default=False)
    read = models.BooleanField(default=False)

    type = models.CharField(max_length=10, choices=BookSource.choices, default="epub")

    created_at = models.DateTimeField(auto_now_add=True)
