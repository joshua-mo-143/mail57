CREATE TABLE IF NOT EXISTS Users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
);

CREATE TABLE IF NOT EXISTS Sessions (
    id SERIAL PRIMARY KEY,
    user_id int NOT NULL FOREIGN KEY REFERENCES Users(id),
    session_id VARCHAR UNIQUE NOT NULL,
    expires TIMESTAMP NOT NULL,
);

CREATE TABLE IF NOT EXISTS Campaigns (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    desc VARCHAR,
    email_body VARCHAR,
    owner_id int,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
);

CREATE TABLE IF NOT EXISTS Subscribers (
    id SERIAL PRIMARY KEY,
    email VARCHAR NOT NULL,
    campaign_id INT NOT NULL FOREIGN KEY REFERENCES Users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
);

CREATE TABLE IF NOT EXISTS ApiKeys (
    id SERIAL PRIMARY KEY,
    key VARCHAR NOT NULL,
    owner_id INT NOT NULL FOREIGN KEY REFERENCES Users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
)