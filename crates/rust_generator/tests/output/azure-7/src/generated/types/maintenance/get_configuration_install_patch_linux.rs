#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetConfigurationInstallPatchLinux {
    /// List of Classification category of patches to be patched.
    #[builder(into)]
    #[serde(rename = "classificationsToIncludes")]
    pub r#classifications_to_includes: Vec<String>,
    /// List of package names to be excluded from patching.
    #[builder(into)]
    #[serde(rename = "packageNamesMaskToExcludes")]
    pub r#package_names_mask_to_excludes: Vec<String>,
    /// List of package names to be included for patching.
    #[builder(into)]
    #[serde(rename = "packageNamesMaskToIncludes")]
    pub r#package_names_mask_to_includes: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetConfigurationInstallPatchLinux {
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
                "classifications_to_includes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#classifications_to_includes,
                )
                .await,
            );
            map.insert(
                "package_names_mask_to_excludes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#package_names_mask_to_excludes,
                )
                .await,
            );
            map.insert(
                "package_names_mask_to_includes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#package_names_mask_to_includes,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetConfigurationInstallPatchLinux {
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
                    r#classifications_to_includes: {
                        let field_value = match fields_map.get("classifications_to_includes") {
                            Some(value) => value,
                            None => bail!("Missing field 'classifications_to_includes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#package_names_mask_to_excludes: {
                        let field_value = match fields_map.get("package_names_mask_to_excludes") {
                            Some(value) => value,
                            None => bail!("Missing field 'package_names_mask_to_excludes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#package_names_mask_to_includes: {
                        let field_value = match fields_map.get("package_names_mask_to_includes") {
                            Some(value) => value,
                            None => bail!("Missing field 'package_names_mask_to_includes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
