mod agent;
mod resource;
mod tenant;
/// TODO 1. Allow agents to connect via a tls grpc connection
/// TODO 2. Maintain a bi-directional connection with each agent
/// TODO    2(a). Allows the manager to send scrape requests
/// TODO    2(b). Provides heartbeats to establish the connection of an agent
/// TODO    2(c). Allows an agent to respond to scrape requests
/// TODO 3. Start the scraper service, which distributes scrape requests to agents
/// TODO 4. Process agent responses to requests (validate that the request came from the correct agent?)
/// TODO 5. Pipeline scrape responses for each tenant
mod timings;

use crate::agent::ScrapeResult;
use agent::{AgentConnection, AgentMetadata, AgentScrapeRequest};
use chrono::{DateTime, Duration, Utc};
use resource::Resource;
use std::collections::{HashMap, HashSet};
use tenant::{
    OnChangeChoices, ScreenshotChoices, Tenant, TenantScrapeInterval, TenantScrapeRegistration,
};
use timings::{Block, OClock};
use uuid::Uuid;

trait ScrapeRegistryStore {
    /// Registers a new scrape request in the store
    fn register_new_scrape(&mut self, registration: TenantScrapeRegistration);

    /// Assigns a result to a specific scrape request
    fn assign_result(&self, interval: TenantScrapeInterval, result: ScrapeResult);

    /// Gets the set of intervals for the resource where the intervals:
    ///     (a) Haven't already got a result assigned, i.e. haven't already been scraped, and
    ///     (b) Overlap with the current operational block
    /// The returned list is ordered based on the end of each interval, with earliest at the front
    /// TODO It also pre-fetches the associated tenant scrape registrations for each interval?
    fn get_intervals_for_resource(
        &self,
        block: Block,
        resource: Resource,
    ) -> Vec<TenantScrapeInterval>;
    /// Gets all resources with the associated intervals for the block, where there is at least
    /// one interval for the resource in the current block.
    ///
    /// Essentially, for every resource registered, r, `self.get_intervals_for_resource(block, r)`
    /// gets called, and if there is at least one interval in the result, the resource is
    /// returned with the result [Note that self.get_intervals_for_resource doesn't have to be used]
    ///
    /// The returned list is ordered based on the earliest finishing interval of each resource
    fn get_all_intervals_for_block(
        &self,
        block: Block,
    ) -> Vec<(Resource, Vec<TenantScrapeInterval>)>;
}

struct DevScrapeRegistryStore {}

impl ScrapeRegistryStore for DevScrapeRegistryStore {
    fn register_new_scrape(&mut self, registration: TenantScrapeRegistration) {
        todo!()
    }

    fn assign_result(&self, interval: TenantScrapeInterval, result: ScrapeResult) {
        todo!()
    }

    fn get_intervals_for_resource(
        &self,
        block: Block,
        resource: Resource,
    ) -> Vec<TenantScrapeInterval> {
        todo!()
    }

    fn get_all_intervals_for_block(
        &self,
        block: Block,
    ) -> Vec<(Resource, Vec<TenantScrapeInterval>)> {
        todo!()
    }
}

/// * `U` - The scrape request store
struct ValueExtractManager<'a, U> {
    scrape_registry_store: U,
    // The current active agents is a mapping between their identifier and the agent definition
    active_agents: HashMap<&'a str, (AgentMetadata<'a>, AgentConnection)>,
}

impl<'a, U> ValueExtractManager<'a, U>
where
    U: ScrapeRegistryStore,
{
    pub fn new(scrape_registry_store: U) -> ValueExtractManager<'a, U> {
        ValueExtractManager {
            scrape_registry_store,
            active_agents: Default::default(),
        }
    }

    fn establish_agent_server(&self) {}

    fn start_scrape_block(&self, block: Block) {
        // Gets the set of resources that need to be scraped, with their associated intervals
        let ordered_resources = self
            .scrape_registry_store
            .get_all_intervals_for_block(block);
        // Splits the resources list into groups, where group `i` has all resources that have the
        // earliest finishing time of (i + 1) O'Clock
        let resource_groups: [HashSet<&(Resource, Vec<TenantScrapeInterval>)>; 24] =
            Default::default();
        // TODO Assign resources to the groups
        // TODO Starts the scheduling algorithm
        // Loops through each block, scraping all resources that need to be scraped
        for i in block.starts_at()..24 {
            // Loops through the resources in the block,
            // [The group exists as 0 <= i < 24, and resource_groups has exactly i elements]
            for (resource, intervals) in &resource_groups[i as usize] {
                /// TODO Add delay to average scrapes throughout the block
                /// TODO Select an agent to execute this scrape
                /// TODO Pipeline result for each interval
            }
        }
    }

    /// Starts the service; every hour it will find every scrape request and run them
    fn start(&mut self) {
        self.start_scrape_block(Block::new(0));
    }
}

fn main() {
    let mut manager = ValueExtractManager::new(DevScrapeRegistryStore {});
    manager.start();
}
