#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionBitbucketCloudConfig {
    /// Required. An access token with the `webhook`, `repository`, `repository:admin` and `pullrequest` scope access. It can be either a workspace, project or repository access token. It's recommended to use a system account to generate these credentials.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "authorizerCredential")]
    pub r#authorizer_credential: Box<super::super::types::cloudbuildv2::ConnectionBitbucketCloudConfigAuthorizerCredential>,
    /// Required. An access token with the `repository` access. It can be either a workspace, project or repository access token. It's recommended to use a system account to generate the credentials.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "readAuthorizerCredential")]
    pub r#read_authorizer_credential: Box<super::super::types::cloudbuildv2::ConnectionBitbucketCloudConfigReadAuthorizerCredential>,
    /// Required. Immutable. SecretManager resource containing the webhook secret used to verify webhook events, formatted as `projects/*/secrets/*/versions/*`.
    #[builder(into)]
    #[serde(rename = "webhookSecretSecretVersion")]
    pub r#webhook_secret_secret_version: String,
    /// The Bitbucket Cloud Workspace ID to be connected to Google Cloud Platform.
    #[builder(into)]
    #[serde(rename = "workspace")]
    pub r#workspace: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectionBitbucketCloudConfig {
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
                    "authorizer_credential",
                    &self.r#authorizer_credential,
                ),
                to_pulumi_object_field(
                    "read_authorizer_credential",
                    &self.r#read_authorizer_credential,
                ),
                to_pulumi_object_field(
                    "webhook_secret_secret_version",
                    &self.r#webhook_secret_secret_version,
                ),
                to_pulumi_object_field(
                    "workspace",
                    &self.r#workspace,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectionBitbucketCloudConfig {
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
                    r#authorizer_credential: {
                        let field_value = match fields_map.get("authorizer_credential") {
                            Some(value) => value,
                            None => bail!("Missing field 'authorizer_credential' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#read_authorizer_credential: {
                        let field_value = match fields_map.get("read_authorizer_credential") {
                            Some(value) => value,
                            None => bail!("Missing field 'read_authorizer_credential' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#webhook_secret_secret_version: {
                        let field_value = match fields_map.get("webhook_secret_secret_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'webhook_secret_secret_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#workspace: {
                        let field_value = match fields_map.get("workspace") {
                            Some(value) => value,
                            None => bail!("Missing field 'workspace' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
