# SQL Formatter & Beautifier

A pipe-friendly command-line SQL beautifier for PostgreSQL, MySQL, SQLite, and T-SQL.

---

### ⚙️ How It Works

- **Zero-config formatting**: Runs your SQL through `sqlformat` to produce clean, consistently indented output.
- **Configurable indentation**: `--indent 2`, `--indent 4`, `--indent tab` — pick what your project uses.
- **Keyword case control**: `--case upper` shouts your keywords in caps, `--case lower` keeps them quiet, `--case preserve` leaves them as written.
- **Multi-dialect aware**: The `--dialect` flag documents your target dialect (Postgres, MySQL, SQLite, T-SQL); output is dialect-neutral formatting suitable for all of them.
- **File or stdin**: Give it a file path or pipe SQL in; write the result to stdout or to `--output`.

---

## 📁 Setup

### 1. Requirements

- Rust 1.75 or higher

### 2. Installation

```bash
git clone https://github.com/fantasywastaken/SQL-Formatter-Beautifier.git
cd SQL-Formatter-Beautifier
cargo build --release
```

Binary will be at `target/release/sqlfmt`.

---

### 🚀 Usage

```bash
sqlfmt query.sql
sqlfmt query.sql --indent 4 --case upper
cat query.sql | sqlfmt --indent tab
sqlfmt query.sql --output pretty.sql --dialect postgres
```

Example:

```
$ echo "select id,name from users where id=1 and status='active' order by id;" | sqlfmt --case upper
SELECT
  id,
  name
FROM
  users
WHERE
  id = 1
  AND status = 'active'
ORDER BY
  id;
```

---

### ✨ Features

- ✅ Clean, consistent indentation for messy SQL
- ✅ Keyword case: `upper`, `lower`, or `preserve`
- ✅ Configurable indent width, including tabs
- ✅ Read from file or stdin, write to stdout or file
- ✅ Multi-dialect aware: Postgres, MySQL, SQLite, T-SQL
- ✅ Pipe-friendly, single-binary distribution
