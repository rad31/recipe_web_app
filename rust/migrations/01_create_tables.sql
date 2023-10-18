CREATE TABLE yield(
    id INTEGER PRIMARY KEY,
    name VARCHAR(200),
    name_f VARCHAR(200)
);

CREATE TABLE refuse(
    id INTEGER PRIMARY KEY,
    name VARCHAR(200),
    name_f VARCHAR(200)
);

CREATE TABLE measure(
    id INTEGER PRIMARY KEY,
    name VARCHAR(200),
    name_f VARCHAR(200)
);

CREATE TABLE nutrient(
    id INTEGER PRIMARY KEY,
    code INTEGER,
    symbol VARCHAR(10),
    unit VARCHAR(8),
    name VARCHAR(200),
    name_f VARCHAR(200),
    tag_name VARCHAR(20),
    nutrient_decimal INTEGER
);

CREATE TABLE food_group(
    id INTEGER PRIMARY KEY,
    code INTEGER,
    name VARCHAR(200),
    name_f VARCHAR(200)
);

CREATE TABLE food_source(
    id INTEGER PRIMARY KEY,
    code INTEGER,
    description VARCHAR(200),
    description_f VARCHAR(200)
);

CREATE TABLE nutrient_source(
    id INTEGER PRIMARY KEY,
    code INTEGER,
    description VARCHAR(200),
    description_f VARCHAR(200)
);

CREATE TABLE food(
    id INTEGER PRIMARY KEY,
    code INTEGER,
    food_group_id INTEGER,
    food_source_id INTEGER,
    description VARCHAR(255),
    description_f VARCHAR(255),
    date_of_entry DATE,
    date_of_publication DATE,
    country_code INTEGER,
    scientific_name VARCHAR(100),
    FOREIGN KEY (food_group_id) REFERENCES food_group(id),
    FOREIGN KEY (food_source_id) REFERENCES food_source(id)
);

CREATE TABLE yield_amount(
    food_id INTEGER,
    yield_id INTEGER,
    amount REAL,
    date_of_entry DATE,
    PRIMARY KEY (food_id, yield_id),
    FOREIGN KEY (food_id) REFERENCES food(id),
    FOREIGN KEY (yield_id) REFERENCES yield(id)
);

CREATE TABLE refuse_amount(
    food_id INTEGER,
    refuse_id INTEGER,
    amount REAL,
    date_of_entry DATE,
    PRIMARY KEY (food_id, refuse_id),
    FOREIGN KEY (food_id) REFERENCES food(id),
    FOREIGN KEY (refuse_id) REFERENCES refuse(id)
);

CREATE TABLE conversion_factor(
    food_id INTEGER,
    measure_id INTEGER,
    factor_value REAL,
    date_of_entry DATE,
    PRIMARY KEY (food_id, measure_id),
    FOREIGN KEY (food_id) REFERENCES food(id),
    FOREIGN KEY (measure_id) REFERENCES measure(id)
);

CREATE TABLE nutrient_amount(
    food_id INTEGER,
    nutrient_id INTEGER,
    nutrient_value REAL,
    standard_error REAL,
    number_of_observation INTEGER,
    nutrient_source_id INTEGER,
    date_of_entry DATE,
    PRIMARY KEY (food_id, nutrient_id, nutrient_source_id),
    FOREIGN KEY (food_id) REFERENCES food(id),
    FOREIGN KEY (nutrient_id) REFERENCES nutrient(id),
    FOREIGN KEY (nutrient_source_id) REFERENCES nutrient_source(id)
);

CREATE TABLE users(
    id UUID PRIMARY KEY,
    username VARCHAR UNIQUE,
    password_hash VARCHAR
);
