#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccessPolicyRequire {
    /// Matches any valid Access service token.
    #[builder(into)]
    #[serde(rename = "anyValidServiceToken")]
    pub r#any_valid_service_token: Option<bool>,
    #[builder(into)]
    #[serde(rename = "authContexts")]
    pub r#auth_contexts: Option<Vec<super::types::AccessPolicyRequireAuthContext>>,
    /// The type of authentication method. Refer to https://datatracker.ietf.org/doc/html/rfc8176#section-2 for possible types.
    #[builder(into)]
    #[serde(rename = "authMethod")]
    pub r#auth_method: Option<String>,
    /// Matches an Azure group. Requires an Azure identity provider.
    #[builder(into)]
    #[serde(rename = "azures")]
    pub r#azures: Option<Vec<super::types::AccessPolicyRequireAzure>>,
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
    pub r#external_evaluation: Option<Box<super::types::AccessPolicyRequireExternalEvaluation>>,
    /// Matches a specific country.
    #[builder(into)]
    #[serde(rename = "geos")]
    pub r#geos: Option<Vec<String>>,
    /// Matches a Github organization. Requires a Github identity provider.
    #[builder(into)]
    #[serde(rename = "githubs")]
    pub r#githubs: Option<Vec<super::types::AccessPolicyRequireGithub>>,
    /// The ID of a previously created Access group.
    #[builder(into)]
    #[serde(rename = "groups")]
    pub r#groups: Option<Vec<String>>,
    /// Matches a group in Google Workspace. Requires a Google Workspace identity provider.
    #[builder(into)]
    #[serde(rename = "gsuites")]
    pub r#gsuites: Option<Vec<super::types::AccessPolicyRequireGsuite>>,
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
    pub r#oktas: Option<Vec<super::types::AccessPolicyRequireOkta>>,
    /// Matches a SAML group. Requires a SAML identity provider.
    #[builder(into)]
    #[serde(rename = "samls")]
    pub r#samls: Option<Vec<super::types::AccessPolicyRequireSaml>>,
    /// The ID of an Access service token.
    #[builder(into)]
    #[serde(rename = "serviceTokens")]
    pub r#service_tokens: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AccessPolicyRequire {
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
                    "any_valid_service_token",
                    &self.r#any_valid_service_token,
                ),
                to_pulumi_object_field(
                    "auth_contexts",
                    &self.r#auth_contexts,
                ),
                to_pulumi_object_field(
                    "auth_method",
                    &self.r#auth_method,
                ),
                to_pulumi_object_field(
                    "azures",
                    &self.r#azures,
                ),
                to_pulumi_object_field(
                    "certificate",
                    &self.r#certificate,
                ),
                to_pulumi_object_field(
                    "common_name",
                    &self.r#common_name,
                ),
                to_pulumi_object_field(
                    "common_names",
                    &self.r#common_names,
                ),
                to_pulumi_object_field(
                    "device_postures",
                    &self.r#device_postures,
                ),
                to_pulumi_object_field(
                    "email_domains",
                    &self.r#email_domains,
                ),
                to_pulumi_object_field(
                    "email_lists",
                    &self.r#email_lists,
                ),
                to_pulumi_object_field(
                    "emails",
                    &self.r#emails,
                ),
                to_pulumi_object_field(
                    "everyone",
                    &self.r#everyone,
                ),
                to_pulumi_object_field(
                    "external_evaluation",
                    &self.r#external_evaluation,
                ),
                to_pulumi_object_field(
                    "geos",
                    &self.r#geos,
                ),
                to_pulumi_object_field(
                    "githubs",
                    &self.r#githubs,
                ),
                to_pulumi_object_field(
                    "groups",
                    &self.r#groups,
                ),
                to_pulumi_object_field(
                    "gsuites",
                    &self.r#gsuites,
                ),
                to_pulumi_object_field(
                    "ip_lists",
                    &self.r#ip_lists,
                ),
                to_pulumi_object_field(
                    "ips",
                    &self.r#ips,
                ),
                to_pulumi_object_field(
                    "login_methods",
                    &self.r#login_methods,
                ),
                to_pulumi_object_field(
                    "oktas",
                    &self.r#oktas,
                ),
                to_pulumi_object_field(
                    "samls",
                    &self.r#samls,
                ),
                to_pulumi_object_field(
                    "service_tokens",
                    &self.r#service_tokens,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AccessPolicyRequire {
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
