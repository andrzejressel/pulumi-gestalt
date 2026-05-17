#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpokeLinkedProducerVpcNetwork {
    /// IP ranges encompassing the subnets to be excluded from peering.
    #[builder(into)]
    #[serde(rename = "excludeExportRanges")]
    pub r#exclude_export_ranges: Option<Vec<String>>,
    /// IP ranges allowed to be included from peering.
    #[builder(into)]
    #[serde(rename = "includeExportRanges")]
    pub r#include_export_ranges: Option<Vec<String>>,
    /// The URI of the Service Consumer VPC that the Producer VPC is peered with.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: String,
    /// The name of the VPC peering between the Service Consumer VPC and the Producer VPC (defined in the Tenant project) which is added to the NCC hub. This peering must be in ACTIVE state.
    #[builder(into)]
    #[serde(rename = "peering")]
    pub r#peering: String,
    /// (Output)
    /// The URI of the Producer VPC.
    #[builder(into)]
    #[serde(rename = "producerNetwork")]
    pub r#producer_network: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SpokeLinkedProducerVpcNetwork {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "exclude_export_ranges",
                    &self.r#exclude_export_ranges,
                ),
                to_pulumi_object_field(
                    "include_export_ranges",
                    &self.r#include_export_ranges,
                ),
                to_pulumi_object_field(
                    "network",
                    &self.r#network,
                ),
                to_pulumi_object_field(
                    "peering",
                    &self.r#peering,
                ),
                to_pulumi_object_field(
                    "producer_network",
                    &self.r#producer_network,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SpokeLinkedProducerVpcNetwork {
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
                    r#exclude_export_ranges: {
                        let field_value = match fields_map.get("exclude_export_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'exclude_export_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_export_ranges: {
                        let field_value = match fields_map.get("include_export_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_export_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network: {
                        let field_value = match fields_map.get("network") {
                            Some(value) => value,
                            None => bail!("Missing field 'network' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#peering: {
                        let field_value = match fields_map.get("peering") {
                            Some(value) => value,
                            None => bail!("Missing field 'peering' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#producer_network: {
                        let field_value = match fields_map.get("producer_network") {
                            Some(value) => value,
                            None => bail!("Missing field 'producer_network' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
