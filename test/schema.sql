CREATE SCHEMA pidi;

CREATE TABLE pidi.user (
  id UUID PRIMARY KEY,
  username VARCHAR(64) NOT NULL
);

CREATE TABLE pidi.product (
  id UUID PRIMARY KEY
);

CREATE TABLE pidi.order (
  id UUID PRIMARY KEY,
  user_id UUID REFERENCES pidi.user(id) NOT NULL
);

CREATE TABLE pidi.order_item (
  id UUID PRIMARY KEY,
  order_id UUID REFERENCES pidi.order(id) NOT NULL,
  product_id UUID REFERENCES pidi.product(id) NOT NULL,
  quantity INTEGER NOT NULL
);

CREATE VIEW pidi.order_order_item_aggregation AS (
  SELECT
    oi.order_id AS id, 
    count(*) AS order_item_count
  FROM pidi.order_item oi
  GROUP BY oi.order_id
);

CREATE TABLE public.blog_post (
  id UUID PRIMARY KEY,
  title TEXT NOT NULL
);

CREATE TABLE pidi.interval_test_table(
	interval INTERVAL,
	interval_year INTERVAL YEAR,
	interval_month INTERVAL MONTH,
	interval_day INTERVAL DAY,
	interval_hour INTERVAL HOUR,
	interval_minute INTERVAL MINUTE,
	interval_second INTERVAL SECOND,
	interval_year_to_month INTERVAL YEAR TO MONTH,
	interval_day_to_hour INTERVAL DAY TO HOUR,
	interval_day_to_minute INTERVAL DAY TO MINUTE,
	interval_day_to_second INTERVAL DAY TO SECOND,
	interval_hour_to_minute INTERVAL HOUR TO MINUTE,
	interval_hour_to_second INTERVAL HOUR TO SECOND,
	interval_minute_to_second INTERVAL MINUTE TO SECOND
);

CREATE TABLE public.serial_test_table (
  id SERIAL PRIMARY KEY
);
