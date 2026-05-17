create database project_db;

\c project_db;

-- enumerador para o status do usuário.
CREATE TYPE USER_STATUS AS ENUM ('available', 'suspended');

-- tabela para os usuários
CREATE TABLE users (
  id             SERIAL      PRIMARY KEY    ,
  user_name      VARCHAR(50) NOT NULL UNIQUE,
  user_mail      VARCHAR(50) NOT NULL UNIQUE,
  latest_change  TIMESTAMPTZ                ,
  current_status USER_STATUS
);

-- tabela para os moderadores
CREATE TABLE moderators (
  id             SERIAL      PRIMARY KEY           ,
  user_id        INT         UNIQUE NOT NULL       ,
  moderator_date TIMESTAMPTZ NOT NULL DEFAULT NOW(),

  CONSTRAINT fk_users_user_id
    FOREIGN KEY (user_id)
    REFERENCES users(id)
    ON DELETE CASCADE
);

-- view para os moderadores
CREATE VIEW moderators_view AS SELECT
  m.id AS "moderator_id",
  u.id AS "user_id",
  u.user_name AS "name",
  u.user_mail AS "mail",
  m.moderator_date AS "since"
FROM moderators m
JOIN users u
ON m.user_id = u.id;
