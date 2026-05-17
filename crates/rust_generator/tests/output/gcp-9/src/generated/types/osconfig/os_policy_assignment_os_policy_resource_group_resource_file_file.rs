#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OsPolicyAssignmentOsPolicyResourceGroupResourceFileFile {
    /// Defaults to false. When false, files are
    /// subject to validations based on the file type: Remote: A checksum must be
    /// specified. Cloud Storage: An object generation number must be specified.
    #[builder(into)]
    #[serde(rename = "allowInsecure")]
    pub r#allow_insecure: Option<bool>,
    /// A Cloud Storage object. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "gcs")]
    pub r#gcs: Option<Box<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourceFileFileGcs>>,
    /// A local path within the VM to use.
    #[builder(into)]
    #[serde(rename = "localPath")]
    pub r#local_path: Option<String>,
    /// A generic remote file. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "remote")]
    pub r#remote: Option<Box<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourceFileFileRemote>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for OsPolicyAssignmentOsPolicyResourceGroupResourceFileFile {
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
                "allow_insecure".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allow_insecure,
                )
                .await,
            );
            map.insert(
                "gcs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gcs,
                )
                .await,
            );
            map.insert(
                "local_path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#local_path,
                )
                .await,
            );
            map.insert(
                "remote".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#remote,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for OsPolicyAssignmentOsPolicyResourceGroupResourceFileFile {
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
                    r#allow_insecure: {
                        let field_value = match fields_map.get("allow_insecure") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_insecure' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gcs: {
                        let field_value = match fields_map.get("gcs") {
                            Some(value) => value,
                            None => bail!("Missing field 'gcs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_path: {
                        let field_value = match fields_map.get("local_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#remote: {
                        let field_value = match fields_map.get("remote") {
                            Some(value) => value,
                            None => bail!("Missing field 'remote' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
