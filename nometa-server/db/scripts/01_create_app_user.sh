#!/bin/bash
set -e

PGPASSWORD="$POSTGRES_PASSWORD" psql -t -A -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" \
    -v app_user="$POSTGRES_APP_USER" -v app_password="$POSTGRES_APP_PASSWORD" <<-EOSQL

SET search_path TO nometa;
SET app.db_user = :app_user;
SET app.db_password = :app_password;

-- Create app user if it doesn't exist
DO
\$do\$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'nometa_app') THEN
        EXECUTE format(
            'CREATE ROLE %I LOGIN PASSWORD %L',
            current_setting('app.db_user'),
            current_setting('app.db_password')
        );

        EXECUTE format('GRANT USAGE ON SCHEMA nometa TO %I', current_setting('app.db_user'));
        EXECUTE format('GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA nometa TO %I', current_setting('app.db_user'));
        EXECUTE format('ALTER DEFAULT PRIVILEGES IN SCHEMA nometa GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO %I', current_setting('app.db_user'));
        EXECUTE format('GRANT USAGE, SELECT, UPDATE ON ALL SEQUENCES IN SCHEMA nometa TO %I', current_setting('app.db_user'));
        EXECUTE format('ALTER DEFAULT PRIVILEGES IN SCHEMA nometa GRANT USAGE, SELECT, UPDATE on sequences TO %I', current_setting('app.db_user'));
    END IF;

    -- Record migration
    IF NOT EXISTS (SELECT 1 FROM migrations WHERE filename = '01_create_app_user.sh') THEN
        INSERT INTO migrations(filename) VALUES ('01_create_app_user.sh');
    END IF;
END
\$do\$;

EOSQL
