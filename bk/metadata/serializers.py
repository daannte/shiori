from rest_framework import serializers


class MetadataSearchSerializer(serializers.Serializer):
    external_id = serializers.CharField(required=True)
    source = serializers.CharField(default="goodreads")


class MetadataAuthorSerializer(serializers.Serializer):
    name = serializers.CharField()
    bio = serializers.CharField(allow_blank=True, required=False)


class MetadataBookSerializer(serializers.Serializer):
    title = serializers.CharField()
    authors = MetadataAuthorSerializer(many=True)
    description = serializers.CharField(allow_blank=True)
    cover_image_url = serializers.CharField(allow_blank=True, required=False)
    asin = serializers.CharField(allow_blank=True, required=False)
    isbn = serializers.CharField(allow_blank=True, required=False)
    isbn13 = serializers.CharField(allow_blank=True, required=False)
    pages = serializers.IntegerField(required=False)
    publication_time = serializers.DateTimeField(required=False)
    publisher = serializers.CharField(allow_blank=True, required=False)
    language = serializers.CharField(allow_blank=True, required=False)
