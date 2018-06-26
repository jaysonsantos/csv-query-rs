extern crate csv_query;

use std::io::{BufReader, Cursor, Write};

use csv_query::Executor;

const USER_CSV: &'static str = "\"user\";\"age\"\n\
                                \"user1\";\"10\"\n\
                                \"user2\";\"4\"\n";
const ORDER_CSV: &'static str = "user;price\n\
                                 user1;10\n\
                                 user2;30\n\
                                 user1;50\n";

#[test]
fn test_nothing() {
    let input = vec![BufReader::new(Cursor::new(&USER_CSV))];
    let output = vec![];
    let mut output_buffer = Cursor::new(output);
    {
        let buf = output_buffer.by_ref();
        let mut executor = Executor::new(input, buf, b';').unwrap();
        executor
            .print_results("select user, age from table1")
            .unwrap();
    }
    let output = output_buffer.into_inner();
    assert_eq!(String::from_utf8(output).unwrap(), USER_CSV);
}

#[test]
fn test_join() {
    let input = vec![
        BufReader::new(Cursor::new(&USER_CSV)),
        BufReader::new(Cursor::new(&ORDER_CSV)),
    ];
    let output = vec![];
    let mut output_buffer = Cursor::new(output);
    {
        let buf = output_buffer.by_ref();
        let mut executor = Executor::new(input, buf, b';').unwrap();
        executor
            .print_results(
                "select u.user, sum(price)
                    from table1 u
                    join table2 o
                        on u.user = o.user
                    group by u.user
                    having sum(price) > 50",
            )
            .unwrap();
    }
    let output = output_buffer.into_inner();
    assert_eq!(
        String::from_utf8(output).unwrap(),
        "\"user\";\"sum(price)\"\n\
         \"user1\";\"60\"\n"
    );
}
