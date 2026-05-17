#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IdentitySourceConfigurationCognitoUserPoolConfiguration {
    /// The unique application client IDs that are associated with the specified Amazon Cognito user pool.
    #[builder(into)]
    #[serde(rename = "clientIds")]
    pub r#client_ids: Option<Vec<String>>,
    /// The type of entity that a policy store maps to groups from an Amazon Cognito user pool identity source. See Group Configuration below.
    #[builder(into)]
    #[serde(rename = "groupConfiguration")]
    pub r#group_configuration: Option<Box<super::super::types::verifiedpermissions::IdentitySourceConfigurationCognitoUserPoolConfigurationGroupConfiguration>>,
    /// The Amazon Resource Name (ARN) of the Amazon Cognito user pool that contains the identities to be authorized.
    #[builder(into)]
    #[serde(rename = "userPoolArn")]
    pub r#user_pool_arn: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for IdentitySourceConfigurationCognitoUserPoolConfiguration {
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
                "client_ids".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#client_ids,
                )
                .await,
            );
            map.insert(
                "group_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#group_configuration,
                )
                .await,
            );
            map.insert(
                "user_pool_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#user_pool_arn,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for IdentitySourceConfigurationCognitoUserPoolConfiguration {
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
                    r#client_ids: {
                        let field_value = match fields_map.get("client_ids") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_ids' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#group_configuration: {
                        let field_value = match fields_map.get("group_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'group_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_pool_arn: {
                        let field_value = match fields_map.get("user_pool_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_pool_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
