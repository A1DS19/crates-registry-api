-- Your SQL goes here
CREATE TABLE user_role (
  id SERIAL PRIMARY KEY,
  user_id integer NOT NULL REFERENCES users(id),
  role_id integer NOT NULL REFERENCES role(id)
)
