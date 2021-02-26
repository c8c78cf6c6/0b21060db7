<h1 align="center">simledger</h1>

<img src="https://i.imgur.com/MxshZ1b.jpg" width=100%/>

<h5 align="center">sims all the ledgers</h5>

<br />

---

<br/>

### Getting started

1. `cargo run -- data/example.csv`
2. Test data generation: `cd scripts/txgen ; make`
   1. generates 'big_test.csv' and 'small_text.csv', 1m lines and 10k lines, respectively.
   2. May take some time as the test data generator is trying to actually produce sensible data.

### What's where

1. cli/ -- code of the CLI (Rust),
2. lib/ -- code of the simledger library (Rust),
3. scripts/txgen/ -- code of the test file generator (TypeScript, requires Node).

### Architecture

1. Asynchronous CSV parser implemented as independent CLI program referencing simledger-lib library (located in /cli),
   1. opens file identified by passed argument string for reading,
   2. streams csv line by line (external crate is using git as - at the time of writing - Tokio 1.0 support was not available in latest crates.io published version),
   3. parses each line into intermediate struct via serde,
   4. transforms intermediate structure into simledger compatible transaction (via TryInto impl),
   5. passes each transaction to ledger instance for execution,
   6. after all lines have been consumed, the program will iterate over all accounts in the ledger and asynchronously write the account summary to the output file one by one.


2. "Simulation ledger" implemented as portable library (located in /lib),
   1. transactions are stored in each individual account, preventing accidential access to unrelated transactions but also improving lookup performance when linear scans would be required (they're not, but it's cleaner this way, too),
   2. each account maintains three books in the form of BTreeMaps:
      1. book, for all balance-flow related transactions (deposit, withdrawal),
      2. book-disputed, for all disputed transactions,
      3. book-chargeback, for all back charged transactions (will only ever be a single item as the account is immediately locked),
   3. available balance is maintained as discrete value for performance reasons,
   4. withheld balance is calculated by iterating through all transactions in book-disputed,
   5. most relevant groups of methods are implemented via traits,
   6. all methods top-to-bottom return a result of either ExecutionResult or ExecutionError, allowing for simple introspection and testability,

### Tests

1. Most basic debit / credit / dispute-process functionality is somewhat covered by behavior tests in simledger-lib.
2. covers several use-cases:
   1. verifies deposit and withdrawal functionality,
   2. verifies withdrawal validation,
   3. verifies account locking,
   4. verifies dispute-process flow,
   5. attempts basic fuzzing including reuse of transaction ids (i.e. deposit of $50 with id 1, deposit of $1 with id 1, dispute id 1, withdraw $50).
3. CLI untested as I consider the business logic in simledger most relevant for testing.

### Other than that

I am not responsible for any damage that is caused by the use of this software. Use at your own risk. ... well technically I am :-)
