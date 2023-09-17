CREATE TABLE yield_name(
    yield_id INTEGER PRIMARY KEY,
    yield_name TEXT,
    yield_name_f TEXT
);

CREATE TABLE refuse_name(
    refuse_id INTEGER PRIMARY KEY,
    refuse_name TEXT,
    refuse_name_f TEXT
);

CREATE TABLE measure_name(
    measure_id INTEGER PRIMARY KEY,
    measure_name TEXT,
    measure_name_f TEXT
);

CREATE TABLE nutrient_name(
    nutrient_name_id INTEGER PRIMARY KEY,
    nutrient_code INTEGER,
    nutrient_symbol TEXT,
    unit TEXT,
    nutrient_name TEXT,
    nutrient_name_f TEXT,
    tag_name TEXT,
    nutrient_decimal INTEGER
);

CREATE TABLE food_group(
    food_group_id INTEGER PRIMARY KEY,
    food_group_code INTEGER,
    food_group_name TEXT,
    food_group_name_f TEXT
);

CREATE TABLE food_source(
    food_source_id INTEGER PRIMARY KEY,
    food_source_code INTEGER,
    food_source_description TEXT,
    food_source_description_f TEXT
);

CREATE TABLE nutrient_source(
    nutrient_source_id INTEGER PRIMARY KEY,
    nutrient_source_code INTEGER,
    nutrient_source_description TEXT,
    nutrient_source_description_f TEXT
);

CREATE TABLE food_name(
    food_id INTEGER PRIMARY KEY,
    food_code INTEGER,
    food_group_id INTEGER,
    food_source_id INTEGER,
    food_description TEXT,
    food_description_f TEXT,
    country_code INTEGER,
    food_date_of_entry TEXT,
    food_date_of_publication TEXT,
    scientific_name TEXT,
    FOREIGN KEY (food_group_id) REFERENCES food_group(food_group_id),
    FOREIGN KEY (food_source_id) REFERENCES food_source(food_source_id)
);

CREATE TABLE yield_amount(
    food_id INTEGER,
    yield_id INTEGER,
    yield_amount REAL,
    yield_date_of_entry TEXT,
    PRIMARY KEY (food_id, yield_id),
    FOREIGN KEY (food_id) REFERENCES food_name(food_id),
    FOREIGN KEY (yield_id) REFERENCES yield_name(yield_id)
);

CREATE TABLE refuse_amount(
    food_id INTEGER,
    refuse_id INTEGER,
    refuse_amount REAL,
    refuse_date_of_entry TEXT,
    PRIMARY KEY (food_id, refuse_id),
    FOREIGN KEY (food_id) REFERENCES food_name(food_id),
    FOREIGN KEY (refuse_id) REFERENCES refuse_name(refuse_id)
);

CREATE TABLE conversion_factor(
    food_id INTEGER,
    measure_id INTEGER,
    conversion_factor_value INTEGER,
    conv_factor_date_of_entry TEXT,
    PRIMARY KEY (food_id, measure_id),
    FOREIGN KEY (food_id) REFERENCES food_name(food_id),
    FOREIGN KEY (measure_id) REFERENCES measure_name(measure_id)
);

CREATE TABLE nutrient_amount(
    food_id INTEGER,
    nutrient_name_id INTEGER,
    nutrient_source_id INTEGER,
    nutrient_value REAL,
    standard_error REAL,
    number_of_observation INTEGER,
    nutriend_date_of_entry TEXT,
    PRIMARY KEY (food_id, nutrient_name_id, nutrient_source_id),
    FOREIGN KEY (food_id) REFERENCES food_name(food_id),
    FOREIGN KEY (nutrient_name_id) REFERENCES nutrient_name(nutrient_name_id),
    FOREIGN KEY (nutrient_source_id) REFERENCES nutrient_source(nutrient_source_id)
);
