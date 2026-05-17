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
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "available".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#available,
                )
                .await,
            );
            map.insert(
                "desired_instances".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#desired_instances,
                )
                .await,
            );
            map.insert(
                "desired_sessions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#desired_sessions,
                )
                .await,
            );
            map.insert(
                "in_use".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#in_use,
                )
                .await,
            );
            map.insert(
                "running".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#running,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FleetComputeCapacity {
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
                    r#available: {
                        let field_value = match fields_map.get("available") {
                            Some(value) => value,
                            None => bail!("Missing field 'available' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#desired_instances: {
                        let field_value = match fields_map.get("desired_instances") {
                            Some(value) => value,
                            None => bail!("Missing field 'desired_instances' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#desired_sessions: {
                        let field_value = match fields_map.get("desired_sessions") {
                            Some(value) => value,
                            None => bail!("Missing field 'desired_sessions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#in_use: {
                        let field_value = match fields_map.get("in_use") {
                            Some(value) => value,
                            None => bail!("Missing field 'in_use' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#running: {
                        let field_value = match fields_map.get("running") {
                            Some(value) => value,
                            None => bail!("Missing field 'running' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
