CREATE TABLE IF NOT EXISTS splits(
	ratio         NUMERIC NOT NULL,
	declared_date DATE    NOT NULL,
	ex_date       DATE    NOT NULL,
	ticker        TEXT    NOT NULL,
    from_factor   NUMERIC NOT NULL,
    to_factor     NUMERIC NOT NULL
);

