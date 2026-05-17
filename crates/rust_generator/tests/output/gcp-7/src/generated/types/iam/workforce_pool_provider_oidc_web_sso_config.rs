#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkforcePoolProviderOidcWebSsoConfig {
    /// Additional scopes to request for in the OIDC authentication request on top of scopes requested by default. By default, the `openid`, `profile` and `email` scopes that are supported by the identity provider are requested.
    /// Each additional scope may be at most 256 characters. A maximum of 10 additional scopes may be configured.
    /// 
    /// <a name="nested_extra_attributes_oauth2_client"></a>The `extra_attributes_oauth2_client` block supports:
    #[builder(into)]
    #[serde(rename = "additionalScopes")]
    pub r#additional_scopes: Option<Vec<String>>,
    /// The behavior for how OIDC Claims are included in the `assertion` object used for attribute mapping and attribute condition.
    /// * MERGE_USER_INFO_OVER_ID_TOKEN_CLAIMS: Merge the UserInfo Endpoint Claims with ID Token Claims, preferring UserInfo Claim Values for the same Claim Name. This option is available only for the Authorization Code Flow.
    /// * ONLY_ID_TOKEN_CLAIMS: Only include ID Token Claims.
    /// Possible values are: `MERGE_USER_INFO_OVER_ID_TOKEN_CLAIMS`, `ONLY_ID_TOKEN_CLAIMS`.
    #[builder(into)]
    #[serde(rename = "assertionClaimsBehavior")]
    pub r#assertion_claims_behavior: String,
    /// The Response Type to request for in the OIDC Authorization Request for web sign-in.
    /// The `CODE` Response Type is recommended to avoid the Implicit Flow, for security reasons.
    /// * CODE: The `response_type=code` selection uses the Authorization Code Flow for web sign-in. Requires a configured client secret.
    /// * ID_TOKEN: The `response_type=id_token` selection uses the Implicit Flow for web sign-in.
    /// Possible values are: `CODE`, `ID_TOKEN`.
    #[builder(into)]
    #[serde(rename = "responseType")]
    pub r#response_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WorkforcePoolProviderOidcWebSsoConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "additional_scopes",
                    &self.r#additional_scopes,
                ),
                to_pulumi_object_field(
                    "assertion_claims_behavior",
                    &self.r#assertion_claims_behavior,
                ),
                to_pulumi_object_field(
                    "response_type",
                    &self.r#response_type,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WorkforcePoolProviderOidcWebSsoConfig {
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
                    r#additional_scopes: {
                        let field_value = match fields_map.get("additional_scopes") {
                            Some(value) => value,
                            None => bail!("Missing field 'additional_scopes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#assertion_claims_behavior: {
                        let field_value = match fields_map.get("assertion_claims_behavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'assertion_claims_behavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#response_type: {
                        let field_value = match fields_map.get("response_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'response_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
