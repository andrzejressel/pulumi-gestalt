#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLoadBalancerPoolsPool {
    /// List of regions (specified by region code) from which to run health checks. Empty means every Cloudflare data center (the default), but requires an Enterprise plan. Region codes can be found [here](https://support.cloudflare.com/hc/en-us/articles/115000540888-Load-Balancing-Geographic-Regions).
    #[builder(into)]
    #[serde(rename = "checkRegions")]
    pub r#check_regions: Vec<String>,
    /// The RFC3339 timestamp of when the load balancer was created.
    #[builder(into)]
    #[serde(rename = "createdOn")]
    pub r#created_on: String,
    /// Brief description of the Load Balancer Pool intention.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// Whether this pool is enabled. Disabled pools will not receive traffic and are excluded from health checks.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    /// ID for this load balancer pool.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// Latitude this pool is physically located at; used for proximity steering.
    #[builder(into)]
    #[serde(rename = "latitude")]
    pub r#latitude: f64,
    /// Setting for controlling load shedding for this pool.
    #[builder(into)]
    #[serde(rename = "loadSheddings")]
    pub r#load_sheddings: Vec<super::types::GetLoadBalancerPoolsPoolLoadShedding>,
    /// Longitude this pool is physically located at; used for proximity steering.
    #[builder(into)]
    #[serde(rename = "longitude")]
    pub r#longitude: f64,
    /// Minimum number of origins that must be healthy for this pool to serve traffic.
    #[builder(into)]
    #[serde(rename = "minimumOrigins")]
    pub r#minimum_origins: i32,
    /// The RFC3339 timestamp of when the load balancer was last modified.
    #[builder(into)]
    #[serde(rename = "modifiedOn")]
    pub r#modified_on: String,
    /// ID of the Monitor to use for health checking origins within this pool.
    #[builder(into)]
    #[serde(rename = "monitor")]
    pub r#monitor: String,
    /// Short name (tag) for the pool.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Email address to send health status notifications to. Multiple emails are set as a comma delimited list.
    #[builder(into)]
    #[serde(rename = "notificationEmail")]
    pub r#notification_email: String,
    /// The list of origins within this pool.
    #[builder(into)]
    #[serde(rename = "origins")]
    pub r#origins: Vec<super::types::GetLoadBalancerPoolsPoolOrigin>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetLoadBalancerPoolsPool {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "check_regions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#check_regions,
                )
                .await,
            );
            map.insert(
                "created_on".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#created_on,
                )
                .await,
            );
            map.insert(
                "description".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#description,
                )
                .await,
            );
            map.insert(
                "enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enabled,
                )
                .await,
            );
            map.insert(
                "id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#id,
                )
                .await,
            );
            map.insert(
                "latitude".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#latitude,
                )
                .await,
            );
            map.insert(
                "load_sheddings".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#load_sheddings,
                )
                .await,
            );
            map.insert(
                "longitude".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#longitude,
                )
                .await,
            );
            map.insert(
                "minimum_origins".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#minimum_origins,
                )
                .await,
            );
            map.insert(
                "modified_on".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#modified_on,
                )
                .await,
            );
            map.insert(
                "monitor".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#monitor,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "notification_email".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#notification_email,
                )
                .await,
            );
            map.insert(
                "origins".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#origins,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetLoadBalancerPoolsPool {
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
                    r#check_regions: {
                        let field_value = match fields_map.get("check_regions") {
                            Some(value) => value,
                            None => bail!("Missing field 'check_regions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#created_on: {
                        let field_value = match fields_map.get("created_on") {
                            Some(value) => value,
                            None => bail!("Missing field 'created_on' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#latitude: {
                        let field_value = match fields_map.get("latitude") {
                            Some(value) => value,
                            None => bail!("Missing field 'latitude' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#load_sheddings: {
                        let field_value = match fields_map.get("load_sheddings") {
                            Some(value) => value,
                            None => bail!("Missing field 'load_sheddings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#longitude: {
                        let field_value = match fields_map.get("longitude") {
                            Some(value) => value,
                            None => bail!("Missing field 'longitude' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#minimum_origins: {
                        let field_value = match fields_map.get("minimum_origins") {
                            Some(value) => value,
                            None => bail!("Missing field 'minimum_origins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#modified_on: {
                        let field_value = match fields_map.get("modified_on") {
                            Some(value) => value,
                            None => bail!("Missing field 'modified_on' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#monitor: {
                        let field_value = match fields_map.get("monitor") {
                            Some(value) => value,
                            None => bail!("Missing field 'monitor' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#notification_email: {
                        let field_value = match fields_map.get("notification_email") {
                            Some(value) => value,
                            None => bail!("Missing field 'notification_email' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#origins: {
                        let field_value = match fields_map.get("origins") {
                            Some(value) => value,
                            None => bail!("Missing field 'origins' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
