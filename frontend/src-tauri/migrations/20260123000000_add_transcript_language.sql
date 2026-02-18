-- Add language column to transcript_settings table
-- Default to 'en' (English) for existing records
ALTER TABLE transcript_settings ADD COLUMN language TEXT DEFAULT 'en';
