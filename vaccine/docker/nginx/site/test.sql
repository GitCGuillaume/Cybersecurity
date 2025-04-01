PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS list (
  id integer,
  vals varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`)
);
INSERT OR IGNORE INTO list VALUES(1,'abc');
CREATE TABLE IF NOT EXISTS users (id int NOT NULL, name varchar(255) NOT NULL, pswd varchar(255) NOT NULL, PRIMARY KEY(id));
INSERT OR IGNORE INTO users VALUES(1,'usr1','test1');
INSERT OR IGNORE INTO users VALUES(2,'usr2','test2');
INSERT OR IGNORE INTO users VALUES(3,'usr3','test3');
INSERT OR IGNORE INTO users VALUES(4,'usr4','test4');
INSERT OR IGNORE INTO users VALUES(5,'usr5','test5');
COMMIT;
