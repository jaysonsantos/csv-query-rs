# CSV Query
[![Build Status](https://travis-ci.org/jaysonsantos/csv-query-rs.svg?branch=master)](https://travis-ci.org/jaysonsantos/csv-query-rs)

This is based on the idea of [rows](https://github.com/turicas/rows) but I want to limit it only to a fast CSV parser with the embeded sqlite query.

**This is still is not fully done**

## Installing
### Homebrew
```
brew install jaysonsantos/tools/csv-query
```
### From source
Assuming you already have cargo just run this:
```
 cargo install csv-query-cli
```

## Usage
Save these two example files.

user.csv
```csv
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
$ csv-query -q "
select
    u.name, sum(c.credit) credits, avg(c.credit) avg_credits
from table1 u
join table2 c
    on u.user_id = c.user_id
group by u.user_id
having avg(c.credit) >= 3" \
-f user.csv -f credits.csv

"name";"credits";"avg_credits"
"User 1";"39";"13"
"User 2";"3";"3"
```

### Or you can use the interactive mode
[![asciicast](https://asciinema.org/a/199320.png)](https://asciinema.org/a/199320)

## Disclaimer
If you don't need the flexibility of SQLite or want to do things real fast and use a more mature project you should use [xsv](https://github.com/BurntSushi/xsv/).
