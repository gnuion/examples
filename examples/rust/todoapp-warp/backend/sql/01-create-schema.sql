-- Todo status enum
CREATE TYPE TODO_STATUS_ENUM AS ENUM (
  'open',
  'close'
);

-- Todo
CREATE TABLE todo (
  id BIGSERIAL,
  cid BIGINT NOT NULL, -- creator user id 
  created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
  mid BIGINT, -- modifer user id
  mtime TIMESTAMP WITH TIME ZONE DEFAULT now(),
  title TEXT NOT NULL,
  "status" TODO_STATUS_ENUM NOT NULL DEFAULT 'open'
);
ALTER SEQUENCE todo_id_seq RESTART WITH 1000; -- start id from 1000
