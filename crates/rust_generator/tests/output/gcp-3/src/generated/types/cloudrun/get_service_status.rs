#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetServiceStatus {
    /// Array of observed Service Conditions, indicating the current ready state of the service.
    #[builder(into)]
    #[serde(rename = "conditions")]
    pub r#conditions: Vec<super::super::types::cloudrun::GetServiceStatusCondition>,
    /// From ConfigurationStatus. LatestCreatedRevisionName is the last revision that was created
    /// from this Service's Configuration. It might not be ready yet, for that use
    /// LatestReadyRevisionName.
    #[builder(into)]
    #[serde(rename = "latestCreatedRevisionName")]
    pub r#latest_created_revision_name: String,
    /// From ConfigurationStatus. LatestReadyRevisionName holds the name of the latest Revision
    /// stamped out from this Service's Configuration that has had its "Ready" condition become
    /// "True".
    #[builder(into)]
    #[serde(rename = "latestReadyRevisionName")]
    pub r#latest_ready_revision_name: String,
    /// ObservedGeneration is the 'Generation' of the Route that was last processed by the
    /// controller.
    /// 
    /// Clients polling for completed reconciliation should poll until observedGeneration =
    /// metadata.generation and the Ready condition's status is True or False.
    #[builder(into)]
    #[serde(rename = "observedGeneration")]
    pub r#observed_generation: i32,
    /// Traffic specifies how to distribute traffic over a collection of Knative Revisions
    /// and Configurations
    #[builder(into)]
    #[serde(rename = "traffics")]
    pub r#traffics: Vec<super::super::types::cloudrun::GetServiceStatusTraffic>,
    /// From RouteStatus. URL holds the url that will distribute traffic over the provided traffic
    /// targets. It generally has the form
    /// https://{route-hash}-{project-hash}-{cluster-level-suffix}.a.run.app
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetServiceStatus {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "conditions",
                    &self.r#conditions,
                ),
                to_pulumi_object_field(
                    "latest_created_revision_name",
                    &self.r#latest_created_revision_name,
                ),
                to_pulumi_object_field(
                    "latest_ready_revision_name",
                    &self.r#latest_ready_revision_name,
                ),
                to_pulumi_object_field(
                    "observed_generation",
                    &self.r#observed_generation,
                ),
                to_pulumi_object_field(
                    "traffics",
                    &self.r#traffics,
                ),
                to_pulumi_object_field(
                    "url",
                    &self.r#url,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetServiceStatus {
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
                    r#conditions: {
                        let field_value = match fields_map.get("conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#latest_created_revision_name: {
                        let field_value = match fields_map.get("latest_created_revision_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'latest_created_revision_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#latest_ready_revision_name: {
                        let field_value = match fields_map.get("latest_ready_revision_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'latest_ready_revision_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#observed_generation: {
                        let field_value = match fields_map.get("observed_generation") {
                            Some(value) => value,
                            None => bail!("Missing field 'observed_generation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#traffics: {
                        let field_value = match fields_map.get("traffics") {
                            Some(value) => value,
                            None => bail!("Missing field 'traffics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#url: {
                        let field_value = match fields_map.get("url") {
                            Some(value) => value,
                            None => bail!("Missing field 'url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
