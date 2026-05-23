#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpringCloudConfigurationServiceRepository {
    /// Specifies the ID of the Certificate Authority used when retrieving the Git Repository via HTTPS.
    #[builder(into)]
    pub r#ca_certificate_id: Option<String>,
    /// Specifies the SSH public key of git repository.
    #[builder(into)]
    pub r#host_key: Option<String>,
    /// Specifies the SSH key algorithm of git repository.
    #[builder(into)]
    pub r#host_key_algorithm: Option<String>,
    /// Specifies the label of the repository.
    #[builder(into)]
    pub r#label: String,
    /// Specifies the name which should be used for this repository.
    #[builder(into)]
    pub r#name: String,
    /// Specifies the password of git repository basic auth.
    #[builder(into)]
    pub r#password: Option<String>,
    /// Specifies the collection of patterns of the repository.
    #[builder(into)]
    pub r#patterns: Vec<String>,
    /// Specifies the SSH private key of git repository.
    #[builder(into)]
    pub r#private_key: Option<String>,
    /// Specifies a list of searching path of the repository
    #[builder(into)]
    pub r#search_paths: Option<Vec<String>>,
    /// Specifies whether enable the strict host key checking.
    #[builder(into)]
    pub r#strict_host_key_checking: Option<bool>,
    /// Specifies the URI of the repository.
    #[builder(into)]
    pub r#uri: String,
    /// Specifies the username of git repository basic auth.
    #[builder(into)]
    pub r#username: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SpringCloudConfigurationServiceRepository {
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
                    "caCertificateId",
                    &self.r#ca_certificate_id,
                ),
                to_pulumi_object_field(
                    "hostKey",
                    &self.r#host_key,
                ),
                to_pulumi_object_field(
                    "hostKeyAlgorithm",
                    &self.r#host_key_algorithm,
                ),
                to_pulumi_object_field(
                    "label",
                    &self.r#label,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "password",
                    &self.r#password,
                ),
                to_pulumi_object_field(
                    "patterns",
                    &self.r#patterns,
                ),
                to_pulumi_object_field(
                    "privateKey",
                    &self.r#private_key,
                ),
                to_pulumi_object_field(
                    "searchPaths",
                    &self.r#search_paths,
                ),
                to_pulumi_object_field(
                    "strictHostKeyChecking",
                    &self.r#strict_host_key_checking,
                ),
                to_pulumi_object_field(
                    "uri",
                    &self.r#uri,
                ),
                to_pulumi_object_field(
                    "username",
                    &self.r#username,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SpringCloudConfigurationServiceRepository {
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
                    r#ca_certificate_id: {
                        let field_value = match fields_map.get("caCertificateId") {
                            Some(value) => value,
                            None => bail!("Missing field 'caCertificateId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_key: {
                        let field_value = match fields_map.get("hostKey") {
                            Some(value) => value,
                            None => bail!("Missing field 'hostKey' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_key_algorithm: {
                        let field_value = match fields_map.get("hostKeyAlgorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'hostKeyAlgorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#label: {
                        let field_value = match fields_map.get("label") {
                            Some(value) => value,
                            None => bail!("Missing field 'label' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#password: {
                        let field_value = match fields_map.get("password") {
                            Some(value) => value,
                            None => bail!("Missing field 'password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#patterns: {
                        let field_value = match fields_map.get("patterns") {
                            Some(value) => value,
                            None => bail!("Missing field 'patterns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_key: {
                        let field_value = match fields_map.get("privateKey") {
                            Some(value) => value,
                            None => bail!("Missing field 'privateKey' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#search_paths: {
                        let field_value = match fields_map.get("searchPaths") {
                            Some(value) => value,
                            None => bail!("Missing field 'searchPaths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#strict_host_key_checking: {
                        let field_value = match fields_map.get("strictHostKeyChecking") {
                            Some(value) => value,
                            None => bail!("Missing field 'strictHostKeyChecking' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#uri: {
                        let field_value = match fields_map.get("uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#username: {
                        let field_value = match fields_map.get("username") {
                            Some(value) => value,
                            None => bail!("Missing field 'username' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
