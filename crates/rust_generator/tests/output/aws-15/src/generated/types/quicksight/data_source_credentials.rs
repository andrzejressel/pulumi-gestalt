#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataSourceCredentials {
    /// The Amazon Resource Name (ARN) of a data source that has the credential pair that you want to use.
    /// When the value is not null, the `credential_pair` from the data source in the ARN is used.
    #[builder(into)]
    #[serde(rename = "copySourceArn")]
    pub r#copy_source_arn: Option<String>,
    /// Credential pair. See Credential Pair below for more details.
    #[builder(into)]
    #[serde(rename = "credentialPair")]
    pub r#credential_pair: Option<Box<super::super::types::quicksight::DataSourceCredentialsCredentialPair>>,
    /// The Amazon Resource Name (ARN) of the secret associated with the data source in Amazon Secrets Manager.
    #[builder(into)]
    #[serde(rename = "secretArn")]
    pub r#secret_arn: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataSourceCredentials {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("copy_source_arn".to_string(), self.r#copy_source_arn.to_pulumi_value().await);
            map.insert("credential_pair".to_string(), self.r#credential_pair.to_pulumi_value().await);
            map.insert("secret_arn".to_string(), self.r#secret_arn.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataSourceCredentials {
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
                    r#copy_source_arn: {
                        let field_value = match fields_map.get("copy_source_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'copy_source_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#credential_pair: {
                        let field_value = match fields_map.get("credential_pair") {
                            Some(value) => value,
                            None => bail!("Missing field 'credential_pair' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::quicksight::DataSourceCredentialsCredentialPair>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#secret_arn: {
                        let field_value = match fields_map.get("secret_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'secret_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
