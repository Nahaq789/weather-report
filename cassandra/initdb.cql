DROP KEYSPACE IF EXISTS sensor_key_space;

CREATE KEYSPACE sensor_key_space WITH REPLICATION {
  'class' : 'SimpleStrategy',
  'replication_factor' : 1
};

CREATE TABLE sensor (
  id text PRIMARY KEY,
  data text
) WITH default_time_to_live = 31536000;

INSERT INTO sensor_key_space.sensor (id, data)
  VALUES('test_id', 'text');
