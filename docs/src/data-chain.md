# Data Chain
The concept of the data chain pertains to the separate "links" between sections of a fully fledged
querust handler.

The data chain can be visualized as so:  
Values <> Objects <> `Database<T: Codable>` <> `Codec` <> `Connector` <> Data I/O

Our queryable Rust structure data chain starts with the raw values from Rust, before moving onto
a Rust object. These two links are straightforward, as these are composed of values utilized
directly at the application. 

The first real link in the data chain is the `Database` object. The database object uses a generic
type, which must implement the `Codable` trait. The `Codable` trait specifies how the querust object 
will report data to the database object. 