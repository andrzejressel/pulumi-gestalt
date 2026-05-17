#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDirectorySamlProperty {
    #[builder(into)]
    #[serde(rename = "relayStateParameterName")]
    pub r#relay_state_parameter_name: String,
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: String,
    #[builder(into)]
    #[serde(rename = "userAccessUrl")]
    pub r#user_access_url: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetDirectorySamlProperty {
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
                "relay_state_parameter_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#relay_state_parameter_name,
                )
                .await,
            );
            map.insert(
                "status".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#status,
                )
                .await,
            );
            map.insert(
                "user_access_url".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#user_access_url,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetDirectorySamlProperty {
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
                    r#relay_state_parameter_name: {
                        let field_value = match fields_map.get("relay_state_parameter_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'relay_state_parameter_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#status: {
                        let field_value = match fields_map.get("status") {
                            Some(value) => value,
                            None => bail!("Missing field 'status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_access_url: {
                        let field_value = match fields_map.get("user_access_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_access_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
