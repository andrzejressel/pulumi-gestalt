#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccessGroupRequire {
    /// Matches any valid Access service token.
    #[builder(into)]
    #[serde(rename = "anyValidServiceToken")]
    pub r#any_valid_service_token: Option<bool>,
    #[builder(into)]
    #[serde(rename = "authContexts")]
    pub r#auth_contexts: Option<Vec<super::types::AccessGroupRequireAuthContext>>,
    /// The type of authentication method. Refer to https://datatracker.ietf.org/doc/html/rfc8176#section-2 for possible types.
    #[builder(into)]
    #[serde(rename = "authMethod")]
    pub r#auth_method: Option<String>,
    /// Matches an Azure group. Requires an Azure identity provider.
    #[builder(into)]
    #[serde(rename = "azures")]
    pub r#azures: Option<Vec<super::types::AccessGroupRequireAzure>>,
    /// Matches any valid client certificate.
    #[builder(into)]
    #[serde(rename = "certificate")]
    pub r#certificate: Option<bool>,
    /// Matches a valid client certificate common name.
    #[builder(into)]
    #[serde(rename = "commonName")]
    pub r#common_name: Option<String>,
    /// Overflow field if you need to have multiple common*name rules in a single policy.  Use in place of the singular common*name field.
    #[builder(into)]
    #[serde(rename = "commonNames")]
    pub r#common_names: Option<Vec<String>>,
    /// The ID of a device posture integration.
    #[builder(into)]
    #[serde(rename = "devicePostures")]
    pub r#device_postures: Option<Vec<String>>,
    /// The email domain to match.
    #[builder(into)]
    #[serde(rename = "emailDomains")]
    pub r#email_domains: Option<Vec<String>>,
    /// The ID of a previously created email list.
    #[builder(into)]
    #[serde(rename = "emailLists")]
    pub r#email_lists: Option<Vec<String>>,
    /// The email of the user.
    #[builder(into)]
    #[serde(rename = "emails")]
    pub r#emails: Option<Vec<String>>,
    /// Matches everyone.
    #[builder(into)]
    #[serde(rename = "everyone")]
    pub r#everyone: Option<bool>,
    /// Create Allow or Block policies which evaluate the user based on custom criteria. https://developers.cloudflare.com/cloudflare-one/policies/access/external-evaluation/.
    #[builder(into)]
    #[serde(rename = "externalEvaluation")]
    pub r#external_evaluation: Option<Box<super::types::AccessGroupRequireExternalEvaluation>>,
    /// Matches a specific country.
    #[builder(into)]
    #[serde(rename = "geos")]
    pub r#geos: Option<Vec<String>>,
    /// Matches a Github organization. Requires a Github identity provider.
    #[builder(into)]
    #[serde(rename = "githubs")]
    pub r#githubs: Option<Vec<super::types::AccessGroupRequireGithub>>,
    /// The ID of a previously created Access group.
    #[builder(into)]
    #[serde(rename = "groups")]
    pub r#groups: Option<Vec<String>>,
    /// Matches a group in Google Workspace. Requires a Google Workspace identity provider.
    #[builder(into)]
    #[serde(rename = "gsuites")]
    pub r#gsuites: Option<Vec<super::types::AccessGroupRequireGsuite>>,
    /// The ID of a previously created IP list.
    #[builder(into)]
    #[serde(rename = "ipLists")]
    pub r#ip_lists: Option<Vec<String>>,
    /// An IPv4 or IPv6 CIDR block.
    #[builder(into)]
    #[serde(rename = "ips")]
    pub r#ips: Option<Vec<String>>,
    /// The ID of a configured identity provider.
    #[builder(into)]
    #[serde(rename = "loginMethods")]
    pub r#login_methods: Option<Vec<String>>,
    /// Matches an Okta group. Requires an Okta identity provider.
    #[builder(into)]
    #[serde(rename = "oktas")]
    pub r#oktas: Option<Vec<super::types::AccessGroupRequireOkta>>,
    /// Matches a SAML group. Requires a SAML identity provider.
    #[builder(into)]
    #[serde(rename = "samls")]
    pub r#samls: Option<Vec<super::types::AccessGroupRequireSaml>>,
    /// The ID of an Access service token.
    #[builder(into)]
    #[serde(rename = "serviceTokens")]
    pub r#service_tokens: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AccessGroupRequire {
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
                "any_valid_service_token".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#any_valid_service_token,
                )
                .await,
            );
            map.insert(
                "auth_contexts".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#auth_contexts,
                )
                .await,
            );
            map.insert(
                "auth_method".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#auth_method,
                )
                .await,
            );
            map.insert(
                "azures".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#azures,
                )
                .await,
            );
            map.insert(
                "certificate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#certificate,
                )
                .await,
            );
            map.insert(
                "common_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#common_name,
                )
                .await,
            );
            map.insert(
                "common_names".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#common_names,
                )
                .await,
            );
            map.insert(
                "device_postures".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#device_postures,
                )
                .await,
            );
            map.insert(
                "email_domains".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#email_domains,
                )
                .await,
            );
            map.insert(
                "email_lists".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#email_lists,
                )
                .await,
            );
            map.insert(
                "emails".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#emails,
                )
                .await,
            );
            map.insert(
                "everyone".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#everyone,
                )
                .await,
            );
            map.insert(
                "external_evaluation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#external_evaluation,
                )
                .await,
            );
            map.insert(
                "geos".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#geos,
                )
                .await,
            );
            map.insert(
                "githubs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#githubs,
                )
                .await,
            );
            map.insert(
                "groups".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#groups,
                )
                .await,
            );
            map.insert(
                "gsuites".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gsuites,
                )
                .await,
            );
            map.insert(
                "ip_lists".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_lists,
                )
                .await,
            );
            map.insert(
                "ips".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ips,
                )
                .await,
            );
            map.insert(
                "login_methods".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#login_methods,
                )
                .await,
            );
            map.insert(
                "oktas".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#oktas,
                )
                .await,
            );
            map.insert(
                "samls".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#samls,
                )
                .await,
            );
            map.insert(
                "service_tokens".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service_tokens,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AccessGroupRequire {
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
                    r#any_valid_service_token: {
                        let field_value = match fields_map.get("any_valid_service_token") {
                            Some(value) => value,
                            None => bail!("Missing field 'any_valid_service_token' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auth_contexts: {
                        let field_value = match fields_map.get("auth_contexts") {
                            Some(value) => value,
                            None => bail!("Missing field 'auth_contexts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auth_method: {
                        let field_value = match fields_map.get("auth_method") {
                            Some(value) => value,
                            None => bail!("Missing field 'auth_method' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#azures: {
                        let field_value = match fields_map.get("azures") {
                            Some(value) => value,
                            None => bail!("Missing field 'azures' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#certificate: {
                        let field_value = match fields_map.get("certificate") {
                            Some(value) => value,
                            None => bail!("Missing field 'certificate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#common_name: {
                        let field_value = match fields_map.get("common_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'common_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#common_names: {
                        let field_value = match fields_map.get("common_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'common_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#device_postures: {
                        let field_value = match fields_map.get("device_postures") {
                            Some(value) => value,
                            None => bail!("Missing field 'device_postures' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#email_domains: {
                        let field_value = match fields_map.get("email_domains") {
                            Some(value) => value,
                            None => bail!("Missing field 'email_domains' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#email_lists: {
                        let field_value = match fields_map.get("email_lists") {
                            Some(value) => value,
                            None => bail!("Missing field 'email_lists' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#emails: {
                        let field_value = match fields_map.get("emails") {
                            Some(value) => value,
                            None => bail!("Missing field 'emails' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#everyone: {
                        let field_value = match fields_map.get("everyone") {
                            Some(value) => value,
                            None => bail!("Missing field 'everyone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#external_evaluation: {
                        let field_value = match fields_map.get("external_evaluation") {
                            Some(value) => value,
                            None => bail!("Missing field 'external_evaluation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#geos: {
                        let field_value = match fields_map.get("geos") {
                            Some(value) => value,
                            None => bail!("Missing field 'geos' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#githubs: {
                        let field_value = match fields_map.get("githubs") {
                            Some(value) => value,
                            None => bail!("Missing field 'githubs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#groups: {
                        let field_value = match fields_map.get("groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gsuites: {
                        let field_value = match fields_map.get("gsuites") {
                            Some(value) => value,
                            None => bail!("Missing field 'gsuites' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_lists: {
                        let field_value = match fields_map.get("ip_lists") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_lists' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ips: {
                        let field_value = match fields_map.get("ips") {
                            Some(value) => value,
                            None => bail!("Missing field 'ips' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#login_methods: {
                        let field_value = match fields_map.get("login_methods") {
                            Some(value) => value,
                            None => bail!("Missing field 'login_methods' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oktas: {
                        let field_value = match fields_map.get("oktas") {
                            Some(value) => value,
                            None => bail!("Missing field 'oktas' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#samls: {
                        let field_value = match fields_map.get("samls") {
                            Some(value) => value,
                            None => bail!("Missing field 'samls' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_tokens: {
                        let field_value = match fields_map.get("service_tokens") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_tokens' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
