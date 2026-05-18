#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClassificationJobS3JobDefinition {
    /// The property- and tag-based conditions that determine which S3 buckets to include or exclude from the analysis. Conflicts with `bucket_definitions`. (documented below)
    #[builder(into)]
    #[serde(rename = "bucketCriteria")]
    pub r#bucket_criteria: Option<Box<super::super::types::macie2::ClassificationJobS3JobDefinitionBucketCriteria>>,
    /// An array of objects, one for each AWS account that owns buckets to analyze. Each object specifies the account ID for an account and one or more buckets to analyze for the account. Conflicts with `bucket_criteria`. (documented below)
    #[builder(into)]
    #[serde(rename = "bucketDefinitions")]
    pub r#bucket_definitions: Option<Vec<super::super::types::macie2::ClassificationJobS3JobDefinitionBucketDefinition>>,
    /// The property- and tag-based conditions that determine which objects to include or exclude from the analysis. (documented below)
    #[builder(into)]
    #[serde(rename = "scoping")]
    pub r#scoping: Option<Box<super::super::types::macie2::ClassificationJobS3JobDefinitionScoping>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClassificationJobS3JobDefinition {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "bucket_criteria",
                    &self.r#bucket_criteria,
                ),
                to_pulumi_object_field(
                    "bucket_definitions",
                    &self.r#bucket_definitions,
                ),
                to_pulumi_object_field(
                    "scoping",
                    &self.r#scoping,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClassificationJobS3JobDefinition {
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
                    r#bucket_criteria: {
                        let field_value = match fields_map.get("bucket_criteria") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucket_criteria' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bucket_definitions: {
                        let field_value = match fields_map.get("bucket_definitions") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucket_definitions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scoping: {
                        let field_value = match fields_map.get("scoping") {
                            Some(value) => value,
                            None => bail!("Missing field 'scoping' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
