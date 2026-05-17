#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetGlobalForwardingRuleMetadataFilter {
    /// The list of label value pairs that must match labels in the
    /// provided metadata based on filterMatchCriteria
    /// 
    /// This list must not be empty and can have at the most 64 entries.
    #[builder(into)]
    #[serde(rename = "filterLabels")]
    pub r#filter_labels: Vec<super::super::types::compute::GetGlobalForwardingRuleMetadataFilterFilterLabel>,
    /// Specifies how individual filterLabel matches within the list of
    /// filterLabels contribute towards the overall metadataFilter match.
    /// 
    /// MATCH_ANY - At least one of the filterLabels must have a matching
    /// label in the provided metadata.
    /// MATCH_ALL - All filterLabels must have matching labels in the
    /// provided metadata. Possible values: ["MATCH_ANY", "MATCH_ALL"]
    #[builder(into)]
    #[serde(rename = "filterMatchCriteria")]
    pub r#filter_match_criteria: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetGlobalForwardingRuleMetadataFilter {
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
                    "filter_labels",
                    &self.r#filter_labels,
                ),
                to_pulumi_object_field(
                    "filter_match_criteria",
                    &self.r#filter_match_criteria,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetGlobalForwardingRuleMetadataFilter {
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
                    r#filter_labels: {
                        let field_value = match fields_map.get("filter_labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'filter_labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filter_match_criteria: {
                        let field_value = match fields_map.get("filter_match_criteria") {
                            Some(value) => value,
                            None => bail!("Missing field 'filter_match_criteria' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
