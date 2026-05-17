#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSecretVersionsVersion {
    #[builder(into)]
    #[serde(rename = "createdTime")]
    pub r#created_time: String,
    /// Date that this version of the secret was last accessed.
    #[builder(into)]
    #[serde(rename = "lastAccessedDate")]
    pub r#last_accessed_date: String,
    /// Unique version identifier of this version of the secret.
    #[builder(into)]
    #[serde(rename = "versionId")]
    pub r#version_id: String,
    #[builder(into)]
    #[serde(rename = "versionStages")]
    pub r#version_stages: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetSecretVersionsVersion {
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
                "created_time".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#created_time,
                )
                .await,
            );
            map.insert(
                "last_accessed_date".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#last_accessed_date,
                )
                .await,
            );
            map.insert(
                "version_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#version_id,
                )
                .await,
            );
            map.insert(
                "version_stages".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#version_stages,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetSecretVersionsVersion {
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
                    r#created_time: {
                        let field_value = match fields_map.get("created_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'created_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_accessed_date: {
                        let field_value = match fields_map.get("last_accessed_date") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_accessed_date' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#version_id: {
                        let field_value = match fields_map.get("version_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'version_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#version_stages: {
                        let field_value = match fields_map.get("version_stages") {
                            Some(value) => value,
                            None => bail!("Missing field 'version_stages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
