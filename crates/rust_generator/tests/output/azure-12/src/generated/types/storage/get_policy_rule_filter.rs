#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPolicyRuleFilter {
    /// An array of predefined values. Valid options are `blockBlob` and `appendBlob`.
    #[builder(into)]
    #[serde(rename = "blobTypes")]
    pub r#blob_types: Vec<String>,
    /// A `match_blob_index_tag` block as defined below. The block defines the blob index tag based filtering for blob objects.
    #[builder(into)]
    #[serde(rename = "matchBlobIndexTags")]
    pub r#match_blob_index_tags: Vec<super::super::types::storage::GetPolicyRuleFilterMatchBlobIndexTag>,
    /// An array of strings for prefixes to be matched.
    #[builder(into)]
    #[serde(rename = "prefixMatches")]
    pub r#prefix_matches: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetPolicyRuleFilter {
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
                    "blob_types",
                    &self.r#blob_types,
                ),
                to_pulumi_object_field(
                    "match_blob_index_tags",
                    &self.r#match_blob_index_tags,
                ),
                to_pulumi_object_field(
                    "prefix_matches",
                    &self.r#prefix_matches,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetPolicyRuleFilter {
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
                    r#blob_types: {
                        let field_value = match fields_map.get("blob_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'blob_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#match_blob_index_tags: {
                        let field_value = match fields_map.get("match_blob_index_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'match_blob_index_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#prefix_matches: {
                        let field_value = match fields_map.get("prefix_matches") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefix_matches' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
