DROP KEYSPACE IF EXISTS sensor_key_space;

CREATE KEYSPACE sensor_key_space WITH REPLICATION = {
  'class' : 'SimpleStrategy',
  'replication_factor' : 3
};

USE sensor_key_space;

CREATE TABLE sensor_key_space.sensor (
    sensor_id text,
    time_stamp timestamp,
    area text,
    latitude double,
    longitude double,
    temperature double,
    humidity double,
    status text,
    PRIMARY KEY (sensor_id, time_stamp)
) WITH CLUSTERING ORDER BY (time_stamp DESC);
