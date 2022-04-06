/// A resource is something that can be scraped by an agent
///
/// # Examples
/// ```
/// let r = Resource::new("acme-ecommerce.com".to_string(), "/products/alice-bags/blue".to_string())
/// ```
#[derive(Clone, Debug)]
pub struct Resource {
    /// The specific site that the resource belongs to,
    /// this is the hostname of the uri specification.
    /// e.g. `acme-ecommerce.com`
    pub site: String,
    /// The path of the specific resource that is being scraped, respective to the site,
    /// this is the path part of the uri specification.
    /// e.g. `/products/alice-bags/blue`
    pub path: String,
}

impl Resource {
    pub fn new(site: String, path: String) -> Resource {
        Resource { site, path }
    }
}
