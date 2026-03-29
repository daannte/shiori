ALTER TABLE media_metadata
ADD COLUMN description TEXT,
ADD COLUMN genres TEXT[] NOT NULL DEFAULT '{}'
