#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetForwardingRulesRule {
    /// The 'ports', 'portRange', and 'allPorts' fields are mutually exclusive.
    /// Only packets addressed to ports in the specified range will be forwarded
    /// to the backends configured with this forwarding rule.
    /// 
    /// The 'allPorts' field has the following limitations:
    /// * It requires that the forwarding rule 'IPProtocol' be TCP, UDP, SCTP, or
    /// L3_DEFAULT.
    /// * It's applicable only to the following products: internal passthrough
    /// Network Load Balancers, backend service-based external passthrough Network
    /// Load Balancers, and internal and external protocol forwarding.
    /// * Set this field to true to allow packets addressed to any port or packets
    /// lacking destination port information (for example, UDP fragments after the
    /// first fragment) to be forwarded to the backends configured with this
    /// forwarding rule. The L3_DEFAULT protocol requires 'allPorts' be set to
    /// true.
    #[builder(into)]
    #[serde(rename = "allPorts")]
    pub r#all_ports: bool,
    /// This field is used along with the 'backend_service' field for
    /// internal load balancing or with the 'target' field for internal
    /// TargetInstance.
    /// 
    /// If the field is set to 'TRUE', clients can access ILB from all
    /// regions.
    /// 
    /// Otherwise only allows access from clients in the same region as the
    /// internal load balancer.
    #[builder(into)]
    #[serde(rename = "allowGlobalAccess")]
    pub r#allow_global_access: bool,
    /// This is used in PSC consumer ForwardingRule to control whether the PSC endpoint can be accessed from another region.
    #[builder(into)]
    #[serde(rename = "allowPscGlobalAccess")]
    pub r#allow_psc_global_access: bool,
    /// Identifies the backend service to which the forwarding rule sends traffic.
    /// 
    /// Required for Internal TCP/UDP Load Balancing and Network Load Balancing;
    /// must be omitted for all other load balancer types.
    #[builder(into)]
    #[serde(rename = "backendService")]
    pub r#backend_service: String,
    /// [Output Only] The URL for the corresponding base Forwarding Rule. By base Forwarding Rule, we mean the Forwarding Rule that has the same IP address, protocol, and port settings with the current Forwarding Rule, but without sourceIPRanges specified. Always empty if the current Forwarding Rule does not have sourceIPRanges specified.
    #[builder(into)]
    #[serde(rename = "baseForwardingRule")]
    pub r#base_forwarding_rule: String,
    /// Creation timestamp in RFC3339 text format.
    #[builder(into)]
    #[serde(rename = "creationTimestamp")]
    pub r#creation_timestamp: String,
    /// An optional description of this resource. Provide this property when
    /// you create the resource.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    #[builder(into)]
    #[serde(rename = "effectiveLabels")]
    pub r#effective_labels: std::collections::HashMap<String, String>,
    /// The unique identifier number for the resource. This identifier is defined by the server.
    #[builder(into)]
    #[serde(rename = "forwardingRuleId")]
    pub r#forwarding_rule_id: i32,
    /// IP address for which this forwarding rule accepts traffic. When a client
    /// sends traffic to this IP address, the forwarding rule directs the traffic
    /// to the referenced 'target' or 'backendService'.
    /// 
    /// While creating a forwarding rule, specifying an 'IPAddress' is
    /// required under the following circumstances:
    /// 
    /// * When the 'target' is set to 'targetGrpcProxy' and
    /// 'validateForProxyless' is set to 'true', the
    /// 'IPAddress' should be set to '0.0.0.0'.
    /// * When the 'target' is a Private Service Connect Google APIs
    /// bundle, you must specify an 'IPAddress'.
    /// 
    /// Otherwise, you can optionally specify an IP address that references an
    /// existing static (reserved) IP address resource. When omitted, Google Cloud
    /// assigns an ephemeral IP address.
    /// 
    /// Use one of the following formats to specify an IP address while creating a
    /// forwarding rule:
    /// 
    /// * IP address number, as in '100.1.2.3'
    /// * IPv6 address range, as in '2600:1234::/96'
    /// * Full resource URL, as in
    /// 'https://www.googleapis.com/compute/v1/projects/project_id/regions/region/addresses/address-name'
    /// * Partial URL or by name, as in:
    ///   * 'projects/project_id/regions/region/addresses/address-name'
    ///   * 'regions/region/addresses/address-name'
    ///   * 'global/addresses/address-name'
    ///   * 'address-name'
    /// 
    /// The forwarding rule's 'target' or 'backendService',
    /// and in most cases, also the 'loadBalancingScheme', determine the
    /// type of IP address that you can use. For detailed information, see
    /// [IP address
    /// specifications](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts#ip_address_specifications).
    /// 
    /// When reading an 'IPAddress', the API always returns the IP
    /// address number.
    #[builder(into)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: String,
    /// The IP protocol to which this rule applies.
    /// 
    /// For protocol forwarding, valid
    /// options are 'TCP', 'UDP', 'ESP',
    /// 'AH', 'SCTP', 'ICMP' and
    /// 'L3_DEFAULT'.
    /// 
    /// The valid IP protocols are different for different load balancing products
    /// as described in [Load balancing
    /// features](https://cloud.google.com/load-balancing/docs/features#protocols_from_the_load_balancer_to_the_backends).
    /// 
    /// A Forwarding Rule with protocol L3_DEFAULT can attach with target instance or
    /// backend service with UNSPECIFIED protocol.
    /// A forwarding rule with "L3_DEFAULT" IPProtocal cannot be attached to a backend service with TCP or UDP. Possible values: ["TCP", "UDP", "ESP", "AH", "SCTP", "ICMP", "L3_DEFAULT"]
    #[builder(into)]
    #[serde(rename = "ipProtocol")]
    pub r#ip_protocol: String,
    /// The IP address version that will be used by this forwarding rule.
    /// Valid options are IPV4 and IPV6.
    /// 
    /// If not set, the IPv4 address will be used by default. Possible values: ["IPV4", "IPV6"]
    #[builder(into)]
    #[serde(rename = "ipVersion")]
    pub r#ip_version: String,
    /// Indicates whether or not this load balancer can be used as a collector for
    /// packet mirroring. To prevent mirroring loops, instances behind this
    /// load balancer will not have their traffic mirrored even if a
    /// 'PacketMirroring' rule applies to them.
    /// 
    /// This can only be set to true for load balancers that have their
    /// 'loadBalancingScheme' set to 'INTERNAL'.
    #[builder(into)]
    #[serde(rename = "isMirroringCollector")]
    pub r#is_mirroring_collector: bool,
    /// The fingerprint used for optimistic locking of this resource.  Used
    /// internally during updates.
    #[builder(into)]
    #[serde(rename = "labelFingerprint")]
    pub r#label_fingerprint: String,
    /// Labels to apply to this forwarding rule.  A list of key->value pairs.
    /// 
    /// 
    /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
    /// Please refer to the field 'effective_labels' for all of the labels present on the resource.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: std::collections::HashMap<String, String>,
    /// Specifies the forwarding rule type.
    /// 
    /// For more information about forwarding rules, refer to
    /// [Forwarding rule concepts](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts). Default value: "EXTERNAL" Possible values: ["EXTERNAL", "EXTERNAL_MANAGED", "INTERNAL", "INTERNAL_MANAGED"]
    #[builder(into)]
    #[serde(rename = "loadBalancingScheme")]
    pub r#load_balancing_scheme: String,
    /// Name of the resource; provided by the client when the resource is created.
    /// The name must be 1-63 characters long, and comply with
    /// [RFC1035](https://www.ietf.org/rfc/rfc1035.txt).
    /// 
    /// Specifically, the name must be 1-63 characters long and match the regular
    /// expression 'a-z?' which means the first
    /// character must be a lowercase letter, and all following characters must
    /// be a dash, lowercase letter, or digit, except the last character, which
    /// cannot be a dash.
    /// 
    /// For Private Service Connect forwarding rules that forward traffic to Google
    /// APIs, the forwarding rule name must be a 1-20 characters string with
    /// lowercase letters and numbers and must start with a letter.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// This field is not used for external load balancing.
    /// 
    /// For Internal TCP/UDP Load Balancing, this field identifies the network that
    /// the load balanced IP should belong to for this Forwarding Rule.
    /// If the subnetwork is specified, the network of the subnetwork will be used.
    /// If neither subnetwork nor this field is specified, the default network will
    /// be used.
    /// 
    /// For Private Service Connect forwarding rules that forward traffic to Google
    /// APIs, a network must be provided.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: String,
    /// This signifies the networking tier used for configuring
    /// this load balancer and can only take the following values:
    /// 'PREMIUM', 'STANDARD'.
    /// 
    /// For regional ForwardingRule, the valid values are 'PREMIUM' and
    /// 'STANDARD'. For GlobalForwardingRule, the valid value is
    /// 'PREMIUM'.
    /// 
    /// If this field is not specified, it is assumed to be 'PREMIUM'.
    /// If 'IPAddress' is specified, this value must be equal to the
    /// networkTier of the Address. Possible values: ["PREMIUM", "STANDARD"]
    #[builder(into)]
    #[serde(rename = "networkTier")]
    pub r#network_tier: String,
    /// This is used in PSC consumer ForwardingRule to control whether it should try to auto-generate a DNS zone or not. Non-PSC forwarding rules do not use this field.
    #[builder(into)]
    #[serde(rename = "noAutomateDnsZone")]
    pub r#no_automate_dns_zone: bool,
    /// The 'ports', 'portRange', and 'allPorts' fields are mutually exclusive.
    /// Only packets addressed to ports in the specified range will be forwarded
    /// to the backends configured with this forwarding rule.
    /// 
    /// The 'portRange' field has the following limitations:
    /// * It requires that the forwarding rule 'IPProtocol' be TCP, UDP, or SCTP,
    /// and
    /// * It's applicable only to the following products: external passthrough
    /// Network Load Balancers, internal and external proxy Network Load
    /// Balancers, internal and external Application Load Balancers, external
    /// protocol forwarding, and Classic VPN.
    /// * Some products have restrictions on what ports can be used. See
    /// [port specifications](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts#port_specifications)
    /// for details.
    /// 
    /// For external forwarding rules, two or more forwarding rules cannot use the
    /// same '[IPAddress, IPProtocol]' pair, and cannot have overlapping
    /// 'portRange's.
    /// 
    /// For internal forwarding rules within the same VPC network, two or more
    /// forwarding rules cannot use the same '[IPAddress, IPProtocol]' pair, and
    /// cannot have overlapping 'portRange's.
    /// 
    /// @pattern: \d+(?:-\d+)?
    #[builder(into)]
    #[serde(rename = "portRange")]
    pub r#port_range: String,
    /// The 'ports', 'portRange', and 'allPorts' fields are mutually exclusive.
    /// Only packets addressed to ports in the specified range will be forwarded
    /// to the backends configured with this forwarding rule.
    /// 
    /// The 'ports' field has the following limitations:
    /// * It requires that the forwarding rule 'IPProtocol' be TCP, UDP, or SCTP,
    /// and
    /// * It's applicable only to the following products: internal passthrough
    /// Network Load Balancers, backend service-based external passthrough Network
    /// Load Balancers, and internal protocol forwarding.
    /// * You can specify a list of up to five ports by number, separated by
    /// commas. The ports can be contiguous or discontiguous.
    /// 
    /// For external forwarding rules, two or more forwarding rules cannot use the
    /// same '[IPAddress, IPProtocol]' pair if they share at least one port
    /// number.
    /// 
    /// For internal forwarding rules within the same VPC network, two or more
    /// forwarding rules cannot use the same '[IPAddress, IPProtocol]' pair if
    /// they share at least one port number.
    /// 
    /// @pattern: \d+(?:-\d+)?
    #[builder(into)]
    #[serde(rename = "ports")]
    pub r#ports: Vec<String>,
    /// The name of the project.
    #[builder(into)]
    #[serde(rename = "project")]
    pub r#project: String,
    /// The PSC connection id of the PSC Forwarding Rule.
    #[builder(into)]
    #[serde(rename = "pscConnectionId")]
    pub r#psc_connection_id: String,
    /// The PSC connection status of the PSC Forwarding Rule. Possible values: 'STATUS_UNSPECIFIED', 'PENDING', 'ACCEPTED', 'REJECTED', 'CLOSED'
    #[builder(into)]
    #[serde(rename = "pscConnectionStatus")]
    pub r#psc_connection_status: String,
    /// The combination of labels configured directly on the resource
    ///  and default labels configured on the provider.
    #[builder(into)]
    #[serde(rename = "pulumiLabels")]
    pub r#pulumi_labels: std::collections::HashMap<String, String>,
    #[builder(into)]
    #[serde(rename = "recreateClosedPsc")]
    pub r#recreate_closed_psc: bool,
    /// The region you want to get the forwarding rules from.
    /// 
    /// These arguments must be set in either the provider or the resource in order for the information to be queried.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: String,
    /// The URI of the resource.
    #[builder(into)]
    #[serde(rename = "selfLink")]
    pub r#self_link: String,
    /// Service Directory resources to register this forwarding rule with.
    /// 
    /// Currently, only supports a single Service Directory resource.
    #[builder(into)]
    #[serde(rename = "serviceDirectoryRegistrations")]
    pub r#service_directory_registrations: Vec<super::super::types::compute::GetForwardingRulesRuleServiceDirectoryRegistration>,
    /// An optional prefix to the service name for this Forwarding Rule.
    /// If specified, will be the first label of the fully qualified service
    /// name.
    /// 
    /// The label must be 1-63 characters long, and comply with RFC1035.
    /// Specifically, the label must be 1-63 characters long and match the
    /// regular expression 'a-z?' which means the first
    /// character must be a lowercase letter, and all following characters
    /// must be a dash, lowercase letter, or digit, except the last
    /// character, which cannot be a dash.
    /// 
    /// This field is only used for INTERNAL load balancing.
    #[builder(into)]
    #[serde(rename = "serviceLabel")]
    pub r#service_label: String,
    /// The internal fully qualified service name for this Forwarding Rule.
    /// 
    /// This field is only used for INTERNAL load balancing.
    #[builder(into)]
    #[serde(rename = "serviceName")]
    pub r#service_name: String,
    /// If not empty, this Forwarding Rule will only forward the traffic when the source IP address matches one of the IP addresses or CIDR ranges set here. Note that a Forwarding Rule can only have up to 64 source IP ranges, and this field can only be used with a regional Forwarding Rule whose scheme is EXTERNAL. Each sourceIpRange entry should be either an IP address (for example, 1.2.3.4) or a CIDR range (for example, 1.2.3.0/24).
    #[builder(into)]
    #[serde(rename = "sourceIpRanges")]
    pub r#source_ip_ranges: Vec<String>,
    /// This field identifies the subnetwork that the load balanced IP should
    /// belong to for this Forwarding Rule, used in internal load balancing and
    /// network load balancing with IPv6.
    /// 
    /// If the network specified is in auto subnet mode, this field is optional.
    /// However, a subnetwork must be specified if the network is in custom subnet
    /// mode or when creating external forwarding rule with IPv6.
    #[builder(into)]
    #[serde(rename = "subnetwork")]
    pub r#subnetwork: String,
    /// The URL of the target resource to receive the matched traffic.  For
    /// regional forwarding rules, this target must be in the same region as the
    /// forwarding rule. For global forwarding rules, this target must be a global
    /// load balancing resource.
    /// 
    /// The forwarded traffic must be of a type appropriate to the target object.
    /// *  For load balancers, see the "Target" column in [Port specifications](https://cloud.google.com/load-balancing/docs/forwarding-rule-concepts#ip_address_specifications).
    /// *  For Private Service Connect forwarding rules that forward traffic to Google APIs, provide the name of a supported Google API bundle:
    ///   *  'vpc-sc' - [ APIs that support VPC Service Controls](https://cloud.google.com/vpc-service-controls/docs/supported-products).
    ///   *  'all-apis' - [All supported Google APIs](https://cloud.google.com/vpc/docs/private-service-connect#supported-apis).
    /// 
    /// For Private Service Connect forwarding rules that forward traffic to managed services, the target must be a service attachment.
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetForwardingRulesRule {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "all_ports",
                    &self.r#all_ports,
                ),
                to_pulumi_object_field(
                    "allow_global_access",
                    &self.r#allow_global_access,
                ),
                to_pulumi_object_field(
                    "allow_psc_global_access",
                    &self.r#allow_psc_global_access,
                ),
                to_pulumi_object_field(
                    "backend_service",
                    &self.r#backend_service,
                ),
                to_pulumi_object_field(
                    "base_forwarding_rule",
                    &self.r#base_forwarding_rule,
                ),
                to_pulumi_object_field(
                    "creation_timestamp",
                    &self.r#creation_timestamp,
                ),
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "effective_labels",
                    &self.r#effective_labels,
                ),
                to_pulumi_object_field(
                    "forwarding_rule_id",
                    &self.r#forwarding_rule_id,
                ),
                to_pulumi_object_field(
                    "ip_address",
                    &self.r#ip_address,
                ),
                to_pulumi_object_field(
                    "ip_protocol",
                    &self.r#ip_protocol,
                ),
                to_pulumi_object_field(
                    "ip_version",
                    &self.r#ip_version,
                ),
                to_pulumi_object_field(
                    "is_mirroring_collector",
                    &self.r#is_mirroring_collector,
                ),
                to_pulumi_object_field(
                    "label_fingerprint",
                    &self.r#label_fingerprint,
                ),
                to_pulumi_object_field(
                    "labels",
                    &self.r#labels,
                ),
                to_pulumi_object_field(
                    "load_balancing_scheme",
                    &self.r#load_balancing_scheme,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "network",
                    &self.r#network,
                ),
                to_pulumi_object_field(
                    "network_tier",
                    &self.r#network_tier,
                ),
                to_pulumi_object_field(
                    "no_automate_dns_zone",
                    &self.r#no_automate_dns_zone,
                ),
                to_pulumi_object_field(
                    "port_range",
                    &self.r#port_range,
                ),
                to_pulumi_object_field(
                    "ports",
                    &self.r#ports,
                ),
                to_pulumi_object_field(
                    "project",
                    &self.r#project,
                ),
                to_pulumi_object_field(
                    "psc_connection_id",
                    &self.r#psc_connection_id,
                ),
                to_pulumi_object_field(
                    "psc_connection_status",
                    &self.r#psc_connection_status,
                ),
                to_pulumi_object_field(
                    "pulumi_labels",
                    &self.r#pulumi_labels,
                ),
                to_pulumi_object_field(
                    "recreate_closed_psc",
                    &self.r#recreate_closed_psc,
                ),
                to_pulumi_object_field(
                    "region",
                    &self.r#region,
                ),
                to_pulumi_object_field(
                    "self_link",
                    &self.r#self_link,
                ),
                to_pulumi_object_field(
                    "service_directory_registrations",
                    &self.r#service_directory_registrations,
                ),
                to_pulumi_object_field(
                    "service_label",
                    &self.r#service_label,
                ),
                to_pulumi_object_field(
                    "service_name",
                    &self.r#service_name,
                ),
                to_pulumi_object_field(
                    "source_ip_ranges",
                    &self.r#source_ip_ranges,
                ),
                to_pulumi_object_field(
                    "subnetwork",
                    &self.r#subnetwork,
                ),
                to_pulumi_object_field(
                    "target",
                    &self.r#target,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetForwardingRulesRule {
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
                    r#all_ports: {
                        let field_value = match fields_map.get("all_ports") {
                            Some(value) => value,
                            None => bail!("Missing field 'all_ports' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allow_global_access: {
                        let field_value = match fields_map.get("allow_global_access") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_global_access' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#allow_psc_global_access: {
                        let field_value = match fields_map.get("allow_psc_global_access") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_psc_global_access' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#backend_service: {
                        let field_value = match fields_map.get("backend_service") {
                            Some(value) => value,
                            None => bail!("Missing field 'backend_service' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#base_forwarding_rule: {
                        let field_value = match fields_map.get("base_forwarding_rule") {
                            Some(value) => value,
                            None => bail!("Missing field 'base_forwarding_rule' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#creation_timestamp: {
                        let field_value = match fields_map.get("creation_timestamp") {
                            Some(value) => value,
                            None => bail!("Missing field 'creation_timestamp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#forwarding_rule_id: {
                        let field_value = match fields_map.get("forwarding_rule_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'forwarding_rule_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_address: {
                        let field_value = match fields_map.get("ip_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_protocol: {
                        let field_value = match fields_map.get("ip_protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_version: {
                        let field_value = match fields_map.get("ip_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_mirroring_collector: {
                        let field_value = match fields_map.get("is_mirroring_collector") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_mirroring_collector' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#label_fingerprint: {
                        let field_value = match fields_map.get("label_fingerprint") {
                            Some(value) => value,
                            None => bail!("Missing field 'label_fingerprint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#load_balancing_scheme: {
                        let field_value = match fields_map.get("load_balancing_scheme") {
                            Some(value) => value,
                            None => bail!("Missing field 'load_balancing_scheme' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#network: {
                        let field_value = match fields_map.get("network") {
                            Some(value) => value,
                            None => bail!("Missing field 'network' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#network_tier: {
                        let field_value = match fields_map.get("network_tier") {
                            Some(value) => value,
                            None => bail!("Missing field 'network_tier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#no_automate_dns_zone: {
                        let field_value = match fields_map.get("no_automate_dns_zone") {
                            Some(value) => value,
                            None => bail!("Missing field 'no_automate_dns_zone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#port_range: {
                        let field_value = match fields_map.get("port_range") {
                            Some(value) => value,
                            None => bail!("Missing field 'port_range' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ports: {
                        let field_value = match fields_map.get("ports") {
                            Some(value) => value,
                            None => bail!("Missing field 'ports' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#psc_connection_id: {
                        let field_value = match fields_map.get("psc_connection_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'psc_connection_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#psc_connection_status: {
                        let field_value = match fields_map.get("psc_connection_status") {
                            Some(value) => value,
                            None => bail!("Missing field 'psc_connection_status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#recreate_closed_psc: {
                        let field_value = match fields_map.get("recreate_closed_psc") {
                            Some(value) => value,
                            None => bail!("Missing field 'recreate_closed_psc' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#region: {
                        let field_value = match fields_map.get("region") {
                            Some(value) => value,
                            None => bail!("Missing field 'region' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#self_link: {
                        let field_value = match fields_map.get("self_link") {
                            Some(value) => value,
                            None => bail!("Missing field 'self_link' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_directory_registrations: {
                        let field_value = match fields_map.get("service_directory_registrations") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_directory_registrations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_label: {
                        let field_value = match fields_map.get("service_label") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_label' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_name: {
                        let field_value = match fields_map.get("service_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_ip_ranges: {
                        let field_value = match fields_map.get("source_ip_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_ip_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnetwork: {
                        let field_value = match fields_map.get("subnetwork") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnetwork' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target: {
                        let field_value = match fields_map.get("target") {
                            Some(value) => value,
                            None => bail!("Missing field 'target' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
