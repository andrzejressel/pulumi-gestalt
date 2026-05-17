#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BatchRuntimeInfo {
    /// (Output)
    /// Approximate workload resource usage, calculated when the workload completes(see [Dataproc Serverless pricing](https://cloud.google.com/dataproc-serverless/pricing))
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "approximateUsages")]
    pub r#approximate_usages: Option<Vec<super::super::types::dataproc::BatchRuntimeInfoApproximateUsage>>,
    /// (Output)
    /// Snapshot of current workload resource usage(see [Dataproc Serverless pricing](https://cloud.google.com/dataproc-serverless/pricing))
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "currentUsages")]
    pub r#current_usages: Option<Vec<super::super::types::dataproc::BatchRuntimeInfoCurrentUsage>>,
    /// (Output)
    /// A URI pointing to the location of the diagnostics tarball.
    #[builder(into)]
    #[serde(rename = "diagnosticOutputUri")]
    pub r#diagnostic_output_uri: Option<String>,
    /// (Output)
    /// Map of remote access endpoints (such as web interfaces and APIs) to their URIs.
    #[builder(into)]
    #[serde(rename = "endpoints")]
    pub r#endpoints: Option<std::collections::HashMap<String, String>>,
    /// (Output)
    /// A URI pointing to the location of the stdout and stderr of the workload.
    #[builder(into)]
    #[serde(rename = "outputUri")]
    pub r#output_uri: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BatchRuntimeInfo {
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
                "approximate_usages".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#approximate_usages,
                )
                .await,
            );
            map.insert(
                "current_usages".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#current_usages,
                )
                .await,
            );
            map.insert(
                "diagnostic_output_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#diagnostic_output_uri,
                )
                .await,
            );
            map.insert(
                "endpoints".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#endpoints,
                )
                .await,
            );
            map.insert(
                "output_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#output_uri,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BatchRuntimeInfo {
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
                    r#approximate_usages: {
                        let field_value = match fields_map.get("approximate_usages") {
                            Some(value) => value,
                            None => bail!("Missing field 'approximate_usages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#current_usages: {
                        let field_value = match fields_map.get("current_usages") {
                            Some(value) => value,
                            None => bail!("Missing field 'current_usages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#diagnostic_output_uri: {
                        let field_value = match fields_map.get("diagnostic_output_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'diagnostic_output_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#endpoints: {
                        let field_value = match fields_map.get("endpoints") {
                            Some(value) => value,
                            None => bail!("Missing field 'endpoints' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#output_uri: {
                        let field_value = match fields_map.get("output_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
