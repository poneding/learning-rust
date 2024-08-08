# Rust React Chat App

# diesel

```bash
diesel migration generate create_users
diesel migration generate create_rooms
diesel migration generate create_conversations

diesel migration generate dummy_data

diesel database setup --database-url=chat.db
diesel migration run --database-url=chat.db
```

![img](screenshot.png)
