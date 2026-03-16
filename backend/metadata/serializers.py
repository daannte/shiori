from rest_framework import serializers


class MetadataSearchSerializer(serializers.Serializer):
    external_id = serializers.CharField(required=True)
    source = serializers.CharField(default="goodreads")
