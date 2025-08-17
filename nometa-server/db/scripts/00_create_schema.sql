-- --------------------------
-- Create schema
-- --------------------------
-- Create schema and citext extension
CREATE SCHEMA IF NOT EXISTS nometa;
CREATE EXTENSION IF NOT EXISTS citext SCHEMA nometa;

SET search_path TO nometa;

-- Migrations table to track applied migrations
CREATE TABLE IF NOT EXISTS migrations
(
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    filename TEXT NOT NULL UNIQUE,
    applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- --------------------------
-- Create types
-- --------------------------
CREATE TYPE NM_USER_ROLE AS ENUM ('banned', 'user', 'admin', 'owner');
CREATE TYPE NM_USER_SUBSCRIPTION AS ENUM ('free', 'premium', 'vip');

-- --------------------------
-- Create users, profiles table
-- --------------------------
-- Users
CREATE TABLE IF NOT EXISTS users
(
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    email CITEXT NOT NULL UNIQUE,
    username CITEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    role NM_USER_ROLE DEFAULT 'user' NOT NULL,
    subscription NM_USER_SUBSCRIPTION DEFAULT 'free' NOT NULL,
    subscription_expires TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Profiles
CREATE TABLE IF NOT EXISTS profiles
(
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    picture TEXT,
    background TEXT,
    description TEXT,
    socials JSONB DEFAULT '{}'::jsonb,
    badges JSONB DEFAULT '{}'::jsonb,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- --------------------------
-- Create indexes, triggers
-- --------------------------
-- Indexes
CREATE INDEX IF NOT EXISTS idx_users_subscription ON users(subscription);
CREATE INDEX IF NOT EXISTS idx_users_role ON users(role);
CREATE INDEX IF NOT EXISTS idx_profiles_user_id ON profiles(user_id);

---- Trigger functions
CREATE OR REPLACE FUNCTION create_profile_for_new_user()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO nometa.profiles(user_id) VALUES (NEW.id);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

---- Triggers
-- Drop existing triggers if they exist
DROP TRIGGER IF EXISTS create_profile_for_new_user ON users;
DROP TRIGGER IF EXISTS trigger_profiles_updated_at ON profiles;

-- Create trigger to auto-create profile
CREATE TRIGGER trigger_create_profile
AFTER INSERT ON users
FOR EACH ROW
EXECUTE FUNCTION create_profile_for_new_user();

-- Create trigger to update profiles.updated_at on update
CREATE TRIGGER trigger_profiles_updated_at
BEFORE UPDATE ON profiles
FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

-- --------------------------
-- Record migration
-- --------------------------
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM migrations WHERE filename = '00_create_schema.sql') THEN
        INSERT INTO migrations(filename) VALUES ('00_create_schema.sql');
    END IF;
END;
$$;
