#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RepositoryCleanupPolicyCondition {
    /// Match versions newer than a duration.
    #[builder(into)]
    #[serde(rename = "newerThan")]
    pub r#newer_than: Option<String>,
    /// Match versions older than a duration.
    #[builder(into)]
    #[serde(rename = "olderThan")]
    pub r#older_than: Option<String>,
    /// Match versions by package prefix. Applied on any prefix match.
    #[builder(into)]
    #[serde(rename = "packageNamePrefixes")]
    pub r#package_name_prefixes: Option<Vec<String>>,
    /// Match versions by tag prefix. Applied on any prefix match.
    #[builder(into)]
    #[serde(rename = "tagPrefixes")]
    pub r#tag_prefixes: Option<Vec<String>>,
    /// Match versions by tag status.
    /// Default value is `ANY`.
    /// Possible values are: `TAGGED`, `UNTAGGED`, `ANY`.
    #[builder(into)]
    #[serde(rename = "tagState")]
    pub r#tag_state: Option<String>,
    /// Match versions by version name prefix. Applied on any prefix match.
    #[builder(into)]
    #[serde(rename = "versionNamePrefixes")]
    pub r#version_name_prefixes: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RepositoryCleanupPolicyCondition {
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
                "newer_than".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#newer_than,
                )
                .await,
            );
            map.insert(
                "older_than".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#older_than,
                )
                .await,
            );
            map.insert(
                "package_name_prefixes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#package_name_prefixes,
                )
                .await,
            );
            map.insert(
                "tag_prefixes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tag_prefixes,
                )
                .await,
            );
            map.insert(
                "tag_state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tag_state,
                )
                .await,
            );
            map.insert(
                "version_name_prefixes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#version_name_prefixes,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RepositoryCleanupPolicyCondition {
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
                    r#newer_than: {
                        let field_value = match fields_map.get("newer_than") {
                            Some(value) => value,
                            None => bail!("Missing field 'newer_than' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#older_than: {
                        let field_value = match fields_map.get("older_than") {
                            Some(value) => value,
                            None => bail!("Missing field 'older_than' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#package_name_prefixes: {
                        let field_value = match fields_map.get("package_name_prefixes") {
                            Some(value) => value,
                            None => bail!("Missing field 'package_name_prefixes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tag_prefixes: {
                        let field_value = match fields_map.get("tag_prefixes") {
                            Some(value) => value,
                            None => bail!("Missing field 'tag_prefixes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tag_state: {
                        let field_value = match fields_map.get("tag_state") {
                            Some(value) => value,
                            None => bail!("Missing field 'tag_state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#version_name_prefixes: {
                        let field_value = match fields_map.get("version_name_prefixes") {
                            Some(value) => value,
                            None => bail!("Missing field 'version_name_prefixes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
