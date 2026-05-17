#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpringCloudConfigurationServiceRepository {
    /// Specifies the ID of the Certificate Authority used when retrieving the Git Repository via HTTPS.
    #[builder(into)]
    #[serde(rename = "caCertificateId")]
    pub r#ca_certificate_id: Option<String>,
    /// Specifies the SSH public key of git repository.
    #[builder(into)]
    #[serde(rename = "hostKey")]
    pub r#host_key: Option<String>,
    /// Specifies the SSH key algorithm of git repository.
    #[builder(into)]
    #[serde(rename = "hostKeyAlgorithm")]
    pub r#host_key_algorithm: Option<String>,
    /// Specifies the label of the repository.
    #[builder(into)]
    #[serde(rename = "label")]
    pub r#label: String,
    /// Specifies the name which should be used for this repository.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Specifies the password of git repository basic auth.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    /// Specifies the collection of patterns of the repository.
    #[builder(into)]
    #[serde(rename = "patterns")]
    pub r#patterns: Vec<String>,
    /// Specifies the SSH private key of git repository.
    #[builder(into)]
    #[serde(rename = "privateKey")]
    pub r#private_key: Option<String>,
    /// Specifies a list of searching path of the repository
    #[builder(into)]
    #[serde(rename = "searchPaths")]
    pub r#search_paths: Option<Vec<String>>,
    /// Specifies whether enable the strict host key checking.
    #[builder(into)]
    #[serde(rename = "strictHostKeyChecking")]
    pub r#strict_host_key_checking: Option<bool>,
    /// Specifies the URI of the repository.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: String,
    /// Specifies the username of git repository basic auth.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SpringCloudConfigurationServiceRepository {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "ca_certificate_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ca_certificate_id,
                )
                .await,
            );
            map.insert(
                "host_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#host_key,
                )
                .await,
            );
            map.insert(
                "host_key_algorithm".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#host_key_algorithm,
                )
                .await,
            );
            map.insert(
                "label".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#label,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "password".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#password,
                )
                .await,
            );
            map.insert(
                "patterns".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#patterns,
                )
                .await,
            );
            map.insert(
                "private_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#private_key,
                )
                .await,
            );
            map.insert(
                "search_paths".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#search_paths,
                )
                .await,
            );
            map.insert(
                "strict_host_key_checking".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#strict_host_key_checking,
                )
                .await,
            );
            map.insert(
                "uri".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#uri,
                )
                .await,
            );
            map.insert(
                "username".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#username,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
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
                        let field_value = match fields_map.get("ca_certificate_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'ca_certificate_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_key: {
                        let field_value = match fields_map.get("host_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_key_algorithm: {
                        let field_value = match fields_map.get("host_key_algorithm") {
                            Some(value) => value,
                            None => bail!("Missing field 'host_key_algorithm' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("private_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#search_paths: {
                        let field_value = match fields_map.get("search_paths") {
                            Some(value) => value,
                            None => bail!("Missing field 'search_paths' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#strict_host_key_checking: {
                        let field_value = match fields_map.get("strict_host_key_checking") {
                            Some(value) => value,
                            None => bail!("Missing field 'strict_host_key_checking' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
