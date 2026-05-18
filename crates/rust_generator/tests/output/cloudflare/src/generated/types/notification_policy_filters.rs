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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NotificationPolicyFilters {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "actions",
                    &self.r#actions,
                ),
                to_pulumi_object_field(
                    "affected_components",
                    &self.r#affected_components,
                ),
                to_pulumi_object_field(
                    "airport_codes",
                    &self.r#airport_codes,
                ),
                to_pulumi_object_field(
                    "alert_trigger_preferences",
                    &self.r#alert_trigger_preferences,
                ),
                to_pulumi_object_field(
                    "enableds",
                    &self.r#enableds,
                ),
                to_pulumi_object_field(
                    "environments",
                    &self.r#environments,
                ),
                to_pulumi_object_field(
                    "event_sources",
                    &self.r#event_sources,
                ),
                to_pulumi_object_field(
                    "event_types",
                    &self.r#event_types,
                ),
                to_pulumi_object_field(
                    "events",
                    &self.r#events,
                ),
                to_pulumi_object_field(
                    "group_bies",
                    &self.r#group_bies,
                ),
                to_pulumi_object_field(
                    "health_check_ids",
                    &self.r#health_check_ids,
                ),
                to_pulumi_object_field(
                    "incident_impacts",
                    &self.r#incident_impacts,
                ),
                to_pulumi_object_field(
                    "input_ids",
                    &self.r#input_ids,
                ),
                to_pulumi_object_field(
                    "limits",
                    &self.r#limits,
                ),
                to_pulumi_object_field(
                    "megabits_per_seconds",
                    &self.r#megabits_per_seconds,
                ),
                to_pulumi_object_field(
                    "new_healths",
                    &self.r#new_healths,
                ),
                to_pulumi_object_field(
                    "new_statuses",
                    &self.r#new_statuses,
                ),
                to_pulumi_object_field(
                    "packets_per_seconds",
                    &self.r#packets_per_seconds,
                ),
                to_pulumi_object_field(
                    "pool_ids",
                    &self.r#pool_ids,
                ),
                to_pulumi_object_field(
                    "products",
                    &self.r#products,
                ),
                to_pulumi_object_field(
                    "project_ids",
                    &self.r#project_ids,
                ),
                to_pulumi_object_field(
                    "protocols",
                    &self.r#protocols,
                ),
                to_pulumi_object_field(
                    "requests_per_seconds",
                    &self.r#requests_per_seconds,
                ),
                to_pulumi_object_field(
                    "selectors",
                    &self.r#selectors,
                ),
                to_pulumi_object_field(
                    "services",
                    &self.r#services,
                ),
                to_pulumi_object_field(
                    "slos",
                    &self.r#slos,
                ),
                to_pulumi_object_field(
                    "statuses",
                    &self.r#statuses,
                ),
                to_pulumi_object_field(
                    "target_hostnames",
                    &self.r#target_hostnames,
                ),
                to_pulumi_object_field(
                    "target_ips",
                    &self.r#target_ips,
                ),
                to_pulumi_object_field(
                    "target_zone_names",
                    &self.r#target_zone_names,
                ),
                to_pulumi_object_field(
                    "tunnel_ids",
                    &self.r#tunnel_ids,
                ),
                to_pulumi_object_field(
                    "tunnel_names",
                    &self.r#tunnel_names,
                ),
                to_pulumi_object_field(
                    "wheres",
                    &self.r#wheres,
                ),
                to_pulumi_object_field(
                    "zones",
                    &self.r#zones,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NotificationPolicyFilters {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#actions: {
                        let field_value = match fields_map.get("actions") {
                            Some(value) => value,
                            None => bail!("Missing field 'actions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#affected_components: {
                        let field_value = match fields_map.get("affected_components") {
                            Some(value) => value,
                            None => bail!("Missing field 'affected_components' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#airport_codes: {
                        let field_value = match fields_map.get("airport_codes") {
                            Some(value) => value,
                            None => bail!("Missing field 'airport_codes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#alert_trigger_preferences: {
                        let field_value = match fields_map.get("alert_trigger_preferences") {
                            Some(value) => value,
                            None => bail!("Missing field 'alert_trigger_preferences' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enableds: {
                        let field_value = match fields_map.get("enableds") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#environments: {
                        let field_value = match fields_map.get("environments") {
                            Some(value) => value,
                            None => bail!("Missing field 'environments' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#event_sources: {
                        let field_value = match fields_map.get("event_sources") {
                            Some(value) => value,
                            None => bail!("Missing field 'event_sources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#event_types: {
                        let field_value = match fields_map.get("event_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'event_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#events: {
                        let field_value = match fields_map.get("events") {
                            Some(value) => value,
                            None => bail!("Missing field 'events' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#group_bies: {
                        let field_value = match fields_map.get("group_bies") {
                            Some(value) => value,
                            None => bail!("Missing field 'group_bies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#health_check_ids: {
                        let field_value = match fields_map.get("health_check_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'health_check_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#incident_impacts: {
                        let field_value = match fields_map.get("incident_impacts") {
                            Some(value) => value,
                            None => bail!("Missing field 'incident_impacts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_ids: {
                        let field_value = match fields_map.get("input_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#limits: {
                        let field_value = match fields_map.get("limits") {
                            Some(value) => value,
                            None => bail!("Missing field 'limits' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#megabits_per_seconds: {
                        let field_value = match fields_map.get("megabits_per_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'megabits_per_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#new_healths: {
                        let field_value = match fields_map.get("new_healths") {
                            Some(value) => value,
                            None => bail!("Missing field 'new_healths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#new_statuses: {
                        let field_value = match fields_map.get("new_statuses") {
                            Some(value) => value,
                            None => bail!("Missing field 'new_statuses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#packets_per_seconds: {
                        let field_value = match fields_map.get("packets_per_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'packets_per_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pool_ids: {
                        let field_value = match fields_map.get("pool_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'pool_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#products: {
                        let field_value = match fields_map.get("products") {
                            Some(value) => value,
                            None => bail!("Missing field 'products' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#project_ids: {
                        let field_value = match fields_map.get("project_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'project_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#protocols: {
                        let field_value = match fields_map.get("protocols") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocols' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#requests_per_seconds: {
                        let field_value = match fields_map.get("requests_per_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'requests_per_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#selectors: {
                        let field_value = match fields_map.get("selectors") {
                            Some(value) => value,
                            None => bail!("Missing field 'selectors' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#services: {
                        let field_value = match fields_map.get("services") {
                            Some(value) => value,
                            None => bail!("Missing field 'services' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#slos: {
                        let field_value = match fields_map.get("slos") {
                            Some(value) => value,
                            None => bail!("Missing field 'slos' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#statuses: {
                        let field_value = match fields_map.get("statuses") {
                            Some(value) => value,
                            None => bail!("Missing field 'statuses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_hostnames: {
                        let field_value = match fields_map.get("target_hostnames") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_hostnames' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_ips: {
                        let field_value = match fields_map.get("target_ips") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_ips' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_zone_names: {
                        let field_value = match fields_map.get("target_zone_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_zone_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tunnel_ids: {
                        let field_value = match fields_map.get("tunnel_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'tunnel_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tunnel_names: {
                        let field_value = match fields_map.get("tunnel_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'tunnel_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#wheres: {
                        let field_value = match fields_map.get("wheres") {
                            Some(value) => value,
                            None => bail!("Missing field 'wheres' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#zones: {
                        let field_value = match fields_map.get("zones") {
                            Some(value) => value,
                            None => bail!("Missing field 'zones' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
