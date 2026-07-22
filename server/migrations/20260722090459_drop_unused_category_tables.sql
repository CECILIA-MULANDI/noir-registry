-- Remove unused category tables. Categories were seeded in 20260226214413_add_categories.sql
-- but the v2 pivot to primitive-tag search (via package_keywords) made them dead weight.
-- No Rust code reads or writes these tables anymore.
-- Drop the child table first so the parent has nothing pointing at it.

DROP TABLE IF EXISTS package_categories;
DROP TABLE IF EXISTS categories;
