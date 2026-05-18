#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesRedshift {
    #[builder(into)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: String,
    #[builder(into)]
    #[serde(rename = "bucketPrefix")]
    pub r#bucket_prefix: Option<String>,
    /// The unique ID that's assigned to an Amazon Redshift cluster.
    #[builder(into)]
    #[serde(rename = "clusterIdentifier")]
    pub r#cluster_identifier: Option<String>,
    /// ARN of the IAM role that permits AppFlow to access the database through Data API.
    #[builder(into)]
    #[serde(rename = "dataApiRoleArn")]
    pub r#data_api_role_arn: Option<String>,
    /// The name of an Amazon Redshift database.
    #[builder(into)]
    #[serde(rename = "databaseName")]
    pub r#database_name: Option<String>,
    /// The JDBC URL of the Amazon Redshift cluster.
    #[builder(into)]
    #[serde(rename = "databaseUrl")]
    pub r#database_url: Option<String>,
    /// ARN of the IAM role.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesRedshift {
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
                    "bucket_name",
                    &self.r#bucket_name,
                ),
                to_pulumi_object_field(
                    "bucket_prefix",
                    &self.r#bucket_prefix,
                ),
                to_pulumi_object_field(
                    "cluster_identifier",
                    &self.r#cluster_identifier,
                ),
                to_pulumi_object_field(
                    "data_api_role_arn",
                    &self.r#data_api_role_arn,
                ),
                to_pulumi_object_field(
                    "database_name",
                    &self.r#database_name,
                ),
                to_pulumi_object_field(
                    "database_url",
                    &self.r#database_url,
                ),
                to_pulumi_object_field(
                    "role_arn",
                    &self.r#role_arn,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesRedshift {
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
                    r#bucket_name: {
                        let field_value = match fields_map.get("bucket_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucket_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bucket_prefix: {
                        let field_value = match fields_map.get("bucket_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucket_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cluster_identifier: {
                        let field_value = match fields_map.get("cluster_identifier") {
                            Some(value) => value,
                            None => bail!("Missing field 'cluster_identifier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#data_api_role_arn: {
                        let field_value = match fields_map.get("data_api_role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_api_role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#database_name: {
                        let field_value = match fields_map.get("database_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'database_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#database_url: {
                        let field_value = match fields_map.get("database_url") {
                            Some(value) => value,
                            None => bail!("Missing field 'database_url' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#role_arn: {
                        let field_value = match fields_map.get("role_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'role_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
