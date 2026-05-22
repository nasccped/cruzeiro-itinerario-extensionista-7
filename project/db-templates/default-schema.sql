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

CREATE TYPE MODERATOR_INSERTION_VARIANT AS ENUM (
  'done',
  'issuspended',
  'notfound',
  'alreadymoderator'
);

-- tipo para quando 'INSERT_INTO_MODERATORS' é chamado.
CREATE TYPE MODERATOR_INSERTION_RESULT AS (
  result  MODERATOR_INSERTION_VARIANT,
  user_id INT
);

-- função que realiza checagens para manter integridade antes de adicionar à tabela.
CREATE FUNCTION insert_into_moderators(IN p_id INTEGER) RETURNS MODERATOR_INSERTION_RESULT
LANGUAGE plpgsql
AS $$
DECLARE
  v_exists BOOLEAN;
  v_is_suspended BOOLEAN;
  v_is_moderator BOOLEAN;
  v_result MODERATOR_INSERTION_RESULT;
BEGIN
  -- testar se existe.
  SELECT EXISTS (
    SELECT 1 FROM users u WHERE u.id = p_id
  ) INTO v_exists;
  IF NOT v_exists THEN
    v_result.result := 'notfound';
    v_result.user_id := p_id;
    RETURN v_result;
  END IF;
  -- testar se está suspenso.
  SELECT (
    u.current_status = 'suspended'
  ) INTO v_is_suspended
  FROM users u WHERE u.id = p_id;
  IF v_is_suspended THEN
    v_result.result := 'issuspended';
    v_result.user_id := p_id;
    RETURN v_result;
  END IF;
  -- testar se já é moderador.
  SELECT EXISTS (
    SELECT 1
    FROM moderators m
    WHERE m.user_id = p_id
  ) INTO v_is_moderator;
  IF v_is_moderator THEN
    v_result.result := 'alreadymoderator';
    v_result.user_id := p_id;
    RETURN v_result;
  END IF;
  -- por fim, inserir.
  INSERT INTO moderators (user_id)
  VALUES (p_id);
  v_result.result := 'done';
  v_result.user_id := p_id;
  RETURN v_result;
END;
$$;

CREATE TYPE MODERATOR_DELETION_VARIANT AS ENUM (
  'done',
  'notfound',
  'notamoderator'
);

-- tipo para quando 'INSERT_INTO_MODERATORS' é chamado.
CREATE TYPE MODERATOR_DELETION_RESULT AS (
  result  MODERATOR_DELETION_VARIANT,
  user_id INT
);

-- função que realiza checagens para manter integridade antes de remover da tabela.
CREATE FUNCTION delete_from_moderators(IN p_id INTEGER) RETURNS MODERATOR_DELETION_RESULT
LANGUAGE plpgsql
AS $$
DECLARE
  v_result MODERATOR_DELETION_RESULT;
BEGIN
  v_result.user_id := p_id;
  IF NOT EXISTS (SELECT 1 FROM users u WHERE u.id = p_id) THEN
    v_result.result := 'notfound';
    RETURN v_result;
  END IF;
  IF NOT EXISTS (SELECT 1 FROM moderators m WHERE m.user_id = p_id) THEN
    v_result.result := 'notamoderator';
    RETURN v_result;
  END IF;
  DELETE FROM moderators m WHERE m.user_id = p_id;
  v_result.result := 'done';
  RETURN v_result;
END;
$$;

-- tipo para as regiões do Brasil
CREATE TYPE REGION AS ENUM ('north', 'northeast', 'midwest', 'southeast', 'south');

-- tabela para os estados do Brasil
CREATE TABLE states (
  fu     char(2)     PRIMARY KEY NOT NULL UNIQUE,
  name   VARCHAR(30) NOT NULL UNIQUE            ,
  region REGION      NOT NULL
);

-- tabela para as cidades do Brasil
CREATE TABLE cities (
  id    SERIAL      PRIMARY KEY,
  name  VARCHAR(50) NOT NULL   ,
  state char(2)     NOT NULL   ,

  CONSTRAINT fk_states_fu
  FOREIGN KEY (state)
  REFERENCES states(fu)
  ON DELETE CASCADE
);

CREATE TABLE neighborhoods (
  id   SERIAL      PRIMARY KEY,
  name VARCHAR(50) NOT NULL   ,
  city INT         NOT NULL   ,

  CONSTRAINT fk_city_id
  FOREIGN KEY (city)
  REFERENCES cities(id)
  ON DELETE CASCADE
);

CREATE TABLE locales (
  id           SERIAL      PRIMARY KEY    ,
  pac          CHAR(9)     UNIQUE NOT NULL,
  name         VARCHAR(50) NOT NULL       ,
  neighborhood INT         NOT NULL       ,

CONSTRAINT fk_neighborhood_id
  FOREIGN KEY (neighborhood)
  REFERENCES neighborhoods(id)
  ON DELETE CASCADE
);

CREATE TYPE RECORD_STATUS AS ENUM ('open', 'suspended', 'canceled');

CREATE TABLE records (
  id            SERIAL        PRIMARY KEY            ,
  open_at       TIMESTAMPTZ   NOT NULL DEFAULT NOW() ,
  record_status RECORD_STATUS NOT NULL DEFAULT 'open',
  locale        INT           NOT NULL               ,

  CONSTRAINT fk_locales_id
    FOREIGN KEY (locale)
    REFERENCES locales(id)
    ON DELETE CASCADE
);

-- tipos para os status de report
CREATE TYPE REPORT_STATUS AS ENUM ('open', 'suspended', 'canceled');

-- tabela de reports
CREATE TABLE reports (
  id            SERIAL        PRIMARY KEY            ,
  user_owner    INT           NOT NULL               ,
  open_at       TIMESTAMPTZ   NOT NULL DEFAULT NOW() ,
  report_status REPORT_STATUS NOT NULL DEFAULT 'open',
  record        INT           NOT NULL               ,

  CONSTRAINT fk_users_user_id
  FOREIGN KEY (user_owner)
  REFERENCES users(id)
  ON DELETE CASCADE,

  CONSTRAINT fk_records_record_id
  FOREIGN KEY (record)
  REFERENCES records(id)
  ON DELETE CASCADE
);
