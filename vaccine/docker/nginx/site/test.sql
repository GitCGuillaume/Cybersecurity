PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE list (
  id integer,
  vals varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`)
);
INSERT INTO list VALUES(1,'abc');
COMMIT;
