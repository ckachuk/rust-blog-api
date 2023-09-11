
-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";


CREATE TABLE
    IF NOT EXISTS categories(
        pk_category_id uuid PRIMARY KEY DEFAULT (uuid_generate_v4()),
        category_name VARCHAR(100) NOT NULL UNIQUE
    );

CREATE TABLE
    IF NOT EXISTS credentials(
        pk_credential_id uuid PRIMARY KEY DEFAULT (uuid_generate_v4()),
        is_author BOOLEAN DEFAULT 'f', 
        is_admin BOOLEAN DEFAULT 'f'
    );
CREATE TABLE
    IF NOT EXISTS users(
        pk_user_id uuid PRIMARY KEY DEFAULT (uuid_generate_v4()),
        credential_id uuid NOT NULL UNIQUE,
        username VARCHAR(50) NOT NULL UNIQUE,
        password VARCHAR(150) NOT NULL,
        fullname VARCHAR (100) NOT NULL,
        CONSTRAINT fk_credential_id FOREIGN KEY(credential_id)
            REFERENCES credentials(pk_credential_id),
        create_at TIMESTAMP 
            WITH TIME ZONE DEFAULT NOW(),
        update_at TIMESTAMP
            WITH TIME ZONE DEFAULT NOW()
    );
CREATE TABLE
    IF NOT EXISTS posts(
        pk_post_id uuid PRIMARY KEY DEFAULT (uuid_generate_v4()),
        user_id uuid NOT NULL UNIQUE,
        category_id uuid NOT NULL UNIQUE,
        title VARCHAR(255) NOT NULL,
        body VARCHAR(255) NOT NULL,
        CONSTRAINT fk_user_id FOREIGN KEY(user_id)
            REFERENCES users(pk_user_id),
        CONSTRAINT fk_category_id FOREIGN KEY(category_id)
            REFERENCES categories(pk_category_id),
        create_at TIMESTAMP 
            WITH TIME ZONE DEFAULT NOW(),
        update_at TIMESTAMP
            WITH TIME ZONE DEFAULT NOW()
    );

CREATE TABLE
    IF NOT EXISTS comments(
        pk_comment_id uuid PRIMARY KEY DEFAULT (uuid_generate_v4()),
        post_id uuid NOT NULL UNIQUE,
        user_id uuid NOT NULL UNIQUE,
        body VARCHAR(255) NOT NULL,
        CONSTRAINT fk_post_id FOREIGN KEY(post_id)
            REFERENCES posts(pk_post_id),
        CONSTRAINT fk_user_id FOREIGN KEY(user_id)
            REFERENCES users(pk_user_id),
        create_at TIMESTAMP 
            WITH TIME ZONE DEFAULT NOW(),
        update_at TIMESTAMP
            WITH TIME ZONE DEFAULT NOW()
    )