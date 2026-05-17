#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointPolicyEndpointMatcherMetadataLabelMatcher {
    /// Specifies how matching should be done.
    /// Possible values are: `MATCH_ANY`, `MATCH_ALL`.
    #[builder(into)]
    #[serde(rename = "metadataLabelMatchCriteria")]
    pub r#metadata_label_match_criteria: String,
    /// The list of label value pairs that must match labels in the provided metadata based on filterMatchCriteria
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "metadataLabels")]
    pub r#metadata_labels: Option<Vec<super::super::types::networkservices::EndpointPolicyEndpointMatcherMetadataLabelMatcherMetadataLabel>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EndpointPolicyEndpointMatcherMetadataLabelMatcher {
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
                    "metadata_label_match_criteria",
                    &self.r#metadata_label_match_criteria,
                ),
                to_pulumi_object_field(
                    "metadata_labels",
                    &self.r#metadata_labels,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EndpointPolicyEndpointMatcherMetadataLabelMatcher {
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
                    r#metadata_label_match_criteria: {
                        let field_value = match fields_map.get("metadata_label_match_criteria") {
                            Some(value) => value,
                            None => bail!("Missing field 'metadata_label_match_criteria' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#metadata_labels: {
                        let field_value = match fields_map.get("metadata_labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'metadata_labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
