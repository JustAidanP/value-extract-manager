use crate::Resource;
use chrono::{DateTime, Utc};

pub struct AgentScrapeRequest {
    identifier: uuid::Uuid,
    /// The resource to scrape
    resource: Resource,
    /// The time at which the scrape was requested by the manager
    time_requested: DateTime<Utc>,
    /// Should the scrape be screenshot
    screenshot: bool,
}

#[derive(Copy, Clone)]
pub enum ScrapeResult {
    PageNotFound,
    CouldNotLoadResource,
    UnrecognisedFormat,
    Success {
        rrp: Option<i64>,
        price: Option<i64>,
    },
}

/// An agent registered to the manager; this agent has specific credentials. Any agent that
/// attempts to connect must match with a registered agent.
///
/// When an agent attempts connection;
///  1. The list of registered agents is retrieved;
///  2. The attempt is validated against the registered;
///     2.1. If the attempt is valid, then the connection is finalised; and
///  3. Finally, the retrieved list of registered agents is disposed of
///
/// The lifetime of this struct is short, it is only used for validation of a connection,
/// as well as registering a new agent.
///
/// Connection Attempt() => Validation Using RegisteredAgents () => (AgentConnection, AgentMetadata)
pub struct RegisteredAgent {
    /// The unique identifier of the agent
    identifier: String,
}

/// The metadata about a connected agent
pub struct AgentMetadata<'a> {
    /// The unique identifier of the agent
    identifier: &'a str, // Lifetime is tied to registered agent
    /// The list of sites that this agent is banned from
    ban_list: Vec<String>,
    /// The set of sites that this agent knows how to scrape
    known_sites: Vec<String>,
    /// The maximum number of scrape requests that can be handled **per minute**
    rate_limit: u32,
}

/// A valid connection with an agent
pub struct AgentConnection {}

impl AgentConnection {
    pub fn scrape_resource<F: FnMut(ScrapeResult)>(request: &AgentScrapeRequest, on_scrape: F) {}

    /// Accepts a new agent connection
    pub fn accept() -> AgentConnection {
        AgentConnection {}
    }
}
