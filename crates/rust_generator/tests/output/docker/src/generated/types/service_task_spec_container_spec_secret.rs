#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceTaskSpecContainerSpecSecret {
    /// Represents the file GID. Defaults to `0`
    #[builder(into)]
    #[serde(rename = "fileGid")]
    pub r#file_gid: Option<String>,
    /// Represents represents the FileMode of the file. Defaults to `0o444`
    #[builder(into)]
    #[serde(rename = "fileMode")]
    pub r#file_mode: Option<i32>,
    /// Represents the final filename in the filesystem
    #[builder(into)]
    #[serde(rename = "fileName")]
    pub r#file_name: String,
    /// Represents the file UID. Defaults to `0`
    #[builder(into)]
    #[serde(rename = "fileUid")]
    pub r#file_uid: Option<String>,
    /// ID of the specific secret that we're referencing
    #[builder(into)]
    #[serde(rename = "secretId")]
    pub r#secret_id: String,
    /// Name of the secret that this references, but this is just provided for lookup/display purposes. The config in the reference will be identified by its ID
    #[builder(into)]
    #[serde(rename = "secretName")]
    pub r#secret_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceTaskSpecContainerSpecSecret {
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
                "file_gid".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#file_gid,
                )
                .await,
            );
            map.insert(
                "file_mode".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#file_mode,
                )
                .await,
            );
            map.insert(
                "file_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#file_name,
                )
                .await,
            );
            map.insert(
                "file_uid".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#file_uid,
                )
                .await,
            );
            map.insert(
                "secret_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secret_id,
                )
                .await,
            );
            map.insert(
                "secret_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secret_name,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceTaskSpecContainerSpecSecret {
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
                    r#file_gid: {
                        let field_value = match fields_map.get("file_gid") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_gid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_mode: {
                        let field_value = match fields_map.get("file_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_name: {
                        let field_value = match fields_map.get("file_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#file_uid: {
                        let field_value = match fields_map.get("file_uid") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_uid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret_id: {
                        let field_value = match fields_map.get("secret_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'secret_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret_name: {
                        let field_value = match fields_map.get("secret_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'secret_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
