ALTER TABLE media_metadata
ALTER COLUMN published_at TYPE DATE USING published_at::DATE;
