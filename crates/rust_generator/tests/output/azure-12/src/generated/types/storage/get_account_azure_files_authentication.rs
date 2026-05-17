#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAccountAzureFilesAuthentication {
    /// An `active_directory` block as documented below.
    #[builder(into)]
    #[serde(rename = "activeDirectories")]
    pub r#active_directories: Vec<super::super::types::storage::GetAccountAzureFilesAuthenticationActiveDirectory>,
    /// The default share level permissions applied to all users.
    #[builder(into)]
    #[serde(rename = "defaultShareLevelPermission")]
    pub r#default_share_level_permission: String,
    /// The directory service used for this Storage Account.
    #[builder(into)]
    #[serde(rename = "directoryType")]
    pub r#directory_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetAccountAzureFilesAuthentication {
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
                "active_directories".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#active_directories,
                )
                .await,
            );
            map.insert(
                "default_share_level_permission".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_share_level_permission,
                )
                .await,
            );
            map.insert(
                "directory_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#directory_type,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetAccountAzureFilesAuthentication {
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
                    r#active_directories: {
                        let field_value = match fields_map.get("active_directories") {
                            Some(value) => value,
                            None => bail!("Missing field 'active_directories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_share_level_permission: {
                        let field_value = match fields_map.get("default_share_level_permission") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_share_level_permission' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#directory_type: {
                        let field_value = match fields_map.get("directory_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'directory_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
