#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OsPolicyAssignmentOsPolicyResourceGroupResource {
    /// Exec resource Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "exec")]
    pub r#exec: Option<Box<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourceExec>>,
    /// File resource Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "file")]
    pub r#file: Option<Box<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourceFile>>,
    /// The id of the resource with the following restrictions:
    /// 
    /// *   Must contain only lowercase letters, numbers, and hyphens.
    /// *   Must start with a letter.
    /// *   Must be between 1-63 characters.
    /// *   Must end with a number or a letter.
    /// *   Must be unique within the OS policy.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// Package resource Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "pkg")]
    pub r#pkg: Option<Box<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourcePkg>>,
    /// Package repository resource Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "repository")]
    pub r#repository: Option<Box<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourceRepository>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for OsPolicyAssignmentOsPolicyResourceGroupResource {
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
                "exec".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#exec,
                )
                .await,
            );
            map.insert(
                "file".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#file,
                )
                .await,
            );
            map.insert(
                "id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#id,
                )
                .await,
            );
            map.insert(
                "pkg".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pkg,
                )
                .await,
            );
            map.insert(
                "repository".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#repository,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for OsPolicyAssignmentOsPolicyResourceGroupResource {
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
                    r#exec: {
                        let field_value = match fields_map.get("exec") {
                            Some(value) => value,
                            None => bail!("Missing field 'exec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file: {
                        let field_value = match fields_map.get("file") {
                            Some(value) => value,
                            None => bail!("Missing field 'file' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pkg: {
                        let field_value = match fields_map.get("pkg") {
                            Some(value) => value,
                            None => bail!("Missing field 'pkg' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#repository: {
                        let field_value = match fields_map.get("repository") {
                            Some(value) => value,
                            None => bail!("Missing field 'repository' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
