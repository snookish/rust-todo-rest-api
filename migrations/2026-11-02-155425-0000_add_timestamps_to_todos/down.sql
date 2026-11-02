-- This file should undo anything in `up.sql`
ALTER TABLE todos
    DROP COLUMN updated_at,
    DROP COLUMN created_at;
