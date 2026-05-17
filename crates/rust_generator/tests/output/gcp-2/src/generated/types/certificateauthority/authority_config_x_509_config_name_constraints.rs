#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AuthorityConfigX509ConfigNameConstraints {
    /// Indicates whether or not the name constraints are marked critical.
    #[builder(into)]
    #[serde(rename = "critical")]
    pub r#critical: bool,
    /// Contains excluded DNS names. Any DNS name that can be
    /// constructed by simply adding zero or more labels to
    /// the left-hand side of the name satisfies the name constraint.
    /// For example, `example.com`, `www.example.com`, `www.sub.example.com`
    /// would satisfy `example.com` while `example1.com` does not.
    #[builder(into)]
    #[serde(rename = "excludedDnsNames")]
    pub r#excluded_dns_names: Option<Vec<String>>,
    /// Contains the excluded email addresses. The value can be a particular
    /// email address, a hostname to indicate all email addresses on that host or
    /// a domain with a leading period (e.g. `.example.com`) to indicate
    /// all email addresses in that domain.
    #[builder(into)]
    #[serde(rename = "excludedEmailAddresses")]
    pub r#excluded_email_addresses: Option<Vec<String>>,
    /// Contains the excluded IP ranges. For IPv4 addresses, the ranges
    /// are expressed using CIDR notation as specified in RFC 4632.
    /// For IPv6 addresses, the ranges are expressed in similar encoding as IPv4
    /// addresses.
    #[builder(into)]
    #[serde(rename = "excludedIpRanges")]
    pub r#excluded_ip_ranges: Option<Vec<String>>,
    /// Contains the excluded URIs that apply to the host part of the name.
    /// The value can be a hostname or a domain with a
    /// leading period (like `.example.com`)
    #[builder(into)]
    #[serde(rename = "excludedUris")]
    pub r#excluded_uris: Option<Vec<String>>,
    /// Contains permitted DNS names. Any DNS name that can be
    /// constructed by simply adding zero or more labels to
    /// the left-hand side of the name satisfies the name constraint.
    /// For example, `example.com`, `www.example.com`, `www.sub.example.com`
    /// would satisfy `example.com` while `example1.com` does not.
    #[builder(into)]
    #[serde(rename = "permittedDnsNames")]
    pub r#permitted_dns_names: Option<Vec<String>>,
    /// Contains the permitted email addresses. The value can be a particular
    /// email address, a hostname to indicate all email addresses on that host or
    /// a domain with a leading period (e.g. `.example.com`) to indicate
    /// all email addresses in that domain.
    #[builder(into)]
    #[serde(rename = "permittedEmailAddresses")]
    pub r#permitted_email_addresses: Option<Vec<String>>,
    /// Contains the permitted IP ranges. For IPv4 addresses, the ranges
    /// are expressed using CIDR notation as specified in RFC 4632.
    /// For IPv6 addresses, the ranges are expressed in similar encoding as IPv4
    /// addresses.
    #[builder(into)]
    #[serde(rename = "permittedIpRanges")]
    pub r#permitted_ip_ranges: Option<Vec<String>>,
    /// Contains the permitted URIs that apply to the host part of the name.
    /// The value can be a hostname or a domain with a
    /// leading period (like `.example.com`)
    #[builder(into)]
    #[serde(rename = "permittedUris")]
    pub r#permitted_uris: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AuthorityConfigX509ConfigNameConstraints {
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
                    "critical",
                    &self.r#critical,
                ),
                to_pulumi_object_field(
                    "excluded_dns_names",
                    &self.r#excluded_dns_names,
                ),
                to_pulumi_object_field(
                    "excluded_email_addresses",
                    &self.r#excluded_email_addresses,
                ),
                to_pulumi_object_field(
                    "excluded_ip_ranges",
                    &self.r#excluded_ip_ranges,
                ),
                to_pulumi_object_field(
                    "excluded_uris",
                    &self.r#excluded_uris,
                ),
                to_pulumi_object_field(
                    "permitted_dns_names",
                    &self.r#permitted_dns_names,
                ),
                to_pulumi_object_field(
                    "permitted_email_addresses",
                    &self.r#permitted_email_addresses,
                ),
                to_pulumi_object_field(
                    "permitted_ip_ranges",
                    &self.r#permitted_ip_ranges,
                ),
                to_pulumi_object_field(
                    "permitted_uris",
                    &self.r#permitted_uris,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AuthorityConfigX509ConfigNameConstraints {
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
                    r#critical: {
                        let field_value = match fields_map.get("critical") {
                            Some(value) => value,
                            None => bail!("Missing field 'critical' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#excluded_dns_names: {
                        let field_value = match fields_map.get("excluded_dns_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'excluded_dns_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#excluded_email_addresses: {
                        let field_value = match fields_map.get("excluded_email_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'excluded_email_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#excluded_ip_ranges: {
                        let field_value = match fields_map.get("excluded_ip_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'excluded_ip_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#excluded_uris: {
                        let field_value = match fields_map.get("excluded_uris") {
                            Some(value) => value,
                            None => bail!("Missing field 'excluded_uris' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#permitted_dns_names: {
                        let field_value = match fields_map.get("permitted_dns_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'permitted_dns_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#permitted_email_addresses: {
                        let field_value = match fields_map.get("permitted_email_addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'permitted_email_addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#permitted_ip_ranges: {
                        let field_value = match fields_map.get("permitted_ip_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'permitted_ip_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#permitted_uris: {
                        let field_value = match fields_map.get("permitted_uris") {
                            Some(value) => value,
                            None => bail!("Missing field 'permitted_uris' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
