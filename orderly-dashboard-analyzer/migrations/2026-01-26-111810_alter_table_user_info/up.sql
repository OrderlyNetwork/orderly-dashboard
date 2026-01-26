-- Your SQL goes here
ALTER TABLE user_info ADD COLUMN id BIGINT;

CREATE SEQUENCE user_info_id_seq;

UPDATE user_info SET id = nextval('user_info_id_seq') WHERE id IS NULL;

SELECT setval(
    'user_info_id_seq',
    (SELECT MAX(id) FROM user_info)
);

ALTER TABLE user_info ALTER COLUMN id SET DEFAULT nextval('user_info_id_seq');

ALTER TABLE user_info ALTER COLUMN id SET NOT NULL;

CREATE INDEX user_info_id ON user_info(id);