DELETE FROM chats WHERE deleted = true;
ALTER TABLE chats DROP COLUMN deleted;
