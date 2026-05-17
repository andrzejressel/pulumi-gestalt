#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BitbucketServerConfigSecrets {
    /// The resource name for the admin access token's secret version.
    #[builder(into)]
    #[serde(rename = "adminAccessTokenVersionName")]
    pub r#admin_access_token_version_name: String,
    /// The resource name for the read access token's secret version.
    #[builder(into)]
    #[serde(rename = "readAccessTokenVersionName")]
    pub r#read_access_token_version_name: String,
    /// Immutable. The resource name for the webhook secret's secret version. Once this field has been set, it cannot be changed.
    /// Changing this field will result in deleting/ recreating the resource.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "webhookSecretVersionName")]
    pub r#webhook_secret_version_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BitbucketServerConfigSecrets {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "admin_access_token_version_name",
                    &self.r#admin_access_token_version_name,
                ),
                to_pulumi_object_field(
                    "read_access_token_version_name",
                    &self.r#read_access_token_version_name,
                ),
                to_pulumi_object_field(
                    "webhook_secret_version_name",
                    &self.r#webhook_secret_version_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BitbucketServerConfigSecrets {
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
                    r#admin_access_token_version_name: {
                        let field_value = match fields_map.get("admin_access_token_version_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'admin_access_token_version_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#read_access_token_version_name: {
                        let field_value = match fields_map.get("read_access_token_version_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'read_access_token_version_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#webhook_secret_version_name: {
                        let field_value = match fields_map.get("webhook_secret_version_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'webhook_secret_version_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
