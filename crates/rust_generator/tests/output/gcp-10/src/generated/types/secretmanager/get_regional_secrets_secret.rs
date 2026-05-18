#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRegionalSecretsSecret {
    /// Custom metadata about the regional secret.
    #[builder(into)]
    #[serde(rename = "annotations")]
    pub r#annotations: std::collections::HashMap<String, String>,
    /// The time at which the regional secret was created.
    #[builder(into)]
    #[serde(rename = "createTime")]
    pub r#create_time: String,
    /// Customer Managed Encryption for the regional secret.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "customerManagedEncryptions")]
    pub r#customer_managed_encryptions: Vec<super::super::types::secretmanager::GetRegionalSecretsSecretCustomerManagedEncryption>,
    #[builder(into)]
    #[serde(rename = "effectiveAnnotations")]
    pub r#effective_annotations: std::collections::HashMap<String, String>,
    #[builder(into)]
    #[serde(rename = "effectiveLabels")]
    pub r#effective_labels: std::collections::HashMap<String, String>,
    /// Timestamp in UTC when the regional secret is scheduled to expire.
    #[builder(into)]
    #[serde(rename = "expireTime")]
    pub r#expire_time: String,
    /// The labels assigned to this regional secret.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: std::collections::HashMap<String, String>,
    /// The location of the regional secret.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: String,
    /// The resource name of the Pub/Sub topic that will be published to.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The ID of the project.
    #[builder(into)]
    #[serde(rename = "project")]
    pub r#project: String,
    /// The combination of labels configured directly on the resource
    ///  and default labels configured on the provider.
    #[builder(into)]
    #[serde(rename = "pulumiLabels")]
    pub r#pulumi_labels: std::collections::HashMap<String, String>,
    /// The rotation time and period for a regional secret.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "rotations")]
    pub r#rotations: Vec<super::super::types::secretmanager::GetRegionalSecretsSecretRotation>,
    /// The unique name of the resource.
    #[builder(into)]
    #[serde(rename = "secretId")]
    pub r#secret_id: String,
    /// A list of up to 10 Pub/Sub topics to which messages are published when control plane operations are called on the regional secret or its versions.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "topics")]
    pub r#topics: Vec<super::super::types::secretmanager::GetRegionalSecretsSecretTopic>,
    /// The TTL for the regional secret. A duration in seconds with up to nine fractional digits,
    /// terminated by 's'. Example: "3.5s". Only one of 'ttl' or 'expire_time' can be provided.
    #[builder(into)]
    #[serde(rename = "ttl")]
    pub r#ttl: String,
    /// Mapping from version alias to version name.
    #[builder(into)]
    #[serde(rename = "versionAliases")]
    pub r#version_aliases: std::collections::HashMap<String, String>,
    /// The version destroy ttl for the regional secret version.
    #[builder(into)]
    #[serde(rename = "versionDestroyTtl")]
    pub r#version_destroy_ttl: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetRegionalSecretsSecret {
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
                    "annotations",
                    &self.r#annotations,
                ),
                to_pulumi_object_field(
                    "create_time",
                    &self.r#create_time,
                ),
                to_pulumi_object_field(
                    "customer_managed_encryptions",
                    &self.r#customer_managed_encryptions,
                ),
                to_pulumi_object_field(
                    "effective_annotations",
                    &self.r#effective_annotations,
                ),
                to_pulumi_object_field(
                    "effective_labels",
                    &self.r#effective_labels,
                ),
                to_pulumi_object_field(
                    "expire_time",
                    &self.r#expire_time,
                ),
                to_pulumi_object_field(
                    "labels",
                    &self.r#labels,
                ),
                to_pulumi_object_field(
                    "location",
                    &self.r#location,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "project",
                    &self.r#project,
                ),
                to_pulumi_object_field(
                    "pulumi_labels",
                    &self.r#pulumi_labels,
                ),
                to_pulumi_object_field(
                    "rotations",
                    &self.r#rotations,
                ),
                to_pulumi_object_field(
                    "secret_id",
                    &self.r#secret_id,
                ),
                to_pulumi_object_field(
                    "topics",
                    &self.r#topics,
                ),
                to_pulumi_object_field(
                    "ttl",
                    &self.r#ttl,
                ),
                to_pulumi_object_field(
                    "version_aliases",
                    &self.r#version_aliases,
                ),
                to_pulumi_object_field(
                    "version_destroy_ttl",
                    &self.r#version_destroy_ttl,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetRegionalSecretsSecret {
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
                    r#annotations: {
                        let field_value = match fields_map.get("annotations") {
                            Some(value) => value,
                            None => bail!("Missing field 'annotations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#create_time: {
                        let field_value = match fields_map.get("create_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'create_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#customer_managed_encryptions: {
                        let field_value = match fields_map.get("customer_managed_encryptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'customer_managed_encryptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#effective_annotations: {
                        let field_value = match fields_map.get("effective_annotations") {
                            Some(value) => value,
                            None => bail!("Missing field 'effective_annotations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#effective_labels: {
                        let field_value = match fields_map.get("effective_labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'effective_labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#expire_time: {
                        let field_value = match fields_map.get("expire_time") {
                            Some(value) => value,
                            None => bail!("Missing field 'expire_time' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#labels: {
                        let field_value = match fields_map.get("labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#location: {
                        let field_value = match fields_map.get("location") {
                            Some(value) => value,
                            None => bail!("Missing field 'location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#project: {
                        let field_value = match fields_map.get("project") {
                            Some(value) => value,
                            None => bail!("Missing field 'project' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pulumi_labels: {
                        let field_value = match fields_map.get("pulumi_labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'pulumi_labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rotations: {
                        let field_value = match fields_map.get("rotations") {
                            Some(value) => value,
                            None => bail!("Missing field 'rotations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret_id: {
                        let field_value = match fields_map.get("secret_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'secret_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#topics: {
                        let field_value = match fields_map.get("topics") {
                            Some(value) => value,
                            None => bail!("Missing field 'topics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ttl: {
                        let field_value = match fields_map.get("ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#version_aliases: {
                        let field_value = match fields_map.get("version_aliases") {
                            Some(value) => value,
                            None => bail!("Missing field 'version_aliases' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#version_destroy_ttl: {
                        let field_value = match fields_map.get("version_destroy_ttl") {
                            Some(value) => value,
                            None => bail!("Missing field 'version_destroy_ttl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
