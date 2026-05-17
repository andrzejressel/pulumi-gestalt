#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TaskDefinitionVolumeDockerVolumeConfiguration {
    /// If this value is `true`, the Docker volume is created if it does not already exist. *Note*: This field is only used if the scope is `shared`.
    #[builder(into)]
    #[serde(rename = "autoprovision")]
    pub r#autoprovision: Option<bool>,
    /// Docker volume driver to use. The driver value must match the driver name provided by Docker because it is used for task placement.
    #[builder(into)]
    #[serde(rename = "driver")]
    pub r#driver: Option<String>,
    /// Map of Docker driver specific options.
    #[builder(into)]
    #[serde(rename = "driverOpts")]
    pub r#driver_opts: Option<std::collections::HashMap<String, String>>,
    /// Map of custom metadata to add to your Docker volume.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Option<std::collections::HashMap<String, String>>,
    /// Scope for the Docker volume, which determines its lifecycle, either `task` or `shared`.  Docker volumes that are scoped to a `task` are automatically provisioned when the task starts and destroyed when the task stops. Docker volumes that are scoped as `shared` persist after the task stops.
    #[builder(into)]
    #[serde(rename = "scope")]
    pub r#scope: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TaskDefinitionVolumeDockerVolumeConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "autoprovision".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#autoprovision,
                )
                .await,
            );
            map.insert(
                "driver".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#driver,
                )
                .await,
            );
            map.insert(
                "driver_opts".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#driver_opts,
                )
                .await,
            );
            map.insert(
                "labels".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#labels,
                )
                .await,
            );
            map.insert(
                "scope".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#scope,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TaskDefinitionVolumeDockerVolumeConfiguration {
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
                    r#autoprovision: {
                        let field_value = match fields_map.get("autoprovision") {
                            Some(value) => value,
                            None => bail!("Missing field 'autoprovision' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#driver: {
                        let field_value = match fields_map.get("driver") {
                            Some(value) => value,
                            None => bail!("Missing field 'driver' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#driver_opts: {
                        let field_value = match fields_map.get("driver_opts") {
                            Some(value) => value,
                            None => bail!("Missing field 'driver_opts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#labels: {
                        let field_value = match fields_map.get("labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scope: {
                        let field_value = match fields_map.get("scope") {
                            Some(value) => value,
                            None => bail!("Missing field 'scope' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
