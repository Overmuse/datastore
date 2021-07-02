CREATE TABLE IF NOT EXISTS aggregates(
    id             UUID      PRIMARY KEY,
	open           NUMERIC   NOT NULL,
	high           NUMERIC   NOT NULL,
	low            NUMERIC   NOT NULL,
	close          NUMERIC   NOT NULL,
	volume         INTEGER   NOT NULL,
	start_datetime TIMESTAMP NOT NULL,
	end_datetime   TIMESTAMP NOT NULL,
	ticker         TEXT      NOT NULL
);

