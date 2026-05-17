CREATE DATABASE project_db;

\c project_db;

CREATE TABLE user_statuses (
  id          INT         PRIMARY KEY,
  status_name VARCHAR(20) NOT NULL UNIQUE
);

INSERT INTO user_statuses (id, status_name)
VALUES 
  (1, 'Disponível'),
  (2, 'Suspenso'  );

CREATE TABLE users (
  id             SERIAL      PRIMARY KEY    ,
  user_name      VARCHAR(50) NOT NULL UNIQUE,
  user_mail      VARCHAR(50) NOT NULL UNIQUE,
  latest_change  TIMESTAMP                  ,
  current_status INT                        ,

  CONSTRAINT fk_users_user_statuses
    FOREIGN KEY (current_status)
    REFERENCES user_statuses(id)
    ON DELETE SET NULL
);

CREATE TABLE moderators (
  id      SERIAL PRIMARY KEY ,
  user_id INT UNIQUE NOT NULL,

  CONSTRAINT fk_users_user_id
    FOREIGN KEY (user_id)
    REFERENCES users(id)
    ON DELETE CASCADE
);
