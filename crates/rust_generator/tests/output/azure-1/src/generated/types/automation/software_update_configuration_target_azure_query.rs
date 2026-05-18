#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SoftwareUpdateConfigurationTargetAzureQuery {
    /// Specifies a list of locations to scope the query to.
    #[builder(into)]
    #[serde(rename = "locations")]
    pub r#locations: Option<Vec<String>>,
    /// Specifies a list of Subscription or Resource Group ARM Ids to query.
    #[builder(into)]
    #[serde(rename = "scopes")]
    pub r#scopes: Option<Vec<String>>,
    /// Specifies how the specified tags to filter VMs. Possible values are `Any` and `All`.
    #[builder(into)]
    #[serde(rename = "tagFilter")]
    pub r#tag_filter: Option<String>,
    /// A mapping of tags used for query filter. One or more `tags` block as defined below.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<Vec<super::super::types::automation::SoftwareUpdateConfigurationTargetAzureQueryTag>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SoftwareUpdateConfigurationTargetAzureQuery {
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
                    "locations",
                    &self.r#locations,
                ),
                to_pulumi_object_field(
                    "scopes",
                    &self.r#scopes,
                ),
                to_pulumi_object_field(
                    "tag_filter",
                    &self.r#tag_filter,
                ),
                to_pulumi_object_field(
                    "tags",
                    &self.r#tags,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SoftwareUpdateConfigurationTargetAzureQuery {
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
                    r#locations: {
                        let field_value = match fields_map.get("locations") {
                            Some(value) => value,
                            None => bail!("Missing field 'locations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scopes: {
                        let field_value = match fields_map.get("scopes") {
                            Some(value) => value,
                            None => bail!("Missing field 'scopes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tag_filter: {
                        let field_value = match fields_map.get("tag_filter") {
                            Some(value) => value,
                            None => bail!("Missing field 'tag_filter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tags: {
                        let field_value = match fields_map.get("tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
