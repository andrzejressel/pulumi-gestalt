#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsInforNexus {
    /// The Access Key portion of the credentials.
    #[builder(into)]
    #[serde(rename = "accessKeyId")]
    pub r#access_key_id: String,
    /// Encryption keys used to encrypt data.
    #[builder(into)]
    #[serde(rename = "datakey")]
    pub r#datakey: String,
    /// The secret key used to sign requests.
    #[builder(into)]
    #[serde(rename = "secretAccessKey")]
    pub r#secret_access_key: String,
    /// Identifier for the user.
    #[builder(into)]
    #[serde(rename = "userId")]
    pub r#user_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsInforNexus {
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
                    "access_key_id",
                    &self.r#access_key_id,
                ),
                to_pulumi_object_field(
                    "datakey",
                    &self.r#datakey,
                ),
                to_pulumi_object_field(
                    "secret_access_key",
                    &self.r#secret_access_key,
                ),
                to_pulumi_object_field(
                    "user_id",
                    &self.r#user_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsInforNexus {
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
                    r#access_key_id: {
                        let field_value = match fields_map.get("access_key_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_key_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#datakey: {
                        let field_value = match fields_map.get("datakey") {
                            Some(value) => value,
                            None => bail!("Missing field 'datakey' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret_access_key: {
                        let field_value = match fields_map.get("secret_access_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'secret_access_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_id: {
                        let field_value = match fields_map.get("user_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
