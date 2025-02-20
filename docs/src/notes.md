# Data Chain

1. `Connector` - Low-level data IO. Connectors are intended to handle data throughput between `appono` and 
                 an external data source. Connectors perform IO on files, connect to TCP sockets, store
                 data directly in memory, etc. Connectors are essentially definitions for how `appono` databases
                 will persistently store data.
                 The `Connector` is important as it must be able to ensure ACID-compliance with the underlying 
                 data layer that it's communicating with. For solutions such as file storage and in-memory storage
                 this becomes trivial, but for solutions such as communicating with an external databases,
                 ACID-compliance becomes reliant on the external database.

2. `Database` - The next step in the data chain. Databases are definitions for objects that implement `Codec`.
                The `Database`'s job is to manage tables and act as an 
                _"encoder"_ for `Codec`s (by encoding them into their bytes, and decoding them from their bytes),
                etc.

3. `Codec` - interface for encoding/decoding records, which writes to/reads from a `Database`. Creates
             definitions for the bytes being written to disk. Becomes an interface for encoding/decoding records,
             which contains definitions for writing the bytes to disk. Technically defines a "table".  

4. Objects - Implement the `Codec` trait. Implemented objects act as tables.   

5. Object properties/values - Object properties act as the column of a table, property values act as the rows.  

Each operation can be tracked through the data chain as so:

Object values <-> New object instance <-> `Codec` <-> `Database` <-> `Connector`

The `Codec` and `Connector` traits are more for library authors. People who want to create new implementations
for `appono` to talk to other data storage mediums will primarily use these two traits. People who wish to use
`appono` strictly for its database/binary serialization functions will use `Database` and their own objects
they wish to perform these operations on.

Consider the following code:

```rust
use std::error::Error;

#[derive(Codec)]
struct Person {
    id: usize,
    name: String,
    age: usize,
}

impl Person {
    pub fn new<T: ToString>(id: usize, name: T, age: usize) -> Self {
        Self {
            id,
            name: name.to_string(),
            age,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let database = Database::new(DatabaseType::Memory);
    let people = database.connect_table::<Person>("people");
    
    let john_smith = Person::new(1, "John Smith", 44);
    let mary_shell = Person::new(2, "Mary Shell", 36);

    let transaction = people.transaction();
    transaction.insert(john_smith);
    transaction.insert(mary_shell);

    transaction.commit()?;
    
    Ok(())
}
```

We now have a `people` table with the following details:

| id: _usize_ | name: _String_ | age: _usize_ |
|-------------|----------------|--------------|
| 1           | John Smith     | 44           |
| 2           | Mary Shell     | 36           |

