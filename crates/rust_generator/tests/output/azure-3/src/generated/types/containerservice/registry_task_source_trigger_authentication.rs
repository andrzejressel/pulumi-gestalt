#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegistryTaskSourceTriggerAuthentication {
    /// Time in seconds that the token remains valid.
    #[builder(into)]
    #[serde(rename = "expireInSeconds")]
    pub r#expire_in_seconds: Option<i32>,
    /// The refresh token used to refresh the access token.
    #[builder(into)]
    #[serde(rename = "refreshToken")]
    pub r#refresh_token: Option<String>,
    /// The scope of the access token.
    #[builder(into)]
    #[serde(rename = "scope")]
    pub r#scope: Option<String>,
    /// The access token used to access the source control provider.
    #[builder(into)]
    #[serde(rename = "token")]
    pub r#token: String,
    /// The type of the token. Possible values are `PAT` (personal access token) and `OAuth`.
    #[builder(into)]
    #[serde(rename = "tokenType")]
    pub r#token_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegistryTaskSourceTriggerAuthentication {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "expire_in_seconds",
                    &self.r#expire_in_seconds,
                ),
                to_pulumi_object_field(
                    "refresh_token",
                    &self.r#refresh_token,
                ),
                to_pulumi_object_field(
                    "scope",
                    &self.r#scope,
                ),
                to_pulumi_object_field(
                    "token",
                    &self.r#token,
                ),
                to_pulumi_object_field(
                    "token_type",
                    &self.r#token_type,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegistryTaskSourceTriggerAuthentication {
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
                    r#expire_in_seconds: {
                        let field_value = match fields_map.get("expire_in_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'expire_in_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#refresh_token: {
                        let field_value = match fields_map.get("refresh_token") {
                            Some(value) => value,
                            None => bail!("Missing field 'refresh_token' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scope: {
                        let field_value = match fields_map.get("scope") {
                            Some(value) => value,
                            None => bail!("Missing field 'scope' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#token: {
                        let field_value = match fields_map.get("token") {
                            Some(value) => value,
                            None => bail!("Missing field 'token' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#token_type: {
                        let field_value = match fields_map.get("token_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'token_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
