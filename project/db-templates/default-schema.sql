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
  current_status USER_STATUS DEFAULT 'available'
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

-- tipo para quando 'INSERT_INTO_MODERATORS' é chamado.
CREATE TYPE MODERATOR_INSERTION_RESULT AS ENUM (
  'done',
  'issuspended',
  'notfound',
  'alreadymoderator'
);

-- função que realiza checagens para manter integridade antes de adicionar à tabela.
CREATE FUNCTION insert_into_moderators(IN p_id INTEGER) RETURNS moderator_insertion_result
LANGUAGE plpgsql
AS $$
DECLARE
  exists BOOLEAN;
  is_suspended BOOLEAN;
  is_moderator BOOLEAN;
BEGIN
  SELECT EXISTS (
    SELECT 1 FROM users AS u WHERE u.id = p_id
  ) INTO exists;
  IF NOT exists THEN
    RETURN 'notfound';
  END IF;
  SELECT (current_status = 'suspended') INTO is_suspended
  FROM users AS u WHERE p_id = u.id;
  IF is_suspended THEN
    RETURN 'issuspended';
  END IF;
  SELECT EXISTS (
    SELECT 1 FROM moderators m WHERE m.user_id = p_id
  ) INTO is_moderator;
  IF is_moderator THEN
    RETURN 'alreadymoderator';
  END IF;
  INSERT INTO moderators (user_id)
  VALUES (p_id);
  RETURN 'done';
END;
$$;
