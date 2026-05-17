#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GuestPoliciesPackageRepositoryApt {
    /// Type of archive files in this repository. The default behavior is DEB.
    /// Default value is `DEB`.
    /// Possible values are: `DEB`, `DEB_SRC`.
    #[builder(into)]
    #[serde(rename = "archiveType")]
    pub r#archive_type: Option<String>,
    /// List of components for this repository. Must contain at least one item.
    #[builder(into)]
    #[serde(rename = "components")]
    pub r#components: Vec<String>,
    /// Distribution of this repository.
    #[builder(into)]
    #[serde(rename = "distribution")]
    pub r#distribution: String,
    /// URI of the key file for this repository. The agent maintains a keyring at
    /// /etc/apt/trusted.gpg.d/osconfig_agent_managed.gpg containing all the keys in any applied guest policy.
    #[builder(into)]
    #[serde(rename = "gpgKey")]
    pub r#gpg_key: Option<String>,
    /// URI for this repository.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GuestPoliciesPackageRepositoryApt {
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
                "archive_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#archive_type,
                )
                .await,
            );
            map.insert(
                "components".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#components,
                )
                .await,
            );
            map.insert(
                "distribution".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#distribution,
                )
                .await,
            );
            map.insert(
                "gpg_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gpg_key,
                )
                .await,
            );
            map.insert(
                "uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#uri,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GuestPoliciesPackageRepositoryApt {
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
                    r#archive_type: {
                        let field_value = match fields_map.get("archive_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'archive_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#components: {
                        let field_value = match fields_map.get("components") {
                            Some(value) => value,
                            None => bail!("Missing field 'components' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#distribution: {
                        let field_value = match fields_map.get("distribution") {
                            Some(value) => value,
                            None => bail!("Missing field 'distribution' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gpg_key: {
                        let field_value = match fields_map.get("gpg_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'gpg_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#uri: {
                        let field_value = match fields_map.get("uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
