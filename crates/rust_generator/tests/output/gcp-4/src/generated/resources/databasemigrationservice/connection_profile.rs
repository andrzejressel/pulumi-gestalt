/// A connection profile definition.
///
///
/// To get more information about ConnectionProfile, see:
///
/// * [API documentation](https://cloud.google.com/database-migration/docs/reference/rest/v1/projects.locations.connectionProfiles/create)
/// * How-to Guides
///     * [Database Migration](https://cloud.google.com/database-migration/docs/)
///
///
///
/// ## Example Usage
///
/// ### Database Migration Service Connection Profile Cloudsql
///
///
/// ```yaml
/// resources:
///   cloudsqldb:
///     type: gcp:sql:DatabaseInstance
///     properties:
///       name: my-database
///       databaseVersion: MYSQL_5_7
///       settings:
///         tier: db-n1-standard-1
///         deletionProtectionEnabled: false
///       deletionProtection: false
///   sqlClientCert:
///     type: gcp:sql:SslCert
///     name: sql_client_cert
///     properties:
///       commonName: my-cert
///       instance: ${cloudsqldb.name}
///     options:
///       dependsOn:
///         - ${cloudsqldb}
///   sqldbUser:
///     type: gcp:sql:User
///     name: sqldb_user
///     properties:
///       name: my-username
///       instance: ${cloudsqldb.name}
///       password: my-password
///     options:
///       dependsOn:
///         - ${sqlClientCert}
///   cloudsqlprofile:
///     type: gcp:databasemigrationservice:ConnectionProfile
///     properties:
///       location: us-central1
///       connectionProfileId: my-fromprofileid
///       displayName: my-fromprofileid_display
///       labels:
///         foo: bar
///       mysql:
///         host: ${cloudsqldb.ipAddresses[0].ipAddress}
///         port: 3306
///         username: ${sqldbUser.name}
///         password: ${sqldbUser.password}
///         ssl:
///           clientKey: ${sqlClientCert.privateKey}
///           clientCertificate: ${sqlClientCert.cert}
///           caCertificate: ${sqlClientCert.serverCaCert}
///         cloudSqlId: my-database
///     options:
///       dependsOn:
///         - ${sqldbUser}
///   cloudsqlprofileDestination:
///     type: gcp:databasemigrationservice:ConnectionProfile
///     name: cloudsqlprofile_destination
///     properties:
///       location: us-central1
///       connectionProfileId: my-toprofileid
///       displayName: my-toprofileid_displayname
///       labels:
///         foo: bar
///       cloudsql:
///         settings:
///           databaseVersion: MYSQL_5_7
///           userLabels:
///             cloudfoo: cloudbar
///           tier: db-n1-standard-1
///           edition: ENTERPRISE
///           storageAutoResizeLimit: '0'
///           activationPolicy: ALWAYS
///           ipConfig:
///             enableIpv4: true
///             requireSsl: true
///           autoStorageIncrease: true
///           dataDiskType: PD_HDD
///           dataDiskSizeGb: '11'
///           zone: us-central1-b
///           sourceId: projects/${project.projectId}/locations/us-central1/connectionProfiles/my-fromprofileid
///           rootPassword: testpasscloudsql
///     options:
///       dependsOn:
///         - ${cloudsqlprofile}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Database Migration Service Connection Profile Postgres
///
///
/// ```yaml
/// resources:
///   postgresqldb:
///     type: gcp:sql:DatabaseInstance
///     properties:
///       name: my-database
///       databaseVersion: POSTGRES_12
///       settings:
///         tier: db-custom-2-13312
///       deletionProtection: false
///   sqlClientCert:
///     type: gcp:sql:SslCert
///     name: sql_client_cert
///     properties:
///       commonName: my-cert
///       instance: ${postgresqldb.name}
///     options:
///       dependsOn:
///         - ${postgresqldb}
///   sqldbUser:
///     type: gcp:sql:User
///     name: sqldb_user
///     properties:
///       name: my-username
///       instance: ${postgresqldb.name}
///       password: my-password
///     options:
///       dependsOn:
///         - ${sqlClientCert}
///   postgresprofile:
///     type: gcp:databasemigrationservice:ConnectionProfile
///     properties:
///       location: us-central1
///       connectionProfileId: my-profileid
///       displayName: my-profileid_display
///       labels:
///         foo: bar
///       postgresql:
///         host: ${postgresqldb.ipAddresses[0].ipAddress}
///         port: 5432
///         username: ${sqldbUser.name}
///         password: ${sqldbUser.password}
///         ssl:
///           clientKey: ${sqlClientCert.privateKey}
///           clientCertificate: ${sqlClientCert.cert}
///           caCertificate: ${sqlClientCert.serverCaCert}
///         cloudSqlId: my-database
///     options:
///       dependsOn:
///         - ${sqldbUser}
/// ```
/// ### Database Migration Service Connection Profile Oracle
///
///
/// ```yaml
/// resources:
///   oracleprofile:
///     type: gcp:databasemigrationservice:ConnectionProfile
///     properties:
///       location: us-central1
///       connectionProfileId: my-profileid
///       displayName: my-profileid_display
///       labels:
///         foo: bar
///       oracle:
///         host: host
///         port: 1521
///         username: username
///         password: password
///         databaseService: dbprovider
///         staticServiceIpConnectivity: {}
/// ```
/// ### Database Migration Service Connection Profile Alloydb
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: vpc-network
///   privateIpAlloc:
///     type: gcp:compute:GlobalAddress
///     name: private_ip_alloc
///     properties:
///       name: private-ip-alloc
///       addressType: INTERNAL
///       purpose: VPC_PEERING
///       prefixLength: 16
///       network: ${default.id}
///   vpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: vpc_connection
///     properties:
///       network: ${default.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${privateIpAlloc.name}
///   alloydbprofile:
///     type: gcp:databasemigrationservice:ConnectionProfile
///     properties:
///       location: us-central1
///       connectionProfileId: my-profileid
///       displayName: my-profileid_display
///       labels:
///         foo: bar
///       alloydb:
///         clusterId: tf-test-dbmsalloycluster_52865
///         settings:
///           initialUser:
///             user: alloyuser_85840
///             password: alloypass_60302
///           vpcNetwork: ${default.id}
///           labels:
///             alloyfoo: alloybar
///           primaryInstanceSettings:
///             id: priminstid
///             machineConfig:
///               cpuCount: 2
///             databaseFlags: {}
///             labels:
///               alloysinstfoo: allowinstbar
///     options:
///       dependsOn:
///         - ${vpcConnection}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Database Migration Service Connection Profile Existing Mysql
///
///
/// ```yaml
/// resources:
///   destinationCsql:
///     type: gcp:sql:DatabaseInstance
///     name: destination_csql
///     properties:
///       name: destination-csql
///       databaseVersion: MYSQL_5_7
///       settings:
///         tier: db-n1-standard-1
///         deletionProtectionEnabled: false
///       deletionProtection: false
///   existing-mysql:
///     type: gcp:databasemigrationservice:ConnectionProfile
///     properties:
///       location: us-central1
///       connectionProfileId: destination-cp
///       displayName: destination-cp_display
///       labels:
///         foo: bar
///       mysql:
///         cloudSqlId: destination-csql
///     options:
///       dependsOn:
///         - ${destinationCsql}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Database Migration Service Connection Profile Existing Postgres
///
///
/// ```yaml
/// resources:
///   destinationCsql:
///     type: gcp:sql:DatabaseInstance
///     name: destination_csql
///     properties:
///       name: destination-csql
///       databaseVersion: POSTGRES_15
///       settings:
///         tier: db-custom-2-13312
///         deletionProtectionEnabled: false
///       deletionProtection: false
///   existing-psql:
///     type: gcp:databasemigrationservice:ConnectionProfile
///     properties:
///       location: us-central1
///       connectionProfileId: destination-cp
///       displayName: destination-cp_display
///       labels:
///         foo: bar
///       postgresql:
///         cloudSqlId: destination-csql
///     options:
///       dependsOn:
///         - ${destinationCsql}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Database Migration Service Connection Profile Existing Alloydb
///
///
/// ```yaml
/// resources:
///   destinationAlloydb:
///     type: gcp:alloydb:Cluster
///     name: destination_alloydb
///     properties:
///       clusterId: destination-alloydb
///       location: us-central1
///       networkConfig:
///         network: ${default.id}
///       databaseVersion: POSTGRES_15
///       initialUser:
///         user: destination-alloydb
///         password: destination-alloydb
///   destinationAlloydbPrimary:
///     type: gcp:alloydb:Instance
///     name: destination_alloydb_primary
///     properties:
///       cluster: ${destinationAlloydb.name}
///       instanceId: destination-alloydb-primary
///       instanceType: PRIMARY
///     options:
///       dependsOn:
///         - ${vpcConnection}
///   privateIpAlloc:
///     type: gcp:compute:GlobalAddress
///     name: private_ip_alloc
///     properties:
///       name: destination-alloydb
///       addressType: INTERNAL
///       purpose: VPC_PEERING
///       prefixLength: 16
///       network: ${default.id}
///   vpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: vpc_connection
///     properties:
///       network: ${default.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${privateIpAlloc.name}
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: destination-alloydb
///   existing-alloydb:
///     type: gcp:databasemigrationservice:ConnectionProfile
///     properties:
///       location: us-central1
///       connectionProfileId: destination-cp
///       displayName: destination-cp_display
///       labels:
///         foo: bar
///       postgresql:
///         alloydbClusterId: destination-alloydb
///     options:
///       dependsOn:
///         - ${destinationAlloydb}
///         - ${destinationAlloydbPrimary}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// ConnectionProfile can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/connectionProfiles/{{connection_profile_id}}`
///
/// * `{{project}}/{{location}}/{{connection_profile_id}}`
///
/// * `{{location}}/{{connection_profile_id}}`
///
/// When using the `pulumi import` command, ConnectionProfile can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:databasemigrationservice/connectionProfile:ConnectionProfile default projects/{{project}}/locations/{{location}}/connectionProfiles/{{connection_profile_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:databasemigrationservice/connectionProfile:ConnectionProfile default {{project}}/{{location}}/{{connection_profile_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:databasemigrationservice/connectionProfile:ConnectionProfile default {{location}}/{{connection_profile_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod connection_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectionProfileArgs {
        /// Specifies required connection parameters, and the parameters required to create an AlloyDB destination cluster.
        /// Structure is documented below.
        #[builder(into, default)]
        pub alloydb: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::databasemigrationservice::ConnectionProfileAlloydb,
            >,
        >,
        /// Specifies required connection parameters, and, optionally, the parameters required to create a Cloud SQL destination database instance.
        /// Structure is documented below.
        #[builder(into, default)]
        pub cloudsql: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::databasemigrationservice::ConnectionProfileCloudsql,
            >,
        >,
        /// The ID of the connection profile.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub connection_profile_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The connection profile display name.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource labels for connection profile to use to annotate any related underlying resources such as Compute Engine VMs.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where the connection profile should reside.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies connection parameters required specifically for MySQL databases.
        /// Structure is documented below.
        #[builder(into, default)]
        pub mysql: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::databasemigrationservice::ConnectionProfileMysql>,
        >,
        /// Specifies connection parameters required specifically for Oracle databases.
        /// Structure is documented below.
        #[builder(into, default)]
        pub oracle: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::databasemigrationservice::ConnectionProfileOracle,
            >,
        >,
        /// Specifies connection parameters required specifically for PostgreSQL databases.
        /// Structure is documented below.
        #[builder(into, default)]
        pub postgresql: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::databasemigrationservice::ConnectionProfilePostgresql,
            >,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ConnectionProfileResult {
        /// Specifies required connection parameters, and the parameters required to create an AlloyDB destination cluster.
        /// Structure is documented below.
        pub alloydb: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::databasemigrationservice::ConnectionProfileAlloydb,
            >,
        >,
        /// Specifies required connection parameters, and, optionally, the parameters required to create a Cloud SQL destination database instance.
        /// Structure is documented below.
        pub cloudsql: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::databasemigrationservice::ConnectionProfileCloudsql,
            >,
        >,
        /// The ID of the connection profile.
        ///
        ///
        /// - - -
        pub connection_profile_id: pulumi_gestalt_rust::Output<String>,
        /// Output only. The timestamp when the resource was created. A timestamp in RFC3339 UTC 'Zulu' format, accurate to nanoseconds. Example: '2014-10-02T15:01:23.045123456Z'.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The database provider.
        pub dbprovider: pulumi_gestalt_rust::Output<String>,
        /// The connection profile display name.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output only. The error details in case of state FAILED.
        /// Structure is documented below.
        pub errors: pulumi_gestalt_rust::Output<
            Vec<super::super::types::databasemigrationservice::ConnectionProfileError>,
        >,
        /// The resource labels for connection profile to use to annotate any related underlying resources such as Compute Engine VMs.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where the connection profile should reside.
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies connection parameters required specifically for MySQL databases.
        /// Structure is documented below.
        pub mysql: pulumi_gestalt_rust::Output<
            Option<super::super::types::databasemigrationservice::ConnectionProfileMysql>,
        >,
        /// The name of this connection profile resource in the form of projects/{project}/locations/{location}/connectionProfiles/{connectionProfile}.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies connection parameters required specifically for Oracle databases.
        /// Structure is documented below.
        pub oracle: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::databasemigrationservice::ConnectionProfileOracle,
            >,
        >,
        /// Specifies connection parameters required specifically for PostgreSQL databases.
        /// Structure is documented below.
        pub postgresql: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::databasemigrationservice::ConnectionProfilePostgresql,
            >,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The current connection profile state.
        pub state: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConnectionProfileArgs,
    ) -> ConnectionProfileResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let alloydb_binding = args.alloydb.get_output(context);
        let cloudsql_binding = args.cloudsql.get_output(context);
        let connection_profile_id_binding = args
            .connection_profile_id
            .get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let mysql_binding = args.mysql.get_output(context);
        let oracle_binding = args.oracle.get_output(context);
        let postgresql_binding = args.postgresql.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:databasemigrationservice/connectionProfile:ConnectionProfile"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "alloydb".into(),
                    value: &alloydb_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cloudsql".into(),
                    value: &cloudsql_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionProfileId".into(),
                    value: &connection_profile_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mysql".into(),
                    value: &mysql_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "oracle".into(),
                    value: &oracle_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "postgresql".into(),
                    value: &postgresql_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConnectionProfileResult {
            alloydb: o.get_field("alloydb"),
            cloudsql: o.get_field("cloudsql"),
            connection_profile_id: o.get_field("connectionProfileId"),
            create_time: o.get_field("createTime"),
            dbprovider: o.get_field("dbprovider"),
            display_name: o.get_field("displayName"),
            effective_labels: o.get_field("effectiveLabels"),
            errors: o.get_field("errors"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            mysql: o.get_field("mysql"),
            name: o.get_field("name"),
            oracle: o.get_field("oracle"),
            postgresql: o.get_field("postgresql"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            state: o.get_field("state"),
        }
    }
}
