#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SystemTopicEventSubscriptionSubjectFilter {
    /// Specifies if `subject_begins_with` and `subject_ends_with` case sensitive. This value
    #[builder(into)]
    #[serde(rename = "caseSensitive")]
    pub r#case_sensitive: Option<bool>,
    /// A string to filter events for an event subscription based on a resource path prefix.
    #[builder(into)]
    #[serde(rename = "subjectBeginsWith")]
    pub r#subject_begins_with: Option<String>,
    /// A string to filter events for an event subscription based on a resource path suffix.
    #[builder(into)]
    #[serde(rename = "subjectEndsWith")]
    pub r#subject_ends_with: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SystemTopicEventSubscriptionSubjectFilter {
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
                    "case_sensitive",
                    &self.r#case_sensitive,
                ),
                to_pulumi_object_field(
                    "subject_begins_with",
                    &self.r#subject_begins_with,
                ),
                to_pulumi_object_field(
                    "subject_ends_with",
                    &self.r#subject_ends_with,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SystemTopicEventSubscriptionSubjectFilter {
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
                    r#case_sensitive: {
                        let field_value = match fields_map.get("case_sensitive") {
                            Some(value) => value,
                            None => bail!("Missing field 'case_sensitive' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subject_begins_with: {
                        let field_value = match fields_map.get("subject_begins_with") {
                            Some(value) => value,
                            None => bail!("Missing field 'subject_begins_with' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subject_ends_with: {
                        let field_value = match fields_map.get("subject_ends_with") {
                            Some(value) => value,
                            None => bail!("Missing field 'subject_ends_with' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
