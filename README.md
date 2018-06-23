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
Save these two example files.

user.csv
```csv
user_id;name
user_id;name
1;User 1
2;User 2
3;User 3
```

credits.csv
```csv
user_id;credit
1;5
1;30
2;3
1;4
3;1
```
and you should be able to run this query over it
```bash
$ csv_query -q "select u.name, sum(c.credit) credits, avg(c.credit) avg_credits from table1 u join table2 c on u.user_id = c.user_id group by u.user_id having avg(c.credit) >= 3" -f user.csv -f credits.csv

"name";"credits";"avg_credits"
"User 1";"39";"13"
"User 2";"3";"3"
```
