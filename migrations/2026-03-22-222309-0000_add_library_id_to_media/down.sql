ALTER TABLE media
DROP CONSTRAINT fk_media_library;

ALTER TABLE media
DROP COLUMN library_id;
