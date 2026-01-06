-- Add down migration script here
DROP TRIGGER IF EXISTS update_transactions_updated_at ON transactions;

-- Drop function
DROP FUNCTION IF EXISTS update_updated_at_column();

-- Drop indexes
DROP INDEX IF EXISTS idx_transactions_created_at;
DROP INDEX IF EXISTS idx_transactions_source;

-- Drop table
DROP TABLE IF EXISTS transactions;
