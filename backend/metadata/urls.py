from django.urls import path, include
from metadata import views
from rest_framework.routers import DefaultRouter

router = DefaultRouter()
router.register("", views.MetadataView, basename="metadata")

urlpatterns = [path("", include(router.urls))]
