PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE if not exists test (id int PRIMARY KEY, test TEXT);
COMMIT;
