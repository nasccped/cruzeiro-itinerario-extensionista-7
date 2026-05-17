CREATE DATABASE project_db;

\c project_db;

-- tabela para status de usuário.
CREATE TABLE user_statuses (
  id          INT         PRIMARY KEY,
  status_name VARCHAR(20) NOT NULL UNIQUE
);

INSERT INTO user_statuses (id, status_name)
VALUES 
  (1, 'Disponível'),
  (2, 'Suspenso'  );

-- tabela para os usuários
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

-- tabela para os moderadores
CREATE TABLE moderators (
  id             SERIAL PRIMARY KEY                ,
  user_id        INT UNIQUE NOT NULL               ,
  moderator_date TIMESTAMPTZ NOT NULL DEFAULT NOW(),

  CONSTRAINT fk_users_user_id
    FOREIGN KEY (user_id)
    REFERENCES users(id)
    ON DELETE CASCADE
);

-- view para os moderadores
CREATE VIEW moderators_view AS SELECT
  m.id             as "moderator_id",
  u.id             as "user_id"     ,
  u.user_name      as "name"        ,
  u.user_mail      as "mail"        ,
  m.moderator_date as "since"
FROM moderators m
JOIN users u
ON m.user_id = u.id;
