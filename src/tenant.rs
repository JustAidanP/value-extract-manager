use crate::Resource;
use std::collections::HashSet;

/// The identifier of a tenant
type TenantId = uuid::Uuid;
/// The identifier of a scrape registration for a tenant
type TenantScrapeRegistrationId = uuid::Uuid;

pub struct Tenant {
    /// The unique identifier of this tenant
    identifier: TenantId,
    /// The internal name of the tenant
    internal_name: String,
    /// The display name of the tenant?
    display_name: String,
}

/// # Examples
/// ```{rust}
/// // A window that lasts for 6 hours, from 9:00 to 15:00
/// let window = ScrapeWindow { start: 9, end: 15 }
/// ```
#[derive(Copy, Clone)]
struct ScrapeWindow {
    pub start: u8,
    pub end: u8,
}

#[derive(Copy, Clone)]
struct ScrapeSchedule {
    window: ScrapeWindow,
    /// The number of times to perform the scrape in the window, should divide the number of hours
    /// that make up the window, e.g. With a window of 9:00 -> 15:00 (6 hours), the valid rates
    /// are 1, 2, 3, and 6
    rate: u8,
}

impl ScrapeSchedule {
    pub fn new(window: ScrapeWindow, rate: u8) -> ScrapeSchedule {
        // TODO: Assertion on rate
        ScrapeSchedule { window, rate }
    }
}

/// The possible options which dictate whether a screenshot should be taken upon a
/// (successful) scrape
pub enum ScreenshotChoices {
    Never,
    OnChange,
    EveryDay,
    EveryDayAndOnChange,
    Always,
}

/// Pipeline options
pub enum OnChangeChoices {
    DoNothing,
    Email(String),      // TODO: Change payload
    ApiRequest(String), // TODO: Change payload
}

pub struct TenantScrapeRegistration {
    /// The identifier of the registration
    identifier: TenantScrapeRegistrationId,
    /// The tenant that this scrape was registered by
    tenant: TenantId,
    /// The resource to scrape
    resource: Resource,
    /// The schedule of the scrape
    schedule: ScrapeSchedule,
    /// How should screenshots occur
    screenshot_settings: ScreenshotChoices,
    /// The action that should be performed when a value being scraped has changed
    on_change: OnChangeChoices,
}

/// An 'instance' of a tenant scrape registration that can have a result assigned to,
/// i.e. An interval is resolved from the scrape's window and rate, this 'instance' is one of those
/// resolved intervals that can have a result assigned to
pub struct TenantScrapeInterval {
    registration: TenantScrapeRegistrationId,
    /// The start time of this request, i.e. the interval starts at `start` o'clock
    start: u8,
    /// The end time of this request, i.e. the interval ends at `end` o'clock
    end: u8,
}
