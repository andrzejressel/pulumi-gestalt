#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TriggerDestination {
    /// The Cloud Function resource name. Only Cloud Functions V2 is supported. Format projects/{project}/locations/{location}/functions/{function} This is a read-only field. [WARNING] Creating Cloud Functions V2 triggers is only supported via the Cloud Functions product. An error will be returned if the user sets this value.
    #[builder(into)]
    #[serde(rename = "cloudFunction")]
    pub r#cloud_function: Option<String>,
    /// Cloud Run fully-managed service that receives the events. The service should be running in the same project of the trigger.
    #[builder(into)]
    #[serde(rename = "cloudRunService")]
    pub r#cloud_run_service: Option<Box<super::super::types::eventarc::TriggerDestinationCloudRunService>>,
    /// A GKE service capable of receiving events. The service should be running in the same project as the trigger.
    #[builder(into)]
    #[serde(rename = "gke")]
    pub r#gke: Option<Box<super::super::types::eventarc::TriggerDestinationGke>>,
    /// An HTTP endpoint destination described by an URI.
    #[builder(into)]
    #[serde(rename = "httpEndpoint")]
    pub r#http_endpoint: Option<Box<super::super::types::eventarc::TriggerDestinationHttpEndpoint>>,
    /// Optional. Network config is used to configure how Eventarc resolves and connect to a destination. This should only be used with HttpEndpoint destination type.
    #[builder(into)]
    #[serde(rename = "networkConfig")]
    pub r#network_config: Option<Box<super::super::types::eventarc::TriggerDestinationNetworkConfig>>,
    /// The resource name of the Workflow whose Executions are triggered by the events. The Workflow resource should be deployed in the same project as the trigger. Format: `projects/{project}/locations/{location}/workflows/{workflow}`
    #[builder(into)]
    #[serde(rename = "workflow")]
    pub r#workflow: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TriggerDestination {
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
                "cloud_function".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cloud_function,
                )
                .await,
            );
            map.insert(
                "cloud_run_service".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cloud_run_service,
                )
                .await,
            );
            map.insert(
                "gke".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gke,
                )
                .await,
            );
            map.insert(
                "http_endpoint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_endpoint,
                )
                .await,
            );
            map.insert(
                "network_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#network_config,
                )
                .await,
            );
            map.insert(
                "workflow".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#workflow,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TriggerDestination {
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
                    r#cloud_function: {
                        let field_value = match fields_map.get("cloud_function") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloud_function' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloud_run_service: {
                        let field_value = match fields_map.get("cloud_run_service") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloud_run_service' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gke: {
                        let field_value = match fields_map.get("gke") {
                            Some(value) => value,
                            None => bail!("Missing field 'gke' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_endpoint: {
                        let field_value = match fields_map.get("http_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_config: {
                        let field_value = match fields_map.get("network_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#workflow: {
                        let field_value = match fields_map.get("workflow") {
                            Some(value) => value,
                            None => bail!("Missing field 'workflow' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
