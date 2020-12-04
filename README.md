# Ado-JDBC connection string parsing

An example project to import a Rust crate to WASM and use it from Node.js.

# Using

It's a good idea to read the [wasm-pack
quickstart](https://rustwasm.github.io/wasm-pack/book/quickstart.html).

To try this out, steps:

- [Install stable Rust](https://rustup.rs/)
- [Install wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- Build the nodejs module: `wasm-pack build-t nodejs`

Using from node:

``` javascript
const JdbcString = require('./pkg/ado_jdbc').JdbcString;
let j = new JdbcString("jdbc:sqlserver://localhost:1433;database=master;user=SA;password=<YourStrong@Passw0rd>;trustServerCertificate=true;encrypt=DANGER_PLAINTEXT");
```

API docs what to do with a `JdbcString` object can be read from [the source](src/lib.rs).

Some things to understand:

- If the function returns `Result`, it can throw an error.
- If the return value is an `Option`, the value can be null.

Have fun!
