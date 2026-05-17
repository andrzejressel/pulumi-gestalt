#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WebAclAssociationConfigRequestBody {
    /// Customizes the request body that your protected Amazon API Gateway REST APIs forward to AWS WAF for inspection. Applicable only when `scope` is set to `CLOUDFRONT`. See `api_gateway` below for details.
    #[builder(into)]
    #[serde(rename = "apiGateways")]
    pub r#api_gateways: Option<Vec<super::super::types::wafv2::WebAclAssociationConfigRequestBodyApiGateway>>,
    /// Customizes the request body that your protected Amazon App Runner services forward to AWS WAF for inspection. Applicable only when `scope` is set to `REGIONAL`. See `app_runner_service` below for details.
    #[builder(into)]
    #[serde(rename = "appRunnerServices")]
    pub r#app_runner_services: Option<Vec<super::super::types::wafv2::WebAclAssociationConfigRequestBodyAppRunnerService>>,
    /// Customizes the request body that your protected Amazon CloudFront distributions forward to AWS WAF for inspection. Applicable only when `scope` is set to `REGIONAL`. See `cloudfront` below for details.
    #[builder(into)]
    #[serde(rename = "cloudfronts")]
    pub r#cloudfronts: Option<Vec<super::super::types::wafv2::WebAclAssociationConfigRequestBodyCloudfront>>,
    /// Customizes the request body that your protected Amazon Cognito user pools forward to AWS WAF for inspection. Applicable only when `scope` is set to `REGIONAL`. See `cognito_user_pool` below for details.
    #[builder(into)]
    #[serde(rename = "cognitoUserPools")]
    pub r#cognito_user_pools: Option<Vec<super::super::types::wafv2::WebAclAssociationConfigRequestBodyCognitoUserPool>>,
    /// Customizes the request body that your protected AWS Verfied Access instances forward to AWS WAF for inspection. Applicable only when `scope` is set to `REGIONAL`. See `verified_access_instance` below for details.
    #[builder(into)]
    #[serde(rename = "verifiedAccessInstances")]
    pub r#verified_access_instances: Option<Vec<super::super::types::wafv2::WebAclAssociationConfigRequestBodyVerifiedAccessInstance>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WebAclAssociationConfigRequestBody {
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
                "api_gateways".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#api_gateways,
                )
                .await,
            );
            map.insert(
                "app_runner_services".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#app_runner_services,
                )
                .await,
            );
            map.insert(
                "cloudfronts".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cloudfronts,
                )
                .await,
            );
            map.insert(
                "cognito_user_pools".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cognito_user_pools,
                )
                .await,
            );
            map.insert(
                "verified_access_instances".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#verified_access_instances,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WebAclAssociationConfigRequestBody {
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
                    r#api_gateways: {
                        let field_value = match fields_map.get("api_gateways") {
                            Some(value) => value,
                            None => bail!("Missing field 'api_gateways' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#app_runner_services: {
                        let field_value = match fields_map.get("app_runner_services") {
                            Some(value) => value,
                            None => bail!("Missing field 'app_runner_services' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloudfronts: {
                        let field_value = match fields_map.get("cloudfronts") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudfronts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cognito_user_pools: {
                        let field_value = match fields_map.get("cognito_user_pools") {
                            Some(value) => value,
                            None => bail!("Missing field 'cognito_user_pools' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#verified_access_instances: {
                        let field_value = match fields_map.get("verified_access_instances") {
                            Some(value) => value,
                            None => bail!("Missing field 'verified_access_instances' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
