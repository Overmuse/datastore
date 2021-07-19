CREATE TABLE IF NOT EXISTS daily_aggregates(
	open     NUMERIC                  NOT NULL,
	high     NUMERIC                  NOT NULL,
	low      NUMERIC                  NOT NULL,
	close    NUMERIC                  NOT NULL,
	volume   NUMERIC                  NOT NULL,
	datetime TIMESTAMP WITH TIME ZONE NOT NULL,
	ticker   TEXT                     NOT NULL
);

CREATE TABLE IF NOT EXISTS minute_aggregates(
	open     NUMERIC                  NOT NULL,
	high     NUMERIC                  NOT NULL,
	low      NUMERIC                  NOT NULL,
	close    NUMERIC                  NOT NULL,
	volume   NUMERIC                  NOT NULL,
	datetime TIMESTAMP WITH TIME ZONE NOT NULL,
	ticker   TEXT                     NOT NULL
);

