#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FleetComputeCapacity {
    /// Number of currently available instances that can be used to stream sessions.
    #[builder(into)]
    #[serde(rename = "available")]
    pub r#available: Option<i32>,
    /// Desired number of streaming instances.
    #[builder(into)]
    #[serde(rename = "desiredInstances")]
    pub r#desired_instances: Option<i32>,
    /// Desired number of user sessions for a multi-session fleet. This is not allowed for single-session fleets.
    #[builder(into)]
    #[serde(rename = "desiredSessions")]
    pub r#desired_sessions: Option<i32>,
    /// Number of instances in use for streaming.
    #[builder(into)]
    #[serde(rename = "inUse")]
    pub r#in_use: Option<i32>,
    /// Total number of simultaneous streaming instances that are running.
    #[builder(into)]
    #[serde(rename = "running")]
    pub r#running: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FleetComputeCapacity {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("available".to_string(), self.r#available.to_pulumi_value().await);
            map.insert("desired_instances".to_string(), self.r#desired_instances.to_pulumi_value().await);
            map.insert("desired_sessions".to_string(), self.r#desired_sessions.to_pulumi_value().await);
            map.insert("in_use".to_string(), self.r#in_use.to_pulumi_value().await);
            map.insert("running".to_string(), self.r#running.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FleetComputeCapacity {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#available: {
                        let field_value = match fields_map.get("available") {
                            Some(value) => value,
                            None => bail!("Missing field 'available' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#desired_instances: {
                        let field_value = match fields_map.get("desired_instances") {
                            Some(value) => value,
                            None => bail!("Missing field 'desired_instances' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#desired_sessions: {
                        let field_value = match fields_map.get("desired_sessions") {
                            Some(value) => value,
                            None => bail!("Missing field 'desired_sessions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#in_use: {
                        let field_value = match fields_map.get("in_use") {
                            Some(value) => value,
                            None => bail!("Missing field 'in_use' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#running: {
                        let field_value = match fields_map.get("running") {
                            Some(value) => value,
                            None => bail!("Missing field 'running' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
