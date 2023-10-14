CREATE TABLE yield(
    yield_id INTEGER PRIMARY KEY,
    yield_name VARCHAR(200),
    yield_name_f VARCHAR(200)
);

CREATE TABLE refuse(
    refuse_id INTEGER PRIMARY KEY,
    refuse_name VARCHAR(200),
    refuse_name_f VARCHAR(200)
);

CREATE TABLE measure(
    measure_id INTEGER PRIMARY KEY,
    measure_name VARCHAR(200),
    measure_name_f VARCHAR(200)
);

CREATE TABLE nutrient(
    nutrient_id INTEGER PRIMARY KEY,
    nutrient_code INTEGER,
    nutrient_symbol VARCHAR(10),
    unit VARCHAR(8),
    nutrient_name VARCHAR(200),
    nutrient_name_f VARCHAR(200),
    tag_name VARCHAR(20),
    nutrient_decimal INTEGER
);

CREATE TABLE food_group(
    food_group_id INTEGER PRIMARY KEY,
    food_group_code INTEGER,
    food_group_name VARCHAR(200),
    food_group_name_f VARCHAR(200)
);

CREATE TABLE food_source(
    food_source_id INTEGER PRIMARY KEY,
    food_source_code INTEGER,
    food_source_description VARCHAR(200),
    food_source_description_f VARCHAR(200)
);

CREATE TABLE nutrient_source(
    nutrient_source_id INTEGER PRIMARY KEY,
    nutrient_source_code INTEGER,
    nutrient_source_description VARCHAR(200),
    nutrient_source_description_f VARCHAR(200)
);

CREATE TABLE food(
    food_id INTEGER PRIMARY KEY,
    food_code INTEGER,
    food_group_id INTEGER,
    food_source_id INTEGER,
    food_description VARCHAR(255),
    food_description_f VARCHAR(255),
    food_date_of_entry DATE,
    food_date_of_publication DATE,
    country_code INTEGER,
    scientific_name VARCHAR(100),
    FOREIGN KEY (food_group_id) REFERENCES food_group(food_group_id),
    FOREIGN KEY (food_source_id) REFERENCES food_source(food_source_id)
);

CREATE TABLE yield_amount(
    food_id INTEGER,
    yield_id INTEGER,
    yield_amount REAL,
    yield_date_of_entry DATE,
    PRIMARY KEY (food_id, yield_id),
    FOREIGN KEY (food_id) REFERENCES food(food_id),
    FOREIGN KEY (yield_id) REFERENCES yield(yield_id)
);

CREATE TABLE refuse_amount(
    food_id INTEGER,
    refuse_id INTEGER,
    refuse_amount REAL,
    refuse_date_of_entry DATE,
    PRIMARY KEY (food_id, refuse_id),
    FOREIGN KEY (food_id) REFERENCES food(food_id),
    FOREIGN KEY (refuse_id) REFERENCES refuse(refuse_id)
);

CREATE TABLE conversion_factor(
    food_id INTEGER,
    measure_id INTEGER,
    conversion_factor_value REAL,
    conv_factor_date_of_entry DATE,
    PRIMARY KEY (food_id, measure_id),
    FOREIGN KEY (food_id) REFERENCES food(food_id),
    FOREIGN KEY (measure_id) REFERENCES measure(measure_id)
);

CREATE TABLE nutrient_amount(
    food_id INTEGER,
    nutrient_id INTEGER,
    nutrient_value REAL,
    standard_error REAL,
    number_of_observation INTEGER,
    nutrient_source_id INTEGER,
    nutriend_date_of_entry DATE,
    PRIMARY KEY (food_id, nutrient_id, nutrient_source_id),
    FOREIGN KEY (food_id) REFERENCES food(food_id),
    FOREIGN KEY (nutrient_id) REFERENCES nutrient(nutrient_id),
    FOREIGN KEY (nutrient_source_id) REFERENCES nutrient_source(nutrient_source_id)
);

CREATE TABLE user(
    user_id UUID PRIMARY KEY,
    username VARCHAR,
    password_hash VARCHAR
);