#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetListenerDefaultAction {
    #[builder(into)]
    #[serde(rename = "authenticateCognitos")]
    pub r#authenticate_cognitos: Vec<super::super::types::lb::GetListenerDefaultActionAuthenticateCognito>,
    #[builder(into)]
    #[serde(rename = "authenticateOidcs")]
    pub r#authenticate_oidcs: Vec<super::super::types::lb::GetListenerDefaultActionAuthenticateOidc>,
    #[builder(into)]
    #[serde(rename = "fixedResponses")]
    pub r#fixed_responses: Vec<super::super::types::lb::GetListenerDefaultActionFixedResponse>,
    #[builder(into)]
    #[serde(rename = "forwards")]
    pub r#forwards: Vec<super::super::types::lb::GetListenerDefaultActionForward>,
    #[builder(into)]
    #[serde(rename = "order")]
    pub r#order: i32,
    #[builder(into)]
    #[serde(rename = "redirects")]
    pub r#redirects: Vec<super::super::types::lb::GetListenerDefaultActionRedirect>,
    #[builder(into)]
    #[serde(rename = "targetGroupArn")]
    pub r#target_group_arn: String,
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetListenerDefaultAction {
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
                "authenticate_cognitos".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#authenticate_cognitos,
                )
                .await,
            );
            map.insert(
                "authenticate_oidcs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#authenticate_oidcs,
                )
                .await,
            );
            map.insert(
                "fixed_responses".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fixed_responses,
                )
                .await,
            );
            map.insert(
                "forwards".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#forwards,
                )
                .await,
            );
            map.insert(
                "order".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#order,
                )
                .await,
            );
            map.insert(
                "redirects".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#redirects,
                )
                .await,
            );
            map.insert(
                "target_group_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_group_arn,
                )
                .await,
            );
            map.insert(
                "type_".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#type_,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetListenerDefaultAction {
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
                    r#authenticate_cognitos: {
                        let field_value = match fields_map.get("authenticate_cognitos") {
                            Some(value) => value,
                            None => bail!("Missing field 'authenticate_cognitos' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#authenticate_oidcs: {
                        let field_value = match fields_map.get("authenticate_oidcs") {
                            Some(value) => value,
                            None => bail!("Missing field 'authenticate_oidcs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fixed_responses: {
                        let field_value = match fields_map.get("fixed_responses") {
                            Some(value) => value,
                            None => bail!("Missing field 'fixed_responses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#forwards: {
                        let field_value = match fields_map.get("forwards") {
                            Some(value) => value,
                            None => bail!("Missing field 'forwards' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#order: {
                        let field_value = match fields_map.get("order") {
                            Some(value) => value,
                            None => bail!("Missing field 'order' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redirects: {
                        let field_value = match fields_map.get("redirects") {
                            Some(value) => value,
                            None => bail!("Missing field 'redirects' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_group_arn: {
                        let field_value = match fields_map.get("target_group_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_group_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
