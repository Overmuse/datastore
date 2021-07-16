CREATE TABLE IF NOT EXISTS dividends(
	amount        NUMERIC NOT NULL,
	declared_date DATE    NOT NULL,
	ex_date       DATE    NOT NULL,
	record_date   DATE    NOT NULL,
	payment_date  DATE    NOT NULL,
	ticker        TEXT    NOT NULL
);

