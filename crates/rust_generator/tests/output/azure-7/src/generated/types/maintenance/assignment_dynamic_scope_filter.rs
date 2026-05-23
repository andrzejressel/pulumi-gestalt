#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AssignmentDynamicScopeFilter {
    /// Specifies a list of locations to scope the query to.
    #[builder(into)]
    pub r#locations: Option<Vec<String>>,
    /// Specifies a list of allowed operating systems.
    #[builder(into)]
    pub r#os_types: Option<Vec<String>>,
    /// Specifies a list of allowed resource groups.
    #[builder(into)]
    pub r#resource_groups: Option<Vec<String>>,
    /// Specifies a list of allowed resources.
    #[builder(into)]
    pub r#resource_types: Option<Vec<String>>,
    /// Filter VMs by `Any` or `All` specified tags. Defaults to `Any`.
    #[builder(into)]
    pub r#tag_filter: Option<String>,
    /// A mapping of tags for the VM
    #[builder(into)]
    pub r#tags: Option<Vec<super::super::types::maintenance::AssignmentDynamicScopeFilterTag>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AssignmentDynamicScopeFilter {
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
                    "osTypes",
                    &self.r#os_types,
                ),
                to_pulumi_object_field(
                    "resourceGroups",
                    &self.r#resource_groups,
                ),
                to_pulumi_object_field(
                    "resourceTypes",
                    &self.r#resource_types,
                ),
                to_pulumi_object_field(
                    "tagFilter",
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AssignmentDynamicScopeFilter {
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
                    r#os_types: {
                        let field_value = match fields_map.get("osTypes") {
                            Some(value) => value,
                            None => bail!("Missing field 'osTypes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_groups: {
                        let field_value = match fields_map.get("resourceGroups") {
                            Some(value) => value,
                            None => bail!("Missing field 'resourceGroups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_types: {
                        let field_value = match fields_map.get("resourceTypes") {
                            Some(value) => value,
                            None => bail!("Missing field 'resourceTypes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tag_filter: {
                        let field_value = match fields_map.get("tagFilter") {
                            Some(value) => value,
                            None => bail!("Missing field 'tagFilter' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
