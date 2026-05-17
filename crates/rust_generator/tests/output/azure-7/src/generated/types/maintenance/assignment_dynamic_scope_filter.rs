#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AssignmentDynamicScopeFilter {
    /// Specifies a list of locations to scope the query to.
    #[builder(into)]
    #[serde(rename = "locations")]
    pub r#locations: Option<Vec<String>>,
    /// Specifies a list of allowed operating systems.
    #[builder(into)]
    #[serde(rename = "osTypes")]
    pub r#os_types: Option<Vec<String>>,
    /// Specifies a list of allowed resource groups.
    #[builder(into)]
    #[serde(rename = "resourceGroups")]
    pub r#resource_groups: Option<Vec<String>>,
    /// Specifies a list of allowed resources.
    #[builder(into)]
    #[serde(rename = "resourceTypes")]
    pub r#resource_types: Option<Vec<String>>,
    /// Filter VMs by `Any` or `All` specified tags. Defaults to `Any`.
    #[builder(into)]
    #[serde(rename = "tagFilter")]
    pub r#tag_filter: Option<String>,
    /// A mapping of tags for the VM
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<Vec<super::super::types::maintenance::AssignmentDynamicScopeFilterTag>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AssignmentDynamicScopeFilter {
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
                "locations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#locations,
                )
                .await,
            );
            map.insert(
                "os_types".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#os_types,
                )
                .await,
            );
            map.insert(
                "resource_groups".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resource_groups,
                )
                .await,
            );
            map.insert(
                "resource_types".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resource_types,
                )
                .await,
            );
            map.insert(
                "tag_filter".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tag_filter,
                )
                .await,
            );
            map.insert(
                "tags".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tags,
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
                        let field_value = match fields_map.get("os_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'os_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_groups: {
                        let field_value = match fields_map.get("resource_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_types: {
                        let field_value = match fields_map.get("resource_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
