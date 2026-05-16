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

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("api_gateways".to_string(), self.r#api_gateways.to_pulumi_value().await);
            map.insert("app_runner_services".to_string(), self.r#app_runner_services.to_pulumi_value().await);
            map.insert("cloudfronts".to_string(), self.r#cloudfronts.to_pulumi_value().await);
            map.insert("cognito_user_pools".to_string(), self.r#cognito_user_pools.to_pulumi_value().await);
            map.insert("verified_access_instances".to_string(), self.r#verified_access_instances.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WebAclAssociationConfigRequestBody {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#api_gateways: {
                        let field_value = match fields_map.get("api_gateways") {
                            Some(value) => value,
                            None => bail!("Missing field 'api_gateways' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::wafv2::WebAclAssociationConfigRequestBodyApiGateway>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#app_runner_services: {
                        let field_value = match fields_map.get("app_runner_services") {
                            Some(value) => value,
                            None => bail!("Missing field 'app_runner_services' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::wafv2::WebAclAssociationConfigRequestBodyAppRunnerService>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#cloudfronts: {
                        let field_value = match fields_map.get("cloudfronts") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudfronts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::wafv2::WebAclAssociationConfigRequestBodyCloudfront>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#cognito_user_pools: {
                        let field_value = match fields_map.get("cognito_user_pools") {
                            Some(value) => value,
                            None => bail!("Missing field 'cognito_user_pools' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::wafv2::WebAclAssociationConfigRequestBodyCognitoUserPool>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#verified_access_instances: {
                        let field_value = match fields_map.get("verified_access_instances") {
                            Some(value) => value,
                            None => bail!("Missing field 'verified_access_instances' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::wafv2::WebAclAssociationConfigRequestBodyVerifiedAccessInstance>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
