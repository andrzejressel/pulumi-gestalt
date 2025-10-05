/// Contains information about a GKEHub Feature Memberships. Feature Memberships configure GKEHub Features that apply to specific memberships rather than the project as a whole. The google_gke_hub is the Fleet API.
///
/// ## Example Usage
///
/// ### Config Management With Config Sync Auto-Upgrades And Without Git/OCI
///
/// With [Config Sync auto-upgrades](https://cloud.devsite.corp.google.com/kubernetes-engine/enterprise/config-sync/docs/how-to/upgrade-config-sync#auto-upgrade-config), Google assumes responsibility for automatically upgrading Config Sync versions
/// and overseeing the lifecycle of its components.
///
/// ```yaml
/// resources:
///   cluster:
///     type: gcp:container:Cluster
///     properties:
///       name: my-cluster
///       location: us-central1-a
///       initialNodeCount: 1
///   membership:
///     type: gcp:gkehub:Membership
///     properties:
///       membershipId: my-membership
///       endpoint:
///         gkeCluster:
///           resourceLink: //container.googleapis.com/${cluster.id}
///   feature:
///     type: gcp:gkehub:Feature
///     properties:
///       name: configmanagement
///       location: global
///       labels:
///         foo: bar
///   featureMember:
///     type: gcp:gkehub:FeatureMembership
///     name: feature_member
///     properties:
///       location: global
///       feature: ${feature.name}
///       membership: ${membership.membershipId}
///       configmanagement:
///         management: MANAGEMENT_AUTOMATIC
///         configSync:
///           enabled: true
/// ```
///
/// ### Config Management With Git
///
/// ```yaml
/// resources:
///   cluster:
///     type: gcp:container:Cluster
///     properties:
///       name: my-cluster
///       location: us-central1-a
///       initialNodeCount: 1
///   membership:
///     type: gcp:gkehub:Membership
///     properties:
///       membershipId: my-membership
///       endpoint:
///         gkeCluster:
///           resourceLink: //container.googleapis.com/${cluster.id}
///   feature:
///     type: gcp:gkehub:Feature
///     properties:
///       name: configmanagement
///       location: global
///       labels:
///         foo: bar
///   featureMember:
///     type: gcp:gkehub:FeatureMembership
///     name: feature_member
///     properties:
///       location: global
///       feature: ${feature.name}
///       membership: ${membership.membershipId}
///       configmanagement:
///         version: 1.19.0
///         configSync:
///           enabled: true
///           git:
///             syncRepo: https://github.com/hashicorp/terraform
/// ```
///
/// ### Config Management With OCI
///
/// ```yaml
/// resources:
///   cluster:
///     type: gcp:container:Cluster
///     properties:
///       name: my-cluster
///       location: us-central1-a
///       initialNodeCount: 1
///   membership:
///     type: gcp:gkehub:Membership
///     properties:
///       membershipId: my-membership
///       endpoint:
///         gkeCluster:
///           resourceLink: //container.googleapis.com/${cluster.id}
///   feature:
///     type: gcp:gkehub:Feature
///     properties:
///       name: configmanagement
///       location: global
///       labels:
///         foo: bar
///   featureMember:
///     type: gcp:gkehub:FeatureMembership
///     name: feature_member
///     properties:
///       location: global
///       feature: ${feature.name}
///       membership: ${membership.membershipId}
///       configmanagement:
///         version: 1.19.0
///         configSync:
///           enabled: true
///           oci:
///             syncRepo: us-central1-docker.pkg.dev/sample-project/config-repo/config-sync-gke:latest
///             policyDir: config-connector
///             syncWaitSecs: '20'
///             secretType: gcpserviceaccount
///             gcpServiceAccountEmail: sa@project-id.iam.gserviceaccount.com
/// ```
///
/// ### Config Management With Regional Membership
///
/// ```yaml
/// resources:
///   cluster:
///     type: gcp:container:Cluster
///     properties:
///       name: my-cluster
///       location: us-central1-a
///       initialNodeCount: 1
///   membership:
///     type: gcp:gkehub:Membership
///     properties:
///       membershipId: my-membership
///       location: us-central1
///       endpoint:
///         gkeCluster:
///           resourceLink: //container.googleapis.com/${cluster.id}
///   feature:
///     type: gcp:gkehub:Feature
///     properties:
///       name: configmanagement
///       location: global
///       labels:
///         foo: bar
///   featureMember:
///     type: gcp:gkehub:FeatureMembership
///     name: feature_member
///     properties:
///       location: global
///       feature: ${feature.name}
///       membership: ${membership.membershipId}
///       membershipLocation: ${membership.location}
///       configmanagement:
///         version: 1.19.0
///         configSync:
///           enabled: true
///           git:
///             syncRepo: https://github.com/hashicorp/terraform
/// ```
///
/// ### Multi Cluster Service Discovery
///
/// ```yaml
/// resources:
///   feature:
///     type: gcp:gkehub:Feature
///     properties:
///       name: multiclusterservicediscovery
///       location: global
///       labels:
///         foo: bar
/// ```
///
/// ### Service Mesh
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let cluster = cluster::create(
///         "cluster",
///         ClusterArgs::builder()
///             .initial_node_count(1)
///             .location("us-central1-a")
///             .name("my-cluster")
///             .build_struct(),
///     );
///     let feature = feature::create(
///         "feature",
///         FeatureArgs::builder().location("global").name("servicemesh").build_struct(),
///     );
///     let featureMember = feature_membership::create(
///         "featureMember",
///         FeatureMembershipArgs::builder()
///             .feature("${feature.name}")
///             .location("global")
///             .membership("${membership.membershipId}")
///             .mesh(
///                 FeatureMembershipMesh::builder()
///                     .management("MANAGEMENT_AUTOMATIC")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let membership = membership::create(
///         "membership",
///         MembershipArgs::builder()
///             .endpoint(
///                 MembershipEndpoint::builder()
///                     .gkeCluster(
///                         MembershipEndpointGkeCluster::builder()
///                             .resourceLink("//container.googleapis.com/${cluster.id}")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .membership_id("my-membership")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Policy Controller With Minimal Configuration
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let cluster = cluster::create(
///         "cluster",
///         ClusterArgs::builder()
///             .initial_node_count(1)
///             .location("us-central1-a")
///             .name("my-cluster")
///             .build_struct(),
///     );
///     let feature = feature::create(
///         "feature",
///         FeatureArgs::builder().location("global").name("policycontroller").build_struct(),
///     );
///     let featureMember = feature_membership::create(
///         "featureMember",
///         FeatureMembershipArgs::builder()
///             .feature("${feature.name}")
///             .location("global")
///             .membership("${membership.membershipId}")
///             .policycontroller(
///                 FeatureMembershipPolicycontroller::builder()
///                     .policyControllerHubConfig(
///                         FeatureMembershipPolicycontrollerPolicyControllerHubConfig::builder()
///                             .installSpec("INSTALL_SPEC_ENABLED")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let membership = membership::create(
///         "membership",
///         MembershipArgs::builder()
///             .endpoint(
///                 MembershipEndpoint::builder()
///                     .gkeCluster(
///                         MembershipEndpointGkeCluster::builder()
///                             .resourceLink("//container.googleapis.com/${cluster.id}")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .membership_id("my-membership")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Policy Controller With Custom Configurations
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let cluster = cluster::create(
///         "cluster",
///         ClusterArgs::builder()
///             .initial_node_count(1)
///             .location("us-central1-a")
///             .name("my-cluster")
///             .build_struct(),
///     );
///     let feature = feature::create(
///         "feature",
///         FeatureArgs::builder().location("global").name("policycontroller").build_struct(),
///     );
///     let featureMember = feature_membership::create(
///         "featureMember",
///         FeatureMembershipArgs::builder()
///             .feature("${feature.name}")
///             .location("global")
///             .membership("${membership.membershipId}")
///             .policycontroller(
///                 FeatureMembershipPolicycontroller::builder()
///                     .policyControllerHubConfig(
///                         FeatureMembershipPolicycontrollerPolicyControllerHubConfig::builder()
///                             .auditIntervalSeconds(120)
///                             .constraintViolationLimit(50)
///                             .installSpec("INSTALL_SPEC_SUSPENDED")
///                             .logDeniesEnabled(true)
///                             .mutationEnabled(true)
///                             .policyContent(
///                                 FeatureMembershipPolicycontrollerPolicyControllerHubConfigPolicyContent::builder()
///                                     .templateLibrary(
///                                         FeatureMembershipPolicycontrollerPolicyControllerHubConfigPolicyContentTemplateLibrary::builder()
///                                             .installation("NOT_INSTALLED")
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .referentialRulesEnabled(true)
///                             .build_struct(),
///                     )
///                     .version("1.17.0")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let membership = membership::create(
///         "membership",
///         MembershipArgs::builder()
///             .endpoint(
///                 MembershipEndpoint::builder()
///                     .gkeCluster(
///                         MembershipEndpointGkeCluster::builder()
///                             .resourceLink("//container.googleapis.com/${cluster.id}")
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .membership_id("my-membership")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// FeatureMembership can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/features/{{feature}}/membershipId/{{membership}}`
///
/// * `{{project}}/{{location}}/{{feature}}/{{membership}}`
///
/// * `{{location}}/{{feature}}/{{membership}}`
///
/// When using the `pulumi import` command, FeatureMembership can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:gkehub/featureMembership:FeatureMembership default projects/{{project}}/locations/{{location}}/features/{{feature}}/membershipId/{{membership}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkehub/featureMembership:FeatureMembership default {{project}}/{{location}}/{{feature}}/{{membership}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:gkehub/featureMembership:FeatureMembership default {{location}}/{{feature}}/{{membership}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod feature_membership {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FeatureMembershipArgs {
        /// Config Management-specific spec. Structure is documented below.
        #[builder(into, default)]
        pub configmanagement: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkehub::FeatureMembershipConfigmanagement>,
        >,
        /// The name of the feature
        #[builder(into)]
        pub feature: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The location of the feature
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the membership
        #[builder(into)]
        pub membership: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The location of the membership, for example, "us-central1". Default is "global".
        #[builder(into, default)]
        pub membership_location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Service mesh specific spec. Structure is documented below.
        #[builder(into, default)]
        pub mesh: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkehub::FeatureMembershipMesh>,
        >,
        /// Policy Controller-specific spec. Structure is documented below.
        #[builder(into, default)]
        pub policycontroller: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::gkehub::FeatureMembershipPolicycontroller>,
        >,
        /// The project of the feature
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FeatureMembershipResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Config Management-specific spec. Structure is documented below.
        pub configmanagement: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkehub::FeatureMembershipConfigmanagement>,
        >,
        /// The name of the feature
        pub feature: pulumi_gestalt_rust::Output<String>,
        /// The location of the feature
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the membership
        pub membership: pulumi_gestalt_rust::Output<String>,
        /// The location of the membership, for example, "us-central1". Default is "global".
        pub membership_location: pulumi_gestalt_rust::Output<Option<String>>,
        /// Service mesh specific spec. Structure is documented below.
        pub mesh: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkehub::FeatureMembershipMesh>,
        >,
        /// Policy Controller-specific spec. Structure is documented below.
        pub policycontroller: pulumi_gestalt_rust::Output<
            Option<super::super::types::gkehub::FeatureMembershipPolicycontroller>,
        >,
        /// The project of the feature
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FeatureMembershipArgs,
    ) -> FeatureMembershipResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let configmanagement_binding = args.configmanagement.get_output(context);
        let feature_binding = args.feature.get_output(context);
        let location_binding = args.location.get_output(context);
        let membership_binding = args.membership.get_output(context);
        let membership_location_binding = args.membership_location.get_output(context);
        let mesh_binding = args.mesh.get_output(context);
        let policycontroller_binding = args.policycontroller.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:gkehub/featureMembership:FeatureMembership".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configmanagement".into(),
                    value: &configmanagement_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "feature".into(),
                    value: &feature_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "membership".into(),
                    value: &membership_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "membershipLocation".into(),
                    value: &membership_location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mesh".into(),
                    value: &mesh_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policycontroller".into(),
                    value: &policycontroller_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FeatureMembershipResult {
            id: o.get_field("id"),
            configmanagement: o.get_field("configmanagement"),
            feature: o.get_field("feature"),
            location: o.get_field("location"),
            membership: o.get_field("membership"),
            membership_location: o.get_field("membershipLocation"),
            mesh: o.get_field("mesh"),
            policycontroller: o.get_field("policycontroller"),
            project: o.get_field("project"),
        }
    }
}
