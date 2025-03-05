use std::env;
use chrono::{DateTime, Local};
use rusqlite::{Connection, OpenFlags};
use rust_decimal::Decimal;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args.clone());

    let path = &args[1];

    let conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY);

    let conn =  match conn {
        Ok(conn) => conn,
        Err(error) => panic!("Problem opening MMEX database: {error:?}"),
    };

    let mut statement = match conn.prepare("SELECT STOCKID, STOCKNAME, SYMBOL FROM STOCK_V1") {
        Ok(res) => res,
        Err(error) => panic!("Problem preparing stock list SQL statement: {error:?}"),
    };
    let stock_iter = match statement.query_map([], |row| {
        Ok(Stock {
            id: row.get(0)?,
            name: row.get(1)?,
            symbol: row.get(2)?,
        })
    }) {
        Ok(iter) => iter,
        Err(error) => panic!("Problem handling results of stock list SQL statement: {error:?}"),
    };

    for stock in stock_iter {
        println!("Found stock {:?}", stock.unwrap());
    }
}


// Holds all of the required information for a single share transaction
struct Transaction {
    date: DateTime<Local>,
    number: Decimal,
    price: Decimal,
    commission: Decimal,
    amount: Decimal,
    commission_accounted_for: bool,
}

// SQL to grab all stock names, save to variable

// SQL to grab all transactions for a particular stock

// Need to group transactions by stock name
    // Need a struct that holds all transactions and info for a particular stock
        // Need a vector of structs that each hold a single transaction
        // Need a function to order transactions by date
        // Need a function to progressively figure out which commissions to offset for this FY
            // Running count of purchased stocks
            // When a sale comes up, attribute the sold stocks to specific
                // Sale of 200 stocks may be made up of separate purchases of 20, 100, 95 for example.
                // The sale commission and each purchase commission need to be summed and removed from
                // the gains or contribute to the loss. The transactions involved need to be flagged as
                // the commission can only be accounted for a single time. A sale of stocks of quantity
                // less than the total of a single purchase will account for the purchase commission and
                // flag it (if not already flagged)
            // Only sales between the time range will trigger capital gains tax
        // Need a variable to hold accumulative capital gains/loss

struct StockRecords {
    stock: Stock,
    transactions: Vec<Transaction>,
    share_count: i32,
    capital_gain: Decimal,
}

#[derive(Debug)]
struct Stock {
    id: u32,
    name: String,
    symbol: String,
}


// Need to account for 50% capital gains savings

// Produce an excel document containing a summary pages of capital gains and individual stock pages