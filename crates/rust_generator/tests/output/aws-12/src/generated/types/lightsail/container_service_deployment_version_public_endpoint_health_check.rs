#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ContainerServiceDeploymentVersionPublicEndpointHealthCheck {
    /// The number of consecutive health checks successes required before moving the container to the Healthy state. Defaults to 2.
    #[builder(into)]
    #[serde(rename = "healthyThreshold")]
    pub r#healthy_threshold: Option<i32>,
    /// The approximate interval, in seconds, between health checks of an individual container. You can specify between 5 and 300 seconds. Defaults to 5.
    #[builder(into)]
    #[serde(rename = "intervalSeconds")]
    pub r#interval_seconds: Option<i32>,
    /// The path on the container on which to perform the health check. Defaults to "/".
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// The HTTP codes to use when checking for a successful response from a container. You can specify values between 200 and 499. Defaults to "200-499".
    #[builder(into)]
    #[serde(rename = "successCodes")]
    pub r#success_codes: Option<String>,
    /// The amount of time, in seconds, during which no response means a failed health check. You can specify between 2 and 60 seconds. Defaults to 2.
    #[builder(into)]
    #[serde(rename = "timeoutSeconds")]
    pub r#timeout_seconds: Option<i32>,
    /// The number of consecutive health checks failures required before moving the container to the Unhealthy state. Defaults to 2.
    #[builder(into)]
    #[serde(rename = "unhealthyThreshold")]
    pub r#unhealthy_threshold: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ContainerServiceDeploymentVersionPublicEndpointHealthCheck {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("healthy_threshold".to_string(), self.r#healthy_threshold.to_pulumi_value().await);
            map.insert("interval_seconds".to_string(), self.r#interval_seconds.to_pulumi_value().await);
            map.insert("path".to_string(), self.r#path.to_pulumi_value().await);
            map.insert("success_codes".to_string(), self.r#success_codes.to_pulumi_value().await);
            map.insert("timeout_seconds".to_string(), self.r#timeout_seconds.to_pulumi_value().await);
            map.insert("unhealthy_threshold".to_string(), self.r#unhealthy_threshold.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ContainerServiceDeploymentVersionPublicEndpointHealthCheck {
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
                    r#healthy_threshold: {
                        let field_value = match fields_map.get("healthy_threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'healthy_threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#interval_seconds: {
                        let field_value = match fields_map.get("interval_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'interval_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#path: {
                        let field_value = match fields_map.get("path") {
                            Some(value) => value,
                            None => bail!("Missing field 'path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#success_codes: {
                        let field_value = match fields_map.get("success_codes") {
                            Some(value) => value,
                            None => bail!("Missing field 'success_codes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#timeout_seconds: {
                        let field_value = match fields_map.get("timeout_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeout_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#unhealthy_threshold: {
                        let field_value = match fields_map.get("unhealthy_threshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'unhealthy_threshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
