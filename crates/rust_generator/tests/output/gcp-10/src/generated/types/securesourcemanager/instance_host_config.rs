#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceHostConfig {
    /// (Output)
    /// API hostname.
    #[builder(into)]
    #[serde(rename = "api")]
    pub r#api: Option<String>,
    /// (Output)
    /// Git HTTP hostname.
    #[builder(into)]
    #[serde(rename = "gitHttp")]
    pub r#git_http: Option<String>,
    /// (Output)
    /// Git SSH hostname.
    #[builder(into)]
    #[serde(rename = "gitSsh")]
    pub r#git_ssh: Option<String>,
    /// (Output)
    /// HTML hostname.
    #[builder(into)]
    #[serde(rename = "html")]
    pub r#html: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceHostConfig {
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
                "api".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#api,
                )
                .await,
            );
            map.insert(
                "git_http".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#git_http,
                )
                .await,
            );
            map.insert(
                "git_ssh".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#git_ssh,
                )
                .await,
            );
            map.insert(
                "html".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#html,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceHostConfig {
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
                    r#api: {
                        let field_value = match fields_map.get("api") {
                            Some(value) => value,
                            None => bail!("Missing field 'api' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#git_http: {
                        let field_value = match fields_map.get("git_http") {
                            Some(value) => value,
                            None => bail!("Missing field 'git_http' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#git_ssh: {
                        let field_value = match fields_map.get("git_ssh") {
                            Some(value) => value,
                            None => bail!("Missing field 'git_ssh' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#html: {
                        let field_value = match fields_map.get("html") {
                            Some(value) => value,
                            None => bail!("Missing field 'html' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
