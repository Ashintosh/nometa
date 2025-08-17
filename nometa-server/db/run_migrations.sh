#!/bin/bash
set -e

run_psql() {
    PGPASSWORD="$POSTGRES_PASSWORD" psql -t -A -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" "$@"
}

log() { echo "[migrate] $*"; }

MIGRATIONS=($(ls /scripts | sort))
for MIGRATION in "${MIGRATIONS[@]}"; do
    # Check if migrations table exists
    TABLE_EXISTS=$(run_psql -c "SELECT to_regclass('$POSTGRES_DB.migrations');")

    if [[ "$TABLE_EXISTS" != "$POSTGRES_DB.migrations" ]]; then
        log "Migrations table not found, applying $MIGRATION..."
    else
        # Check if already applied
        ALREADY_APPLIED=$(run_psql -c "SELECT filename FROM $POSTGRES_DB.migrations WHERE filename='$MIGRATION';")
        if [[ -n "$ALREADY_APPLIED" ]]; then
            log "Skipping $MIGRATION, already applied."
            continue
        fi
    fi

    log "Applying migration $MIGRATION..."

    # For SQL files
    if [[ "$MIGRATION" == *.sql ]]; then
        run_psql -f "/scripts/$MIGRATION"
    fi

    # For shell scripts
    if [[ "$MIGRATION" == *.sh ]]; then
        bash "/scripts/$MIGRATION"
    fi
done
