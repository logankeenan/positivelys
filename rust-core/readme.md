`cargo watch -s "cargo build"`

## Developing w/ Diesel

For local development you need a database so Diesel and generate the schema.

Create the local database

```bash
 diesel setup
```

Run migrations

```bash
diesel migration run
```

**WARNING** You'll need to update the created_at and updated_at columns to `Timestamp` in the schema.rs file. Otherwise,
it will not compile.

Create migration files

```bash
diesel migration generate create_table_name
```

