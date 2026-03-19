from django.db import models


def book_directory_path(instance, filename):
    return f"{instance.id}/{filename}"


class Author(models.Model):
    name = models.CharField(max_length=255)
    bio = models.TextField(blank=True, null=True)


class BookSource(models.TextChoices):
    EPUB = "epub", "Epub"
    WISHLIST = "wishlist", "Wishlist"


class Book(models.Model):
    title = models.CharField(max_length=255)
    authors = models.ManyToManyField(Author)
    description = models.TextField(blank=True, default="")

    epub_file = models.FileField(upload_to=book_directory_path, blank=True)
    cover_image = models.ImageField(upload_to=book_directory_path, blank=True)

    asin = models.CharField(max_length=20, blank=True, default="")
    isbn = models.CharField(max_length=20, blank=True, default="")
    isbn13 = models.CharField(max_length=20, blank=True, default="")
    pages = models.IntegerField(blank=True, default=0)
    publication_time = models.DateTimeField(blank=True, null=True)
    publisher = models.CharField(max_length=255, blank=True, default="")
    language = models.CharField(max_length=50, blank=True, default="")

    currently_reading = models.BooleanField(default=False)
    read = models.BooleanField(default=False)

    type = models.CharField(max_length=10, choices=BookSource.choices, default="epub")

    created_at = models.DateTimeField(auto_now_add=True)
