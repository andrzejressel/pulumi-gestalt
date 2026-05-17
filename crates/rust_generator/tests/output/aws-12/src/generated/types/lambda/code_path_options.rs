#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CodePathOptions {
    /// Packages to explicitly exclude from the Assets for a serialized closure. This can be used when clients want to trim down the size of a closure, and they know that some package won't ever actually be needed at runtime, but is still a dependency of some package that is being used at runtime.
    #[builder(into)]
    #[serde(rename = "extraExcludePackages")]
    pub r#extra_exclude_packages: Option<Vec<String>>,
    /// Extra packages to include when producing the Assets for a serialized closure. This can be useful if the packages are acquired in a way that the serialization code does not understand. For example, if there was some sort of module that was pulled in based off of a computed string.
    #[builder(into)]
    #[serde(rename = "extraIncludePackages")]
    pub r#extra_include_packages: Option<Vec<String>>,
    /// Local file/directory paths that should be included when producing the Assets for a serialized closure.
    #[builder(into)]
    #[serde(rename = "extraIncludePaths")]
    pub r#extra_include_paths: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CodePathOptions {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "extra_exclude_packages".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#extra_exclude_packages,
                )
                .await,
            );
            map.insert(
                "extra_include_packages".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#extra_include_packages,
                )
                .await,
            );
            map.insert(
                "extra_include_paths".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#extra_include_paths,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CodePathOptions {
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
                    r#extra_exclude_packages: {
                        let field_value = match fields_map.get("extra_exclude_packages") {
                            Some(value) => value,
                            None => bail!("Missing field 'extra_exclude_packages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#extra_include_packages: {
                        let field_value = match fields_map.get("extra_include_packages") {
                            Some(value) => value,
                            None => bail!("Missing field 'extra_include_packages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#extra_include_paths: {
                        let field_value = match fields_map.get("extra_include_paths") {
                            Some(value) => value,
                            None => bail!("Missing field 'extra_include_paths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
