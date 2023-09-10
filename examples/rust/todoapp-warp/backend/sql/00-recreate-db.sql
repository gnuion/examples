--- DEV ONLY -- Comment out for keeping the database between restarts
DROP DATABASE IF EXISTS app_db;
DROP USER IF EXISTS app_user;

--- DEV ONLY -- for quick iterations
CREATE USER app_user PASSWORD 'secret_app_pwd_to_change';
CREATE DATABASE app_db OWNER app_user ENCODING = 'UTF-8';
