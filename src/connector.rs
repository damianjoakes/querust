//! Connector module. Contains all traits related to creating `Connector`s, which are workers
//! for `Database` objects.
//!
//! Connectors act as a *transport layer* for `Database` objects. These Connectors' job is to take
//! bytes of information from a struct (which were created from a `Codec`), and send these bytes to
//! some storage means.
//!
//! This storage means can be memory (not recommended for production, only debugging), a file,
//! a `TcpSocket`, and anything in between.

pub mod error;

use std::error::Error;
use crate::connector::error::ConnectorError;

/// Trait for establishing and managing the connection between the persistent data
/// store and the database.
pub trait Connector {
    /// An optional type used for passing options into the `connect` function.
    type Options;

    /// An optional type used for returning information from this connector once the connector has
    /// finished establishing a connection.
    type Connection;

    /// Used to create a connection between this connector and the persistent data
    /// store.
    ///
    /// It is up to the trait implementor whether this function is successful or not, and what this
    /// function does. Some `Connector` implementations will require data back from the connection,
    /// and some will need options. Doing this with trait generics limits flexibility in `Connector`
    /// implementations.
    fn connect(&self, options: Self::Options) -> Result<Self::Connection, ConnectorError>;

    /// Checks whether the current connector is connected or not.
    fn connected(&mut self) -> &mut bool;

    /// Sends data to the underlying storage area. This pulls from an array of
    /// bytes.
    fn push(&self, bytes: &[u8]) -> Result<usize, ConnectorError>;

    /// Requests data from the underlying storage area. This pushes to an array of
    /// bytes.
    fn pull(&self, buffer: &mut [u8]) -> Result<usize, ConnectorError>;

    /// Internally sets a "connected" value, stating whether this current connector was
    /// successfully able to establish a data flow or not.
    fn set_connected(&mut self, connected: bool) {
        let connected_ref = self.connected();
        *connected_ref = connected;
    }
}

/// A deferred connector trait.
///
/// This trait is primarily used when communicating with a data source, where the result of the
/// operation is dependent on the data source itself, rather than within the connector's parent
/// `Database` object.
///
/// This is particularly useful in SQL-like connectors, where the `Database` and `Connector`
/// objects are not responsible for maintaining ACID compliance or validating the success of any
/// input/output requests.
pub trait DeferredConnector: Connector {
    /// Attempts to push some data to the deferred data source.
    fn try_push(&self, bytes: &[u8]) -> Result<(), Box<dyn Error>>;

    /// Attempts to perform a request on the deferred data source.
    fn try_pull(&self, buffer: &mut [u8]) -> Result<(), Box<dyn Error>>;
}
