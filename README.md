# CSV Query
This is based on the idea of [rows](https://github.com/turicas/rows) but I want to limit it only to a fast CSV parser with the embeded sqlite query.

**This is still is not fully done**

## Installing
Assuming you already have cargo just run this:
```
 cargo install --git https://github.com/jaysonsantos/csv-query-rs
```
When the project is mature enough it will be sent to crates.io.

## Usage
Right know CSV files have to come from the stdin, this will change in the future.
```bash
$ echo -e '"name";"value"\n"User 1";30\n"User 1";40\n"User 2";51\n"User 3";10\n"User 3";15' | \
    csv_query 'select name, avg(value) from table1 group by name having avg(value) > 30'
[String("User 1"), Float(35.0)]
[String("User 2"), Float(51.0)]
```