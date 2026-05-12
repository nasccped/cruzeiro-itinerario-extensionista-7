CREATE TABLE userStatuses (
  id         INT         PRIMARY KEY,
  statusName VARCHAR(20) NOT NULL UNIQUE
);

INSERT INTO userStatuses (id, statusName)
VALUES 
  (1, 'Disponível'),
  (2, 'Suspenso'  );

CREATE TABLE users (
  id            SERIAL      PRIMARY KEY    ,
  userName      VARCHAR(50) NOT NULL UNIQUE,
  userMail      VARCHAR(50) NOT NULL UNIQUE,
  latestChange  TIMESTAMP                  ,
  currentStatus INT                        ,

  CONSTRAINT fk_users_userStatuses
    FOREIGN KEY (currentStatus)
    REFERENCES userStatuses(id)
    ON DELETE SET NULL
);
