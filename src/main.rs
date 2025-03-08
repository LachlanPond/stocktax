use std::{env, str::FromStr};
use chrono::{DateTime, FixedOffset, Local, Months, NaiveDate, NaiveDateTime, Utc};
use rusqlite::{Connection, OpenFlags};
use rust_decimal::{prelude::{FromPrimitive, Zero}, Decimal};
use rust_decimal_macros::dec;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args.clone());

    let path = &args[1];

    let date_from = match NaiveDate::parse_from_str(&args[2], "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => panic!("Invalid from date, format should be: YYYY-MM-DD"),
    };

    let date_to = match NaiveDate::parse_from_str(&args[2], "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => panic!("Invalid to date, format should be: YYYY-MM-DD"),
    };

    let conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY);

    let conn =  match conn {
        Ok(conn) => conn,
        Err(error) => panic!("Problem opening MMEX database: {error:?}"),
    };

    // Get a list of all unique stocks in the MMEX DB
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
    
    let mut stock_data_vec = Vec::new();

    // Pull out all of the transactions for every stock
    for stock in stock_iter {
        let mut stock_data = StockData {
            stock: stock.unwrap(),
            transactions: Vec::new(),
            share_count: 0,
            capital_gain: Decimal::zero(),
        };

        // Collect all of the transactions for a particular stock
        let statement = format!(
            "SELECT 
                sinfo.SHARENUMBER,
                sinfo.SHAREPRICE,
                sinfo.SHARECOMMISSION,
                ca.TRANSAMOUNT,
                ca.TRANSDATE
            FROM SHAREINFO_V1 AS sinfo
            JOIN CHECKINGACCOUNT_V1 AS ca ON sinfo.CHECKINGACCOUNTID = ca.TRANSID
            JOIN TRANSLINK_V1 AS tl ON sinfo.CHECKINGACCOUNTID = tl.CHECKINGACCOUNTID
            JOIN STOCK_V1 AS s ON s.STOCKID = tl.LINKRECORDID
            WHERE s.STOCKID = {}
            ORDER BY ca.TRANSDATE ASC", stock_data.stock.id);

        let mut statement =  match conn.prepare(statement.as_str()) {
            Ok(res) => res,
            Err(error) => panic!("Problem preparing stock data SQL statement: {error:?}"),
        };

        let transaction_iter = match statement.query_map([], |row| {
            let date: String = row.get(4)?;
            let date = NaiveDate::parse_from_str(date.as_str(), "%Y-%m-%dT%H:%M:%S")
                .expect("Could not parse date");

            let number: f64 = row.get(0)?;
            let number = Decimal::from_f64(number)
                .expect("Could not parse share number");

            let price: f64 = row.get(1)?;
            let price = Decimal::from_f64(price)
                .expect("Could not parse price");

            let commission: f64 = row.get(2)?;
            let commission = Decimal::from_f64(commission)
                .expect("Could not parse commission");

            let amount: f64 = row.get(3)?;
            let amount = Decimal::from_f64(amount)
                .expect("Could not parse amount");

            Ok(Transaction {
                date: date,
                number: number,
                price: price,
                commission: commission,
                amount: amount,
                commission_accounted_for: false,
            })
        }) {
            Ok(iter) => iter,
            Err(error) => panic!("Problem handling results of the stock transactions statement: {error:?}"),
        };

        for transaction in transaction_iter {
            stock_data.transactions.push(transaction.unwrap());
        }

        stock_data_vec.push(stock_data);
    }

    for stock in stock_data_vec {
        println!("{:?}", stock);
    }

}


// Holds all of the required information for a single share transaction
#[derive(Debug)]
#[derive(Clone)]
struct Transaction {
    date: NaiveDate,
    number: Decimal,
    price: Decimal,
    commission: Decimal,
    amount: Decimal,
    commission_accounted_for: bool,
}

#[derive(Debug)]
struct StockData {
    stock: Stock,
    transactions: Vec<Transaction>,
    share_count: i32,
    capital_gain: Decimal,
}

#[derive(Debug)]
struct Stock {
    id: u64,
    name: String,
    symbol: String,
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


// Need to account for 50% capital gains savings

// Produce an excel document containing a summary pages of capital gains and individual stock pages