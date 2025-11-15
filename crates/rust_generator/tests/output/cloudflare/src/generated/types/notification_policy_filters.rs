#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NotificationPolicyFilters {
    /// Targeted actions for alert.
    #[builder(into)]
    #[serde(rename = "actions")]
    pub r#actions: Option<Vec<String>>,
    /// Affected components for alert. Available values: `API`, `API Shield`, `Access`, `Always Online`, `Analytics`, `Apps Marketplace`, `Argo Smart Routing`, `Audit Logs`, `Authoritative DNS`, `Billing`, `Bot Management`, `Bring Your Own IP (BYOIP)`, `Browser Isolation`, `CDN Cache Purge`, `CDN/Cache`, `Cache Reserve`, `Challenge Platform`, `Cloud Access Security Broker (CASB)`, `Community Site`, `DNS Root Servers`, `DNS Updates`, `Dashboard`, `Data Loss Prevention (DLP)`, `Developer's Site`, `Digital Experience Monitoring (DEX)`, `Distributed Web Gateway`, `Durable Objects`, `Email Routing`, `Ethereum Gateway`, `Firewall`, `Gateway`, `Geo-Key Manager`, `Image Resizing`, `Images`, `Infrastructure`, `Lists`, `Load Balancing and Monitoring`, `Logs`, `Magic Firewall`, `Magic Transit`, `Magic WAN`, `Magic WAN Connector`, `Marketing Site`, `Mirage`, `Network`, `Notifications`, `Observatory`, `Page Shield`, `Pages`, `R2`, `Radar`, `Randomness Beacon`, `Recursive DNS`, `Registrar`, `Registration Data Access Protocol (RDAP)`, `SSL Certificate Provisioning`, `SSL for SaaS Provisioning`, `Security Center`, `Snippets`, `Spectrum`, `Speed Optimizations`, `Stream`, `Support Site`, `Time Services`, `Trace`, `Tunnel`, `Turnstile`, `WARP`, `Waiting Room`, `Web Analytics`, `Workers`, `Workers KV`, `Workers Preview`, `Zaraz`, `Zero Trust`, `Zero Trust Dashboard`, `Zone Versioning`.
    #[builder(into)]
    #[serde(rename = "affectedComponents")]
    pub r#affected_components: Option<Vec<String>>,
    /// Filter on Points of Presence.
    #[builder(into)]
    #[serde(rename = "airportCodes")]
    pub r#airport_codes: Option<Vec<String>>,
    /// Alert trigger preferences. Example: `slo`.
    #[builder(into)]
    #[serde(rename = "alertTriggerPreferences")]
    pub r#alert_trigger_preferences: Option<Vec<String>>,
    /// State of the pool to alert on.
    #[builder(into)]
    #[serde(rename = "enableds")]
    pub r#enableds: Option<Vec<String>>,
    /// Environment of pages. Available values: `ENVIRONMENT_PREVIEW`, `ENVIRONMENT_PRODUCTION`.
    #[builder(into)]
    #[serde(rename = "environments")]
    pub r#environments: Option<Vec<String>>,
    /// Source configuration to alert on for pool or origin.
    #[builder(into)]
    #[serde(rename = "eventSources")]
    pub r#event_sources: Option<Vec<String>>,
    /// Stream event type to alert on.
    #[builder(into)]
    #[serde(rename = "eventTypes")]
    pub r#event_types: Option<Vec<String>>,
    /// Pages event to alert. Available values: `EVENT_DEPLOYMENT_STARTED`, `EVENT_DEPLOYMENT_FAILED`, `EVENT_DEPLOYMENT_SUCCESS`.
    #[builder(into)]
    #[serde(rename = "events")]
    pub r#events: Option<Vec<String>>,
    /// Alert grouping.
    #[builder(into)]
    #[serde(rename = "groupBies")]
    pub r#group_bies: Option<Vec<String>>,
    /// Identifier health check. Required when using `filters.0.status`.
    #[builder(into)]
    #[serde(rename = "healthCheckIds")]
    pub r#health_check_ids: Option<Vec<String>>,
    /// The incident impact level that will trigger the dispatch of a notification. Available values: `INCIDENT_IMPACT_NONE`, `INCIDENT_IMPACT_MINOR`, `INCIDENT_IMPACT_MAJOR`, `INCIDENT_IMPACT_CRITICAL`.
    #[builder(into)]
    #[serde(rename = "incidentImpacts")]
    pub r#incident_impacts: Option<Vec<String>>,
    /// Stream input id to alert on.
    #[builder(into)]
    #[serde(rename = "inputIds")]
    pub r#input_ids: Option<Vec<String>>,
    /// A numerical limit. Example: `100`.
    #[builder(into)]
    #[serde(rename = "limits")]
    pub r#limits: Option<Vec<String>>,
    /// Megabits per second threshold for dos alert.
    #[builder(into)]
    #[serde(rename = "megabitsPerSeconds")]
    pub r#megabits_per_seconds: Option<Vec<String>>,
    /// Health status to alert on for pool or origin.
    #[builder(into)]
    #[serde(rename = "newHealths")]
    pub r#new_healths: Option<Vec<String>>,
    /// Tunnel health status to alert on.
    #[builder(into)]
    #[serde(rename = "newStatuses")]
    pub r#new_statuses: Option<Vec<String>>,
    /// Packets per second threshold for dos alert.
    #[builder(into)]
    #[serde(rename = "packetsPerSeconds")]
    pub r#packets_per_seconds: Option<Vec<String>>,
    /// Load balancer pool identifier.
    #[builder(into)]
    #[serde(rename = "poolIds")]
    pub r#pool_ids: Option<Vec<String>>,
    /// Product name. Available values: `worker_requests`, `worker_durable_objects_requests`, `worker_durable_objects_duration`, `worker_durable_objects_data_transfer`, `worker_durable_objects_stored_data`, `worker_durable_objects_storage_deletes`, `worker_durable_objects_storage_writes`, `worker_durable_objects_storage_reads`.
    #[builder(into)]
    #[serde(rename = "products")]
    pub r#products: Option<Vec<String>>,
    /// Identifier of pages project.
    #[builder(into)]
    #[serde(rename = "projectIds")]
    pub r#project_ids: Option<Vec<String>>,
    /// Protocol to alert on for dos.
    #[builder(into)]
    #[serde(rename = "protocols")]
    pub r#protocols: Option<Vec<String>>,
    /// Requests per second threshold for dos alert.
    #[builder(into)]
    #[serde(rename = "requestsPerSeconds")]
    pub r#requests_per_seconds: Option<Vec<String>>,
    /// Selectors for alert. Valid options depend on the alert type.
    #[builder(into)]
    #[serde(rename = "selectors")]
    pub r#selectors: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "services")]
    pub r#services: Option<Vec<String>>,
    /// A numerical limit. Example: `99.9`.
    #[builder(into)]
    #[serde(rename = "slos")]
    pub r#slos: Option<Vec<String>>,
    /// Status to alert on.
    #[builder(into)]
    #[serde(rename = "statuses")]
    pub r#statuses: Option<Vec<String>>,
    /// Target host to alert on for dos.
    #[builder(into)]
    #[serde(rename = "targetHostnames")]
    pub r#target_hostnames: Option<Vec<String>>,
    /// Target ip to alert on for dos in CIDR notation.
    #[builder(into)]
    #[serde(rename = "targetIps")]
    pub r#target_ips: Option<Vec<String>>,
    /// Target domain to alert on.
    #[builder(into)]
    #[serde(rename = "targetZoneNames")]
    pub r#target_zone_names: Option<Vec<String>>,
    /// Tunnel IDs to alert on.
    #[builder(into)]
    #[serde(rename = "tunnelIds")]
    pub r#tunnel_ids: Option<Vec<String>>,
    /// Tunnel Names to alert on.
    #[builder(into)]
    #[serde(rename = "tunnelNames")]
    pub r#tunnel_names: Option<Vec<String>>,
    /// Filter for alert.
    #[builder(into)]
    #[serde(rename = "wheres")]
    pub r#wheres: Option<Vec<String>>,
    /// A list of zone identifiers.
    #[builder(into)]
    #[serde(rename = "zones")]
    pub r#zones: Option<Vec<String>>,
}
