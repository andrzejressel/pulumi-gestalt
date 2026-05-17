#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RunCommandSource {
    #[builder(into)]
    #[serde(rename = "commandId")]
    pub r#command_id: Option<String>,
    #[builder(into)]
    #[serde(rename = "script")]
    pub r#script: Option<String>,
    #[builder(into)]
    #[serde(rename = "scriptUri")]
    pub r#script_uri: Option<String>,
    /// A `script_uri_managed_identity` block as defined above.
    #[builder(into)]
    #[serde(rename = "scriptUriManagedIdentity")]
    pub r#script_uri_managed_identity: Option<Box<super::super::types::compute::RunCommandSourceScriptUriManagedIdentity>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RunCommandSource {
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
                "command_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#command_id,
                )
                .await,
            );
            map.insert(
                "script".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#script,
                )
                .await,
            );
            map.insert(
                "script_uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#script_uri,
                )
                .await,
            );
            map.insert(
                "script_uri_managed_identity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#script_uri_managed_identity,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RunCommandSource {
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
                    r#command_id: {
                        let field_value = match fields_map.get("command_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'command_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#script: {
                        let field_value = match fields_map.get("script") {
                            Some(value) => value,
                            None => bail!("Missing field 'script' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#script_uri: {
                        let field_value = match fields_map.get("script_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'script_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#script_uri_managed_identity: {
                        let field_value = match fields_map.get("script_uri_managed_identity") {
                            Some(value) => value,
                            None => bail!("Missing field 'script_uri_managed_identity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
