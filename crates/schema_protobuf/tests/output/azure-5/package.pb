
azureAzure"6.14.0*±Ã
H
domainservices
ReplicaSet*azure:domainservices/replicaSet:ReplicaSetç√Manages a Replica Set for an Active Directory Domain Service.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";
import * as azuread from "@pulumi/azuread";

const primary = new azure.core.ResourceGroup("primary", {
    name: "aadds-primary-rg",
    location: "West Europe",
});
const primaryVirtualNetwork = new azure.network.VirtualNetwork("primary", {
    name: "aadds-primary-vnet",
    location: primary.location,
    resourceGroupName: primary.name,
    addressSpaces: ["10.0.1.0/16"],
});
const primarySubnet = new azure.network.Subnet("primary", {
    name: "aadds-primary-subnet",
    resourceGroupName: primary.name,
    virtualNetworkName: primaryVirtualNetwork.name,
    addressPrefixes: ["10.0.1.0/24"],
});
const primaryNetworkSecurityGroup = new azure.network.NetworkSecurityGroup("primary", {
    name: "aadds-primary-nsg",
    location: primary.location,
    resourceGroupName: primary.name,
    securityRules: [
        {
            name: "AllowSyncWithAzureAD",
            priority: 101,
            direction: "Inbound",
            access: "Allow",
            protocol: "Tcp",
            sourcePortRange: "*",
            destinationPortRange: "443",
            sourceAddressPrefix: "AzureActiveDirectoryDomainServices",
            destinationAddressPrefix: "*",
        },
        {
            name: "AllowRD",
            priority: 201,
            direction: "Inbound",
            access: "Allow",
            protocol: "Tcp",
            sourcePortRange: "*",
            destinationPortRange: "3389",
            sourceAddressPrefix: "CorpNetSaw",
            destinationAddressPrefix: "*",
        },
        {
            name: "AllowPSRemoting",
            priority: 301,
            direction: "Inbound",
            access: "Allow",
            protocol: "Tcp",
            sourcePortRange: "*",
            destinationPortRange: "5986",
            sourceAddressPrefix: "AzureActiveDirectoryDomainServices",
            destinationAddressPrefix: "*",
        },
        {
            name: "AllowLDAPS",
            priority: 401,
            direction: "Inbound",
            access: "Allow",
            protocol: "Tcp",
            sourcePortRange: "*",
            destinationPortRange: "636",
            sourceAddressPrefix: "*",
            destinationAddressPrefix: "*",
        },
    ],
});
const primarySubnetNetworkSecurityGroupAssociation = new azure.network.SubnetNetworkSecurityGroupAssociation("primary", {
    subnetId: primarySubnet.id,
    networkSecurityGroupId: primaryNetworkSecurityGroup.id,
});
const dcAdmins = new azuread.Group("dc_admins", {
    displayName: "aad-dc-administrators",
    securityEnabled: true,
});
const admin = new azuread.User("admin", {
    userPrincipalName: "dc-admin@hashicorp-example.net",
    displayName: "DC Administrator",
    password: "Pa55w0Rd!!1",
});
const adminGroupMember = new azuread.GroupMember("admin", {
    groupObjectId: dcAdmins.objectId,
    memberObjectId: admin.objectId,
});
const example = new azuread.ServicePrincipal("example", {applicationId: "2565bd9d-da50-47d4-8b85-4c97f669dc36"});
const aadds = new azure.core.ResourceGroup("aadds", {
    name: "aadds-rg",
    location: "westeurope",
});
const exampleService = new azure.domainservices.Service("example", {
    name: "example-aadds",
    location: aadds.location,
    resourceGroupName: aadds.name,
    domainName: "widgetslogin.net",
    sku: "Enterprise",
    filteredSyncEnabled: false,
    initialReplicaSet: {
        location: primaryVirtualNetwork.location,
        subnetId: primarySubnet.id,
    },
    notifications: {
        additionalRecipients: [
            "notifyA@example.net",
            "notifyB@example.org",
        ],
        notifyDcAdmins: true,
        notifyGlobalAdmins: true,
    },
    security: {
        syncKerberosPasswords: true,
        syncNtlmPasswords: true,
        syncOnPremPasswords: true,
    },
    tags: {
        Environment: "prod",
    },
}, {
    dependsOn: [
        example,
        primarySubnetNetworkSecurityGroupAssociation,
    ],
});
const replica = new azure.core.ResourceGroup("replica", {
    name: "aadds-replica-rg",
    location: "North Europe",
});
const replicaVirtualNetwork = new azure.network.VirtualNetwork("replica", {
    name: "aadds-replica-vnet",
    location: replica.location,
    resourceGroupName: replica.name,
    addressSpaces: ["10.20.0.0/16"],
});
const aaddsReplica = new azure.network.Subnet("aadds_replica", {
    name: "aadds-replica-subnet",
    resourceGroupName: replica.name,
    virtualNetworkName: replicaVirtualNetwork.name,
    addressPrefixes: ["10.20.0.0/24"],
});
const aaddsReplicaNetworkSecurityGroup = new azure.network.NetworkSecurityGroup("aadds_replica", {
    name: "aadds-replica-nsg",
    location: replica.location,
    resourceGroupName: replica.name,
    securityRules: [
        {
            name: "AllowSyncWithAzureAD",
            priority: 101,
            direction: "Inbound",
            access: "Allow",
            protocol: "Tcp",
            sourcePortRange: "*",
            destinationPortRange: "443",
            sourceAddressPrefix: "AzureActiveDirectoryDomainServices",
            destinationAddressPrefix: "*",
        },
        {
            name: "AllowRD",
            priority: 201,
            direction: "Inbound",
            access: "Allow",
            protocol: "Tcp",
            sourcePortRange: "*",
            destinationPortRange: "3389",
            sourceAddressPrefix: "CorpNetSaw",
            destinationAddressPrefix: "*",
        },
        {
            name: "AllowPSRemoting",
            priority: 301,
            direction: "Inbound",
            access: "Allow",
            protocol: "Tcp",
            sourcePortRange: "*",
            destinationPortRange: "5986",
            sourceAddressPrefix: "AzureActiveDirectoryDomainServices",
            destinationAddressPrefix: "*",
        },
        {
            name: "AllowLDAPS",
            priority: 401,
            direction: "Inbound",
            access: "Allow",
            protocol: "Tcp",
            sourcePortRange: "*",
            destinationPortRange: "636",
            sourceAddressPrefix: "*",
            destinationAddressPrefix: "*",
        },
    ],
});
const replicaSubnetNetworkSecurityGroupAssociation = new azure.network.SubnetNetworkSecurityGroupAssociation("replica", {
    subnetId: aaddsReplica.id,
    networkSecurityGroupId: aaddsReplicaNetworkSecurityGroup.id,
});
const primaryReplica = new azure.network.VirtualNetworkPeering("primary_replica", {
    name: "aadds-primary-replica",
    resourceGroupName: primaryVirtualNetwork.resourceGroupName,
    virtualNetworkName: primaryVirtualNetwork.name,
    remoteVirtualNetworkId: replicaVirtualNetwork.id,
    allowForwardedTraffic: true,
    allowGatewayTransit: false,
    allowVirtualNetworkAccess: true,
    useRemoteGateways: false,
});
const replicaPrimary = new azure.network.VirtualNetworkPeering("replica_primary", {
    name: "aadds-replica-primary",
    resourceGroupName: replicaVirtualNetwork.resourceGroupName,
    virtualNetworkName: replicaVirtualNetwork.name,
    remoteVirtualNetworkId: primaryVirtualNetwork.id,
    allowForwardedTraffic: true,
    allowGatewayTransit: false,
    allowVirtualNetworkAccess: true,
    useRemoteGateways: false,
});
const replicaVirtualNetworkDnsServers = new azure.network.VirtualNetworkDnsServers("replica", {
    virtualNetworkId: replicaVirtualNetwork.id,
    dnsServers: exampleService.initialReplicaSet.apply(initialReplicaSet => initialReplicaSet.domainControllerIpAddresses),
});
const replicaReplicaSet = new azure.domainservices.ReplicaSet("replica", {
    domainServiceId: exampleService.id,
    location: replica.location,
    subnetId: aaddsReplica.id,
}, {
    dependsOn: [
        replicaSubnetNetworkSecurityGroupAssociation,
        primaryReplica,
        replicaPrimary,
    ],
});
```
```python
import pulumi
import pulumi_azure as azure
import pulumi_azuread as azuread

primary = azure.core.ResourceGroup("primary",
    name="aadds-primary-rg",
    location="West Europe")
primary_virtual_network = azure.network.VirtualNetwork("primary",
    name="aadds-primary-vnet",
    location=primary.location,
    resource_group_name=primary.name,
    address_spaces=["10.0.1.0/16"])
primary_subnet = azure.network.Subnet("primary",
    name="aadds-primary-subnet",
    resource_group_name=primary.name,
    virtual_network_name=primary_virtual_network.name,
    address_prefixes=["10.0.1.0/24"])
primary_network_security_group = azure.network.NetworkSecurityGroup("primary",
    name="aadds-primary-nsg",
    location=primary.location,
    resource_group_name=primary.name,
    security_rules=[
        {
            "name": "AllowSyncWithAzureAD",
            "priority": 101,
            "direction": "Inbound",
            "access": "Allow",
            "protocol": "Tcp",
            "source_port_range": "*",
            "destination_port_range": "443",
            "source_address_prefix": "AzureActiveDirectoryDomainServices",
            "destination_address_prefix": "*",
        },
        {
            "name": "AllowRD",
            "priority": 201,
            "direction": "Inbound",
            "access": "Allow",
            "protocol": "Tcp",
            "source_port_range": "*",
            "destination_port_range": "3389",
            "source_address_prefix": "CorpNetSaw",
            "destination_address_prefix": "*",
        },
        {
            "name": "AllowPSRemoting",
            "priority": 301,
            "direction": "Inbound",
            "access": "Allow",
            "protocol": "Tcp",
            "source_port_range": "*",
            "destination_port_range": "5986",
            "source_address_prefix": "AzureActiveDirectoryDomainServices",
            "destination_address_prefix": "*",
        },
        {
            "name": "AllowLDAPS",
            "priority": 401,
            "direction": "Inbound",
            "access": "Allow",
            "protocol": "Tcp",
            "source_port_range": "*",
            "destination_port_range": "636",
            "source_address_prefix": "*",
            "destination_address_prefix": "*",
        },
    ])
primary_subnet_network_security_group_association = azure.network.SubnetNetworkSecurityGroupAssociation("primary",
    subnet_id=primary_subnet.id,
    network_security_group_id=primary_network_security_group.id)
dc_admins = azuread.Group("dc_admins",
    display_name="aad-dc-administrators",
    security_enabled=True)
admin = azuread.User("admin",
    user_principal_name="dc-admin@hashicorp-example.net",
    display_name="DC Administrator",
    password="Pa55w0Rd!!1")
admin_group_member = azuread.GroupMember("admin",
    group_object_id=dc_admins.object_id,
    member_object_id=admin.object_id)
example = azuread.ServicePrincipal("example", application_id="2565bd9d-da50-47d4-8b85-4c97f669dc36")
aadds = azure.core.ResourceGroup("aadds",
    name="aadds-rg",
    location="westeurope")
example_service = azure.domainservices.Service("example",
    name="example-aadds",
    location=aadds.location,
    resource_group_name=aadds.name,
    domain_name="widgetslogin.net",
    sku="Enterprise",
    filtered_sync_enabled=False,
    initial_replica_set={
        "location": primary_virtual_network.location,
        "subnet_id": primary_subnet.id,
    },
    notifications={
        "additional_recipients": [
            "notifyA@example.net",
            "notifyB@example.org",
        ],
        "notify_dc_admins": True,
        "notify_global_admins": True,
    },
    security={
        "sync_kerberos_passwords": True,
        "sync_ntlm_passwords": True,
        "sync_on_prem_passwords": True,
    },
    tags={
        "Environment": "prod",
    },
    opts = pulumi.ResourceOptions(depends_on=[
            example,
            primary_subnet_network_security_group_association,
        ]))
replica = azure.core.ResourceGroup("replica",
    name="aadds-replica-rg",
    location="North Europe")
replica_virtual_network = azure.network.VirtualNetwork("replica",
    name="aadds-replica-vnet",
    location=replica.location,
    resource_group_name=replica.name,
    address_spaces=["10.20.0.0/16"])
aadds_replica = azure.network.Subnet("aadds_replica",
    name="aadds-replica-subnet",
    resource_group_name=replica.name,
    virtual_network_name=replica_virtual_network.name,
    address_prefixes=["10.20.0.0/24"])
aadds_replica_network_security_group = azure.network.NetworkSecurityGroup("aadds_replica",
    name="aadds-replica-nsg",
    location=replica.location,
    resource_group_name=replica.name,
    security_rules=[
        {
            "name": "AllowSyncWithAzureAD",
            "priority": 101,
            "direction": "Inbound",
            "access": "Allow",
            "protocol": "Tcp",
            "source_port_range": "*",
            "destination_port_range": "443",
            "source_address_prefix": "AzureActiveDirectoryDomainServices",
            "destination_address_prefix": "*",
        },
        {
            "name": "AllowRD",
            "priority": 201,
            "direction": "Inbound",
            "access": "Allow",
            "protocol": "Tcp",
            "source_port_range": "*",
            "destination_port_range": "3389",
            "source_address_prefix": "CorpNetSaw",
            "destination_address_prefix": "*",
        },
        {
            "name": "AllowPSRemoting",
            "priority": 301,
            "direction": "Inbound",
            "access": "Allow",
            "protocol": "Tcp",
            "source_port_range": "*",
            "destination_port_range": "5986",
            "source_address_prefix": "AzureActiveDirectoryDomainServices",
            "destination_address_prefix": "*",
        },
        {
            "name": "AllowLDAPS",
            "priority": 401,
            "direction": "Inbound",
            "access": "Allow",
            "protocol": "Tcp",
            "source_port_range": "*",
            "destination_port_range": "636",
            "source_address_prefix": "*",
            "destination_address_prefix": "*",
        },
    ])
replica_subnet_network_security_group_association = azure.network.SubnetNetworkSecurityGroupAssociation("replica",
    subnet_id=aadds_replica.id,
    network_security_group_id=aadds_replica_network_security_group.id)
primary_replica = azure.network.VirtualNetworkPeering("primary_replica",
    name="aadds-primary-replica",
    resource_group_name=primary_virtual_network.resource_group_name,
    virtual_network_name=primary_virtual_network.name,
    remote_virtual_network_id=replica_virtual_network.id,
    allow_forwarded_traffic=True,
    allow_gateway_transit=False,
    allow_virtual_network_access=True,
    use_remote_gateways=False)
replica_primary = azure.network.VirtualNetworkPeering("replica_primary",
    name="aadds-replica-primary",
    resource_group_name=replica_virtual_network.resource_group_name,
    virtual_network_name=replica_virtual_network.name,
    remote_virtual_network_id=primary_virtual_network.id,
    allow_forwarded_traffic=True,
    allow_gateway_transit=False,
    allow_virtual_network_access=True,
    use_remote_gateways=False)
replica_virtual_network_dns_servers = azure.network.VirtualNetworkDnsServers("replica",
    virtual_network_id=replica_virtual_network.id,
    dns_servers=example_service.initial_replica_set.domain_controller_ip_addresses)
replica_replica_set = azure.domainservices.ReplicaSet("replica",
    domain_service_id=example_service.id,
    location=replica.location,
    subnet_id=aadds_replica.id,
    opts = pulumi.ResourceOptions(depends_on=[
            replica_subnet_network_security_group_association,
            primary_replica,
            replica_primary,
        ]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;
using AzureAD = Pulumi.AzureAD;

return await Deployment.RunAsync(() => 
{
    var primary = new Azure.Core.ResourceGroup("primary", new()
    {
        Name = "aadds-primary-rg",
        Location = "West Europe",
    });

    var primaryVirtualNetwork = new Azure.Network.VirtualNetwork("primary", new()
    {
        Name = "aadds-primary-vnet",
        Location = primary.Location,
        ResourceGroupName = primary.Name,
        AddressSpaces = new[]
        {
            "10.0.1.0/16",
        },
    });

    var primarySubnet = new Azure.Network.Subnet("primary", new()
    {
        Name = "aadds-primary-subnet",
        ResourceGroupName = primary.Name,
        VirtualNetworkName = primaryVirtualNetwork.Name,
        AddressPrefixes = new[]
        {
            "10.0.1.0/24",
        },
    });

    var primaryNetworkSecurityGroup = new Azure.Network.NetworkSecurityGroup("primary", new()
    {
        Name = "aadds-primary-nsg",
        Location = primary.Location,
        ResourceGroupName = primary.Name,
        SecurityRules = new[]
        {
            new Azure.Network.Inputs.NetworkSecurityGroupSecurityRuleArgs
            {
                Name = "AllowSyncWithAzureAD",
                Priority = 101,
                Direction = "Inbound",
                Access = "Allow",
                Protocol = "Tcp",
                SourcePortRange = "*",
                DestinationPortRange = "443",
                SourceAddressPrefix = "AzureActiveDirectoryDomainServices",
                DestinationAddressPrefix = "*",
            },
            new Azure.Network.Inputs.NetworkSecurityGroupSecurityRuleArgs
            {
                Name = "AllowRD",
                Priority = 201,
                Direction = "Inbound",
                Access = "Allow",
                Protocol = "Tcp",
                SourcePortRange = "*",
                DestinationPortRange = "3389",
                SourceAddressPrefix = "CorpNetSaw",
                DestinationAddressPrefix = "*",
            },
            new Azure.Network.Inputs.NetworkSecurityGroupSecurityRuleArgs
            {
                Name = "AllowPSRemoting",
                Priority = 301,
                Direction = "Inbound",
                Access = "Allow",
                Protocol = "Tcp",
                SourcePortRange = "*",
                DestinationPortRange = "5986",
                SourceAddressPrefix = "AzureActiveDirectoryDomainServices",
                DestinationAddressPrefix = "*",
            },
            new Azure.Network.Inputs.NetworkSecurityGroupSecurityRuleArgs
            {
                Name = "AllowLDAPS",
                Priority = 401,
                Direction = "Inbound",
                Access = "Allow",
                Protocol = "Tcp",
                SourcePortRange = "*",
                DestinationPortRange = "636",
                SourceAddressPrefix = "*",
                DestinationAddressPrefix = "*",
            },
        },
    });

    var primarySubnetNetworkSecurityGroupAssociation = new Azure.Network.SubnetNetworkSecurityGroupAssociation("primary", new()
    {
        SubnetId = primarySubnet.Id,
        NetworkSecurityGroupId = primaryNetworkSecurityGroup.Id,
    });

    var dcAdmins = new AzureAD.Group("dc_admins", new()
    {
        DisplayName = "aad-dc-administrators",
        SecurityEnabled = true,
    });

    var admin = new AzureAD.User("admin", new()
    {
        UserPrincipalName = "dc-admin@hashicorp-example.net",
        DisplayName = "DC Administrator",
        Password = "Pa55w0Rd!!1",
    });

    var adminGroupMember = new AzureAD.GroupMember("admin", new()
    {
        GroupObjectId = dcAdmins.ObjectId,
        MemberObjectId = admin.ObjectId,
    });

    var example = new AzureAD.ServicePrincipal("example", new()
    {
        ApplicationId = "2565bd9d-da50-47d4-8b85-4c97f669dc36",
    });

    var aadds = new Azure.Core.ResourceGroup("aadds", new()
    {
        Name = "aadds-rg",
        Location = "westeurope",
    });

    var exampleService = new Azure.DomainServices.Service("example", new()
    {
        Name = "example-aadds",
        Location = aadds.Location,
        ResourceGroupName = aadds.Name,
        DomainName = "widgetslogin.net",
        Sku = "Enterprise",
        FilteredSyncEnabled = false,
        InitialReplicaSet = new Azure.DomainServices.Inputs.ServiceInitialReplicaSetArgs
        {
            Location = primaryVirtualNetwork.Location,
            SubnetId = primarySubnet.Id,
        },
        Notifications = new Azure.DomainServices.Inputs.ServiceNotificationsArgs
        {
            AdditionalRecipients = new[]
            {
                "notifyA@example.net",
                "notifyB@example.org",
            },
            NotifyDcAdmins = true,
            NotifyGlobalAdmins = true,
        },
        Security = new Azure.DomainServices.Inputs.ServiceSecurityArgs
        {
            SyncKerberosPasswords = true,
            SyncNtlmPasswords = true,
            SyncOnPremPasswords = true,
        },
        Tags = 
        {
            { "Environment", "prod" },
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            example,
            primarySubnetNetworkSecurityGroupAssociation,
        },
    });

    var replica = new Azure.Core.ResourceGroup("replica", new()
    {
        Name = "aadds-replica-rg",
        Location = "North Europe",
    });

    var replicaVirtualNetwork = new Azure.Network.VirtualNetwork("replica", new()
    {
        Name = "aadds-replica-vnet",
        Location = replica.Location,
        ResourceGroupName = replica.Name,
        AddressSpaces = new[]
        {
            "10.20.0.0/16",
        },
    });

    var aaddsReplica = new Azure.Network.Subnet("aadds_replica", new()
    {
        Name = "aadds-replica-subnet",
        ResourceGroupName = replica.Name,
        VirtualNetworkName = replicaVirtualNetwork.Name,
        AddressPrefixes = new[]
        {
            "10.20.0.0/24",
        },
    });

    var aaddsReplicaNetworkSecurityGroup = new Azure.Network.NetworkSecurityGroup("aadds_replica", new()
    {
        Name = "aadds-replica-nsg",
        Location = replica.Location,
        ResourceGroupName = replica.Name,
        SecurityRules = new[]
        {
            new Azure.Network.Inputs.NetworkSecurityGroupSecurityRuleArgs
            {
                Name = "AllowSyncWithAzureAD",
                Priority = 101,
                Direction = "Inbound",
                Access = "Allow",
                Protocol = "Tcp",
                SourcePortRange = "*",
                DestinationPortRange = "443",
                SourceAddressPrefix = "AzureActiveDirectoryDomainServices",
                DestinationAddressPrefix = "*",
            },
            new Azure.Network.Inputs.NetworkSecurityGroupSecurityRuleArgs
            {
                Name = "AllowRD",
                Priority = 201,
                Direction = "Inbound",
                Access = "Allow",
                Protocol = "Tcp",
                SourcePortRange = "*",
                DestinationPortRange = "3389",
                SourceAddressPrefix = "CorpNetSaw",
                DestinationAddressPrefix = "*",
            },
            new Azure.Network.Inputs.NetworkSecurityGroupSecurityRuleArgs
            {
                Name = "AllowPSRemoting",
                Priority = 301,
                Direction = "Inbound",
                Access = "Allow",
                Protocol = "Tcp",
                SourcePortRange = "*",
                DestinationPortRange = "5986",
                SourceAddressPrefix = "AzureActiveDirectoryDomainServices",
                DestinationAddressPrefix = "*",
            },
            new Azure.Network.Inputs.NetworkSecurityGroupSecurityRuleArgs
            {
                Name = "AllowLDAPS",
                Priority = 401,
                Direction = "Inbound",
                Access = "Allow",
                Protocol = "Tcp",
                SourcePortRange = "*",
                DestinationPortRange = "636",
                SourceAddressPrefix = "*",
                DestinationAddressPrefix = "*",
            },
        },
    });

    var replicaSubnetNetworkSecurityGroupAssociation = new Azure.Network.SubnetNetworkSecurityGroupAssociation("replica", new()
    {
        SubnetId = aaddsReplica.Id,
        NetworkSecurityGroupId = aaddsReplicaNetworkSecurityGroup.Id,
    });

    var primaryReplica = new Azure.Network.VirtualNetworkPeering("primary_replica", new()
    {
        Name = "aadds-primary-replica",
        ResourceGroupName = primaryVirtualNetwork.ResourceGroupName,
        VirtualNetworkName = primaryVirtualNetwork.Name,
        RemoteVirtualNetworkId = replicaVirtualNetwork.Id,
        AllowForwardedTraffic = true,
        AllowGatewayTransit = false,
        AllowVirtualNetworkAccess = true,
        UseRemoteGateways = false,
    });

    var replicaPrimary = new Azure.Network.VirtualNetworkPeering("replica_primary", new()
    {
        Name = "aadds-replica-primary",
        ResourceGroupName = replicaVirtualNetwork.ResourceGroupName,
        VirtualNetworkName = replicaVirtualNetwork.Name,
        RemoteVirtualNetworkId = primaryVirtualNetwork.Id,
        AllowForwardedTraffic = true,
        AllowGatewayTransit = false,
        AllowVirtualNetworkAccess = true,
        UseRemoteGateways = false,
    });

    var replicaVirtualNetworkDnsServers = new Azure.Network.VirtualNetworkDnsServers("replica", new()
    {
        VirtualNetworkId = replicaVirtualNetwork.Id,
        DnsServers = exampleService.InitialReplicaSet.Apply(initialReplicaSet => initialReplicaSet.DomainControllerIpAddresses),
    });

    var replicaReplicaSet = new Azure.DomainServices.ReplicaSet("replica", new()
    {
        DomainServiceId = exampleService.Id,
        Location = replica.Location,
        SubnetId = aaddsReplica.Id,
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            replicaSubnetNetworkSecurityGroupAssociation,
            primaryReplica,
            replicaPrimary,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/domainservices"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/network"
	"github.com/pulumi/pulumi-azuread/sdk/v5/go/azuread"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)
func main() {
pulumi.Run(func(ctx *pulumi.Context) error {
primary, err := core.NewResourceGroup(ctx, "primary", &core.ResourceGroupArgs{
Name: pulumi.String("aadds-primary-rg"),
Location: pulumi.String("West Europe"),
})
if err != nil {
return err
}
primaryVirtualNetwork, err := network.NewVirtualNetwork(ctx, "primary", &network.VirtualNetworkArgs{
Name: pulumi.String("aadds-primary-vnet"),
Location: primary.Location,
ResourceGroupName: primary.Name,
AddressSpaces: pulumi.StringArray{
pulumi.String("10.0.1.0/16"),
},
})
if err != nil {
return err
}
primarySubnet, err := network.NewSubnet(ctx, "primary", &network.SubnetArgs{
Name: pulumi.String("aadds-primary-subnet"),
ResourceGroupName: primary.Name,
VirtualNetworkName: primaryVirtualNetwork.Name,
AddressPrefixes: pulumi.StringArray{
pulumi.String("10.0.1.0/24"),
},
})
if err != nil {
return err
}
primaryNetworkSecurityGroup, err := network.NewNetworkSecurityGroup(ctx, "primary", &network.NetworkSecurityGroupArgs{
Name: pulumi.String("aadds-primary-nsg"),
Location: primary.Location,
ResourceGroupName: primary.Name,
SecurityRules: network.NetworkSecurityGroupSecurityRuleArray{
&network.NetworkSecurityGroupSecurityRuleArgs{
Name: pulumi.String("AllowSyncWithAzureAD"),
Priority: pulumi.Int(101),
Direction: pulumi.String("Inbound"),
Access: pulumi.String("Allow"),
Protocol: pulumi.String("Tcp"),
SourcePortRange: pulumi.String("*"),
DestinationPortRange: pulumi.String("443"),
SourceAddressPrefix: pulumi.String("AzureActiveDirectoryDomainServices"),
DestinationAddressPrefix: pulumi.String("*"),
},
&network.NetworkSecurityGroupSecurityRuleArgs{
Name: pulumi.String("AllowRD"),
Priority: pulumi.Int(201),
Direction: pulumi.String("Inbound"),
Access: pulumi.String("Allow"),
Protocol: pulumi.String("Tcp"),
SourcePortRange: pulumi.String("*"),
DestinationPortRange: pulumi.String("3389"),
SourceAddressPrefix: pulumi.String("CorpNetSaw"),
DestinationAddressPrefix: pulumi.String("*"),
},
&network.NetworkSecurityGroupSecurityRuleArgs{
Name: pulumi.String("AllowPSRemoting"),
Priority: pulumi.Int(301),
Direction: pulumi.String("Inbound"),
Access: pulumi.String("Allow"),
Protocol: pulumi.String("Tcp"),
SourcePortRange: pulumi.String("*"),
DestinationPortRange: pulumi.String("5986"),
SourceAddressPrefix: pulumi.String("AzureActiveDirectoryDomainServices"),
DestinationAddressPrefix: pulumi.String("*"),
},
&network.NetworkSecurityGroupSecurityRuleArgs{
Name: pulumi.String("AllowLDAPS"),
Priority: pulumi.Int(401),
Direction: pulumi.String("Inbound"),
Access: pulumi.String("Allow"),
Protocol: pulumi.String("Tcp"),
SourcePortRange: pulumi.String("*"),
DestinationPortRange: pulumi.String("636"),
SourceAddressPrefix: pulumi.String("*"),
DestinationAddressPrefix: pulumi.String("*"),
},
},
})
if err != nil {
return err
}
primarySubnetNetworkSecurityGroupAssociation, err := network.NewSubnetNetworkSecurityGroupAssociation(ctx, "primary", &network.SubnetNetworkSecurityGroupAssociationArgs{
SubnetId: primarySubnet.ID(),
NetworkSecurityGroupId: primaryNetworkSecurityGroup.ID(),
})
if err != nil {
return err
}
dcAdmins, err := azuread.NewGroup(ctx, "dc_admins", &azuread.GroupArgs{
DisplayName: pulumi.String("aad-dc-administrators"),
SecurityEnabled: pulumi.Bool(true),
})
if err != nil {
return err
}
admin, err := azuread.NewUser(ctx, "admin", &azuread.UserArgs{
UserPrincipalName: pulumi.String("dc-admin@hashicorp-example.net"),
DisplayName: pulumi.String("DC Administrator"),
Password: pulumi.String("Pa55w0Rd!!1"),
})
if err != nil {
return err
}
_, err = azuread.NewGroupMember(ctx, "admin", &azuread.GroupMemberArgs{
GroupObjectId: dcAdmins.ObjectId,
MemberObjectId: admin.ObjectId,
})
if err != nil {
return err
}
example, err := azuread.NewServicePrincipal(ctx, "example", &azuread.ServicePrincipalArgs{
ApplicationId: pulumi.String("2565bd9d-da50-47d4-8b85-4c97f669dc36"),
})
if err != nil {
return err
}
aadds, err := core.NewResourceGroup(ctx, "aadds", &core.ResourceGroupArgs{
Name: pulumi.String("aadds-rg"),
Location: pulumi.String("westeurope"),
})
if err != nil {
return err
}
exampleService, err := domainservices.NewService(ctx, "example", &domainservices.ServiceArgs{
Name: pulumi.String("example-aadds"),
Location: aadds.Location,
ResourceGroupName: aadds.Name,
DomainName: pulumi.String("widgetslogin.net"),
Sku: pulumi.String("Enterprise"),
FilteredSyncEnabled: pulumi.Bool(false),
InitialReplicaSet: &domainservices.ServiceInitialReplicaSetArgs{
Location: primaryVirtualNetwork.Location,
SubnetId: primarySubnet.ID(),
},
Notifications: &domainservices.ServiceNotificationsArgs{
AdditionalRecipients: pulumi.StringArray{
pulumi.String("notifyA@example.net"),
pulumi.String("notifyB@example.org"),
},
NotifyDcAdmins: pulumi.Bool(true),
NotifyGlobalAdmins: pulumi.Bool(true),
},
Security: &domainservices.ServiceSecurityArgs{
SyncKerberosPasswords: pulumi.Bool(true),
SyncNtlmPasswords: pulumi.Bool(true),
SyncOnPremPasswords: pulumi.Bool(true),
},
Tags: pulumi.StringMap{
"Environment": pulumi.String("prod"),
},
}, pulumi.DependsOn([]pulumi.Resource{
example,
primarySubnetNetworkSecurityGroupAssociation,
}))
if err != nil {
return err
}
replica, err := core.NewResourceGroup(ctx, "replica", &core.ResourceGroupArgs{
Name: pulumi.String("aadds-replica-rg"),
Location: pulumi.String("North Europe"),
})
if err != nil {
return err
}
replicaVirtualNetwork, err := network.NewVirtualNetwork(ctx, "replica", &network.VirtualNetworkArgs{
Name: pulumi.String("aadds-replica-vnet"),
Location: replica.Location,
ResourceGroupName: replica.Name,
AddressSpaces: pulumi.StringArray{
pulumi.String("10.20.0.0/16"),
},
})
if err != nil {
return err
}
aaddsReplica, err := network.NewSubnet(ctx, "aadds_replica", &network.SubnetArgs{
Name: pulumi.String("aadds-replica-subnet"),
ResourceGroupName: replica.Name,
VirtualNetworkName: replicaVirtualNetwork.Name,
AddressPrefixes: pulumi.StringArray{
pulumi.String("10.20.0.0/24"),
},
})
if err != nil {
return err
}
aaddsReplicaNetworkSecurityGroup, err := network.NewNetworkSecurityGroup(ctx, "aadds_replica", &network.NetworkSecurityGroupArgs{
Name: pulumi.String("aadds-replica-nsg"),
Location: replica.Location,
ResourceGroupName: replica.Name,
SecurityRules: network.NetworkSecurityGroupSecurityRuleArray{
&network.NetworkSecurityGroupSecurityRuleArgs{
Name: pulumi.String("AllowSyncWithAzureAD"),
Priority: pulumi.Int(101),
Direction: pulumi.String("Inbound"),
Access: pulumi.String("Allow"),
Protocol: pulumi.String("Tcp"),
SourcePortRange: pulumi.String("*"),
DestinationPortRange: pulumi.String("443"),
SourceAddressPrefix: pulumi.String("AzureActiveDirectoryDomainServices"),
DestinationAddressPrefix: pulumi.String("*"),
},
&network.NetworkSecurityGroupSecurityRuleArgs{
Name: pulumi.String("AllowRD"),
Priority: pulumi.Int(201),
Direction: pulumi.String("Inbound"),
Access: pulumi.String("Allow"),
Protocol: pulumi.String("Tcp"),
SourcePortRange: pulumi.String("*"),
DestinationPortRange: pulumi.String("3389"),
SourceAddressPrefix: pulumi.String("CorpNetSaw"),
DestinationAddressPrefix: pulumi.String("*"),
},
&network.NetworkSecurityGroupSecurityRuleArgs{
Name: pulumi.String("AllowPSRemoting"),
Priority: pulumi.Int(301),
Direction: pulumi.String("Inbound"),
Access: pulumi.String("Allow"),
Protocol: pulumi.String("Tcp"),
SourcePortRange: pulumi.String("*"),
DestinationPortRange: pulumi.String("5986"),
SourceAddressPrefix: pulumi.String("AzureActiveDirectoryDomainServices"),
DestinationAddressPrefix: pulumi.String("*"),
},
&network.NetworkSecurityGroupSecurityRuleArgs{
Name: pulumi.String("AllowLDAPS"),
Priority: pulumi.Int(401),
Direction: pulumi.String("Inbound"),
Access: pulumi.String("Allow"),
Protocol: pulumi.String("Tcp"),
SourcePortRange: pulumi.String("*"),
DestinationPortRange: pulumi.String("636"),
SourceAddressPrefix: pulumi.String("*"),
DestinationAddressPrefix: pulumi.String("*"),
},
},
})
if err != nil {
return err
}
replicaSubnetNetworkSecurityGroupAssociation, err := network.NewSubnetNetworkSecurityGroupAssociation(ctx, "replica", &network.SubnetNetworkSecurityGroupAssociationArgs{
SubnetId: aaddsReplica.ID(),
NetworkSecurityGroupId: aaddsReplicaNetworkSecurityGroup.ID(),
})
if err != nil {
return err
}
primaryReplica, err := network.NewVirtualNetworkPeering(ctx, "primary_replica", &network.VirtualNetworkPeeringArgs{
Name: pulumi.String("aadds-primary-replica"),
ResourceGroupName: primaryVirtualNetwork.ResourceGroupName,
VirtualNetworkName: primaryVirtualNetwork.Name,
RemoteVirtualNetworkId: replicaVirtualNetwork.ID(),
AllowForwardedTraffic: pulumi.Bool(true),
AllowGatewayTransit: pulumi.Bool(false),
AllowVirtualNetworkAccess: pulumi.Bool(true),
UseRemoteGateways: pulumi.Bool(false),
})
if err != nil {
return err
}
replicaPrimary, err := network.NewVirtualNetworkPeering(ctx, "replica_primary", &network.VirtualNetworkPeeringArgs{
Name: pulumi.String("aadds-replica-primary"),
ResourceGroupName: replicaVirtualNetwork.ResourceGroupName,
VirtualNetworkName: replicaVirtualNetwork.Name,
RemoteVirtualNetworkId: primaryVirtualNetwork.ID(),
AllowForwardedTraffic: pulumi.Bool(true),
AllowGatewayTransit: pulumi.Bool(false),
AllowVirtualNetworkAccess: pulumi.Bool(true),
UseRemoteGateways: pulumi.Bool(false),
})
if err != nil {
return err
}
_, err = network.NewVirtualNetworkDnsServers(ctx, "replica", &network.VirtualNetworkDnsServersArgs{
VirtualNetworkId: replicaVirtualNetwork.ID(),
DnsServers: pulumi.StringArray(exampleService.InitialReplicaSet.ApplyT(func(initialReplicaSet domainservices.ServiceInitialReplicaSet) (interface{}, error) {
return initialReplicaSet.DomainControllerIpAddresses, nil
}).(pulumi.Interface{}Output)),
})
if err != nil {
return err
}
_, err = domainservices.NewReplicaSet(ctx, "replica", &domainservices.ReplicaSetArgs{
DomainServiceId: exampleService.ID(),
Location: replica.Location,
SubnetId: aaddsReplica.ID(),
}, pulumi.DependsOn([]pulumi.Resource{
replicaSubnetNetworkSecurityGroupAssociation,
primaryReplica,
replicaPrimary,
}))
if err != nil {
return err
}
return nil
})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.network.VirtualNetwork;
import com.pulumi.azure.network.VirtualNetworkArgs;
import com.pulumi.azure.network.Subnet;
import com.pulumi.azure.network.SubnetArgs;
import com.pulumi.azure.network.NetworkSecurityGroup;
import com.pulumi.azure.network.NetworkSecurityGroupArgs;
import com.pulumi.azure.network.inputs.NetworkSecurityGroupSecurityRuleArgs;
import com.pulumi.azure.network.SubnetNetworkSecurityGroupAssociation;
import com.pulumi.azure.network.SubnetNetworkSecurityGroupAssociationArgs;
import com.pulumi.azuread.Group;
import com.pulumi.azuread.GroupArgs;
import com.pulumi.azuread.User;
import com.pulumi.azuread.UserArgs;
import com.pulumi.azuread.GroupMember;
import com.pulumi.azuread.GroupMemberArgs;
import com.pulumi.azuread.ServicePrincipal;
import com.pulumi.azuread.ServicePrincipalArgs;
import com.pulumi.azure.domainservices.Service;
import com.pulumi.azure.domainservices.ServiceArgs;
import com.pulumi.azure.domainservices.inputs.ServiceInitialReplicaSetArgs;
import com.pulumi.azure.domainservices.inputs.ServiceNotificationsArgs;
import com.pulumi.azure.domainservices.inputs.ServiceSecurityArgs;
import com.pulumi.azure.network.VirtualNetworkPeering;
import com.pulumi.azure.network.VirtualNetworkPeeringArgs;
import com.pulumi.azure.network.VirtualNetworkDnsServers;
import com.pulumi.azure.network.VirtualNetworkDnsServersArgs;
import com.pulumi.azure.domainservices.ReplicaSet;
import com.pulumi.azure.domainservices.ReplicaSetArgs;
import com.pulumi.resources.CustomResourceOptions;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var primary = new ResourceGroup("primary", ResourceGroupArgs.builder()
            .name("aadds-primary-rg")
            .location("West Europe")
            .build());

        var primaryVirtualNetwork = new VirtualNetwork("primaryVirtualNetwork", VirtualNetworkArgs.builder()
            .name("aadds-primary-vnet")
            .location(primary.location())
            .resourceGroupName(primary.name())
            .addressSpaces("10.0.1.0/16")
            .build());

        var primarySubnet = new Subnet("primarySubnet", SubnetArgs.builder()
            .name("aadds-primary-subnet")
            .resourceGroupName(primary.name())
            .virtualNetworkName(primaryVirtualNetwork.name())
            .addressPrefixes("10.0.1.0/24")
            .build());

        var primaryNetworkSecurityGroup = new NetworkSecurityGroup("primaryNetworkSecurityGroup", NetworkSecurityGroupArgs.builder()
            .name("aadds-primary-nsg")
            .location(primary.location())
            .resourceGroupName(primary.name())
            .securityRules(            
                NetworkSecurityGroupSecurityRuleArgs.builder()
                    .name("AllowSyncWithAzureAD")
                    .priority(101)
                    .direction("Inbound")
                    .access("Allow")
                    .protocol("Tcp")
                    .sourcePortRange("*")
                    .destinationPortRange("443")
                    .sourceAddressPrefix("AzureActiveDirectoryDomainServices")
                    .destinationAddressPrefix("*")
                    .build(),
                NetworkSecurityGroupSecurityRuleArgs.builder()
                    .name("AllowRD")
                    .priority(201)
                    .direction("Inbound")
                    .access("Allow")
                    .protocol("Tcp")
                    .sourcePortRange("*")
                    .destinationPortRange("3389")
                    .sourceAddressPrefix("CorpNetSaw")
                    .destinationAddressPrefix("*")
                    .build(),
                NetworkSecurityGroupSecurityRuleArgs.builder()
                    .name("AllowPSRemoting")
                    .priority(301)
                    .direction("Inbound")
                    .access("Allow")
                    .protocol("Tcp")
                    .sourcePortRange("*")
                    .destinationPortRange("5986")
                    .sourceAddressPrefix("AzureActiveDirectoryDomainServices")
                    .destinationAddressPrefix("*")
                    .build(),
                NetworkSecurityGroupSecurityRuleArgs.builder()
                    .name("AllowLDAPS")
                    .priority(401)
                    .direction("Inbound")
                    .access("Allow")
                    .protocol("Tcp")
                    .sourcePortRange("*")
                    .destinationPortRange("636")
                    .sourceAddressPrefix("*")
                    .destinationAddressPrefix("*")
                    .build())
            .build());

        var primarySubnetNetworkSecurityGroupAssociation = new SubnetNetworkSecurityGroupAssociation("primarySubnetNetworkSecurityGroupAssociation", SubnetNetworkSecurityGroupAssociationArgs.builder()
            .subnetId(primarySubnet.id())
            .networkSecurityGroupId(primaryNetworkSecurityGroup.id())
            .build());

        var dcAdmins = new Group("dcAdmins", GroupArgs.builder()
            .displayName("aad-dc-administrators")
            .securityEnabled(true)
            .build());

        var admin = new User("admin", UserArgs.builder()
            .userPrincipalName("dc-admin@hashicorp-example.net")
            .displayName("DC Administrator")
            .password("Pa55w0Rd!!1")
            .build());

        var adminGroupMember = new GroupMember("adminGroupMember", GroupMemberArgs.builder()
            .groupObjectId(dcAdmins.objectId())
            .memberObjectId(admin.objectId())
            .build());

        var example = new ServicePrincipal("example", ServicePrincipalArgs.builder()
            .applicationId("2565bd9d-da50-47d4-8b85-4c97f669dc36")
            .build());

        var aadds = new ResourceGroup("aadds", ResourceGroupArgs.builder()
            .name("aadds-rg")
            .location("westeurope")
            .build());

        var exampleService = new Service("exampleService", ServiceArgs.builder()
            .name("example-aadds")
            .location(aadds.location())
            .resourceGroupName(aadds.name())
            .domainName("widgetslogin.net")
            .sku("Enterprise")
            .filteredSyncEnabled(false)
            .initialReplicaSet(ServiceInitialReplicaSetArgs.builder()
                .location(primaryVirtualNetwork.location())
                .subnetId(primarySubnet.id())
                .build())
            .notifications(ServiceNotificationsArgs.builder()
                .additionalRecipients(                
                    "notifyA@example.net",
                    "notifyB@example.org")
                .notifyDcAdmins(true)
                .notifyGlobalAdmins(true)
                .build())
            .security(ServiceSecurityArgs.builder()
                .syncKerberosPasswords(true)
                .syncNtlmPasswords(true)
                .syncOnPremPasswords(true)
                .build())
            .tags(Map.of("Environment", "prod"))
            .build(), CustomResourceOptions.builder()
                .dependsOn(                
                    example,
                    primarySubnetNetworkSecurityGroupAssociation)
                .build());

        var replica = new ResourceGroup("replica", ResourceGroupArgs.builder()
            .name("aadds-replica-rg")
            .location("North Europe")
            .build());

        var replicaVirtualNetwork = new VirtualNetwork("replicaVirtualNetwork", VirtualNetworkArgs.builder()
            .name("aadds-replica-vnet")
            .location(replica.location())
            .resourceGroupName(replica.name())
            .addressSpaces("10.20.0.0/16")
            .build());

        var aaddsReplica = new Subnet("aaddsReplica", SubnetArgs.builder()
            .name("aadds-replica-subnet")
            .resourceGroupName(replica.name())
            .virtualNetworkName(replicaVirtualNetwork.name())
            .addressPrefixes("10.20.0.0/24")
            .build());

        var aaddsReplicaNetworkSecurityGroup = new NetworkSecurityGroup("aaddsReplicaNetworkSecurityGroup", NetworkSecurityGroupArgs.builder()
            .name("aadds-replica-nsg")
            .location(replica.location())
            .resourceGroupName(replica.name())
            .securityRules(            
                NetworkSecurityGroupSecurityRuleArgs.builder()
                    .name("AllowSyncWithAzureAD")
                    .priority(101)
                    .direction("Inbound")
                    .access("Allow")
                    .protocol("Tcp")
                    .sourcePortRange("*")
                    .destinationPortRange("443")
                    .sourceAddressPrefix("AzureActiveDirectoryDomainServices")
                    .destinationAddressPrefix("*")
                    .build(),
                NetworkSecurityGroupSecurityRuleArgs.builder()
                    .name("AllowRD")
                    .priority(201)
                    .direction("Inbound")
                    .access("Allow")
                    .protocol("Tcp")
                    .sourcePortRange("*")
                    .destinationPortRange("3389")
                    .sourceAddressPrefix("CorpNetSaw")
                    .destinationAddressPrefix("*")
                    .build(),
                NetworkSecurityGroupSecurityRuleArgs.builder()
                    .name("AllowPSRemoting")
                    .priority(301)
                    .direction("Inbound")
                    .access("Allow")
                    .protocol("Tcp")
                    .sourcePortRange("*")
                    .destinationPortRange("5986")
                    .sourceAddressPrefix("AzureActiveDirectoryDomainServices")
                    .destinationAddressPrefix("*")
                    .build(),
                NetworkSecurityGroupSecurityRuleArgs.builder()
                    .name("AllowLDAPS")
                    .priority(401)
                    .direction("Inbound")
                    .access("Allow")
                    .protocol("Tcp")
                    .sourcePortRange("*")
                    .destinationPortRange("636")
                    .sourceAddressPrefix("*")
                    .destinationAddressPrefix("*")
                    .build())
            .build());

        var replicaSubnetNetworkSecurityGroupAssociation = new SubnetNetworkSecurityGroupAssociation("replicaSubnetNetworkSecurityGroupAssociation", SubnetNetworkSecurityGroupAssociationArgs.builder()
            .subnetId(aaddsReplica.id())
            .networkSecurityGroupId(aaddsReplicaNetworkSecurityGroup.id())
            .build());

        var primaryReplica = new VirtualNetworkPeering("primaryReplica", VirtualNetworkPeeringArgs.builder()
            .name("aadds-primary-replica")
            .resourceGroupName(primaryVirtualNetwork.resourceGroupName())
            .virtualNetworkName(primaryVirtualNetwork.name())
            .remoteVirtualNetworkId(replicaVirtualNetwork.id())
            .allowForwardedTraffic(true)
            .allowGatewayTransit(false)
            .allowVirtualNetworkAccess(true)
            .useRemoteGateways(false)
            .build());

        var replicaPrimary = new VirtualNetworkPeering("replicaPrimary", VirtualNetworkPeeringArgs.builder()
            .name("aadds-replica-primary")
            .resourceGroupName(replicaVirtualNetwork.resourceGroupName())
            .virtualNetworkName(replicaVirtualNetwork.name())
            .remoteVirtualNetworkId(primaryVirtualNetwork.id())
            .allowForwardedTraffic(true)
            .allowGatewayTransit(false)
            .allowVirtualNetworkAccess(true)
            .useRemoteGateways(false)
            .build());

        var replicaVirtualNetworkDnsServers = new VirtualNetworkDnsServers("replicaVirtualNetworkDnsServers", VirtualNetworkDnsServersArgs.builder()
            .virtualNetworkId(replicaVirtualNetwork.id())
            .dnsServers(exampleService.initialReplicaSet().applyValue(initialReplicaSet -> initialReplicaSet.domainControllerIpAddresses()))
            .build());

        var replicaReplicaSet = new ReplicaSet("replicaReplicaSet", ReplicaSetArgs.builder()
            .domainServiceId(exampleService.id())
            .location(replica.location())
            .subnetId(aaddsReplica.id())
            .build(), CustomResourceOptions.builder()
                .dependsOn(                
                    replicaSubnetNetworkSecurityGroupAssociation,
                    primaryReplica,
                    replicaPrimary)
                .build());

    }
}
```
```yaml
resources:
  primary:
    type: azure:core:ResourceGroup
    properties:
      name: aadds-primary-rg
      location: West Europe
  primaryVirtualNetwork:
    type: azure:network:VirtualNetwork
    name: primary
    properties:
      name: aadds-primary-vnet
      location: ${primary.location}
      resourceGroupName: ${primary.name}
      addressSpaces:
        - 10.0.1.0/16
  primarySubnet:
    type: azure:network:Subnet
    name: primary
    properties:
      name: aadds-primary-subnet
      resourceGroupName: ${primary.name}
      virtualNetworkName: ${primaryVirtualNetwork.name}
      addressPrefixes:
        - 10.0.1.0/24
  primaryNetworkSecurityGroup:
    type: azure:network:NetworkSecurityGroup
    name: primary
    properties:
      name: aadds-primary-nsg
      location: ${primary.location}
      resourceGroupName: ${primary.name}
      securityRules:
        - name: AllowSyncWithAzureAD
          priority: 101
          direction: Inbound
          access: Allow
          protocol: Tcp
          sourcePortRange: '*'
          destinationPortRange: '443'
          sourceAddressPrefix: AzureActiveDirectoryDomainServices
          destinationAddressPrefix: '*'
        - name: AllowRD
          priority: 201
          direction: Inbound
          access: Allow
          protocol: Tcp
          sourcePortRange: '*'
          destinationPortRange: '3389'
          sourceAddressPrefix: CorpNetSaw
          destinationAddressPrefix: '*'
        - name: AllowPSRemoting
          priority: 301
          direction: Inbound
          access: Allow
          protocol: Tcp
          sourcePortRange: '*'
          destinationPortRange: '5986'
          sourceAddressPrefix: AzureActiveDirectoryDomainServices
          destinationAddressPrefix: '*'
        - name: AllowLDAPS
          priority: 401
          direction: Inbound
          access: Allow
          protocol: Tcp
          sourcePortRange: '*'
          destinationPortRange: '636'
          sourceAddressPrefix: '*'
          destinationAddressPrefix: '*'
  primarySubnetNetworkSecurityGroupAssociation:
    type: azure:network:SubnetNetworkSecurityGroupAssociation
    name: primary
    properties:
      subnetId: ${primarySubnet.id}
      networkSecurityGroupId: ${primaryNetworkSecurityGroup.id}
  dcAdmins:
    type: azuread:Group
    name: dc_admins
    properties:
      displayName: aad-dc-administrators
      securityEnabled: true
  admin:
    type: azuread:User
    properties:
      userPrincipalName: dc-admin@hashicorp-example.net
      displayName: DC Administrator
      password: Pa55w0Rd!!1
  adminGroupMember:
    type: azuread:GroupMember
    name: admin
    properties:
      groupObjectId: ${dcAdmins.objectId}
      memberObjectId: ${admin.objectId}
  example:
    type: azuread:ServicePrincipal
    properties:
      applicationId: 2565bd9d-da50-47d4-8b85-4c97f669dc36
  aadds:
    type: azure:core:ResourceGroup
    properties:
      name: aadds-rg
      location: westeurope
  exampleService:
    type: azure:domainservices:Service
    name: example
    properties:
      name: example-aadds
      location: ${aadds.location}
      resourceGroupName: ${aadds.name}
      domainName: widgetslogin.net
      sku: Enterprise
      filteredSyncEnabled: false
      initialReplicaSet:
        location: ${primaryVirtualNetwork.location}
        subnetId: ${primarySubnet.id}
      notifications:
        additionalRecipients:
          - notifyA@example.net
          - notifyB@example.org
        notifyDcAdmins: true
        notifyGlobalAdmins: true
      security:
        syncKerberosPasswords: true
        syncNtlmPasswords: true
        syncOnPremPasswords: true
      tags:
        Environment: prod
    options:
      dependsOn:
        - ${example}
        - ${primarySubnetNetworkSecurityGroupAssociation}
  replica:
    type: azure:core:ResourceGroup
    properties:
      name: aadds-replica-rg
      location: North Europe
  replicaVirtualNetwork:
    type: azure:network:VirtualNetwork
    name: replica
    properties:
      name: aadds-replica-vnet
      location: ${replica.location}
      resourceGroupName: ${replica.name}
      addressSpaces:
        - 10.20.0.0/16
  aaddsReplica:
    type: azure:network:Subnet
    name: aadds_replica
    properties:
      name: aadds-replica-subnet
      resourceGroupName: ${replica.name}
      virtualNetworkName: ${replicaVirtualNetwork.name}
      addressPrefixes:
        - 10.20.0.0/24
  aaddsReplicaNetworkSecurityGroup:
    type: azure:network:NetworkSecurityGroup
    name: aadds_replica
    properties:
      name: aadds-replica-nsg
      location: ${replica.location}
      resourceGroupName: ${replica.name}
      securityRules:
        - name: AllowSyncWithAzureAD
          priority: 101
          direction: Inbound
          access: Allow
          protocol: Tcp
          sourcePortRange: '*'
          destinationPortRange: '443'
          sourceAddressPrefix: AzureActiveDirectoryDomainServices
          destinationAddressPrefix: '*'
        - name: AllowRD
          priority: 201
          direction: Inbound
          access: Allow
          protocol: Tcp
          sourcePortRange: '*'
          destinationPortRange: '3389'
          sourceAddressPrefix: CorpNetSaw
          destinationAddressPrefix: '*'
        - name: AllowPSRemoting
          priority: 301
          direction: Inbound
          access: Allow
          protocol: Tcp
          sourcePortRange: '*'
          destinationPortRange: '5986'
          sourceAddressPrefix: AzureActiveDirectoryDomainServices
          destinationAddressPrefix: '*'
        - name: AllowLDAPS
          priority: 401
          direction: Inbound
          access: Allow
          protocol: Tcp
          sourcePortRange: '*'
          destinationPortRange: '636'
          sourceAddressPrefix: '*'
          destinationAddressPrefix: '*'
  replicaSubnetNetworkSecurityGroupAssociation:
    type: azure:network:SubnetNetworkSecurityGroupAssociation
    name: replica
    properties:
      subnetId: ${aaddsReplica.id}
      networkSecurityGroupId: ${aaddsReplicaNetworkSecurityGroup.id}
  primaryReplica:
    type: azure:network:VirtualNetworkPeering
    name: primary_replica
    properties:
      name: aadds-primary-replica
      resourceGroupName: ${primaryVirtualNetwork.resourceGroupName}
      virtualNetworkName: ${primaryVirtualNetwork.name}
      remoteVirtualNetworkId: ${replicaVirtualNetwork.id}
      allowForwardedTraffic: true
      allowGatewayTransit: false
      allowVirtualNetworkAccess: true
      useRemoteGateways: false
  replicaPrimary:
    type: azure:network:VirtualNetworkPeering
    name: replica_primary
    properties:
      name: aadds-replica-primary
      resourceGroupName: ${replicaVirtualNetwork.resourceGroupName}
      virtualNetworkName: ${replicaVirtualNetwork.name}
      remoteVirtualNetworkId: ${primaryVirtualNetwork.id}
      allowForwardedTraffic: true
      allowGatewayTransit: false
      allowVirtualNetworkAccess: true
      useRemoteGateways: false
  replicaVirtualNetworkDnsServers:
    type: azure:network:VirtualNetworkDnsServers
    name: replica
    properties:
      virtualNetworkId: ${replicaVirtualNetwork.id}
      dnsServers: ${exampleService.initialReplicaSet.domainControllerIpAddresses}
  replicaReplicaSet:
    type: azure:domainservices:ReplicaSet
    name: replica
    properties:
      domainServiceId: ${exampleService.id}
      location: ${replica.location}
      subnetId: ${aaddsReplica.id}
    options:
      dependsOn:
        - ${replicaSubnetNetworkSecurityGroupAssociation}
        - ${primaryReplica}
        - ${replicaPrimary}
```
<!--End PulumiCodeChooser -->

## Import

Domain Service Replica Sets can be imported using the resource ID of the parent Domain Service and the Replica Set ID, e.g.

```sh
$ pulumi import azure:domainservices/replicaSet:ReplicaSet example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.AAD/domainServices/instance1/replicaSets/00000000-0000-0000-0000-000000000000
```

ç
domainServiceId" vThe ID of the Domain Service for which to create this Replica Set. Changing this forces a new resource to be created.
}
locationB" kThe Azure location where this Replica Set should exist. Changing this forces a new resource to be created.
|
subnetId" lThe ID of the subnet in which to place this Replica Set. Changing this forces a new resource to be created.
"Ç
domainControllerIpAddresses*" ]A list of subnet IP addresses for the domain controllers in this Replica Set, typically two.
"ç
domainServiceId" vThe ID of the Domain Service for which to create this Replica Set. Changing this forces a new resource to be created.
"p
externalAccessIpAddress" QThe publicly routable IP address for the domain controllers in this Replica Set.
"{
location" kThe Azure location where this Replica Set should exist. Changing this forces a new resource to be created.
"E
serviceStatus" 0The current service status for the replica set.
"|
subnetId" lThe ID of the subnet in which to place this Replica Set. Changing this forces a new resource to be created.
*˝î
?
domainservicesService$azure:domainservices/service:ServiceãÙ## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";
import * as azuread from "@pulumi/azuread";

const deploy = new azure.core.ResourceGroup("deploy", {
    name: "example-resources",
    location: "West Europe",
});
const deployVirtualNetwork = new azure.network.VirtualNetwork("deploy", {
    name: "deploy-vnet",
    location: deploy.location,
    resourceGroupName: deploy.name,
    addressSpaces: ["10.0.1.0/16"],
});
const deploySubnet = new azure.network.Subnet("deploy", {
    name: "deploy-subnet",
    resourceGroupName: deploy.name,
    virtualNetworkName: deployVirtualNetwork.name,
    addressPrefixes: ["10.0.1.0/24"],
});
const deployNetworkSecurityGroup = new azure.network.NetworkSecurityGroup("deploy", {
    name: "deploy-nsg",
    location: deploy.location,
    resourceGroupName: deploy.name,
    securityRules: [
        {
            name: "AllowSyncWithAzureAD",
            priority: 101,
            direction: "Inbound",
            access: "Allow",
            protocol: "Tcp",
            sourcePortRange: "*",
            destinationPortRange: "443",
            sourceAddressPrefix: "AzureActiveDirectoryDomainServices",
            destinationAddressPrefix: "*",
        },
        {
            name: "AllowRD",
            priority: 201,
            direction: "Inbound",
            access: "Allow",
            protocol: "Tcp",
            sourcePortRange: "*",
            destinationPortRange: "3389",
            sourceAddressPrefix: "CorpNetSaw",
            destinationAddressPrefix: "*",
        },
        {
            name: "AllowPSRemoting",
            priority: 301,
            direction: "Inbound",
            access: "Allow",
            protocol: "Tcp",
            sourcePortRange: "*",
            destinationPortRange: "5986",
            sourceAddressPrefix: "AzureActiveDirectoryDomainServices",
            destinationAddressPrefix: "*",
        },
        {
            name: "AllowLDAPS",
            priority: 401,
            direction: "Inbound",
            access: "Allow",
            protocol: "Tcp",
            sourcePortRange: "*",
            destinationPortRange: "636",
            sourceAddressPrefix: "*",
            destinationAddressPrefix: "*",
        },
    ],
});
const deploySubnetNetworkSecurityGroupAssociation = new azure.network.SubnetNetworkSecurityGroupAssociation("deploy", {
    subnetId: deploySubnet.id,
    networkSecurityGroupId: deployNetworkSecurityGroup.id,
});
const dcAdmins = new azuread.Group("dc_admins", {
    displayName: "AAD DC Administrators",
    securityEnabled: true,
});
const admin = new azuread.User("admin", {
    userPrincipalName: "dc-admin@hashicorp-example.com",
    displayName: "DC Administrator",
    password: "Pa55w0Rd!!1",
});
const adminGroupMember = new azuread.GroupMember("admin", {
    groupObjectId: dcAdmins.objectId,
    memberObjectId: admin.objectId,
});
const example = new azuread.ServicePrincipal("example", {applicationId: "2565bd9d-da50-47d4-8b85-4c97f669dc36"});
const aadds = new azure.core.ResourceGroup("aadds", {
    name: "aadds-rg",
    location: "westeurope",
});
const exampleService = new azure.domainservices.Service("example", {
    name: "example-aadds",
    location: aadds.location,
    resourceGroupName: aadds.name,
    domainName: "widgetslogin.net",
    sku: "Enterprise",
    filteredSyncEnabled: false,
    initialReplicaSet: {
        subnetId: deploySubnet.id,
    },
    notifications: {
        additionalRecipients: [
            "notifyA@example.net",
            "notifyB@example.org",
        ],
        notifyDcAdmins: true,
        notifyGlobalAdmins: true,
    },
    security: {
        syncKerberosPasswords: true,
        syncNtlmPasswords: true,
        syncOnPremPasswords: true,
    },
    tags: {
        Environment: "prod",
    },
}, {
    dependsOn: [
        example,
        deploySubnetNetworkSecurityGroupAssociation,
    ],
});
```
```python
import pulumi
import pulumi_azure as azure
import pulumi_azuread as azuread

deploy = azure.core.ResourceGroup("deploy",
    name="example-resources",
    location="West Europe")
deploy_virtual_network = azure.network.VirtualNetwork("deploy",
    name="deploy-vnet",
    location=deploy.location,
    resource_group_name=deploy.name,
    address_spaces=["10.0.1.0/16"])
deploy_subnet = azure.network.Subnet("deploy",
    name="deploy-subnet",
    resource_group_name=deploy.name,
    virtual_network_name=deploy_virtual_network.name,
    address_prefixes=["10.0.1.0/24"])
deploy_network_security_group = azure.network.NetworkSecurityGroup("deploy",
    name="deploy-nsg",
    location=deploy.location,
    resource_group_name=deploy.name,
    security_rules=[
        {
            "name": "AllowSyncWithAzureAD",
            "priority": 101,
            "direction": "Inbound",
            "access": "Allow",
            "protocol": "Tcp",
            "source_port_range": "*",
            "destination_port_range": "443",
            "source_address_prefix": "AzureActiveDirectoryDomainServices",
            "destination_address_prefix": "*",
        },
        {
            "name": "AllowRD",
            "priority": 201,
            "direction": "Inbound",
            "access": "Allow",
            "protocol": "Tcp",
            "source_port_range": "*",
            "destination_port_range": "3389",
            "source_address_prefix": "CorpNetSaw",
            "destination_address_prefix": "*",
        },
        {
            "name": "AllowPSRemoting",
            "priority": 301,
            "direction": "Inbound",
            "access": "Allow",
            "protocol": "Tcp",
            "source_port_range": "*",
            "destination_port_range": "5986",
            "source_address_prefix": "AzureActiveDirectoryDomainServices",
            "destination_address_prefix": "*",
        },
        {
            "name": "AllowLDAPS",
            "priority": 401,
            "direction": "Inbound",
            "access": "Allow",
            "protocol": "Tcp",
            "source_port_range": "*",
            "destination_port_range": "636",
            "source_address_prefix": "*",
            "destination_address_prefix": "*",
        },
    ])
deploy_subnet_network_security_group_association = azure.network.SubnetNetworkSecurityGroupAssociation("deploy",
    subnet_id=deploy_subnet.id,
    network_security_group_id=deploy_network_security_group.id)
dc_admins = azuread.Group("dc_admins",
    display_name="AAD DC Administrators",
    security_enabled=True)
admin = azuread.User("admin",
    user_principal_name="dc-admin@hashicorp-example.com",
    display_name="DC Administrator",
    password="Pa55w0Rd!!1")
admin_group_member = azuread.GroupMember("admin",
    group_object_id=dc_admins.object_id,
    member_object_id=admin.object_id)
example = azuread.ServicePrincipal("example", application_id="2565bd9d-da50-47d4-8b85-4c97f669dc36")
aadds = azure.core.ResourceGroup("aadds",
    name="aadds-rg",
    location="westeurope")
example_service = azure.domainservices.Service("example",
    name="example-aadds",
    location=aadds.location,
    resource_group_name=aadds.name,
    domain_name="widgetslogin.net",
    sku="Enterprise",
    filtered_sync_enabled=False,
    initial_replica_set={
        "subnet_id": deploy_subnet.id,
    },
    notifications={
        "additional_recipients": [
            "notifyA@example.net",
            "notifyB@example.org",
        ],
        "notify_dc_admins": True,
        "notify_global_admins": True,
    },
    security={
        "sync_kerberos_passwords": True,
        "sync_ntlm_passwords": True,
        "sync_on_prem_passwords": True,
    },
    tags={
        "Environment": "prod",
    },
    opts = pulumi.ResourceOptions(depends_on=[
            example,
            deploy_subnet_network_security_group_association,
        ]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;
using AzureAD = Pulumi.AzureAD;

return await Deployment.RunAsync(() => 
{
    var deploy = new Azure.Core.ResourceGroup("deploy", new()
    {
        Name = "example-resources",
        Location = "West Europe",
    });

    var deployVirtualNetwork = new Azure.Network.VirtualNetwork("deploy", new()
    {
        Name = "deploy-vnet",
        Location = deploy.Location,
        ResourceGroupName = deploy.Name,
        AddressSpaces = new[]
        {
            "10.0.1.0/16",
        },
    });

    var deploySubnet = new Azure.Network.Subnet("deploy", new()
    {
        Name = "deploy-subnet",
        ResourceGroupName = deploy.Name,
        VirtualNetworkName = deployVirtualNetwork.Name,
        AddressPrefixes = new[]
        {
            "10.0.1.0/24",
        },
    });

    var deployNetworkSecurityGroup = new Azure.Network.NetworkSecurityGroup("deploy", new()
    {
        Name = "deploy-nsg",
        Location = deploy.Location,
        ResourceGroupName = deploy.Name,
        SecurityRules = new[]
        {
            new Azure.Network.Inputs.NetworkSecurityGroupSecurityRuleArgs
            {
                Name = "AllowSyncWithAzureAD",
                Priority = 101,
                Direction = "Inbound",
                Access = "Allow",
                Protocol = "Tcp",
                SourcePortRange = "*",
                DestinationPortRange = "443",
                SourceAddressPrefix = "AzureActiveDirectoryDomainServices",
                DestinationAddressPrefix = "*",
            },
            new Azure.Network.Inputs.NetworkSecurityGroupSecurityRuleArgs
            {
                Name = "AllowRD",
                Priority = 201,
                Direction = "Inbound",
                Access = "Allow",
                Protocol = "Tcp",
                SourcePortRange = "*",
                DestinationPortRange = "3389",
                SourceAddressPrefix = "CorpNetSaw",
                DestinationAddressPrefix = "*",
            },
            new Azure.Network.Inputs.NetworkSecurityGroupSecurityRuleArgs
            {
                Name = "AllowPSRemoting",
                Priority = 301,
                Direction = "Inbound",
                Access = "Allow",
                Protocol = "Tcp",
                SourcePortRange = "*",
                DestinationPortRange = "5986",
                SourceAddressPrefix = "AzureActiveDirectoryDomainServices",
                DestinationAddressPrefix = "*",
            },
            new Azure.Network.Inputs.NetworkSecurityGroupSecurityRuleArgs
            {
                Name = "AllowLDAPS",
                Priority = 401,
                Direction = "Inbound",
                Access = "Allow",
                Protocol = "Tcp",
                SourcePortRange = "*",
                DestinationPortRange = "636",
                SourceAddressPrefix = "*",
                DestinationAddressPrefix = "*",
            },
        },
    });

    var deploySubnetNetworkSecurityGroupAssociation = new Azure.Network.SubnetNetworkSecurityGroupAssociation("deploy", new()
    {
        SubnetId = deploySubnet.Id,
        NetworkSecurityGroupId = deployNetworkSecurityGroup.Id,
    });

    var dcAdmins = new AzureAD.Group("dc_admins", new()
    {
        DisplayName = "AAD DC Administrators",
        SecurityEnabled = true,
    });

    var admin = new AzureAD.User("admin", new()
    {
        UserPrincipalName = "dc-admin@hashicorp-example.com",
        DisplayName = "DC Administrator",
        Password = "Pa55w0Rd!!1",
    });

    var adminGroupMember = new AzureAD.GroupMember("admin", new()
    {
        GroupObjectId = dcAdmins.ObjectId,
        MemberObjectId = admin.ObjectId,
    });

    var example = new AzureAD.ServicePrincipal("example", new()
    {
        ApplicationId = "2565bd9d-da50-47d4-8b85-4c97f669dc36",
    });

    var aadds = new Azure.Core.ResourceGroup("aadds", new()
    {
        Name = "aadds-rg",
        Location = "westeurope",
    });

    var exampleService = new Azure.DomainServices.Service("example", new()
    {
        Name = "example-aadds",
        Location = aadds.Location,
        ResourceGroupName = aadds.Name,
        DomainName = "widgetslogin.net",
        Sku = "Enterprise",
        FilteredSyncEnabled = false,
        InitialReplicaSet = new Azure.DomainServices.Inputs.ServiceInitialReplicaSetArgs
        {
            SubnetId = deploySubnet.Id,
        },
        Notifications = new Azure.DomainServices.Inputs.ServiceNotificationsArgs
        {
            AdditionalRecipients = new[]
            {
                "notifyA@example.net",
                "notifyB@example.org",
            },
            NotifyDcAdmins = true,
            NotifyGlobalAdmins = true,
        },
        Security = new Azure.DomainServices.Inputs.ServiceSecurityArgs
        {
            SyncKerberosPasswords = true,
            SyncNtlmPasswords = true,
            SyncOnPremPasswords = true,
        },
        Tags = 
        {
            { "Environment", "prod" },
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            example,
            deploySubnetNetworkSecurityGroupAssociation,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/domainservices"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/network"
	"github.com/pulumi/pulumi-azuread/sdk/v5/go/azuread"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		deploy, err := core.NewResourceGroup(ctx, "deploy", &core.ResourceGroupArgs{
			Name:     pulumi.String("example-resources"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		deployVirtualNetwork, err := network.NewVirtualNetwork(ctx, "deploy", &network.VirtualNetworkArgs{
			Name:              pulumi.String("deploy-vnet"),
			Location:          deploy.Location,
			ResourceGroupName: deploy.Name,
			AddressSpaces: pulumi.StringArray{
				pulumi.String("10.0.1.0/16"),
			},
		})
		if err != nil {
			return err
		}
		deploySubnet, err := network.NewSubnet(ctx, "deploy", &network.SubnetArgs{
			Name:               pulumi.String("deploy-subnet"),
			ResourceGroupName:  deploy.Name,
			VirtualNetworkName: deployVirtualNetwork.Name,
			AddressPrefixes: pulumi.StringArray{
				pulumi.String("10.0.1.0/24"),
			},
		})
		if err != nil {
			return err
		}
		deployNetworkSecurityGroup, err := network.NewNetworkSecurityGroup(ctx, "deploy", &network.NetworkSecurityGroupArgs{
			Name:              pulumi.String("deploy-nsg"),
			Location:          deploy.Location,
			ResourceGroupName: deploy.Name,
			SecurityRules: network.NetworkSecurityGroupSecurityRuleArray{
				&network.NetworkSecurityGroupSecurityRuleArgs{
					Name:                     pulumi.String("AllowSyncWithAzureAD"),
					Priority:                 pulumi.Int(101),
					Direction:                pulumi.String("Inbound"),
					Access:                   pulumi.String("Allow"),
					Protocol:                 pulumi.String("Tcp"),
					SourcePortRange:          pulumi.String("*"),
					DestinationPortRange:     pulumi.String("443"),
					SourceAddressPrefix:      pulumi.String("AzureActiveDirectoryDomainServices"),
					DestinationAddressPrefix: pulumi.String("*"),
				},
				&network.NetworkSecurityGroupSecurityRuleArgs{
					Name:                     pulumi.String("AllowRD"),
					Priority:                 pulumi.Int(201),
					Direction:                pulumi.String("Inbound"),
					Access:                   pulumi.String("Allow"),
					Protocol:                 pulumi.String("Tcp"),
					SourcePortRange:          pulumi.String("*"),
					DestinationPortRange:     pulumi.String("3389"),
					SourceAddressPrefix:      pulumi.String("CorpNetSaw"),
					DestinationAddressPrefix: pulumi.String("*"),
				},
				&network.NetworkSecurityGroupSecurityRuleArgs{
					Name:                     pulumi.String("AllowPSRemoting"),
					Priority:                 pulumi.Int(301),
					Direction:                pulumi.String("Inbound"),
					Access:                   pulumi.String("Allow"),
					Protocol:                 pulumi.String("Tcp"),
					SourcePortRange:          pulumi.String("*"),
					DestinationPortRange:     pulumi.String("5986"),
					SourceAddressPrefix:      pulumi.String("AzureActiveDirectoryDomainServices"),
					DestinationAddressPrefix: pulumi.String("*"),
				},
				&network.NetworkSecurityGroupSecurityRuleArgs{
					Name:                     pulumi.String("AllowLDAPS"),
					Priority:                 pulumi.Int(401),
					Direction:                pulumi.String("Inbound"),
					Access:                   pulumi.String("Allow"),
					Protocol:                 pulumi.String("Tcp"),
					SourcePortRange:          pulumi.String("*"),
					DestinationPortRange:     pulumi.String("636"),
					SourceAddressPrefix:      pulumi.String("*"),
					DestinationAddressPrefix: pulumi.String("*"),
				},
			},
		})
		if err != nil {
			return err
		}
		deploySubnetNetworkSecurityGroupAssociation, err := network.NewSubnetNetworkSecurityGroupAssociation(ctx, "deploy", &network.SubnetNetworkSecurityGroupAssociationArgs{
			SubnetId:               deploySubnet.ID(),
			NetworkSecurityGroupId: deployNetworkSecurityGroup.ID(),
		})
		if err != nil {
			return err
		}
		dcAdmins, err := azuread.NewGroup(ctx, "dc_admins", &azuread.GroupArgs{
			DisplayName:     pulumi.String("AAD DC Administrators"),
			SecurityEnabled: pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		admin, err := azuread.NewUser(ctx, "admin", &azuread.UserArgs{
			UserPrincipalName: pulumi.String("dc-admin@hashicorp-example.com"),
			DisplayName:       pulumi.String("DC Administrator"),
			Password:          pulumi.String("Pa55w0Rd!!1"),
		})
		if err != nil {
			return err
		}
		_, err = azuread.NewGroupMember(ctx, "admin", &azuread.GroupMemberArgs{
			GroupObjectId:  dcAdmins.ObjectId,
			MemberObjectId: admin.ObjectId,
		})
		if err != nil {
			return err
		}
		example, err := azuread.NewServicePrincipal(ctx, "example", &azuread.ServicePrincipalArgs{
			ApplicationId: pulumi.String("2565bd9d-da50-47d4-8b85-4c97f669dc36"),
		})
		if err != nil {
			return err
		}
		aadds, err := core.NewResourceGroup(ctx, "aadds", &core.ResourceGroupArgs{
			Name:     pulumi.String("aadds-rg"),
			Location: pulumi.String("westeurope"),
		})
		if err != nil {
			return err
		}
		_, err = domainservices.NewService(ctx, "example", &domainservices.ServiceArgs{
			Name:                pulumi.String("example-aadds"),
			Location:            aadds.Location,
			ResourceGroupName:   aadds.Name,
			DomainName:          pulumi.String("widgetslogin.net"),
			Sku:                 pulumi.String("Enterprise"),
			FilteredSyncEnabled: pulumi.Bool(false),
			InitialReplicaSet: &domainservices.ServiceInitialReplicaSetArgs{
				SubnetId: deploySubnet.ID(),
			},
			Notifications: &domainservices.ServiceNotificationsArgs{
				AdditionalRecipients: pulumi.StringArray{
					pulumi.String("notifyA@example.net"),
					pulumi.String("notifyB@example.org"),
				},
				NotifyDcAdmins:     pulumi.Bool(true),
				NotifyGlobalAdmins: pulumi.Bool(true),
			},
			Security: &domainservices.ServiceSecurityArgs{
				SyncKerberosPasswords: pulumi.Bool(true),
				SyncNtlmPasswords:     pulumi.Bool(true),
				SyncOnPremPasswords:   pulumi.Bool(true),
			},
			Tags: pulumi.StringMap{
				"Environment": pulumi.String("prod"),
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			example,
			deploySubnetNetworkSecurityGroupAssociation,
		}))
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.network.VirtualNetwork;
import com.pulumi.azure.network.VirtualNetworkArgs;
import com.pulumi.azure.network.Subnet;
import com.pulumi.azure.network.SubnetArgs;
import com.pulumi.azure.network.NetworkSecurityGroup;
import com.pulumi.azure.network.NetworkSecurityGroupArgs;
import com.pulumi.azure.network.inputs.NetworkSecurityGroupSecurityRuleArgs;
import com.pulumi.azure.network.SubnetNetworkSecurityGroupAssociation;
import com.pulumi.azure.network.SubnetNetworkSecurityGroupAssociationArgs;
import com.pulumi.azuread.Group;
import com.pulumi.azuread.GroupArgs;
import com.pulumi.azuread.User;
import com.pulumi.azuread.UserArgs;
import com.pulumi.azuread.GroupMember;
import com.pulumi.azuread.GroupMemberArgs;
import com.pulumi.azuread.ServicePrincipal;
import com.pulumi.azuread.ServicePrincipalArgs;
import com.pulumi.azure.domainservices.Service;
import com.pulumi.azure.domainservices.ServiceArgs;
import com.pulumi.azure.domainservices.inputs.ServiceInitialReplicaSetArgs;
import com.pulumi.azure.domainservices.inputs.ServiceNotificationsArgs;
import com.pulumi.azure.domainservices.inputs.ServiceSecurityArgs;
import com.pulumi.resources.CustomResourceOptions;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var deploy = new ResourceGroup("deploy", ResourceGroupArgs.builder()
            .name("example-resources")
            .location("West Europe")
            .build());

        var deployVirtualNetwork = new VirtualNetwork("deployVirtualNetwork", VirtualNetworkArgs.builder()
            .name("deploy-vnet")
            .location(deploy.location())
            .resourceGroupName(deploy.name())
            .addressSpaces("10.0.1.0/16")
            .build());

        var deploySubnet = new Subnet("deploySubnet", SubnetArgs.builder()
            .name("deploy-subnet")
            .resourceGroupName(deploy.name())
            .virtualNetworkName(deployVirtualNetwork.name())
            .addressPrefixes("10.0.1.0/24")
            .build());

        var deployNetworkSecurityGroup = new NetworkSecurityGroup("deployNetworkSecurityGroup", NetworkSecurityGroupArgs.builder()
            .name("deploy-nsg")
            .location(deploy.location())
            .resourceGroupName(deploy.name())
            .securityRules(            
                NetworkSecurityGroupSecurityRuleArgs.builder()
                    .name("AllowSyncWithAzureAD")
                    .priority(101)
                    .direction("Inbound")
                    .access("Allow")
                    .protocol("Tcp")
                    .sourcePortRange("*")
                    .destinationPortRange("443")
                    .sourceAddressPrefix("AzureActiveDirectoryDomainServices")
                    .destinationAddressPrefix("*")
                    .build(),
                NetworkSecurityGroupSecurityRuleArgs.builder()
                    .name("AllowRD")
                    .priority(201)
                    .direction("Inbound")
                    .access("Allow")
                    .protocol("Tcp")
                    .sourcePortRange("*")
                    .destinationPortRange("3389")
                    .sourceAddressPrefix("CorpNetSaw")
                    .destinationAddressPrefix("*")
                    .build(),
                NetworkSecurityGroupSecurityRuleArgs.builder()
                    .name("AllowPSRemoting")
                    .priority(301)
                    .direction("Inbound")
                    .access("Allow")
                    .protocol("Tcp")
                    .sourcePortRange("*")
                    .destinationPortRange("5986")
                    .sourceAddressPrefix("AzureActiveDirectoryDomainServices")
                    .destinationAddressPrefix("*")
                    .build(),
                NetworkSecurityGroupSecurityRuleArgs.builder()
                    .name("AllowLDAPS")
                    .priority(401)
                    .direction("Inbound")
                    .access("Allow")
                    .protocol("Tcp")
                    .sourcePortRange("*")
                    .destinationPortRange("636")
                    .sourceAddressPrefix("*")
                    .destinationAddressPrefix("*")
                    .build())
            .build());

        var deploySubnetNetworkSecurityGroupAssociation = new SubnetNetworkSecurityGroupAssociation("deploySubnetNetworkSecurityGroupAssociation", SubnetNetworkSecurityGroupAssociationArgs.builder()
            .subnetId(deploySubnet.id())
            .networkSecurityGroupId(deployNetworkSecurityGroup.id())
            .build());

        var dcAdmins = new Group("dcAdmins", GroupArgs.builder()
            .displayName("AAD DC Administrators")
            .securityEnabled(true)
            .build());

        var admin = new User("admin", UserArgs.builder()
            .userPrincipalName("dc-admin@hashicorp-example.com")
            .displayName("DC Administrator")
            .password("Pa55w0Rd!!1")
            .build());

        var adminGroupMember = new GroupMember("adminGroupMember", GroupMemberArgs.builder()
            .groupObjectId(dcAdmins.objectId())
            .memberObjectId(admin.objectId())
            .build());

        var example = new ServicePrincipal("example", ServicePrincipalArgs.builder()
            .applicationId("2565bd9d-da50-47d4-8b85-4c97f669dc36")
            .build());

        var aadds = new ResourceGroup("aadds", ResourceGroupArgs.builder()
            .name("aadds-rg")
            .location("westeurope")
            .build());

        var exampleService = new Service("exampleService", ServiceArgs.builder()
            .name("example-aadds")
            .location(aadds.location())
            .resourceGroupName(aadds.name())
            .domainName("widgetslogin.net")
            .sku("Enterprise")
            .filteredSyncEnabled(false)
            .initialReplicaSet(ServiceInitialReplicaSetArgs.builder()
                .subnetId(deploySubnet.id())
                .build())
            .notifications(ServiceNotificationsArgs.builder()
                .additionalRecipients(                
                    "notifyA@example.net",
                    "notifyB@example.org")
                .notifyDcAdmins(true)
                .notifyGlobalAdmins(true)
                .build())
            .security(ServiceSecurityArgs.builder()
                .syncKerberosPasswords(true)
                .syncNtlmPasswords(true)
                .syncOnPremPasswords(true)
                .build())
            .tags(Map.of("Environment", "prod"))
            .build(), CustomResourceOptions.builder()
                .dependsOn(                
                    example,
                    deploySubnetNetworkSecurityGroupAssociation)
                .build());

    }
}
```
```yaml
resources:
  deploy:
    type: azure:core:ResourceGroup
    properties:
      name: example-resources
      location: West Europe
  deployVirtualNetwork:
    type: azure:network:VirtualNetwork
    name: deploy
    properties:
      name: deploy-vnet
      location: ${deploy.location}
      resourceGroupName: ${deploy.name}
      addressSpaces:
        - 10.0.1.0/16
  deploySubnet:
    type: azure:network:Subnet
    name: deploy
    properties:
      name: deploy-subnet
      resourceGroupName: ${deploy.name}
      virtualNetworkName: ${deployVirtualNetwork.name}
      addressPrefixes:
        - 10.0.1.0/24
  deployNetworkSecurityGroup:
    type: azure:network:NetworkSecurityGroup
    name: deploy
    properties:
      name: deploy-nsg
      location: ${deploy.location}
      resourceGroupName: ${deploy.name}
      securityRules:
        - name: AllowSyncWithAzureAD
          priority: 101
          direction: Inbound
          access: Allow
          protocol: Tcp
          sourcePortRange: '*'
          destinationPortRange: '443'
          sourceAddressPrefix: AzureActiveDirectoryDomainServices
          destinationAddressPrefix: '*'
        - name: AllowRD
          priority: 201
          direction: Inbound
          access: Allow
          protocol: Tcp
          sourcePortRange: '*'
          destinationPortRange: '3389'
          sourceAddressPrefix: CorpNetSaw
          destinationAddressPrefix: '*'
        - name: AllowPSRemoting
          priority: 301
          direction: Inbound
          access: Allow
          protocol: Tcp
          sourcePortRange: '*'
          destinationPortRange: '5986'
          sourceAddressPrefix: AzureActiveDirectoryDomainServices
          destinationAddressPrefix: '*'
        - name: AllowLDAPS
          priority: 401
          direction: Inbound
          access: Allow
          protocol: Tcp
          sourcePortRange: '*'
          destinationPortRange: '636'
          sourceAddressPrefix: '*'
          destinationAddressPrefix: '*'
  deploySubnetNetworkSecurityGroupAssociation:
    type: azure:network:SubnetNetworkSecurityGroupAssociation
    name: deploy
    properties:
      subnetId: ${deploySubnet.id}
      networkSecurityGroupId: ${deployNetworkSecurityGroup.id}
  dcAdmins:
    type: azuread:Group
    name: dc_admins
    properties:
      displayName: AAD DC Administrators
      securityEnabled: true
  admin:
    type: azuread:User
    properties:
      userPrincipalName: dc-admin@hashicorp-example.com
      displayName: DC Administrator
      password: Pa55w0Rd!!1
  adminGroupMember:
    type: azuread:GroupMember
    name: admin
    properties:
      groupObjectId: ${dcAdmins.objectId}
      memberObjectId: ${admin.objectId}
  example:
    type: azuread:ServicePrincipal
    properties:
      applicationId: 2565bd9d-da50-47d4-8b85-4c97f669dc36
  aadds:
    type: azure:core:ResourceGroup
    properties:
      name: aadds-rg
      location: westeurope
  exampleService:
    type: azure:domainservices:Service
    name: example
    properties:
      name: example-aadds
      location: ${aadds.location}
      resourceGroupName: ${aadds.name}
      domainName: widgetslogin.net
      sku: Enterprise
      filteredSyncEnabled: false
      initialReplicaSet:
        subnetId: ${deploySubnet.id}
      notifications:
        additionalRecipients:
          - notifyA@example.net
          - notifyB@example.org
        notifyDcAdmins: true
        notifyGlobalAdmins: true
      security:
        syncKerberosPasswords: true
        syncNtlmPasswords: true
        syncOnPremPasswords: true
      tags:
        Environment: prod
    options:
      dependsOn:
        - ${example}
        - ${deploySubnetNetworkSecurityGroupAssociation}
```
<!--End PulumiCodeChooser -->

## Import

Domain Services can be imported using the resource ID, together with the Replica Set ID that you wish to designate as the initial replica set, e.g.

```sh
$ pulumi import azure:domainservices/service:Service example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.AAD/domainServices/instance1/initialReplicaSetId/00000000-0000-0000-0000-000000000000
```

«
domainConfigurationTypeB" •The configuration type of this Active Directory Domain. Possible values are `FullySynced` and `ResourceTrusting`. Changing this forces a new resource to be created.
†

domainName" çThe Active Directory domain to use. See [official documentation](https://docs.microsoft.com/azure/active-directory-domain-services/tutorial-create-instance#create-a-managed-domain) for constraints and recommendations. Changing this forces a new resource to be created.
Ñ
filteredSyncEnabledB
 gWhether to enable group-based filtered sync (also called scoped synchronisation). Defaults to `false`.
í
initialReplicaSetv:t
r
domainservicesServiceInitialReplicaSetFazure:domainservices/ServiceInitialReplicaSet:ServiceInitialReplicaSetÑAn `initial_replica_set` block as defined below. The initial replica set inherits the same location as the Domain Service resource.
y
locationB" gThe Azure location where the Domain Service exists. Changing this forces a new resource to be created.
ç
nameB" The display name for your managed Active Directory Domain Service resource. Changing this forces a new resource to be created.
©
notificationslBj:h
f
domainservicesServiceNotifications>azure:domainservices/ServiceNotifications:ServiceNotifications*A `notifications` block as defined below.
ï
resourceGroupName" |The name of the Resource Group in which the Domain Service should exist. Changing this forces a new resource to be created.
õ

secureLdapcBa:_
]
domainservicesServiceSecureLdap8azure:domainservices/ServiceSecureLdap:ServiceSecureLdap(A `secure_ldap` block as defined below.
ê
security]B[:Y
W
domainservicesServiceSecurity4azure:domainservices/ServiceSecurity:ServiceSecurity%A `security` block as defined below.
w
sku" lThe SKU to use when provisioning the Domain Service resource. One of `Standard`, `Enterprise` or `Premium`.
<
tagsB2" ,A mapping of tags assigned to the resource.
"C
deploymentId" /A unique ID for the managed domain deployment.
"«
domainConfigurationTypeB" •The configuration type of this Active Directory Domain. Possible values are `FullySynced` and `ResourceTrusting`. Changing this forces a new resource to be created.
"†

domainName" çThe Active Directory domain to use. See [official documentation](https://docs.microsoft.com/azure/active-directory-domain-services/tutorial-create-instance#create-a-managed-domain) for constraints and recommendations. Changing this forces a new resource to be created.
"Ñ
filteredSyncEnabledB
 gWhether to enable group-based filtered sync (also called scoped synchronisation). Defaults to `false`.
"í
initialReplicaSetv:t
r
domainservicesServiceInitialReplicaSetFazure:domainservices/ServiceInitialReplicaSet:ServiceInitialReplicaSetÑAn `initial_replica_set` block as defined below. The initial replica set inherits the same location as the Domain Service resource.
"w
location" gThe Azure location where the Domain Service exists. Changing this forces a new resource to be created.
"ã
name" The display name for your managed Active Directory Domain Service resource. Changing this forces a new resource to be created.
"ß
notificationsj:h
f
domainservicesServiceNotifications>azure:domainservices/ServiceNotifications:ServiceNotifications*A `notifications` block as defined below.
"ï
resourceGroupName" |The name of the Resource Group in which the Domain Service should exist. Changing this forces a new resource to be created.
"@

resourceId" .The Azure resource ID for the domain service.
"ô

secureLdapa:_
]
domainservicesServiceSecureLdap8azure:domainservices/ServiceSecureLdap:ServiceSecureLdap(A `secure_ldap` block as defined below.
"é
security[:Y
W
domainservicesServiceSecurity4azure:domainservices/ServiceSecurity:ServiceSecurity%A `security` block as defined below.
"w
sku" lThe SKU to use when provisioning the Domain Service resource. One of `Standard`, `Enterprise` or `Premium`.
"
	syncOwner" "<
tagsB2" ,A mapping of tags assigned to the resource.
"
tenantId" "
version *à0
N
domainservicesServiceTrust.azure:domainservices/serviceTrust:ServiceTrustó%Manages a Active Directory Domain Service Trust.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = azure.domainservices.getService({
    name: "example-ds",
    resourceGroupName: "example-rg",
});
const exampleServiceTrust = new azure.domainservices.ServiceTrust("example", {
    name: "example-trust",
    domainServiceId: example.then(example => example.id),
    trustedDomainFqdn: "example.com",
    trustedDomainDnsIps: [
        "10.1.0.3",
        "10.1.0.4",
    ],
    password: "Password123",
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.domainservices.get_service(name="example-ds",
    resource_group_name="example-rg")
example_service_trust = azure.domainservices.ServiceTrust("example",
    name="example-trust",
    domain_service_id=example.id,
    trusted_domain_fqdn="example.com",
    trusted_domain_dns_ips=[
        "10.1.0.3",
        "10.1.0.4",
    ],
    password="Password123")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = Azure.DomainServices.GetService.Invoke(new()
    {
        Name = "example-ds",
        ResourceGroupName = "example-rg",
    });

    var exampleServiceTrust = new Azure.DomainServices.ServiceTrust("example", new()
    {
        Name = "example-trust",
        DomainServiceId = example.Apply(getServiceResult => getServiceResult.Id),
        TrustedDomainFqdn = "example.com",
        TrustedDomainDnsIps = new[]
        {
            "10.1.0.3",
            "10.1.0.4",
        },
        Password = "Password123",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/domainservices"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := domainservices.LookupService(ctx, &domainservices.LookupServiceArgs{
			Name:              "example-ds",
			ResourceGroupName: "example-rg",
		}, nil)
		if err != nil {
			return err
		}
		_, err = domainservices.NewServiceTrust(ctx, "example", &domainservices.ServiceTrustArgs{
			Name:              pulumi.String("example-trust"),
			DomainServiceId:   pulumi.String(example.Id),
			TrustedDomainFqdn: pulumi.String("example.com"),
			TrustedDomainDnsIps: pulumi.StringArray{
				pulumi.String("10.1.0.3"),
				pulumi.String("10.1.0.4"),
			},
			Password: pulumi.String("Password123"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.domainservices.DomainservicesFunctions;
import com.pulumi.azure.domainservices.inputs.GetServiceArgs;
import com.pulumi.azure.domainservices.ServiceTrust;
import com.pulumi.azure.domainservices.ServiceTrustArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = DomainservicesFunctions.getService(GetServiceArgs.builder()
            .name("example-ds")
            .resourceGroupName("example-rg")
            .build());

        var exampleServiceTrust = new ServiceTrust("exampleServiceTrust", ServiceTrustArgs.builder()
            .name("example-trust")
            .domainServiceId(example.applyValue(getServiceResult -> getServiceResult.id()))
            .trustedDomainFqdn("example.com")
            .trustedDomainDnsIps(            
                "10.1.0.3",
                "10.1.0.4")
            .password("Password123")
            .build());

    }
}
```
```yaml
resources:
  exampleServiceTrust:
    type: azure:domainservices:ServiceTrust
    name: example
    properties:
      name: example-trust
      domainServiceId: ${example.id}
      trustedDomainFqdn: example.com
      trustedDomainDnsIps:
        - 10.1.0.3
        - 10.1.0.4
      password: Password123
variables:
  example:
    fn::invoke:
      function: azure:domainservices:getService
      arguments:
        name: example-ds
        resourceGroupName: example-rg
```
<!--End PulumiCodeChooser -->

## Import

Active Directory Domain Service Trusts can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:domainservices/serviceTrust:ServiceTrust example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Microsoft.AAD/domainServices/DomainService1/trusts/trust1
```

ñ
domainServiceId" The ID of the Active Directory Domain Service. Changing this forces a new Active Directory Domain Service Trust to be created.
≠
nameB" ûThe name which should be used for this Active Directory Domain Service Trust. Changing this forces a new Active Directory Domain Service Trust to be created.
i
password" YThe password of the inbound trust set in the on-premise Active Directory Domain Service.
Ç
trustedDomainDnsIps*" eSpecifies a list of DNS IPs that are used to resolve the on-premise Active Directory Domain Service.
U
trustedDomainFqdn" <The FQDN of the on-premise Active Directory Domain Service.
"ñ
domainServiceId" The ID of the Active Directory Domain Service. Changing this forces a new Active Directory Domain Service Trust to be created.
"´
name" ûThe name which should be used for this Active Directory Domain Service Trust. Changing this forces a new Active Directory Domain Service Trust to be created.
"i
password" YThe password of the inbound trust set in the on-premise Active Directory Domain Service.
"Ç
trustedDomainDnsIps*" eSpecifies a list of DNS IPs that are used to resolve the on-premise Active Directory Domain Service.
"U
trustedDomainFqdn" <The FQDN of the on-premise Active Directory Domain Service.
*€
5
	dynatraceMonitorazure:dynatrace/monitor:Monitorî	Manages Dynatrace monitors.

## Example Usage

<!--Start PulumiCodeChooser -->
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: example-resources
      location: West Europe
  exampleMonitor:
    type: azure:dynatrace:Monitor
    name: example
    properties:
      name: exmpledynatracemonitor
      resourceGroupName: ${example.name}
      location: ${test.location}
      monitoringEnabled: true
      marketplaceSubscriptionStatus: Active
      identity:
        type: SystemAssigned
      user:
        firstName: Alice
        lastName: Bobab
        email: alice@microsoft.com
        phoneNumber: '123456'
        country: westus
      plan:
        usageType: COMMITTED
        billingCycle: MONTHLY
        plan: azureportalintegration_privatepreview@TIDhjdtn7tfnxcy
        effectiveDate: 2019-08-30T15:14:33Z
```
<!--End PulumiCodeChooser -->

## Import

Dynatrace monitor can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:dynatrace/monitor:Monitor example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/resGroup1/providers/Dynatrace.Observability/monitors/monitor1
```

Ω
identityQ:O
M
	dynatraceMonitorIdentity/azure:dynatrace/MonitorIdentity:MonitorIdentity^The kind of managed identity assigned to this resource.  A `identity` block as defined below.
Ä
locationB" nThe Azure Region where the Dynatrace monitor should exist. Changing this forces a new resource to be created.
‹
marketplaceSubscription" ºFlag specifying the Marketplace Subscription Status of the resource. If payment is not made in time, the resource will go in Suspended state. Possible values are `Active` and `Suspended`.
q
monitoringEnabledB
 VFlag specifying if the resource monitoring is enabled or disabled. Default is `true`.
`
nameB" RName of the Dynatrace monitor. Changing this forces a new resource to be created.
Ω
planE:C
A
	dynatraceMonitorPlan'azure:dynatrace/MonitorPlan:MonitorPlannBilling plan information. A `plan` block as defined below. Changing this forces a new resource to be created.
ï
resourceGroupName" |The name of the Resource Group where the Dynatrace monitor should exist. Changing this forces a new resource to be created.
=
tagsB2" -A mapping of tags to assign to the resource.
∏
userE:C
A
	dynatraceMonitorUser'azure:dynatrace/MonitorUser:MonitorUseriUser's information. A `user` block as defined below. Chainging this forces a new resource to be created.
"Ω
identityQ:O
M
	dynatraceMonitorIdentity/azure:dynatrace/MonitorIdentity:MonitorIdentity^The kind of managed identity assigned to this resource.  A `identity` block as defined below.
"~
location" nThe Azure Region where the Dynatrace monitor should exist. Changing this forces a new resource to be created.
"‹
marketplaceSubscription" ºFlag specifying the Marketplace Subscription Status of the resource. If payment is not made in time, the resource will go in Suspended state. Possible values are `Active` and `Suspended`.
"q
monitoringEnabledB
 VFlag specifying if the resource monitoring is enabled or disabled. Default is `true`.
"^
name" RName of the Dynatrace monitor. Changing this forces a new resource to be created.
"Ω
planE:C
A
	dynatraceMonitorPlan'azure:dynatrace/MonitorPlan:MonitorPlannBilling plan information. A `plan` block as defined below. Changing this forces a new resource to be created.
"ï
resourceGroupName" |The name of the Resource Group where the Dynatrace monitor should exist. Changing this forces a new resource to be created.
"=
tagsB2" -A mapping of tags to assign to the resource.
"∏
userE:C
A
	dynatraceMonitorUser'azure:dynatrace/MonitorUser:MonitorUseriUser's information. A `user` block as defined below. Chainging this forces a new resource to be created.
*òB
M
elasticcloudElasticsearch.azure:elasticcloud/elasticsearch:Elasticsearchæ"Manages an Elasticsearch in Elastic Cloud.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const test = new azure.core.ResourceGroup("test", {
    name: "example-resources",
    location: "West Europe",
});
const testElasticsearch = new azure.elasticcloud.Elasticsearch("test", {
    name: "example-elasticsearch",
    resourceGroupName: test.name,
    location: test.location,
    skuName: "ess-consumption-2024_Monthly",
    elasticCloudEmailAddress: "user@example.com",
});
```
```python
import pulumi
import pulumi_azure as azure

test = azure.core.ResourceGroup("test",
    name="example-resources",
    location="West Europe")
test_elasticsearch = azure.elasticcloud.Elasticsearch("test",
    name="example-elasticsearch",
    resource_group_name=test.name,
    location=test.location,
    sku_name="ess-consumption-2024_Monthly",
    elastic_cloud_email_address="user@example.com")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var test = new Azure.Core.ResourceGroup("test", new()
    {
        Name = "example-resources",
        Location = "West Europe",
    });

    var testElasticsearch = new Azure.ElasticCloud.Elasticsearch("test", new()
    {
        Name = "example-elasticsearch",
        ResourceGroupName = test.Name,
        Location = test.Location,
        SkuName = "ess-consumption-2024_Monthly",
        ElasticCloudEmailAddress = "user@example.com",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/elasticcloud"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		test, err := core.NewResourceGroup(ctx, "test", &core.ResourceGroupArgs{
			Name:     pulumi.String("example-resources"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		_, err = elasticcloud.NewElasticsearch(ctx, "test", &elasticcloud.ElasticsearchArgs{
			Name:                     pulumi.String("example-elasticsearch"),
			ResourceGroupName:        test.Name,
			Location:                 test.Location,
			SkuName:                  pulumi.String("ess-consumption-2024_Monthly"),
			ElasticCloudEmailAddress: pulumi.String("user@example.com"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.elasticcloud.Elasticsearch;
import com.pulumi.azure.elasticcloud.ElasticsearchArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var test = new ResourceGroup("test", ResourceGroupArgs.builder()
            .name("example-resources")
            .location("West Europe")
            .build());

        var testElasticsearch = new Elasticsearch("testElasticsearch", ElasticsearchArgs.builder()
            .name("example-elasticsearch")
            .resourceGroupName(test.name())
            .location(test.location())
            .skuName("ess-consumption-2024_Monthly")
            .elasticCloudEmailAddress("user@example.com")
            .build());

    }
}
```
```yaml
resources:
  test:
    type: azure:core:ResourceGroup
    properties:
      name: example-resources
      location: West Europe
  testElasticsearch:
    type: azure:elasticcloud:Elasticsearch
    name: test
    properties:
      name: example-elasticsearch
      resourceGroupName: ${test.name}
      location: ${test.location}
      skuName: ess-consumption-2024_Monthly
      elasticCloudEmailAddress: user@example.com
```
<!--End PulumiCodeChooser -->

## Import

Elasticsearch's can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:elasticcloud/elasticsearch:Elasticsearch example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Elastic/monitors/monitor1
```

±
elasticCloudEmailAddress" êSpecifies the Email Address which should be associated with this Elasticsearch account. Changing this forces a new Elasticsearch to be created.
ä
locationB" xThe Azure Region where the Elasticsearch resource should exist. Changing this forces a new Elasticsearch to be created.
ä
logs_B]:[
Y
elasticcloudElasticsearchLogs6azure:elasticcloud/ElasticsearchLogs:ElasticsearchLogs!A `logs` block as defined below.
™
monitoringEnabledB
 éSpecifies if the Elasticsearch should have monitoring configured? Defaults to `true`. Changing this forces a new Elasticsearch to be created.
Ö
nameB" wThe name which should be used for this Elasticsearch resource. Changing this forces a new Elasticsearch to be created.
†
resourceGroupName" ÜThe name of the Resource Group where the Elasticsearch resource should exist. Changing this forces a new Elasticsearch to be created.
©
skuName" ôSpecifies the name of the SKU for this Elasticsearch. Changing this forces a new Elasticsearch to be created.

> **NOTE:** The SKU depends on the Elasticsearch Plans available for your account and is a combination of PlanID_Term.
Ex: If the plan ID is "planXYZ" and term is "Yearly", the SKU will be "planXYZ_Yearly".
You may find your eligible plans [here](https://portal.azure.com/#view/Microsoft_Azure_Marketplace/GalleryItemDetailsBladeNopdl/id/elastic.ec-azure-pp) or in the online documentation [here](https://azuremarketplace.microsoft.com/en-us/marketplace/apps/elastic.ec-azure-pp?tab=PlansAndPrice) for more details or in case of any issues with the SKU.
Z
tagsB2" JA mapping of tags which should be assigned to the Elasticsearch resource.
"O
elasticCloudDeploymentId" /The ID of the Deployment within Elastic Cloud.
"±
elasticCloudEmailAddress" êSpecifies the Email Address which should be associated with this Elasticsearch account. Changing this forces a new Elasticsearch to be created.
"a
elasticCloudSsoDefaultUrl" @The Default URL used for Single Sign On (SSO) to Elastic Cloud.
"K
elasticCloudUserId" 1The ID of the User Account within Elastic Cloud.
"h
elasticsearchServiceUrl" IThe URL to the Elasticsearch Service associated with this Elasticsearch.
"\
kibanaServiceUrl" DThe URL to the Kibana Dashboard associated with this Elasticsearch.
"e
kibanaSsoUri" QThe URI used for SSO to the Kibana Dashboard associated with this Elasticsearch.
"à
location" xThe Azure Region where the Elasticsearch resource should exist. Changing this forces a new Elasticsearch to be created.
"ä
logs_B]:[
Y
elasticcloudElasticsearchLogs6azure:elasticcloud/ElasticsearchLogs:ElasticsearchLogs!A `logs` block as defined below.
"™
monitoringEnabledB
 éSpecifies if the Elasticsearch should have monitoring configured? Defaults to `true`. Changing this forces a new Elasticsearch to be created.
"É
name" wThe name which should be used for this Elasticsearch resource. Changing this forces a new Elasticsearch to be created.
"†
resourceGroupName" ÜThe name of the Resource Group where the Elasticsearch resource should exist. Changing this forces a new Elasticsearch to be created.
"©
skuName" ôSpecifies the name of the SKU for this Elasticsearch. Changing this forces a new Elasticsearch to be created.

> **NOTE:** The SKU depends on the Elasticsearch Plans available for your account and is a combination of PlanID_Term.
Ex: If the plan ID is "planXYZ" and term is "Yearly", the SKU will be "planXYZ_Yearly".
You may find your eligible plans [here](https://portal.azure.com/#view/Microsoft_Azure_Marketplace/GalleryItemDetailsBladeNopdl/id/elastic.ec-azure-pp) or in the online documentation [here](https://azuremarketplace.microsoft.com/en-us/marketplace/apps/elastic.ec-azure-pp?tab=PlansAndPrice) for more details or in case of any issues with the SKU.
"Z
tagsB2" JA mapping of tags which should be assigned to the Elasticsearch resource.
*⁄;
@

elasticsan
ElasticSan&azure:elasticsan/elasticSan:ElasticSan÷#Manages an Elastic SAN resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "example-resources",
    location: "West Europe",
});
const exampleElasticSan = new azure.elasticsan.ElasticSan("example", {
    name: "example",
    resourceGroupName: example.name,
    location: example.location,
    baseSizeInTib: 1,
    extendedSizeInTib: 2,
    sku: {
        name: "example-value",
    },
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="example-resources",
    location="West Europe")
example_elastic_san = azure.elasticsan.ElasticSan("example",
    name="example",
    resource_group_name=example.name,
    location=example.location,
    base_size_in_tib=1,
    extended_size_in_tib=2,
    sku={
        "name": "example-value",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "example-resources",
        Location = "West Europe",
    });

    var exampleElasticSan = new Azure.ElasticSan.ElasticSan("example", new()
    {
        Name = "example",
        ResourceGroupName = example.Name,
        Location = example.Location,
        BaseSizeInTib = 1,
        ExtendedSizeInTib = 2,
        Sku = new Azure.ElasticSan.Inputs.ElasticSanSkuArgs
        {
            Name = "example-value",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/elasticsan"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("example-resources"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		_, err = elasticsan.NewElasticSan(ctx, "example", &elasticsan.ElasticSanArgs{
			Name:              pulumi.String("example"),
			ResourceGroupName: example.Name,
			Location:          example.Location,
			BaseSizeInTib:     pulumi.Int(1),
			ExtendedSizeInTib: pulumi.Int(2),
			Sku: &elasticsan.ElasticSanSkuArgs{
				Name: pulumi.String("example-value"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.elasticsan.ElasticSan;
import com.pulumi.azure.elasticsan.ElasticSanArgs;
import com.pulumi.azure.elasticsan.inputs.ElasticSanSkuArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("example-resources")
            .location("West Europe")
            .build());

        var exampleElasticSan = new ElasticSan("exampleElasticSan", ElasticSanArgs.builder()
            .name("example")
            .resourceGroupName(example.name())
            .location(example.location())
            .baseSizeInTib(1)
            .extendedSizeInTib(2)
            .sku(ElasticSanSkuArgs.builder()
                .name("example-value")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: example-resources
      location: West Europe
  exampleElasticSan:
    type: azure:elasticsan:ElasticSan
    name: example
    properties:
      name: example
      resourceGroupName: ${example.name}
      location: ${example.location}
      baseSizeInTib: 1
      extendedSizeInTib: 2
      sku:
        name: example-value
```
<!--End PulumiCodeChooser -->

## Import

An existing Elastic SAN can be imported into Pulumi using the `resource id`, e.g.

```sh
$ pulumi import azure:elasticsan/elasticSan:ElasticSan example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.ElasticSan/elasticSans/esan1
```

‚
baseSizeInTib ÃSpecifies the base size of the Elastic SAN resource in TiB. Possible values are between `1` and `100`.

> **NOTE** When updating `base_size_in_tib`, the new value should be greater than the existing one.
Ü
extendedSizeInTibB ÍSpecifies the extended size of the Elastic SAN resource in TiB. Possible values are between `1` and `100`.

> **NOTE** `extended_size_in_tib` cannot be removed and when updating, the new value should be greater than the existing one.
É
locationB" qThe Azure Region where the Elastic SAN resource should exist. Changing this forces a new resource to be created.
r
nameB" dSpecifies the name of this Elastic SAN resource. Changing this forces a new resource to be created.
´
resourceGroupName" ëSpecifies the name of the Resource Group within which this Elastic SAN resource should exist. Changing this forces a new resource to be created.
v
skuM:K
I

elasticsanElasticSanSku,azure:elasticsan/ElasticSanSku:ElasticSanSku A `sku` block as defined below.
X
tagsB2" HA mapping of tags which should be assigned to the Elastic SAN resource.
ø
zonesB*" ≠Logical zone for the Elastic SAN resource. Changing this forces a new resource to be created.

> **NOTE** `zones` cannot be specified if `sku.name` is set to `Premium_ZRS`.
"‚
baseSizeInTib ÃSpecifies the base size of the Elastic SAN resource in TiB. Possible values are between `1` and `100`.

> **NOTE** When updating `base_size_in_tib`, the new value should be greater than the existing one.
"Ü
extendedSizeInTibB ÍSpecifies the extended size of the Elastic SAN resource in TiB. Possible values are between `1` and `100`.

> **NOTE** `extended_size_in_tib` cannot be removed and when updating, the new value should be greater than the existing one.
"Å
location" qThe Azure Region where the Elastic SAN resource should exist. Changing this forces a new resource to be created.
"p
name" dSpecifies the name of this Elastic SAN resource. Changing this forces a new resource to be created.
"´
resourceGroupName" ëSpecifies the name of the Resource Group within which this Elastic SAN resource should exist. Changing this forces a new resource to be created.
"v
skuM:K
I

elasticsanElasticSanSku,azure:elasticsan/ElasticSanSku:ElasticSanSku A `sku` block as defined below.
"X
tagsB2" HA mapping of tags which should be assigned to the Elastic SAN resource.
"E
	totalIops 4Total Provisioned IOps of the Elastic SAN resource.
">
	totalMbps -Total Provisioned MBps Elastic SAN resource.
"D
totalSizeInTib .Total size of the Elastic SAN resource in TB.
"J
totalVolumeSizeInGib .Total size of the provisioned Volumes in GiB.
"T
volumeGroupCount <Total number of volume groups in this Elastic SAN resource.
"ø
zonesB*" ≠Logical zone for the Elastic SAN resource. Changing this forces a new resource to be created.

> **NOTE** `zones` cannot be specified if `sku.name` is set to `Premium_ZRS`.
*˜õ
4

elasticsanVolumeazure:elasticsan/volume:VolumeâçManages an Elastic SAN Volume resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "example-rg",
    location: "West Europe",
});
const exampleElasticSan = new azure.elasticsan.ElasticSan("example", {
    name: "example-es",
    resourceGroupName: example.name,
    location: example.location,
    baseSizeInTib: 1,
    sku: {
        name: "Premium_LRS",
    },
});
const exampleVolumeGroup = new azure.elasticsan.VolumeGroup("example", {
    name: "example-esvg",
    elasticSanId: exampleElasticSan.id,
});
const exampleVolume = new azure.elasticsan.Volume("example", {
    name: "example-esv",
    volumeGroupId: exampleVolumeGroup.id,
    sizeInGib: 1,
});
export const targetIqn = exampleVolume.targetIqn;
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="example-rg",
    location="West Europe")
example_elastic_san = azure.elasticsan.ElasticSan("example",
    name="example-es",
    resource_group_name=example.name,
    location=example.location,
    base_size_in_tib=1,
    sku={
        "name": "Premium_LRS",
    })
example_volume_group = azure.elasticsan.VolumeGroup("example",
    name="example-esvg",
    elastic_san_id=example_elastic_san.id)
example_volume = azure.elasticsan.Volume("example",
    name="example-esv",
    volume_group_id=example_volume_group.id,
    size_in_gib=1)
pulumi.export("targetIqn", example_volume.target_iqn)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "example-rg",
        Location = "West Europe",
    });

    var exampleElasticSan = new Azure.ElasticSan.ElasticSan("example", new()
    {
        Name = "example-es",
        ResourceGroupName = example.Name,
        Location = example.Location,
        BaseSizeInTib = 1,
        Sku = new Azure.ElasticSan.Inputs.ElasticSanSkuArgs
        {
            Name = "Premium_LRS",
        },
    });

    var exampleVolumeGroup = new Azure.ElasticSan.VolumeGroup("example", new()
    {
        Name = "example-esvg",
        ElasticSanId = exampleElasticSan.Id,
    });

    var exampleVolume = new Azure.ElasticSan.Volume("example", new()
    {
        Name = "example-esv",
        VolumeGroupId = exampleVolumeGroup.Id,
        SizeInGib = 1,
    });

    return new Dictionary<string, object?>
    {
        ["targetIqn"] = exampleVolume.TargetIqn,
    };
});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/elasticsan"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("example-rg"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		exampleElasticSan, err := elasticsan.NewElasticSan(ctx, "example", &elasticsan.ElasticSanArgs{
			Name:              pulumi.String("example-es"),
			ResourceGroupName: example.Name,
			Location:          example.Location,
			BaseSizeInTib:     pulumi.Int(1),
			Sku: &elasticsan.ElasticSanSkuArgs{
				Name: pulumi.String("Premium_LRS"),
			},
		})
		if err != nil {
			return err
		}
		exampleVolumeGroup, err := elasticsan.NewVolumeGroup(ctx, "example", &elasticsan.VolumeGroupArgs{
			Name:         pulumi.String("example-esvg"),
			ElasticSanId: exampleElasticSan.ID(),
		})
		if err != nil {
			return err
		}
		exampleVolume, err := elasticsan.NewVolume(ctx, "example", &elasticsan.VolumeArgs{
			Name:          pulumi.String("example-esv"),
			VolumeGroupId: exampleVolumeGroup.ID(),
			SizeInGib:     pulumi.Int(1),
		})
		if err != nil {
			return err
		}
		ctx.Export("targetIqn", exampleVolume.TargetIqn)
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.elasticsan.ElasticSan;
import com.pulumi.azure.elasticsan.ElasticSanArgs;
import com.pulumi.azure.elasticsan.inputs.ElasticSanSkuArgs;
import com.pulumi.azure.elasticsan.VolumeGroup;
import com.pulumi.azure.elasticsan.VolumeGroupArgs;
import com.pulumi.azure.elasticsan.Volume;
import com.pulumi.azure.elasticsan.VolumeArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("example-rg")
            .location("West Europe")
            .build());

        var exampleElasticSan = new ElasticSan("exampleElasticSan", ElasticSanArgs.builder()
            .name("example-es")
            .resourceGroupName(example.name())
            .location(example.location())
            .baseSizeInTib(1)
            .sku(ElasticSanSkuArgs.builder()
                .name("Premium_LRS")
                .build())
            .build());

        var exampleVolumeGroup = new VolumeGroup("exampleVolumeGroup", VolumeGroupArgs.builder()
            .name("example-esvg")
            .elasticSanId(exampleElasticSan.id())
            .build());

        var exampleVolume = new Volume("exampleVolume", VolumeArgs.builder()
            .name("example-esv")
            .volumeGroupId(exampleVolumeGroup.id())
            .sizeInGib(1)
            .build());

        ctx.export("targetIqn", exampleVolume.targetIqn());
    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: example-rg
      location: West Europe
  exampleElasticSan:
    type: azure:elasticsan:ElasticSan
    name: example
    properties:
      name: example-es
      resourceGroupName: ${example.name}
      location: ${example.location}
      baseSizeInTib: 1
      sku:
        name: Premium_LRS
  exampleVolumeGroup:
    type: azure:elasticsan:VolumeGroup
    name: example
    properties:
      name: example-esvg
      elasticSanId: ${exampleElasticSan.id}
  exampleVolume:
    type: azure:elasticsan:Volume
    name: example
    properties:
      name: example-esv
      volumeGroupId: ${exampleVolumeGroup.id}
      sizeInGib: 1
outputs:
  targetIqn: ${exampleVolume.targetIqn}
```
<!--End PulumiCodeChooser -->

## Example of creating an Elastic SAN Volume from a Disk Snapshot

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "example-rg",
    location: "West Europe",
});
const exampleElasticSan = new azure.elasticsan.ElasticSan("example", {
    name: "example-es",
    resourceGroupName: example.name,
    location: example.location,
    baseSizeInTib: 1,
    sku: {
        name: "Premium_LRS",
    },
});
const exampleVolumeGroup = new azure.elasticsan.VolumeGroup("example", {
    name: "example-esvg",
    elasticSanId: exampleElasticSan.id,
});
const exampleManagedDisk = new azure.compute.ManagedDisk("example", {
    name: "example-disk",
    location: example.location,
    resourceGroupName: example.name,
    createOption: "Empty",
    storageAccountType: "Standard_LRS",
    diskSizeGb: 2,
});
const exampleSnapshot = new azure.compute.Snapshot("example", {
    name: "example-ss",
    location: example.location,
    resourceGroupName: example.name,
    createOption: "Copy",
    sourceUri: exampleManagedDisk.id,
});
const example2 = new azure.elasticsan.Volume("example2", {
    name: "example-esv2",
    volumeGroupId: exampleVolumeGroup.id,
    sizeInGib: 2,
    createSource: {
        sourceType: "DiskSnapshot",
        sourceId: exampleSnapshot.id,
    },
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="example-rg",
    location="West Europe")
example_elastic_san = azure.elasticsan.ElasticSan("example",
    name="example-es",
    resource_group_name=example.name,
    location=example.location,
    base_size_in_tib=1,
    sku={
        "name": "Premium_LRS",
    })
example_volume_group = azure.elasticsan.VolumeGroup("example",
    name="example-esvg",
    elastic_san_id=example_elastic_san.id)
example_managed_disk = azure.compute.ManagedDisk("example",
    name="example-disk",
    location=example.location,
    resource_group_name=example.name,
    create_option="Empty",
    storage_account_type="Standard_LRS",
    disk_size_gb=2)
example_snapshot = azure.compute.Snapshot("example",
    name="example-ss",
    location=example.location,
    resource_group_name=example.name,
    create_option="Copy",
    source_uri=example_managed_disk.id)
example2 = azure.elasticsan.Volume("example2",
    name="example-esv2",
    volume_group_id=example_volume_group.id,
    size_in_gib=2,
    create_source={
        "source_type": "DiskSnapshot",
        "source_id": example_snapshot.id,
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "example-rg",
        Location = "West Europe",
    });

    var exampleElasticSan = new Azure.ElasticSan.ElasticSan("example", new()
    {
        Name = "example-es",
        ResourceGroupName = example.Name,
        Location = example.Location,
        BaseSizeInTib = 1,
        Sku = new Azure.ElasticSan.Inputs.ElasticSanSkuArgs
        {
            Name = "Premium_LRS",
        },
    });

    var exampleVolumeGroup = new Azure.ElasticSan.VolumeGroup("example", new()
    {
        Name = "example-esvg",
        ElasticSanId = exampleElasticSan.Id,
    });

    var exampleManagedDisk = new Azure.Compute.ManagedDisk("example", new()
    {
        Name = "example-disk",
        Location = example.Location,
        ResourceGroupName = example.Name,
        CreateOption = "Empty",
        StorageAccountType = "Standard_LRS",
        DiskSizeGb = 2,
    });

    var exampleSnapshot = new Azure.Compute.Snapshot("example", new()
    {
        Name = "example-ss",
        Location = example.Location,
        ResourceGroupName = example.Name,
        CreateOption = "Copy",
        SourceUri = exampleManagedDisk.Id,
    });

    var example2 = new Azure.ElasticSan.Volume("example2", new()
    {
        Name = "example-esv2",
        VolumeGroupId = exampleVolumeGroup.Id,
        SizeInGib = 2,
        CreateSource = new Azure.ElasticSan.Inputs.VolumeCreateSourceArgs
        {
            SourceType = "DiskSnapshot",
            SourceId = exampleSnapshot.Id,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/compute"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/elasticsan"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("example-rg"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		exampleElasticSan, err := elasticsan.NewElasticSan(ctx, "example", &elasticsan.ElasticSanArgs{
			Name:              pulumi.String("example-es"),
			ResourceGroupName: example.Name,
			Location:          example.Location,
			BaseSizeInTib:     pulumi.Int(1),
			Sku: &elasticsan.ElasticSanSkuArgs{
				Name: pulumi.String("Premium_LRS"),
			},
		})
		if err != nil {
			return err
		}
		exampleVolumeGroup, err := elasticsan.NewVolumeGroup(ctx, "example", &elasticsan.VolumeGroupArgs{
			Name:         pulumi.String("example-esvg"),
			ElasticSanId: exampleElasticSan.ID(),
		})
		if err != nil {
			return err
		}
		exampleManagedDisk, err := compute.NewManagedDisk(ctx, "example", &compute.ManagedDiskArgs{
			Name:               pulumi.String("example-disk"),
			Location:           example.Location,
			ResourceGroupName:  example.Name,
			CreateOption:       pulumi.String("Empty"),
			StorageAccountType: pulumi.String("Standard_LRS"),
			DiskSizeGb:         pulumi.Int(2),
		})
		if err != nil {
			return err
		}
		exampleSnapshot, err := compute.NewSnapshot(ctx, "example", &compute.SnapshotArgs{
			Name:              pulumi.String("example-ss"),
			Location:          example.Location,
			ResourceGroupName: example.Name,
			CreateOption:      pulumi.String("Copy"),
			SourceUri:         exampleManagedDisk.ID(),
		})
		if err != nil {
			return err
		}
		_, err = elasticsan.NewVolume(ctx, "example2", &elasticsan.VolumeArgs{
			Name:          pulumi.String("example-esv2"),
			VolumeGroupId: exampleVolumeGroup.ID(),
			SizeInGib:     pulumi.Int(2),
			CreateSource: &elasticsan.VolumeCreateSourceArgs{
				SourceType: pulumi.String("DiskSnapshot"),
				SourceId:   exampleSnapshot.ID(),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.elasticsan.ElasticSan;
import com.pulumi.azure.elasticsan.ElasticSanArgs;
import com.pulumi.azure.elasticsan.inputs.ElasticSanSkuArgs;
import com.pulumi.azure.elasticsan.VolumeGroup;
import com.pulumi.azure.elasticsan.VolumeGroupArgs;
import com.pulumi.azure.compute.ManagedDisk;
import com.pulumi.azure.compute.ManagedDiskArgs;
import com.pulumi.azure.compute.Snapshot;
import com.pulumi.azure.compute.SnapshotArgs;
import com.pulumi.azure.elasticsan.Volume;
import com.pulumi.azure.elasticsan.VolumeArgs;
import com.pulumi.azure.elasticsan.inputs.VolumeCreateSourceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("example-rg")
            .location("West Europe")
            .build());

        var exampleElasticSan = new ElasticSan("exampleElasticSan", ElasticSanArgs.builder()
            .name("example-es")
            .resourceGroupName(example.name())
            .location(example.location())
            .baseSizeInTib(1)
            .sku(ElasticSanSkuArgs.builder()
                .name("Premium_LRS")
                .build())
            .build());

        var exampleVolumeGroup = new VolumeGroup("exampleVolumeGroup", VolumeGroupArgs.builder()
            .name("example-esvg")
            .elasticSanId(exampleElasticSan.id())
            .build());

        var exampleManagedDisk = new ManagedDisk("exampleManagedDisk", ManagedDiskArgs.builder()
            .name("example-disk")
            .location(example.location())
            .resourceGroupName(example.name())
            .createOption("Empty")
            .storageAccountType("Standard_LRS")
            .diskSizeGb(2)
            .build());

        var exampleSnapshot = new Snapshot("exampleSnapshot", SnapshotArgs.builder()
            .name("example-ss")
            .location(example.location())
            .resourceGroupName(example.name())
            .createOption("Copy")
            .sourceUri(exampleManagedDisk.id())
            .build());

        var example2 = new Volume("example2", VolumeArgs.builder()
            .name("example-esv2")
            .volumeGroupId(exampleVolumeGroup.id())
            .sizeInGib(2)
            .createSource(VolumeCreateSourceArgs.builder()
                .sourceType("DiskSnapshot")
                .sourceId(exampleSnapshot.id())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: example-rg
      location: West Europe
  exampleElasticSan:
    type: azure:elasticsan:ElasticSan
    name: example
    properties:
      name: example-es
      resourceGroupName: ${example.name}
      location: ${example.location}
      baseSizeInTib: 1
      sku:
        name: Premium_LRS
  exampleVolumeGroup:
    type: azure:elasticsan:VolumeGroup
    name: example
    properties:
      name: example-esvg
      elasticSanId: ${exampleElasticSan.id}
  exampleManagedDisk:
    type: azure:compute:ManagedDisk
    name: example
    properties:
      name: example-disk
      location: ${example.location}
      resourceGroupName: ${example.name}
      createOption: Empty
      storageAccountType: Standard_LRS
      diskSizeGb: 2
  exampleSnapshot:
    type: azure:compute:Snapshot
    name: example
    properties:
      name: example-ss
      location: ${example.location}
      resourceGroupName: ${example.name}
      createOption: Copy
      sourceUri: ${exampleManagedDisk.id}
  example2:
    type: azure:elasticsan:Volume
    properties:
      name: example-esv2
      volumeGroupId: ${exampleVolumeGroup.id}
      sizeInGib: 2
      createSource:
        sourceType: DiskSnapshot
        sourceId: ${exampleSnapshot.id}
```
<!--End PulumiCodeChooser -->

## Import

An existing Elastic SAN Volume can be imported into Pulumi using the `resource id`, e.g.

```sh
$ pulumi import azure:elasticsan/volume:Volume example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.ElasticSan/elasticSans/esan1/volumeGroups/vg1/volumes/vol1
```

ö
createSource^B\:Z
X

elasticsanVolumeCreateSource6azure:elasticsan/VolumeCreateSource:VolumeCreateSource*A `create_source` block as defined below.
p
nameB" bSpecifies the name of this Elastic SAN Volume. Changing this forces a new resource to be created.
Ÿ
	sizeInGib «Specifies the size of the Elastic SAN Volume in GiB. The size should be within the remaining capacity of the parent Elastic SAN. Possible values are between `1` and `65536` (16 TiB).

> **NOTE:** The size can only be increased. If `create_source` is specified, then the size must be equal to or greater than the source's size.
ö
volumeGroupId" ÑSpecifies the Volume Group ID within which this Elastic SAN Volume should exist. Changing this forces a new resource to be created.
"ö
createSource^B\:Z
X

elasticsanVolumeCreateSource6azure:elasticsan/VolumeCreateSource:VolumeCreateSource*A `create_source` block as defined below.
"n
name" bSpecifies the name of this Elastic SAN Volume. Changing this forces a new resource to be created.
"Ÿ
	sizeInGib «Specifies the size of the Elastic SAN Volume in GiB. The size should be within the remaining capacity of the parent Elastic SAN. Possible values are between `1` and `65536` (16 TiB).

> **NOTE:** The size can only be increased. If `create_source` is specified, then the size must be equal to or greater than the source's size.
"A
	targetIqn" 0The iSCSI Target IQN of the Elastic SAN Volume.
"Y
targetPortalHostname" =The iSCSI Target Portal Host Name of the Elastic SAN Volume.
"P
targetPortalPort 8The iSCSI Target Portal Port of the Elastic SAN Volume.
"ö
volumeGroupId" ÑSpecifies the Volume Group ID within which this Elastic SAN Volume should exist. Changing this forces a new resource to be created.
"4
volumeId" $The UUID of the Elastic SAN Volume.
*∑Á
C

elasticsanVolumeGroup(azure:elasticsan/volumeGroup:VolumeGroupŒ—Manages an Elastic SAN Volume Group resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "example-rg",
    location: "West Europe",
});
const exampleElasticSan = new azure.elasticsan.ElasticSan("example", {
    name: "examplees-es",
    resourceGroupName: example.name,
    location: example.location,
    baseSizeInTib: 1,
    sku: {
        name: "Premium_LRS",
    },
});
const current = azure.core.getClientConfig({});
const exampleUserAssignedIdentity = new azure.authorization.UserAssignedIdentity("example", {
    name: "example-uai",
    location: example.location,
    resourceGroupName: example.name,
});
const exampleVirtualNetwork = new azure.network.VirtualNetwork("example", {
    name: "example-vnet",
    addressSpaces: ["10.0.0.0/16"],
    location: example.location,
    resourceGroupName: example.name,
});
const exampleSubnet = new azure.network.Subnet("example", {
    name: "example-subnet",
    resourceGroupName: example.name,
    virtualNetworkName: exampleVirtualNetwork.name,
    addressPrefixes: ["10.0.1.0/24"],
    serviceEndpoints: ["Microsoft.Storage.Global"],
});
const exampleKeyVault = new azure.keyvault.KeyVault("example", {
    name: "examplekv",
    location: example.location,
    resourceGroupName: example.name,
    enabledForDiskEncryption: true,
    tenantId: current.then(current => current.tenantId),
    softDeleteRetentionDays: 7,
    purgeProtectionEnabled: true,
    skuName: "standard",
});
const userAssignedIdentity = new azure.keyvault.AccessPolicy("userAssignedIdentity", {
    keyVaultId: exampleKeyVault.id,
    tenantId: current.then(current => current.tenantId),
    objectId: exampleUserAssignedIdentity.principalId,
    keyPermissions: [
        "Get",
        "UnwrapKey",
        "WrapKey",
    ],
    secretPermissions: ["Get"],
});
const client = new azure.keyvault.AccessPolicy("client", {
    keyVaultId: exampleKeyVault.id,
    tenantId: current.then(current => current.tenantId),
    objectId: current.then(current => current.objectId),
    keyPermissions: [
        "Get",
        "Create",
        "Delete",
        "List",
        "Restore",
        "Recover",
        "UnwrapKey",
        "WrapKey",
        "Purge",
        "Encrypt",
        "Decrypt",
        "Sign",
        "Verify",
        "GetRotationPolicy",
    ],
    secretPermissions: ["Get"],
});
const exampleKey = new azure.keyvault.Key("example", {
    name: "example-kvk",
    keyVaultId: exampleKeyVault.id,
    keyType: "RSA",
    keySize: 2048,
    keyOpts: [
        "decrypt",
        "encrypt",
        "sign",
        "unwrapKey",
        "verify",
        "wrapKey",
    ],
}, {
    dependsOn: [
        userAssignedIdentity,
        client,
    ],
});
const exampleVolumeGroup = new azure.elasticsan.VolumeGroup("example", {
    name: "example-esvg",
    elasticSanId: exampleElasticSan.id,
    encryptionType: "EncryptionAtRestWithCustomerManagedKey",
    encryption: {
        keyVaultKeyId: exampleKey.versionlessId,
        userAssignedIdentityId: exampleUserAssignedIdentity.id,
    },
    identity: {
        type: "UserAssigned",
        identityIds: [exampleUserAssignedIdentity.id],
    },
    networkRules: [{
        subnetId: exampleSubnet.id,
        action: "Allow",
    }],
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="example-rg",
    location="West Europe")
example_elastic_san = azure.elasticsan.ElasticSan("example",
    name="examplees-es",
    resource_group_name=example.name,
    location=example.location,
    base_size_in_tib=1,
    sku={
        "name": "Premium_LRS",
    })
current = azure.core.get_client_config()
example_user_assigned_identity = azure.authorization.UserAssignedIdentity("example",
    name="example-uai",
    location=example.location,
    resource_group_name=example.name)
example_virtual_network = azure.network.VirtualNetwork("example",
    name="example-vnet",
    address_spaces=["10.0.0.0/16"],
    location=example.location,
    resource_group_name=example.name)
example_subnet = azure.network.Subnet("example",
    name="example-subnet",
    resource_group_name=example.name,
    virtual_network_name=example_virtual_network.name,
    address_prefixes=["10.0.1.0/24"],
    service_endpoints=["Microsoft.Storage.Global"])
example_key_vault = azure.keyvault.KeyVault("example",
    name="examplekv",
    location=example.location,
    resource_group_name=example.name,
    enabled_for_disk_encryption=True,
    tenant_id=current.tenant_id,
    soft_delete_retention_days=7,
    purge_protection_enabled=True,
    sku_name="standard")
user_assigned_identity = azure.keyvault.AccessPolicy("userAssignedIdentity",
    key_vault_id=example_key_vault.id,
    tenant_id=current.tenant_id,
    object_id=example_user_assigned_identity.principal_id,
    key_permissions=[
        "Get",
        "UnwrapKey",
        "WrapKey",
    ],
    secret_permissions=["Get"])
client = azure.keyvault.AccessPolicy("client",
    key_vault_id=example_key_vault.id,
    tenant_id=current.tenant_id,
    object_id=current.object_id,
    key_permissions=[
        "Get",
        "Create",
        "Delete",
        "List",
        "Restore",
        "Recover",
        "UnwrapKey",
        "WrapKey",
        "Purge",
        "Encrypt",
        "Decrypt",
        "Sign",
        "Verify",
        "GetRotationPolicy",
    ],
    secret_permissions=["Get"])
example_key = azure.keyvault.Key("example",
    name="example-kvk",
    key_vault_id=example_key_vault.id,
    key_type="RSA",
    key_size=2048,
    key_opts=[
        "decrypt",
        "encrypt",
        "sign",
        "unwrapKey",
        "verify",
        "wrapKey",
    ],
    opts = pulumi.ResourceOptions(depends_on=[
            user_assigned_identity,
            client,
        ]))
example_volume_group = azure.elasticsan.VolumeGroup("example",
    name="example-esvg",
    elastic_san_id=example_elastic_san.id,
    encryption_type="EncryptionAtRestWithCustomerManagedKey",
    encryption={
        "key_vault_key_id": example_key.versionless_id,
        "user_assigned_identity_id": example_user_assigned_identity.id,
    },
    identity={
        "type": "UserAssigned",
        "identity_ids": [example_user_assigned_identity.id],
    },
    network_rules=[{
        "subnet_id": example_subnet.id,
        "action": "Allow",
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "example-rg",
        Location = "West Europe",
    });

    var exampleElasticSan = new Azure.ElasticSan.ElasticSan("example", new()
    {
        Name = "examplees-es",
        ResourceGroupName = example.Name,
        Location = example.Location,
        BaseSizeInTib = 1,
        Sku = new Azure.ElasticSan.Inputs.ElasticSanSkuArgs
        {
            Name = "Premium_LRS",
        },
    });

    var current = Azure.Core.GetClientConfig.Invoke();

    var exampleUserAssignedIdentity = new Azure.Authorization.UserAssignedIdentity("example", new()
    {
        Name = "example-uai",
        Location = example.Location,
        ResourceGroupName = example.Name,
    });

    var exampleVirtualNetwork = new Azure.Network.VirtualNetwork("example", new()
    {
        Name = "example-vnet",
        AddressSpaces = new[]
        {
            "10.0.0.0/16",
        },
        Location = example.Location,
        ResourceGroupName = example.Name,
    });

    var exampleSubnet = new Azure.Network.Subnet("example", new()
    {
        Name = "example-subnet",
        ResourceGroupName = example.Name,
        VirtualNetworkName = exampleVirtualNetwork.Name,
        AddressPrefixes = new[]
        {
            "10.0.1.0/24",
        },
        ServiceEndpoints = new[]
        {
            "Microsoft.Storage.Global",
        },
    });

    var exampleKeyVault = new Azure.KeyVault.KeyVault("example", new()
    {
        Name = "examplekv",
        Location = example.Location,
        ResourceGroupName = example.Name,
        EnabledForDiskEncryption = true,
        TenantId = current.Apply(getClientConfigResult => getClientConfigResult.TenantId),
        SoftDeleteRetentionDays = 7,
        PurgeProtectionEnabled = true,
        SkuName = "standard",
    });

    var userAssignedIdentity = new Azure.KeyVault.AccessPolicy("userAssignedIdentity", new()
    {
        KeyVaultId = exampleKeyVault.Id,
        TenantId = current.Apply(getClientConfigResult => getClientConfigResult.TenantId),
        ObjectId = exampleUserAssignedIdentity.PrincipalId,
        KeyPermissions = new[]
        {
            "Get",
            "UnwrapKey",
            "WrapKey",
        },
        SecretPermissions = new[]
        {
            "Get",
        },
    });

    var client = new Azure.KeyVault.AccessPolicy("client", new()
    {
        KeyVaultId = exampleKeyVault.Id,
        TenantId = current.Apply(getClientConfigResult => getClientConfigResult.TenantId),
        ObjectId = current.Apply(getClientConfigResult => getClientConfigResult.ObjectId),
        KeyPermissions = new[]
        {
            "Get",
            "Create",
            "Delete",
            "List",
            "Restore",
            "Recover",
            "UnwrapKey",
            "WrapKey",
            "Purge",
            "Encrypt",
            "Decrypt",
            "Sign",
            "Verify",
            "GetRotationPolicy",
        },
        SecretPermissions = new[]
        {
            "Get",
        },
    });

    var exampleKey = new Azure.KeyVault.Key("example", new()
    {
        Name = "example-kvk",
        KeyVaultId = exampleKeyVault.Id,
        KeyType = "RSA",
        KeySize = 2048,
        KeyOpts = new[]
        {
            "decrypt",
            "encrypt",
            "sign",
            "unwrapKey",
            "verify",
            "wrapKey",
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            userAssignedIdentity,
            client,
        },
    });

    var exampleVolumeGroup = new Azure.ElasticSan.VolumeGroup("example", new()
    {
        Name = "example-esvg",
        ElasticSanId = exampleElasticSan.Id,
        EncryptionType = "EncryptionAtRestWithCustomerManagedKey",
        Encryption = new Azure.ElasticSan.Inputs.VolumeGroupEncryptionArgs
        {
            KeyVaultKeyId = exampleKey.VersionlessId,
            UserAssignedIdentityId = exampleUserAssignedIdentity.Id,
        },
        Identity = new Azure.ElasticSan.Inputs.VolumeGroupIdentityArgs
        {
            Type = "UserAssigned",
            IdentityIds = new[]
            {
                exampleUserAssignedIdentity.Id,
            },
        },
        NetworkRules = new[]
        {
            new Azure.ElasticSan.Inputs.VolumeGroupNetworkRuleArgs
            {
                SubnetId = exampleSubnet.Id,
                Action = "Allow",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/authorization"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/elasticsan"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/keyvault"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/network"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("example-rg"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		exampleElasticSan, err := elasticsan.NewElasticSan(ctx, "example", &elasticsan.ElasticSanArgs{
			Name:              pulumi.String("examplees-es"),
			ResourceGroupName: example.Name,
			Location:          example.Location,
			BaseSizeInTib:     pulumi.Int(1),
			Sku: &elasticsan.ElasticSanSkuArgs{
				Name: pulumi.String("Premium_LRS"),
			},
		})
		if err != nil {
			return err
		}
		current, err := core.GetClientConfig(ctx, map[string]interface{}{}, nil)
		if err != nil {
			return err
		}
		exampleUserAssignedIdentity, err := authorization.NewUserAssignedIdentity(ctx, "example", &authorization.UserAssignedIdentityArgs{
			Name:              pulumi.String("example-uai"),
			Location:          example.Location,
			ResourceGroupName: example.Name,
		})
		if err != nil {
			return err
		}
		exampleVirtualNetwork, err := network.NewVirtualNetwork(ctx, "example", &network.VirtualNetworkArgs{
			Name: pulumi.String("example-vnet"),
			AddressSpaces: pulumi.StringArray{
				pulumi.String("10.0.0.0/16"),
			},
			Location:          example.Location,
			ResourceGroupName: example.Name,
		})
		if err != nil {
			return err
		}
		exampleSubnet, err := network.NewSubnet(ctx, "example", &network.SubnetArgs{
			Name:               pulumi.String("example-subnet"),
			ResourceGroupName:  example.Name,
			VirtualNetworkName: exampleVirtualNetwork.Name,
			AddressPrefixes: pulumi.StringArray{
				pulumi.String("10.0.1.0/24"),
			},
			ServiceEndpoints: pulumi.StringArray{
				pulumi.String("Microsoft.Storage.Global"),
			},
		})
		if err != nil {
			return err
		}
		exampleKeyVault, err := keyvault.NewKeyVault(ctx, "example", &keyvault.KeyVaultArgs{
			Name:                     pulumi.String("examplekv"),
			Location:                 example.Location,
			ResourceGroupName:        example.Name,
			EnabledForDiskEncryption: pulumi.Bool(true),
			TenantId:                 pulumi.String(current.TenantId),
			SoftDeleteRetentionDays:  pulumi.Int(7),
			PurgeProtectionEnabled:   pulumi.Bool(true),
			SkuName:                  pulumi.String("standard"),
		})
		if err != nil {
			return err
		}
		userAssignedIdentity, err := keyvault.NewAccessPolicy(ctx, "userAssignedIdentity", &keyvault.AccessPolicyArgs{
			KeyVaultId: exampleKeyVault.ID(),
			TenantId:   pulumi.String(current.TenantId),
			ObjectId:   exampleUserAssignedIdentity.PrincipalId,
			KeyPermissions: pulumi.StringArray{
				pulumi.String("Get"),
				pulumi.String("UnwrapKey"),
				pulumi.String("WrapKey"),
			},
			SecretPermissions: pulumi.StringArray{
				pulumi.String("Get"),
			},
		})
		if err != nil {
			return err
		}
		client, err := keyvault.NewAccessPolicy(ctx, "client", &keyvault.AccessPolicyArgs{
			KeyVaultId: exampleKeyVault.ID(),
			TenantId:   pulumi.String(current.TenantId),
			ObjectId:   pulumi.String(current.ObjectId),
			KeyPermissions: pulumi.StringArray{
				pulumi.String("Get"),
				pulumi.String("Create"),
				pulumi.String("Delete"),
				pulumi.String("List"),
				pulumi.String("Restore"),
				pulumi.String("Recover"),
				pulumi.String("UnwrapKey"),
				pulumi.String("WrapKey"),
				pulumi.String("Purge"),
				pulumi.String("Encrypt"),
				pulumi.String("Decrypt"),
				pulumi.String("Sign"),
				pulumi.String("Verify"),
				pulumi.String("GetRotationPolicy"),
			},
			SecretPermissions: pulumi.StringArray{
				pulumi.String("Get"),
			},
		})
		if err != nil {
			return err
		}
		exampleKey, err := keyvault.NewKey(ctx, "example", &keyvault.KeyArgs{
			Name:       pulumi.String("example-kvk"),
			KeyVaultId: exampleKeyVault.ID(),
			KeyType:    pulumi.String("RSA"),
			KeySize:    pulumi.Int(2048),
			KeyOpts: pulumi.StringArray{
				pulumi.String("decrypt"),
				pulumi.String("encrypt"),
				pulumi.String("sign"),
				pulumi.String("unwrapKey"),
				pulumi.String("verify"),
				pulumi.String("wrapKey"),
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			userAssignedIdentity,
			client,
		}))
		if err != nil {
			return err
		}
		_, err = elasticsan.NewVolumeGroup(ctx, "example", &elasticsan.VolumeGroupArgs{
			Name:           pulumi.String("example-esvg"),
			ElasticSanId:   exampleElasticSan.ID(),
			EncryptionType: pulumi.String("EncryptionAtRestWithCustomerManagedKey"),
			Encryption: &elasticsan.VolumeGroupEncryptionArgs{
				KeyVaultKeyId:          exampleKey.VersionlessId,
				UserAssignedIdentityId: exampleUserAssignedIdentity.ID(),
			},
			Identity: &elasticsan.VolumeGroupIdentityArgs{
				Type: pulumi.String("UserAssigned"),
				IdentityIds: pulumi.StringArray{
					exampleUserAssignedIdentity.ID(),
				},
			},
			NetworkRules: elasticsan.VolumeGroupNetworkRuleArray{
				&elasticsan.VolumeGroupNetworkRuleArgs{
					SubnetId: exampleSubnet.ID(),
					Action:   pulumi.String("Allow"),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.elasticsan.ElasticSan;
import com.pulumi.azure.elasticsan.ElasticSanArgs;
import com.pulumi.azure.elasticsan.inputs.ElasticSanSkuArgs;
import com.pulumi.azure.core.CoreFunctions;
import com.pulumi.azure.authorization.UserAssignedIdentity;
import com.pulumi.azure.authorization.UserAssignedIdentityArgs;
import com.pulumi.azure.network.VirtualNetwork;
import com.pulumi.azure.network.VirtualNetworkArgs;
import com.pulumi.azure.network.Subnet;
import com.pulumi.azure.network.SubnetArgs;
import com.pulumi.azure.keyvault.KeyVault;
import com.pulumi.azure.keyvault.KeyVaultArgs;
import com.pulumi.azure.keyvault.AccessPolicy;
import com.pulumi.azure.keyvault.AccessPolicyArgs;
import com.pulumi.azure.keyvault.Key;
import com.pulumi.azure.keyvault.KeyArgs;
import com.pulumi.azure.elasticsan.VolumeGroup;
import com.pulumi.azure.elasticsan.VolumeGroupArgs;
import com.pulumi.azure.elasticsan.inputs.VolumeGroupEncryptionArgs;
import com.pulumi.azure.elasticsan.inputs.VolumeGroupIdentityArgs;
import com.pulumi.azure.elasticsan.inputs.VolumeGroupNetworkRuleArgs;
import com.pulumi.resources.CustomResourceOptions;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("example-rg")
            .location("West Europe")
            .build());

        var exampleElasticSan = new ElasticSan("exampleElasticSan", ElasticSanArgs.builder()
            .name("examplees-es")
            .resourceGroupName(example.name())
            .location(example.location())
            .baseSizeInTib(1)
            .sku(ElasticSanSkuArgs.builder()
                .name("Premium_LRS")
                .build())
            .build());

        final var current = CoreFunctions.getClientConfig();

        var exampleUserAssignedIdentity = new UserAssignedIdentity("exampleUserAssignedIdentity", UserAssignedIdentityArgs.builder()
            .name("example-uai")
            .location(example.location())
            .resourceGroupName(example.name())
            .build());

        var exampleVirtualNetwork = new VirtualNetwork("exampleVirtualNetwork", VirtualNetworkArgs.builder()
            .name("example-vnet")
            .addressSpaces("10.0.0.0/16")
            .location(example.location())
            .resourceGroupName(example.name())
            .build());

        var exampleSubnet = new Subnet("exampleSubnet", SubnetArgs.builder()
            .name("example-subnet")
            .resourceGroupName(example.name())
            .virtualNetworkName(exampleVirtualNetwork.name())
            .addressPrefixes("10.0.1.0/24")
            .serviceEndpoints("Microsoft.Storage.Global")
            .build());

        var exampleKeyVault = new KeyVault("exampleKeyVault", KeyVaultArgs.builder()
            .name("examplekv")
            .location(example.location())
            .resourceGroupName(example.name())
            .enabledForDiskEncryption(true)
            .tenantId(current.applyValue(getClientConfigResult -> getClientConfigResult.tenantId()))
            .softDeleteRetentionDays(7)
            .purgeProtectionEnabled(true)
            .skuName("standard")
            .build());

        var userAssignedIdentity = new AccessPolicy("userAssignedIdentity", AccessPolicyArgs.builder()
            .keyVaultId(exampleKeyVault.id())
            .tenantId(current.applyValue(getClientConfigResult -> getClientConfigResult.tenantId()))
            .objectId(exampleUserAssignedIdentity.principalId())
            .keyPermissions(            
                "Get",
                "UnwrapKey",
                "WrapKey")
            .secretPermissions("Get")
            .build());

        var client = new AccessPolicy("client", AccessPolicyArgs.builder()
            .keyVaultId(exampleKeyVault.id())
            .tenantId(current.applyValue(getClientConfigResult -> getClientConfigResult.tenantId()))
            .objectId(current.applyValue(getClientConfigResult -> getClientConfigResult.objectId()))
            .keyPermissions(            
                "Get",
                "Create",
                "Delete",
                "List",
                "Restore",
                "Recover",
                "UnwrapKey",
                "WrapKey",
                "Purge",
                "Encrypt",
                "Decrypt",
                "Sign",
                "Verify",
                "GetRotationPolicy")
            .secretPermissions("Get")
            .build());

        var exampleKey = new Key("exampleKey", KeyArgs.builder()
            .name("example-kvk")
            .keyVaultId(exampleKeyVault.id())
            .keyType("RSA")
            .keySize(2048)
            .keyOpts(            
                "decrypt",
                "encrypt",
                "sign",
                "unwrapKey",
                "verify",
                "wrapKey")
            .build(), CustomResourceOptions.builder()
                .dependsOn(                
                    userAssignedIdentity,
                    client)
                .build());

        var exampleVolumeGroup = new VolumeGroup("exampleVolumeGroup", VolumeGroupArgs.builder()
            .name("example-esvg")
            .elasticSanId(exampleElasticSan.id())
            .encryptionType("EncryptionAtRestWithCustomerManagedKey")
            .encryption(VolumeGroupEncryptionArgs.builder()
                .keyVaultKeyId(exampleKey.versionlessId())
                .userAssignedIdentityId(exampleUserAssignedIdentity.id())
                .build())
            .identity(VolumeGroupIdentityArgs.builder()
                .type("UserAssigned")
                .identityIds(exampleUserAssignedIdentity.id())
                .build())
            .networkRules(VolumeGroupNetworkRuleArgs.builder()
                .subnetId(exampleSubnet.id())
                .action("Allow")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: example-rg
      location: West Europe
  exampleElasticSan:
    type: azure:elasticsan:ElasticSan
    name: example
    properties:
      name: examplees-es
      resourceGroupName: ${example.name}
      location: ${example.location}
      baseSizeInTib: 1
      sku:
        name: Premium_LRS
  exampleUserAssignedIdentity:
    type: azure:authorization:UserAssignedIdentity
    name: example
    properties:
      name: example-uai
      location: ${example.location}
      resourceGroupName: ${example.name}
  exampleVirtualNetwork:
    type: azure:network:VirtualNetwork
    name: example
    properties:
      name: example-vnet
      addressSpaces:
        - 10.0.0.0/16
      location: ${example.location}
      resourceGroupName: ${example.name}
  exampleSubnet:
    type: azure:network:Subnet
    name: example
    properties:
      name: example-subnet
      resourceGroupName: ${example.name}
      virtualNetworkName: ${exampleVirtualNetwork.name}
      addressPrefixes:
        - 10.0.1.0/24
      serviceEndpoints:
        - Microsoft.Storage.Global
  exampleKeyVault:
    type: azure:keyvault:KeyVault
    name: example
    properties:
      name: examplekv
      location: ${example.location}
      resourceGroupName: ${example.name}
      enabledForDiskEncryption: true
      tenantId: ${current.tenantId}
      softDeleteRetentionDays: 7
      purgeProtectionEnabled: true
      skuName: standard
  userAssignedIdentity:
    type: azure:keyvault:AccessPolicy
    properties:
      keyVaultId: ${exampleKeyVault.id}
      tenantId: ${current.tenantId}
      objectId: ${exampleUserAssignedIdentity.principalId}
      keyPermissions:
        - Get
        - UnwrapKey
        - WrapKey
      secretPermissions:
        - Get
  client:
    type: azure:keyvault:AccessPolicy
    properties:
      keyVaultId: ${exampleKeyVault.id}
      tenantId: ${current.tenantId}
      objectId: ${current.objectId}
      keyPermissions:
        - Get
        - Create
        - Delete
        - List
        - Restore
        - Recover
        - UnwrapKey
        - WrapKey
        - Purge
        - Encrypt
        - Decrypt
        - Sign
        - Verify
        - GetRotationPolicy
      secretPermissions:
        - Get
  exampleKey:
    type: azure:keyvault:Key
    name: example
    properties:
      name: example-kvk
      keyVaultId: ${exampleKeyVault.id}
      keyType: RSA
      keySize: 2048
      keyOpts:
        - decrypt
        - encrypt
        - sign
        - unwrapKey
        - verify
        - wrapKey
    options:
      dependsOn:
        - ${userAssignedIdentity}
        - ${client}
  exampleVolumeGroup:
    type: azure:elasticsan:VolumeGroup
    name: example
    properties:
      name: example-esvg
      elasticSanId: ${exampleElasticSan.id}
      encryptionType: EncryptionAtRestWithCustomerManagedKey
      encryption:
        keyVaultKeyId: ${exampleKey.versionlessId}
        userAssignedIdentityId: ${exampleUserAssignedIdentity.id}
      identity:
        type: UserAssigned
        identityIds:
          - ${exampleUserAssignedIdentity.id}
      networkRules:
        - subnetId: ${exampleSubnet.id}
          action: Allow
variables:
  current:
    fn::invoke:
      function: azure:core:getClientConfig
      arguments: {}
```
<!--End PulumiCodeChooser -->

## Import

An existing Elastic SAN Volume Group can be imported into Pulumi using the `resource id`, e.g.

```sh
$ pulumi import azure:elasticsan/volumeGroup:VolumeGroup example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.ElasticSan/elasticSans/esan1/volumeGroups/vg1
```

û
elasticSanId" âSpecifies the Elastic SAN ID within which this Elastic SAN Volume Group should exist. Changing this forces a new resource to be created.
ü

encryptiongBe:c
a

elasticsanVolumeGroupEncryption<azure:elasticsan/VolumeGroupEncryption:VolumeGroupEncryptionßAn `encryption` block as defined below.

> **NOTE:** The `encryption` block can only be set when `encryption_type` is set to `EncryptionAtRestWithCustomerManagedKey`.

encryptionTypeB" ◊Specifies the type of the key used to encrypt the data of the disk. Possible values are `EncryptionAtRestWithCustomerManagedKey` and `EncryptionAtRestWithPlatformKey`. Defaults to `EncryptionAtRestWithPlatformKey`.

identityaB_:]
[

elasticsanVolumeGroupIdentity8azure:elasticsan/VolumeGroupIdentity:VolumeGroupIdentityÄAn `identity` block as defined below. Specifies the Managed Identity which should be assigned to this Elastic SAN Volume Group.
v
nameB" hSpecifies the name of this Elastic SAN Volume Group. Changing this forces a new resource to be created.
≤
networkRuleslBj*h:f
d

elasticsanVolumeGroupNetworkRule>azure:elasticsan/VolumeGroupNetworkRule:VolumeGroupNetworkRule4One or more `network_rule` blocks as defined below.
y
protocolTypeB" cSpecifies the type of the storage target. The only possible value is `Iscsi`. Defaults to `Iscsi`.
"û
elasticSanId" âSpecifies the Elastic SAN ID within which this Elastic SAN Volume Group should exist. Changing this forces a new resource to be created.
"ü

encryptiongBe:c
a

elasticsanVolumeGroupEncryption<azure:elasticsan/VolumeGroupEncryption:VolumeGroupEncryptionßAn `encryption` block as defined below.

> **NOTE:** The `encryption` block can only be set when `encryption_type` is set to `EncryptionAtRestWithCustomerManagedKey`.
"
encryptionTypeB" ◊Specifies the type of the key used to encrypt the data of the disk. Possible values are `EncryptionAtRestWithCustomerManagedKey` and `EncryptionAtRestWithPlatformKey`. Defaults to `EncryptionAtRestWithPlatformKey`.
"
identityaB_:]
[

elasticsanVolumeGroupIdentity8azure:elasticsan/VolumeGroupIdentity:VolumeGroupIdentityÄAn `identity` block as defined below. Specifies the Managed Identity which should be assigned to this Elastic SAN Volume Group.
"t
name" hSpecifies the name of this Elastic SAN Volume Group. Changing this forces a new resource to be created.
"≤
networkRuleslBj*h:f
d

elasticsanVolumeGroupNetworkRule>azure:elasticsan/VolumeGroupNetworkRule:VolumeGroupNetworkRule4One or more `network_rule` blocks as defined below.
"y
protocolTypeB" cSpecifies the type of the storage target. The only possible value is `Iscsi`. Defaults to `Iscsi`.
*ÇD
2
	eventgridDomainazure:eventgrid/domain:DomainÃManages an EventGrid Domain

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "example-resources",
    location: "West Europe",
});
const exampleDomain = new azure.eventgrid.Domain("example", {
    name: "my-eventgrid-domain",
    location: example.location,
    resourceGroupName: example.name,
    tags: {
        environment: "Production",
    },
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="example-resources",
    location="West Europe")
example_domain = azure.eventgrid.Domain("example",
    name="my-eventgrid-domain",
    location=example.location,
    resource_group_name=example.name,
    tags={
        "environment": "Production",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "example-resources",
        Location = "West Europe",
    });

    var exampleDomain = new Azure.EventGrid.Domain("example", new()
    {
        Name = "my-eventgrid-domain",
        Location = example.Location,
        ResourceGroupName = example.Name,
        Tags = 
        {
            { "environment", "Production" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventgrid"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("example-resources"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		_, err = eventgrid.NewDomain(ctx, "example", &eventgrid.DomainArgs{
			Name:              pulumi.String("my-eventgrid-domain"),
			Location:          example.Location,
			ResourceGroupName: example.Name,
			Tags: pulumi.StringMap{
				"environment": pulumi.String("Production"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.eventgrid.Domain;
import com.pulumi.azure.eventgrid.DomainArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("example-resources")
            .location("West Europe")
            .build());

        var exampleDomain = new Domain("exampleDomain", DomainArgs.builder()
            .name("my-eventgrid-domain")
            .location(example.location())
            .resourceGroupName(example.name())
            .tags(Map.of("environment", "Production"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: example-resources
      location: West Europe
  exampleDomain:
    type: azure:eventgrid:Domain
    name: example
    properties:
      name: my-eventgrid-domain
      location: ${example.location}
      resourceGroupName: ${example.name}
      tags:
        environment: Production
```
<!--End PulumiCodeChooser -->

## Import

EventGrid Domains can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:eventgrid/domain:Domain domain1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventGrid/domains/domain1
```

µ
$autoCreateTopicWithFirstSubscriptionB
 ÜWhether to create the domain topic when the first event subscription at the scope of the domain topic is created. Defaults to `true`.
≥
#autoDeleteTopicWithLastSubscriptionB
 ÖWhether to delete the domain topic when the last event subscription at the scope of the domain topic is deleted. Defaults to `true`.
Ñ
identityPBN:L
J
	eventgridDomainIdentity-azure:eventgrid/DomainIdentity:DomainIdentity&An `identity` block as defined below.
¨
inboundIpRulesaB_*]:[
Y
	eventgridDomainInboundIpRule7azure:eventgrid/DomainInboundIpRule:DomainInboundIpRule7One or more `inbound_ip_rule` blocks as defined below.
ê
inputMappingDefaultValuesÑBÅ:
}
	eventgridDomainInputMappingDefaultValuesOazure:eventgrid/DomainInputMappingDefaultValues:DomainInputMappingDefaultValueslA `input_mapping_default_values` block as defined below. Changing this forces a new resource to be created.
Í
inputMappingFieldsnBl:j
h
	eventgridDomainInputMappingFieldsAazure:eventgrid/DomainInputMappingFields:DomainInputMappingFieldsdA `input_mapping_fields` block as defined below. Changing this forces a new resource to be created.
é
inputSchemaB" ¯Specifies the schema in which incoming events will be published to this domain. Allowed values are `CloudEventSchemaV1_0`, `CustomEventSchema`, or `EventGridSchema`. Defaults to `EventGridSchema`. Changing this forces a new resource to be created.
x
localAuthEnabledB
 ^Whether local authentication methods is enabled for the EventGrid Domain. Defaults to `true`.
á
locationB" uSpecifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
v
nameB" hSpecifies the name of the EventGrid Domain resource. Changing this forces a new resource to be created.
y
publicNetworkAccessEnabledB
 UWhether or not public network access is allowed for this server. Defaults to `true`.
ë
resourceGroupName" xThe name of the resource group in which the EventGrid Domain exists. Changing this forces a new resource to be created.
=
tagsB2" -A mapping of tags to assign to the resource.
"µ
$autoCreateTopicWithFirstSubscriptionB
 ÜWhether to create the domain topic when the first event subscription at the scope of the domain topic is created. Defaults to `true`.
"≥
#autoDeleteTopicWithLastSubscriptionB
 ÖWhether to delete the domain topic when the last event subscription at the scope of the domain topic is deleted. Defaults to `true`.
"C
endpoint" 3The Endpoint associated with the EventGrid Domain.
"Ñ
identityPBN:L
J
	eventgridDomainIdentity-azure:eventgrid/DomainIdentity:DomainIdentity&An `identity` block as defined below.
"¨
inboundIpRulesaB_*]:[
Y
	eventgridDomainInboundIpRule7azure:eventgrid/DomainInboundIpRule:DomainInboundIpRule7One or more `inbound_ip_rule` blocks as defined below.
"ê
inputMappingDefaultValuesÑBÅ:
}
	eventgridDomainInputMappingDefaultValuesOazure:eventgrid/DomainInputMappingDefaultValues:DomainInputMappingDefaultValueslA `input_mapping_default_values` block as defined below. Changing this forces a new resource to be created.
"Í
inputMappingFieldsnBl:j
h
	eventgridDomainInputMappingFieldsAazure:eventgrid/DomainInputMappingFields:DomainInputMappingFieldsdA `input_mapping_fields` block as defined below. Changing this forces a new resource to be created.
"é
inputSchemaB" ¯Specifies the schema in which incoming events will be published to this domain. Allowed values are `CloudEventSchemaV1_0`, `CustomEventSchema`, or `EventGridSchema`. Defaults to `EventGridSchema`. Changing this forces a new resource to be created.
"x
localAuthEnabledB
 ^Whether local authentication methods is enabled for the EventGrid Domain. Defaults to `true`.
"Ö
location" uSpecifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
"t
name" hSpecifies the name of the EventGrid Domain resource. Changing this forces a new resource to be created.
"\
primaryAccessKey" DThe Primary Shared Access Key associated with the EventGrid Domain.
"y
publicNetworkAccessEnabledB
 UWhether or not public network access is allowed for this server. Defaults to `true`.
"ë
resourceGroupName" xThe name of the resource group in which the EventGrid Domain exists. Changing this forces a new resource to be created.
"`
secondaryAccessKey" FThe Secondary Shared Access Key associated with the EventGrid Domain.
"=
tagsB2" -A mapping of tags to assign to the resource.
*÷1
A
	eventgridDomainTopic'azure:eventgrid/domainTopic:DomainTopicà+Manages an EventGrid Domain Topic

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "example-resources",
    location: "West Europe",
});
const exampleDomain = new azure.eventgrid.Domain("example", {
    name: "my-eventgrid-domain",
    location: example.location,
    resourceGroupName: example.name,
    tags: {
        environment: "Production",
    },
});
const exampleDomainTopic = new azure.eventgrid.DomainTopic("example", {
    name: "my-eventgrid-domain-topic",
    domainName: exampleDomain.name,
    resourceGroupName: example.name,
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="example-resources",
    location="West Europe")
example_domain = azure.eventgrid.Domain("example",
    name="my-eventgrid-domain",
    location=example.location,
    resource_group_name=example.name,
    tags={
        "environment": "Production",
    })
example_domain_topic = azure.eventgrid.DomainTopic("example",
    name="my-eventgrid-domain-topic",
    domain_name=example_domain.name,
    resource_group_name=example.name)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "example-resources",
        Location = "West Europe",
    });

    var exampleDomain = new Azure.EventGrid.Domain("example", new()
    {
        Name = "my-eventgrid-domain",
        Location = example.Location,
        ResourceGroupName = example.Name,
        Tags = 
        {
            { "environment", "Production" },
        },
    });

    var exampleDomainTopic = new Azure.EventGrid.DomainTopic("example", new()
    {
        Name = "my-eventgrid-domain-topic",
        DomainName = exampleDomain.Name,
        ResourceGroupName = example.Name,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventgrid"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("example-resources"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		exampleDomain, err := eventgrid.NewDomain(ctx, "example", &eventgrid.DomainArgs{
			Name:              pulumi.String("my-eventgrid-domain"),
			Location:          example.Location,
			ResourceGroupName: example.Name,
			Tags: pulumi.StringMap{
				"environment": pulumi.String("Production"),
			},
		})
		if err != nil {
			return err
		}
		_, err = eventgrid.NewDomainTopic(ctx, "example", &eventgrid.DomainTopicArgs{
			Name:              pulumi.String("my-eventgrid-domain-topic"),
			DomainName:        exampleDomain.Name,
			ResourceGroupName: example.Name,
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.eventgrid.Domain;
import com.pulumi.azure.eventgrid.DomainArgs;
import com.pulumi.azure.eventgrid.DomainTopic;
import com.pulumi.azure.eventgrid.DomainTopicArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("example-resources")
            .location("West Europe")
            .build());

        var exampleDomain = new Domain("exampleDomain", DomainArgs.builder()
            .name("my-eventgrid-domain")
            .location(example.location())
            .resourceGroupName(example.name())
            .tags(Map.of("environment", "Production"))
            .build());

        var exampleDomainTopic = new DomainTopic("exampleDomainTopic", DomainTopicArgs.builder()
            .name("my-eventgrid-domain-topic")
            .domainName(exampleDomain.name())
            .resourceGroupName(example.name())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: example-resources
      location: West Europe
  exampleDomain:
    type: azure:eventgrid:Domain
    name: example
    properties:
      name: my-eventgrid-domain
      location: ${example.location}
      resourceGroupName: ${example.name}
      tags:
        environment: Production
  exampleDomainTopic:
    type: azure:eventgrid:DomainTopic
    name: example
    properties:
      name: my-eventgrid-domain-topic
      domainName: ${exampleDomain.name}
      resourceGroupName: ${example.name}
```
<!--End PulumiCodeChooser -->

## Import

EventGrid Domain Topics can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:eventgrid/domainTopic:DomainTopic topic1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventGrid/domains/domain1/topics/topic1
```

q

domainName" _Specifies the name of the EventGrid Domain. Changing this forces a new resource to be created.
|
nameB" nSpecifies the name of the EventGrid Domain Topic resource. Changing this forces a new resource to be created.
ë
resourceGroupName" xThe name of the resource group in which the EventGrid Domain exists. Changing this forces a new resource to be created.
"q

domainName" _Specifies the name of the EventGrid Domain. Changing this forces a new resource to be created.
"z
name" nSpecifies the name of the EventGrid Domain Topic resource. Changing this forces a new resource to be created.
"ë
resourceGroupName" xThe name of the resource group in which the EventGrid Domain exists. Changing this forces a new resource to be created.
*€{
S
	eventgridEventSubscription3azure:eventgrid/eventSubscription:EventSubscriptionè=Manages an EventGrid Event Subscription

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "example-resources",
    location: "West Europe",
});
const exampleAccount = new azure.storage.Account("example", {
    name: "exampleasa",
    resourceGroupName: example.name,
    location: example.location,
    accountTier: "Standard",
    accountReplicationType: "LRS",
    tags: {
        environment: "staging",
    },
});
const exampleQueue = new azure.storage.Queue("example", {
    name: "example-astq",
    storageAccountName: exampleAccount.name,
});
const exampleEventSubscription = new azure.eventgrid.EventSubscription("example", {
    name: "example-aees",
    scope: example.id,
    storageQueueEndpoint: {
        storageAccountId: exampleAccount.id,
        queueName: exampleQueue.name,
    },
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="example-resources",
    location="West Europe")
example_account = azure.storage.Account("example",
    name="exampleasa",
    resource_group_name=example.name,
    location=example.location,
    account_tier="Standard",
    account_replication_type="LRS",
    tags={
        "environment": "staging",
    })
example_queue = azure.storage.Queue("example",
    name="example-astq",
    storage_account_name=example_account.name)
example_event_subscription = azure.eventgrid.EventSubscription("example",
    name="example-aees",
    scope=example.id,
    storage_queue_endpoint={
        "storage_account_id": example_account.id,
        "queue_name": example_queue.name,
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "example-resources",
        Location = "West Europe",
    });

    var exampleAccount = new Azure.Storage.Account("example", new()
    {
        Name = "exampleasa",
        ResourceGroupName = example.Name,
        Location = example.Location,
        AccountTier = "Standard",
        AccountReplicationType = "LRS",
        Tags = 
        {
            { "environment", "staging" },
        },
    });

    var exampleQueue = new Azure.Storage.Queue("example", new()
    {
        Name = "example-astq",
        StorageAccountName = exampleAccount.Name,
    });

    var exampleEventSubscription = new Azure.EventGrid.EventSubscription("example", new()
    {
        Name = "example-aees",
        Scope = example.Id,
        StorageQueueEndpoint = new Azure.EventGrid.Inputs.EventSubscriptionStorageQueueEndpointArgs
        {
            StorageAccountId = exampleAccount.Id,
            QueueName = exampleQueue.Name,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventgrid"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/storage"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("example-resources"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		exampleAccount, err := storage.NewAccount(ctx, "example", &storage.AccountArgs{
			Name:                   pulumi.String("exampleasa"),
			ResourceGroupName:      example.Name,
			Location:               example.Location,
			AccountTier:            pulumi.String("Standard"),
			AccountReplicationType: pulumi.String("LRS"),
			Tags: pulumi.StringMap{
				"environment": pulumi.String("staging"),
			},
		})
		if err != nil {
			return err
		}
		exampleQueue, err := storage.NewQueue(ctx, "example", &storage.QueueArgs{
			Name:               pulumi.String("example-astq"),
			StorageAccountName: exampleAccount.Name,
		})
		if err != nil {
			return err
		}
		_, err = eventgrid.NewEventSubscription(ctx, "example", &eventgrid.EventSubscriptionArgs{
			Name:  pulumi.String("example-aees"),
			Scope: example.ID(),
			StorageQueueEndpoint: &eventgrid.EventSubscriptionStorageQueueEndpointArgs{
				StorageAccountId: exampleAccount.ID(),
				QueueName:        exampleQueue.Name,
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.storage.Account;
import com.pulumi.azure.storage.AccountArgs;
import com.pulumi.azure.storage.Queue;
import com.pulumi.azure.storage.QueueArgs;
import com.pulumi.azure.eventgrid.EventSubscription;
import com.pulumi.azure.eventgrid.EventSubscriptionArgs;
import com.pulumi.azure.eventgrid.inputs.EventSubscriptionStorageQueueEndpointArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("example-resources")
            .location("West Europe")
            .build());

        var exampleAccount = new Account("exampleAccount", AccountArgs.builder()
            .name("exampleasa")
            .resourceGroupName(example.name())
            .location(example.location())
            .accountTier("Standard")
            .accountReplicationType("LRS")
            .tags(Map.of("environment", "staging"))
            .build());

        var exampleQueue = new Queue("exampleQueue", QueueArgs.builder()
            .name("example-astq")
            .storageAccountName(exampleAccount.name())
            .build());

        var exampleEventSubscription = new EventSubscription("exampleEventSubscription", EventSubscriptionArgs.builder()
            .name("example-aees")
            .scope(example.id())
            .storageQueueEndpoint(EventSubscriptionStorageQueueEndpointArgs.builder()
                .storageAccountId(exampleAccount.id())
                .queueName(exampleQueue.name())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: example-resources
      location: West Europe
  exampleAccount:
    type: azure:storage:Account
    name: example
    properties:
      name: exampleasa
      resourceGroupName: ${example.name}
      location: ${example.location}
      accountTier: Standard
      accountReplicationType: LRS
      tags:
        environment: staging
  exampleQueue:
    type: azure:storage:Queue
    name: example
    properties:
      name: example-astq
      storageAccountName: ${exampleAccount.name}
  exampleEventSubscription:
    type: azure:eventgrid:EventSubscription
    name: example
    properties:
      name: example-aees
      scope: ${example.id}
      storageQueueEndpoint:
        storageAccountId: ${exampleAccount.id}
        queueName: ${exampleQueue.name}
```
<!--End PulumiCodeChooser -->

## Import

EventGrid Event Subscription's can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:eventgrid/eventSubscription:EventSubscription eventSubscription1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventGrid/topics/topic1/providers/Microsoft.EventGrid/eventSubscriptions/eventSubscription1
```

≈
advancedFilterÑBÅ:
}
	eventgridEventSubscriptionAdvancedFilterOazure:eventgrid/EventSubscriptionAdvancedFilter:EventSubscriptionAdvancedFilter,A `advanced_filter` block as defined below.
π
 advancedFilteringOnArraysEnabledB
 éSpecifies whether advanced filters should be evaluated against an array of values instead of expecting a singular value. Defaults to `false`.
Ï
azureFunctionEndpointõBò:ï
í
	eventgrid&EventSubscriptionAzureFunctionEndpoint]azure:eventgrid/EventSubscriptionAzureFunctionEndpoint:EventSubscriptionAzureFunctionEndpoint5An `azure_function_endpoint` block as defined below.
Œ
deadLetterIdentityíBè:å
â
	eventgrid#EventSubscriptionDeadLetterIdentityWazure:eventgrid/EventSubscriptionDeadLetterIdentity:EventSubscriptionDeadLetterIdentity¢A `dead_letter_identity` block as defined below.

> **Note:** `storage_blob_dead_letter_destination` must be specified when a `dead_letter_identity` is specified
—
deliveryIdentityåBâ:Ü
É
	eventgrid!EventSubscriptionDeliveryIdentitySazure:eventgrid/EventSubscriptionDeliveryIdentity:EventSubscriptionDeliveryIdentity.A `delivery_identity` block as defined below.
·
deliveryPropertiesèBå*â:Ü
É
	eventgrid!EventSubscriptionDeliveryPropertySazure:eventgrid/EventSubscriptionDeliveryProperty:EventSubscriptionDeliveryProperty9One or more `delivery_property` blocks as defined below.
â
eventDeliverySchemaB" ÎSpecifies the event delivery schema for the event subscription. Possible values include: `EventGridSchema`, `CloudEventSchemaV1_0`, `CustomInputSchema`. Defaults to `EventGridSchema`. Changing this forces a new resource to be created.
M
eventhubEndpointIdB" 1Specifies the id where the Event Hub is located.
q
expirationTimeUtcB" VSpecifies the expiration time of the event subscription (Datetime Format `RFC 3339`).
]
hybridConnectionEndpointIdB" 9Specifies the id where the Hybrid Connection is located.
o
includedEventTypesB*" QA list of applicable event types that need to be part of the event subscription.
H
labelsB*" 6A list of labels to assign to the event subscription.
Ç
nameB" tSpecifies the name of the EventGrid Event Subscription resource. Changing this forces a new resource to be created.
¥
retryPolicyzBx:v
t
	eventgridEventSubscriptionRetryPolicyIazure:eventgrid/EventSubscriptionRetryPolicy:EventSubscriptionRetryPolicy)A `retry_policy` block as defined below.
í
scope" ÑSpecifies the scope at which the EventGrid Event Subscription should be created. Changing this forces a new resource to be created.
\
serviceBusQueueEndpointIdB" 9Specifies the id where the Service Bus Queue is located.
\
serviceBusTopicEndpointIdB" 9Specifies the id where the Service Bus Topic is located.
§
 storageBlobDeadLetterDestinationºBπ:∂
≥
	eventgrid1EventSubscriptionStorageBlobDeadLetterDestinationsazure:eventgrid/EventSubscriptionStorageBlobDeadLetterDestination:EventSubscriptionStorageBlobDeadLetterDestinationAA `storage_blob_dead_letter_destination` block as defined below.
Ê
storageQueueEndpointòBï:í
è
	eventgrid%EventSubscriptionStorageQueueEndpoint[azure:eventgrid/EventSubscriptionStorageQueueEndpoint:EventSubscriptionStorageQueueEndpoint3A `storage_queue_endpoint` block as defined below.
ø
subjectFilterÄB~:|
z
	eventgridEventSubscriptionSubjectFilterMazure:eventgrid/EventSubscriptionSubjectFilter:EventSubscriptionSubjectFilter+A `subject_filter` block as defined below.
π
webhookEndpointâBÜ:É
Ä
	eventgrid EventSubscriptionWebhookEndpointQazure:eventgrid/EventSubscriptionWebhookEndpoint:EventSubscriptionWebhookEndpointôA `webhook_endpoint` block as defined below.

> **NOTE:** One of `eventhub_endpoint_id`, `hybrid_connection_endpoint_id`, `service_bus_queue_endpoint_id`, `service_bus_topic_endpoint_id`, `storage_queue_endpoint`, `webhook_endpoint` or `azure_function_endpoint` must be specified.
"≈
advancedFilterÑBÅ:
}
	eventgridEventSubscriptionAdvancedFilterOazure:eventgrid/EventSubscriptionAdvancedFilter:EventSubscriptionAdvancedFilter,A `advanced_filter` block as defined below.
"π
 advancedFilteringOnArraysEnabledB
 éSpecifies whether advanced filters should be evaluated against an array of values instead of expecting a singular value. Defaults to `false`.
"Ï
azureFunctionEndpointõBò:ï
í
	eventgrid&EventSubscriptionAzureFunctionEndpoint]azure:eventgrid/EventSubscriptionAzureFunctionEndpoint:EventSubscriptionAzureFunctionEndpoint5An `azure_function_endpoint` block as defined below.
"Œ
deadLetterIdentityíBè:å
â
	eventgrid#EventSubscriptionDeadLetterIdentityWazure:eventgrid/EventSubscriptionDeadLetterIdentity:EventSubscriptionDeadLetterIdentity¢A `dead_letter_identity` block as defined below.

> **Note:** `storage_blob_dead_letter_destination` must be specified when a `dead_letter_identity` is specified
"—
deliveryIdentityåBâ:Ü
É
	eventgrid!EventSubscriptionDeliveryIdentitySazure:eventgrid/EventSubscriptionDeliveryIdentity:EventSubscriptionDeliveryIdentity.A `delivery_identity` block as defined below.
"·
deliveryPropertiesèBå*â:Ü
É
	eventgrid!EventSubscriptionDeliveryPropertySazure:eventgrid/EventSubscriptionDeliveryProperty:EventSubscriptionDeliveryProperty9One or more `delivery_property` blocks as defined below.
"â
eventDeliverySchemaB" ÎSpecifies the event delivery schema for the event subscription. Possible values include: `EventGridSchema`, `CloudEventSchemaV1_0`, `CustomInputSchema`. Defaults to `EventGridSchema`. Changing this forces a new resource to be created.
"K
eventhubEndpointId" 1Specifies the id where the Event Hub is located.
"q
expirationTimeUtcB" VSpecifies the expiration time of the event subscription (Datetime Format `RFC 3339`).
"[
hybridConnectionEndpointId" 9Specifies the id where the Hybrid Connection is located.
"m
includedEventTypes*" QA list of applicable event types that need to be part of the event subscription.
"H
labelsB*" 6A list of labels to assign to the event subscription.
"Ä
name" tSpecifies the name of the EventGrid Event Subscription resource. Changing this forces a new resource to be created.
"≤
retryPolicyx:v
t
	eventgridEventSubscriptionRetryPolicyIazure:eventgrid/EventSubscriptionRetryPolicy:EventSubscriptionRetryPolicy)A `retry_policy` block as defined below.
"í
scope" ÑSpecifies the scope at which the EventGrid Event Subscription should be created. Changing this forces a new resource to be created.
"\
serviceBusQueueEndpointIdB" 9Specifies the id where the Service Bus Queue is located.
"\
serviceBusTopicEndpointIdB" 9Specifies the id where the Service Bus Topic is located.
"§
 storageBlobDeadLetterDestinationºBπ:∂
≥
	eventgrid1EventSubscriptionStorageBlobDeadLetterDestinationsazure:eventgrid/EventSubscriptionStorageBlobDeadLetterDestination:EventSubscriptionStorageBlobDeadLetterDestinationAA `storage_blob_dead_letter_destination` block as defined below.
"Ê
storageQueueEndpointòBï:í
è
	eventgrid%EventSubscriptionStorageQueueEndpoint[azure:eventgrid/EventSubscriptionStorageQueueEndpoint:EventSubscriptionStorageQueueEndpoint3A `storage_queue_endpoint` block as defined below.
"ø
subjectFilterÄB~:|
z
	eventgridEventSubscriptionSubjectFilterMazure:eventgrid/EventSubscriptionSubjectFilter:EventSubscriptionSubjectFilter+A `subject_filter` block as defined below.
"π
webhookEndpointâBÜ:É
Ä
	eventgrid EventSubscriptionWebhookEndpointQazure:eventgrid/EventSubscriptionWebhookEndpoint:EventSubscriptionWebhookEndpointôA `webhook_endpoint` block as defined below.

> **NOTE:** One of `eventhub_endpoint_id`, `hybrid_connection_endpoint_id`, `service_bus_queue_endpoint_id`, `service_bus_topic_endpoint_id`, `storage_queue_endpoint`, `webhook_endpoint` or `azure_function_endpoint` must be specified.
*‡6
;
	eventgrid	Namespace#azure:eventgrid/namespace:Namespace§ Manages an EventGrid Namespace

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "example-resources",
    location: "West Europe",
});
const exampleNamespace = new azure.eventgrid.Namespace("example", {
    name: "my-eventgrid-namespace",
    location: example.location,
    resourceGroupName: example.name,
    tags: {
        environment: "Production",
    },
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="example-resources",
    location="West Europe")
example_namespace = azure.eventgrid.Namespace("example",
    name="my-eventgrid-namespace",
    location=example.location,
    resource_group_name=example.name,
    tags={
        "environment": "Production",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "example-resources",
        Location = "West Europe",
    });

    var exampleNamespace = new Azure.EventGrid.Namespace("example", new()
    {
        Name = "my-eventgrid-namespace",
        Location = example.Location,
        ResourceGroupName = example.Name,
        Tags = 
        {
            { "environment", "Production" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventgrid"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("example-resources"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		_, err = eventgrid.NewNamespace(ctx, "example", &eventgrid.NamespaceArgs{
			Name:              pulumi.String("my-eventgrid-namespace"),
			Location:          example.Location,
			ResourceGroupName: example.Name,
			Tags: pulumi.StringMap{
				"environment": pulumi.String("Production"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.eventgrid.Namespace;
import com.pulumi.azure.eventgrid.NamespaceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("example-resources")
            .location("West Europe")
            .build());

        var exampleNamespace = new Namespace("exampleNamespace", NamespaceArgs.builder()
            .name("my-eventgrid-namespace")
            .location(example.location())
            .resourceGroupName(example.name())
            .tags(Map.of("environment", "Production"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: example-resources
      location: West Europe
  exampleNamespace:
    type: azure:eventgrid:Namespace
    name: example
    properties:
      name: my-eventgrid-namespace
      location: ${example.location}
      resourceGroupName: ${example.name}
      tags:
        environment: Production
```
<!--End PulumiCodeChooser -->

## Import

EventGrid Namespace's can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:eventgrid/namespace:Namespace namespace1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventGrid/namespaces/namespace1
```

Ç
capacityB pSpecifies the Capacity / Throughput Units for an Eventgrid Namespace. Valid values can be between `1` and `40`.
ç
identityYBW:U
S
	eventgridNamespaceIdentity3azure:eventgrid/NamespaceIdentity:NamespaceIdentity&An `identity` block as defined below.
µ
inboundIpRulesjBh*f:d
b
	eventgridNamespaceInboundIpRule=azure:eventgrid/NamespaceInboundIpRule:NamespaceInboundIpRule7One or more `inbound_ip_rule` blocks as defined below.
ç
locationB" {Specifies the supported Azure location where the resource should exist. Changing this forces a new resource to be created.
z
nameB" lSpecifies the name of the Event Grid Namespace resource. Changing this forces a new resource to be created.
u
publicNetworkAccessB" XWhether or not public network access is allowed for this server. Defaults to `Enabled`.
ú
resourceGroupName" ÇThe name of the resource group in which the Event Grid Namespace should exist. Changing this forces a new resource to be created.
g
skuB" ZDefines which tier to use. The only possible value is `Standard`. Defaults to `Standard`.
=
tagsB2" -A mapping of tags to assign to the resource.
Ê
topicSpacesConfigurationsèBå*â:Ü
É
	eventgrid!NamespaceTopicSpacesConfigurationSazure:eventgrid/NamespaceTopicSpacesConfiguration:NamespaceTopicSpacesConfiguration7A `topic_spaces_configuration` block as defined below.
"Ç
capacityB pSpecifies the Capacity / Throughput Units for an Eventgrid Namespace. Valid values can be between `1` and `40`.
"ç
identityYBW:U
S
	eventgridNamespaceIdentity3azure:eventgrid/NamespaceIdentity:NamespaceIdentity&An `identity` block as defined below.
"µ
inboundIpRulesjBh*f:d
b
	eventgridNamespaceInboundIpRule=azure:eventgrid/NamespaceInboundIpRule:NamespaceInboundIpRule7One or more `inbound_ip_rule` blocks as defined below.
"ã
location" {Specifies the supported Azure location where the resource should exist. Changing this forces a new resource to be created.
"x
name" lSpecifies the name of the Event Grid Namespace resource. Changing this forces a new resource to be created.
"u
publicNetworkAccessB" XWhether or not public network access is allowed for this server. Defaults to `Enabled`.
"ú
resourceGroupName" ÇThe name of the resource group in which the Event Grid Namespace should exist. Changing this forces a new resource to be created.
"g
skuB" ZDefines which tier to use. The only possible value is `Standard`. Defaults to `Standard`.
"=
tagsB2" -A mapping of tags to assign to the resource.
"Ê
topicSpacesConfigurationsèBå*â:Ü
É
	eventgrid!NamespaceTopicSpacesConfigurationSazure:eventgrid/NamespaceTopicSpacesConfiguration:NamespaceTopicSpacesConfiguration7A `topic_spaces_configuration` block as defined below.
*ˆR
A
	eventgridSystemTopic'azure:eventgrid/systemTopic:SystemTopicà3Manages an Event Grid System Topic.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "example-resources",
    location: "West Europe",
});
const exampleAccount = new azure.storage.Account("example", {
    name: "examplestoracct",
    resourceGroupName: example.name,
    location: example.location,
    accountTier: "Standard",
    accountReplicationType: "LRS",
    tags: {
        environment: "staging",
    },
});
const exampleSystemTopic = new azure.eventgrid.SystemTopic("example", {
    name: "example-topic",
    resourceGroupName: example.name,
    location: example.location,
    sourceArmResourceId: exampleAccount.id,
    topicType: "Microsoft.Storage.StorageAccounts",
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="example-resources",
    location="West Europe")
example_account = azure.storage.Account("example",
    name="examplestoracct",
    resource_group_name=example.name,
    location=example.location,
    account_tier="Standard",
    account_replication_type="LRS",
    tags={
        "environment": "staging",
    })
example_system_topic = azure.eventgrid.SystemTopic("example",
    name="example-topic",
    resource_group_name=example.name,
    location=example.location,
    source_arm_resource_id=example_account.id,
    topic_type="Microsoft.Storage.StorageAccounts")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "example-resources",
        Location = "West Europe",
    });

    var exampleAccount = new Azure.Storage.Account("example", new()
    {
        Name = "examplestoracct",
        ResourceGroupName = example.Name,
        Location = example.Location,
        AccountTier = "Standard",
        AccountReplicationType = "LRS",
        Tags = 
        {
            { "environment", "staging" },
        },
    });

    var exampleSystemTopic = new Azure.EventGrid.SystemTopic("example", new()
    {
        Name = "example-topic",
        ResourceGroupName = example.Name,
        Location = example.Location,
        SourceArmResourceId = exampleAccount.Id,
        TopicType = "Microsoft.Storage.StorageAccounts",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventgrid"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/storage"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("example-resources"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		exampleAccount, err := storage.NewAccount(ctx, "example", &storage.AccountArgs{
			Name:                   pulumi.String("examplestoracct"),
			ResourceGroupName:      example.Name,
			Location:               example.Location,
			AccountTier:            pulumi.String("Standard"),
			AccountReplicationType: pulumi.String("LRS"),
			Tags: pulumi.StringMap{
				"environment": pulumi.String("staging"),
			},
		})
		if err != nil {
			return err
		}
		_, err = eventgrid.NewSystemTopic(ctx, "example", &eventgrid.SystemTopicArgs{
			Name:                pulumi.String("example-topic"),
			ResourceGroupName:   example.Name,
			Location:            example.Location,
			SourceArmResourceId: exampleAccount.ID(),
			TopicType:           pulumi.String("Microsoft.Storage.StorageAccounts"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.storage.Account;
import com.pulumi.azure.storage.AccountArgs;
import com.pulumi.azure.eventgrid.SystemTopic;
import com.pulumi.azure.eventgrid.SystemTopicArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("example-resources")
            .location("West Europe")
            .build());

        var exampleAccount = new Account("exampleAccount", AccountArgs.builder()
            .name("examplestoracct")
            .resourceGroupName(example.name())
            .location(example.location())
            .accountTier("Standard")
            .accountReplicationType("LRS")
            .tags(Map.of("environment", "staging"))
            .build());

        var exampleSystemTopic = new SystemTopic("exampleSystemTopic", SystemTopicArgs.builder()
            .name("example-topic")
            .resourceGroupName(example.name())
            .location(example.location())
            .sourceArmResourceId(exampleAccount.id())
            .topicType("Microsoft.Storage.StorageAccounts")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: example-resources
      location: West Europe
  exampleAccount:
    type: azure:storage:Account
    name: example
    properties:
      name: examplestoracct
      resourceGroupName: ${example.name}
      location: ${example.location}
      accountTier: Standard
      accountReplicationType: LRS
      tags:
        environment: staging
  exampleSystemTopic:
    type: azure:eventgrid:SystemTopic
    name: example
    properties:
      name: example-topic
      resourceGroupName: ${example.name}
      location: ${example.location}
      sourceArmResourceId: ${exampleAccount.id}
      topicType: Microsoft.Storage.StorageAccounts
```
<!--End PulumiCodeChooser -->

## Import

Event Grid System Topic can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:eventgrid/systemTopic:SystemTopic example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventGrid/systemTopics/systemTopic1
```

ì
identity_B]:[
Y
	eventgridSystemTopicIdentity7azure:eventgrid/SystemTopicIdentity:SystemTopicIdentity&An `identity` block as defined below.
ñ
locationB" ÉThe Azure Region where the Event Grid System Topic should exist. Changing this forces a new Event Grid System Topic to be created.
ë
nameB" ÇThe name which should be used for this Event Grid System Topic. Changing this forces a new Event Grid System Topic to be created.
´
resourceGroupName" ëThe name of the Resource Group where the Event Grid System Topic should exist. Changing this forces a new Event Grid System Topic to be created.
è
sourceArmResourceId" tThe ID of the Event Grid System Topic ARM Source. Changing this forces a new Event Grid System Topic to be created.

tagsB2" î	
	topicType" Ç	The Topic Type of the Event Grid System Topic. The topic type is validated by Azure and there may be additional topic types beyond the following: `Microsoft.AppConfiguration.ConfigurationStores`, `Microsoft.Communication.CommunicationServices`, `Microsoft.ContainerRegistry.Registries`, `Microsoft.Devices.IoTHubs`, `Microsoft.EventGrid.Domains`, `Microsoft.EventGrid.Topics`, `Microsoft.Eventhub.Namespaces`, `Microsoft.KeyVault.vaults`, `Microsoft.MachineLearningServices.Workspaces`, `Microsoft.Maps.Accounts`, `Microsoft.Media.MediaServices`, `Microsoft.Resources.ResourceGroups`, `Microsoft.Resources.Subscriptions`, `Microsoft.ServiceBus.Namespaces`, `Microsoft.SignalRService.SignalR`, `Microsoft.Storage.StorageAccounts`, `Microsoft.Web.ServerFarms` and `Microsoft.Web.Sites`. Changing this forces a new Event Grid System Topic to be created.

> **NOTE:** Some `topic_type`s (e.g. **Microsoft.Resources.Subscriptions**) requires location to be set to `Global` instead of a real location like `West US`.

> **NOTE:** You can use Azure CLI to get a full list of the available topic types: `az eventgrid topic-type  list --output json | grep -w id`
"ì
identity_B]:[
Y
	eventgridSystemTopicIdentity7azure:eventgrid/SystemTopicIdentity:SystemTopicIdentity&An `identity` block as defined below.
"î
location" ÉThe Azure Region where the Event Grid System Topic should exist. Changing this forces a new Event Grid System Topic to be created.
"V
metricArmResourceId" ;The Metric ARM Resource ID of the Event Grid System Topic.
"è
name" ÇThe name which should be used for this Event Grid System Topic. Changing this forces a new Event Grid System Topic to be created.
"´
resourceGroupName" ëThe name of the Resource Group where the Event Grid System Topic should exist. Changing this forces a new Event Grid System Topic to be created.
"è
sourceArmResourceId" tThe ID of the Event Grid System Topic ARM Source. Changing this forces a new Event Grid System Topic to be created.
"
tagsB2" "î	
	topicType" Ç	The Topic Type of the Event Grid System Topic. The topic type is validated by Azure and there may be additional topic types beyond the following: `Microsoft.AppConfiguration.ConfigurationStores`, `Microsoft.Communication.CommunicationServices`, `Microsoft.ContainerRegistry.Registries`, `Microsoft.Devices.IoTHubs`, `Microsoft.EventGrid.Domains`, `Microsoft.EventGrid.Topics`, `Microsoft.Eventhub.Namespaces`, `Microsoft.KeyVault.vaults`, `Microsoft.MachineLearningServices.Workspaces`, `Microsoft.Maps.Accounts`, `Microsoft.Media.MediaServices`, `Microsoft.Resources.ResourceGroups`, `Microsoft.Resources.Subscriptions`, `Microsoft.ServiceBus.Namespaces`, `Microsoft.SignalRService.SignalR`, `Microsoft.Storage.StorageAccounts`, `Microsoft.Web.ServerFarms` and `Microsoft.Web.Sites`. Changing this forces a new Event Grid System Topic to be created.

> **NOTE:** Some `topic_type`s (e.g. **Microsoft.Resources.Subscriptions**) requires location to be set to `Global` instead of a real location like `West US`.

> **NOTE:** You can use Azure CLI to get a full list of the available topic types: `az eventgrid topic-type  list --output json | grep -w id`
*©ô
t
	eventgridSystemTopicEventSubscriptionIazure:eventgrid/systemTopicEventSubscription:SystemTopicEventSubscriptionùRManages an EventGrid System Topic Event Subscription.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "example-rg",
    location: "West Europe",
});
const exampleAccount = new azure.storage.Account("example", {
    name: "examplestorageaccount",
    resourceGroupName: example.name,
    location: example.location,
    accountTier: "Standard",
    accountReplicationType: "LRS",
    tags: {
        environment: "staging",
    },
});
const exampleQueue = new azure.storage.Queue("example", {
    name: "examplestoragequeue",
    storageAccountName: exampleAccount.name,
});
const exampleSystemTopic = new azure.eventgrid.SystemTopic("example", {
    name: "example-system-topic",
    location: "Global",
    resourceGroupName: example.name,
    sourceArmResourceId: example.id,
    topicType: "Microsoft.Resources.ResourceGroups",
});
const exampleSystemTopicEventSubscription = new azure.eventgrid.SystemTopicEventSubscription("example", {
    name: "example-event-subscription",
    systemTopic: exampleSystemTopic.name,
    resourceGroupName: example.name,
    storageQueueEndpoint: {
        storageAccountId: exampleAccount.id,
        queueName: exampleQueue.name,
    },
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="example-rg",
    location="West Europe")
example_account = azure.storage.Account("example",
    name="examplestorageaccount",
    resource_group_name=example.name,
    location=example.location,
    account_tier="Standard",
    account_replication_type="LRS",
    tags={
        "environment": "staging",
    })
example_queue = azure.storage.Queue("example",
    name="examplestoragequeue",
    storage_account_name=example_account.name)
example_system_topic = azure.eventgrid.SystemTopic("example",
    name="example-system-topic",
    location="Global",
    resource_group_name=example.name,
    source_arm_resource_id=example.id,
    topic_type="Microsoft.Resources.ResourceGroups")
example_system_topic_event_subscription = azure.eventgrid.SystemTopicEventSubscription("example",
    name="example-event-subscription",
    system_topic=example_system_topic.name,
    resource_group_name=example.name,
    storage_queue_endpoint={
        "storage_account_id": example_account.id,
        "queue_name": example_queue.name,
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "example-rg",
        Location = "West Europe",
    });

    var exampleAccount = new Azure.Storage.Account("example", new()
    {
        Name = "examplestorageaccount",
        ResourceGroupName = example.Name,
        Location = example.Location,
        AccountTier = "Standard",
        AccountReplicationType = "LRS",
        Tags = 
        {
            { "environment", "staging" },
        },
    });

    var exampleQueue = new Azure.Storage.Queue("example", new()
    {
        Name = "examplestoragequeue",
        StorageAccountName = exampleAccount.Name,
    });

    var exampleSystemTopic = new Azure.EventGrid.SystemTopic("example", new()
    {
        Name = "example-system-topic",
        Location = "Global",
        ResourceGroupName = example.Name,
        SourceArmResourceId = example.Id,
        TopicType = "Microsoft.Resources.ResourceGroups",
    });

    var exampleSystemTopicEventSubscription = new Azure.EventGrid.SystemTopicEventSubscription("example", new()
    {
        Name = "example-event-subscription",
        SystemTopic = exampleSystemTopic.Name,
        ResourceGroupName = example.Name,
        StorageQueueEndpoint = new Azure.EventGrid.Inputs.SystemTopicEventSubscriptionStorageQueueEndpointArgs
        {
            StorageAccountId = exampleAccount.Id,
            QueueName = exampleQueue.Name,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventgrid"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/storage"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("example-rg"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		exampleAccount, err := storage.NewAccount(ctx, "example", &storage.AccountArgs{
			Name:                   pulumi.String("examplestorageaccount"),
			ResourceGroupName:      example.Name,
			Location:               example.Location,
			AccountTier:            pulumi.String("Standard"),
			AccountReplicationType: pulumi.String("LRS"),
			Tags: pulumi.StringMap{
				"environment": pulumi.String("staging"),
			},
		})
		if err != nil {
			return err
		}
		exampleQueue, err := storage.NewQueue(ctx, "example", &storage.QueueArgs{
			Name:               pulumi.String("examplestoragequeue"),
			StorageAccountName: exampleAccount.Name,
		})
		if err != nil {
			return err
		}
		exampleSystemTopic, err := eventgrid.NewSystemTopic(ctx, "example", &eventgrid.SystemTopicArgs{
			Name:                pulumi.String("example-system-topic"),
			Location:            pulumi.String("Global"),
			ResourceGroupName:   example.Name,
			SourceArmResourceId: example.ID(),
			TopicType:           pulumi.String("Microsoft.Resources.ResourceGroups"),
		})
		if err != nil {
			return err
		}
		_, err = eventgrid.NewSystemTopicEventSubscription(ctx, "example", &eventgrid.SystemTopicEventSubscriptionArgs{
			Name:              pulumi.String("example-event-subscription"),
			SystemTopic:       exampleSystemTopic.Name,
			ResourceGroupName: example.Name,
			StorageQueueEndpoint: &eventgrid.SystemTopicEventSubscriptionStorageQueueEndpointArgs{
				StorageAccountId: exampleAccount.ID(),
				QueueName:        exampleQueue.Name,
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.storage.Account;
import com.pulumi.azure.storage.AccountArgs;
import com.pulumi.azure.storage.Queue;
import com.pulumi.azure.storage.QueueArgs;
import com.pulumi.azure.eventgrid.SystemTopic;
import com.pulumi.azure.eventgrid.SystemTopicArgs;
import com.pulumi.azure.eventgrid.SystemTopicEventSubscription;
import com.pulumi.azure.eventgrid.SystemTopicEventSubscriptionArgs;
import com.pulumi.azure.eventgrid.inputs.SystemTopicEventSubscriptionStorageQueueEndpointArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("example-rg")
            .location("West Europe")
            .build());

        var exampleAccount = new Account("exampleAccount", AccountArgs.builder()
            .name("examplestorageaccount")
            .resourceGroupName(example.name())
            .location(example.location())
            .accountTier("Standard")
            .accountReplicationType("LRS")
            .tags(Map.of("environment", "staging"))
            .build());

        var exampleQueue = new Queue("exampleQueue", QueueArgs.builder()
            .name("examplestoragequeue")
            .storageAccountName(exampleAccount.name())
            .build());

        var exampleSystemTopic = new SystemTopic("exampleSystemTopic", SystemTopicArgs.builder()
            .name("example-system-topic")
            .location("Global")
            .resourceGroupName(example.name())
            .sourceArmResourceId(example.id())
            .topicType("Microsoft.Resources.ResourceGroups")
            .build());

        var exampleSystemTopicEventSubscription = new SystemTopicEventSubscription("exampleSystemTopicEventSubscription", SystemTopicEventSubscriptionArgs.builder()
            .name("example-event-subscription")
            .systemTopic(exampleSystemTopic.name())
            .resourceGroupName(example.name())
            .storageQueueEndpoint(SystemTopicEventSubscriptionStorageQueueEndpointArgs.builder()
                .storageAccountId(exampleAccount.id())
                .queueName(exampleQueue.name())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: example-rg
      location: West Europe
  exampleAccount:
    type: azure:storage:Account
    name: example
    properties:
      name: examplestorageaccount
      resourceGroupName: ${example.name}
      location: ${example.location}
      accountTier: Standard
      accountReplicationType: LRS
      tags:
        environment: staging
  exampleQueue:
    type: azure:storage:Queue
    name: example
    properties:
      name: examplestoragequeue
      storageAccountName: ${exampleAccount.name}
  exampleSystemTopic:
    type: azure:eventgrid:SystemTopic
    name: example
    properties:
      name: example-system-topic
      location: Global
      resourceGroupName: ${example.name}
      sourceArmResourceId: ${example.id}
      topicType: Microsoft.Resources.ResourceGroups
  exampleSystemTopicEventSubscription:
    type: azure:eventgrid:SystemTopicEventSubscription
    name: example
    properties:
      name: example-event-subscription
      systemTopic: ${exampleSystemTopic.name}
      resourceGroupName: ${example.name}
      storageQueueEndpoint:
        storageAccountId: ${exampleAccount.id}
        queueName: ${exampleQueue.name}
```
<!--End PulumiCodeChooser -->

## Import

EventGrid System Topic Event Subscriptions can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:eventgrid/systemTopicEventSubscription:SystemTopicEventSubscription example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventGrid/systemTopics/topic1/eventSubscriptions/subscription1
```

Ë
advancedFilterßB§:°
û
	eventgrid*SystemTopicEventSubscriptionAdvancedFiltereazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilter:SystemTopicEventSubscriptionAdvancedFilter,A `advanced_filter` block as defined below.
π
 advancedFilteringOnArraysEnabledB
 éSpecifies whether advanced filters should be evaluated against an array of values instead of expecting a singular value. Defaults to `false`.
ç
azureFunctionEndpointºBπ:∂
≥
	eventgrid1SystemTopicEventSubscriptionAzureFunctionEndpointsazure:eventgrid/SystemTopicEventSubscriptionAzureFunctionEndpoint:SystemTopicEventSubscriptionAzureFunctionEndpoint5An `azure_function_endpoint` block as defined below.
Ô
deadLetterIdentity≥B∞:≠
™
	eventgrid.SystemTopicEventSubscriptionDeadLetterIdentitymazure:eventgrid/SystemTopicEventSubscriptionDeadLetterIdentity:SystemTopicEventSubscriptionDeadLetterIdentity¢A `dead_letter_identity` block as defined below.

> **Note:** `storage_blob_dead_letter_destination` must be specified when a `dead_letter_identity` is specified
Ú
deliveryIdentity≠B™:ß
§
	eventgrid,SystemTopicEventSubscriptionDeliveryIdentityiazure:eventgrid/SystemTopicEventSubscriptionDeliveryIdentity:SystemTopicEventSubscriptionDeliveryIdentity.A `delivery_identity` block as defined below.
Ç
deliveryProperties∞B≠*™:ß
§
	eventgrid,SystemTopicEventSubscriptionDeliveryPropertyiazure:eventgrid/SystemTopicEventSubscriptionDeliveryProperty:SystemTopicEventSubscriptionDeliveryProperty9One or more `delivery_property` blocks as defined below.
â
eventDeliverySchemaB" ÎSpecifies the event delivery schema for the event subscription. Possible values include: `EventGridSchema`, `CloudEventSchemaV1_0`, `CustomInputSchema`. Defaults to `EventGridSchema`. Changing this forces a new resource to be created.
M
eventhubEndpointIdB" 1Specifies the id where the Event Hub is located.
q
expirationTimeUtcB" VSpecifies the expiration time of the event subscription (Datetime Format `RFC 3339`).
]
hybridConnectionEndpointIdB" 9Specifies the id where the Hybrid Connection is located.
o
includedEventTypesB*" QA list of applicable event types that need to be part of the event subscription.
H
labelsB*" 6A list of labels to assign to the event subscription.
Ü
nameB" xThe name which should be used for this Event Subscription. Changing this forces a new Event Subscription to be created.
î
resourceGroupName" {The name of the Resource Group where the System Topic exists. Changing this forces a new Event Subscription to be created.
Ÿ
retryPolicyûBõ:ò
ï
	eventgrid'SystemTopicEventSubscriptionRetryPolicy_azure:eventgrid/SystemTopicEventSubscriptionRetryPolicy:SystemTopicEventSubscriptionRetryPolicy)A `retry_policy` block as defined below.
\
serviceBusQueueEndpointIdB" 9Specifies the id where the Service Bus Queue is located.
\
serviceBusTopicEndpointIdB" 9Specifies the id where the Service Bus Topic is located.
∆
 storageBlobDeadLetterDestinationﬁB€:ÿ
’
	eventgrid<SystemTopicEventSubscriptionStorageBlobDeadLetterDestinationâazure:eventgrid/SystemTopicEventSubscriptionStorageBlobDeadLetterDestination:SystemTopicEventSubscriptionStorageBlobDeadLetterDestinationAA `storage_blob_dead_letter_destination` block as defined below.
á
storageQueueEndpointπB∂:≥
∞
	eventgrid0SystemTopicEventSubscriptionStorageQueueEndpointqazure:eventgrid/SystemTopicEventSubscriptionStorageQueueEndpoint:SystemTopicEventSubscriptionStorageQueueEndpoint3A `storage_queue_endpoint` block as defined below.
„
subjectFilter§B°:û
õ
	eventgrid)SystemTopicEventSubscriptionSubjectFiltercazure:eventgrid/SystemTopicEventSubscriptionSubjectFilter:SystemTopicEventSubscriptionSubjectFilter+A `subject_filter` block as defined below.
ï
systemTopic" ÅThe System Topic where the Event Subscription should be created in. Changing this forces a new Event Subscription to be created.
¯
webhookEndpoint™Bß:§
°
	eventgrid+SystemTopicEventSubscriptionWebhookEndpointgazure:eventgrid/SystemTopicEventSubscriptionWebhookEndpoint:SystemTopicEventSubscriptionWebhookEndpoint∑A `webhook_endpoint` block as defined below.

> **NOTE:** One of `azure_function_endpoint`, `eventhub_endpoint_id`, `hybrid_connection_endpoint`, `hybrid_connection_endpoint_id`, `service_bus_queue_endpoint_id`, `service_bus_topic_endpoint_id`, `storage_queue_endpoint` or `webhook_endpoint` must be specified.
"Ë
advancedFilterßB§:°
û
	eventgrid*SystemTopicEventSubscriptionAdvancedFiltereazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilter:SystemTopicEventSubscriptionAdvancedFilter,A `advanced_filter` block as defined below.
"π
 advancedFilteringOnArraysEnabledB
 éSpecifies whether advanced filters should be evaluated against an array of values instead of expecting a singular value. Defaults to `false`.
"ç
azureFunctionEndpointºBπ:∂
≥
	eventgrid1SystemTopicEventSubscriptionAzureFunctionEndpointsazure:eventgrid/SystemTopicEventSubscriptionAzureFunctionEndpoint:SystemTopicEventSubscriptionAzureFunctionEndpoint5An `azure_function_endpoint` block as defined below.
"Ô
deadLetterIdentity≥B∞:≠
™
	eventgrid.SystemTopicEventSubscriptionDeadLetterIdentitymazure:eventgrid/SystemTopicEventSubscriptionDeadLetterIdentity:SystemTopicEventSubscriptionDeadLetterIdentity¢A `dead_letter_identity` block as defined below.

> **Note:** `storage_blob_dead_letter_destination` must be specified when a `dead_letter_identity` is specified
"Ú
deliveryIdentity≠B™:ß
§
	eventgrid,SystemTopicEventSubscriptionDeliveryIdentityiazure:eventgrid/SystemTopicEventSubscriptionDeliveryIdentity:SystemTopicEventSubscriptionDeliveryIdentity.A `delivery_identity` block as defined below.
"Ç
deliveryProperties∞B≠*™:ß
§
	eventgrid,SystemTopicEventSubscriptionDeliveryPropertyiazure:eventgrid/SystemTopicEventSubscriptionDeliveryProperty:SystemTopicEventSubscriptionDeliveryProperty9One or more `delivery_property` blocks as defined below.
"â
eventDeliverySchemaB" ÎSpecifies the event delivery schema for the event subscription. Possible values include: `EventGridSchema`, `CloudEventSchemaV1_0`, `CustomInputSchema`. Defaults to `EventGridSchema`. Changing this forces a new resource to be created.
"K
eventhubEndpointId" 1Specifies the id where the Event Hub is located.
"q
expirationTimeUtcB" VSpecifies the expiration time of the event subscription (Datetime Format `RFC 3339`).
"[
hybridConnectionEndpointId" 9Specifies the id where the Hybrid Connection is located.
"m
includedEventTypes*" QA list of applicable event types that need to be part of the event subscription.
"H
labelsB*" 6A list of labels to assign to the event subscription.
"Ñ
name" xThe name which should be used for this Event Subscription. Changing this forces a new Event Subscription to be created.
"î
resourceGroupName" {The name of the Resource Group where the System Topic exists. Changing this forces a new Event Subscription to be created.
"÷
retryPolicyõ:ò
ï
	eventgrid'SystemTopicEventSubscriptionRetryPolicy_azure:eventgrid/SystemTopicEventSubscriptionRetryPolicy:SystemTopicEventSubscriptionRetryPolicy)A `retry_policy` block as defined below.
"\
serviceBusQueueEndpointIdB" 9Specifies the id where the Service Bus Queue is located.
"\
serviceBusTopicEndpointIdB" 9Specifies the id where the Service Bus Topic is located.
"∆
 storageBlobDeadLetterDestinationﬁB€:ÿ
’
	eventgrid<SystemTopicEventSubscriptionStorageBlobDeadLetterDestinationâazure:eventgrid/SystemTopicEventSubscriptionStorageBlobDeadLetterDestination:SystemTopicEventSubscriptionStorageBlobDeadLetterDestinationAA `storage_blob_dead_letter_destination` block as defined below.
"á
storageQueueEndpointπB∂:≥
∞
	eventgrid0SystemTopicEventSubscriptionStorageQueueEndpointqazure:eventgrid/SystemTopicEventSubscriptionStorageQueueEndpoint:SystemTopicEventSubscriptionStorageQueueEndpoint3A `storage_queue_endpoint` block as defined below.
"„
subjectFilter§B°:û
õ
	eventgrid)SystemTopicEventSubscriptionSubjectFiltercazure:eventgrid/SystemTopicEventSubscriptionSubjectFilter:SystemTopicEventSubscriptionSubjectFilter+A `subject_filter` block as defined below.
"ï
systemTopic" ÅThe System Topic where the Event Subscription should be created in. Changing this forces a new Event Subscription to be created.
"¯
webhookEndpoint™Bß:§
°
	eventgrid+SystemTopicEventSubscriptionWebhookEndpointgazure:eventgrid/SystemTopicEventSubscriptionWebhookEndpoint:SystemTopicEventSubscriptionWebhookEndpoint∑A `webhook_endpoint` block as defined below.

> **NOTE:** One of `azure_function_endpoint`, `eventhub_endpoint_id`, `hybrid_connection_endpoint`, `hybrid_connection_endpoint_id`, `service_bus_queue_endpoint_id`, `service_bus_topic_endpoint_id`, `storage_queue_endpoint` or `webhook_endpoint` must be specified.
*√>
/
	eventgridTopicazure:eventgrid/topic:Topicè Manages an EventGrid Topic

> **Note:** at this time EventGrid Topic's are only available in a limited number of regions.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "example-resources",
    location: "West Europe",
});
const exampleTopic = new azure.eventgrid.Topic("example", {
    name: "my-eventgrid-topic",
    location: example.location,
    resourceGroupName: example.name,
    tags: {
        environment: "Production",
    },
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="example-resources",
    location="West Europe")
example_topic = azure.eventgrid.Topic("example",
    name="my-eventgrid-topic",
    location=example.location,
    resource_group_name=example.name,
    tags={
        "environment": "Production",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "example-resources",
        Location = "West Europe",
    });

    var exampleTopic = new Azure.EventGrid.Topic("example", new()
    {
        Name = "my-eventgrid-topic",
        Location = example.Location,
        ResourceGroupName = example.Name,
        Tags = 
        {
            { "environment", "Production" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventgrid"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("example-resources"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		_, err = eventgrid.NewTopic(ctx, "example", &eventgrid.TopicArgs{
			Name:              pulumi.String("my-eventgrid-topic"),
			Location:          example.Location,
			ResourceGroupName: example.Name,
			Tags: pulumi.StringMap{
				"environment": pulumi.String("Production"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.eventgrid.Topic;
import com.pulumi.azure.eventgrid.TopicArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("example-resources")
            .location("West Europe")
            .build());

        var exampleTopic = new Topic("exampleTopic", TopicArgs.builder()
            .name("my-eventgrid-topic")
            .location(example.location())
            .resourceGroupName(example.name())
            .tags(Map.of("environment", "Production"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: example-resources
      location: West Europe
  exampleTopic:
    type: azure:eventgrid:Topic
    name: example
    properties:
      name: my-eventgrid-topic
      location: ${example.location}
      resourceGroupName: ${example.name}
      tags:
        environment: Production
```
<!--End PulumiCodeChooser -->

## Import

EventGrid Topic's can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:eventgrid/topic:Topic topic1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventGrid/topics/topic1
```

Å
identityMBK:I
G
	eventgridTopicIdentity+azure:eventgrid/TopicIdentity:TopicIdentity&An `identity` block as defined below.
©
inboundIpRules^B\*Z:X
V
	eventgridTopicInboundIpRule5azure:eventgrid/TopicInboundIpRule:TopicInboundIpRule7One or more `inbound_ip_rule` blocks as defined below.
å
inputMappingDefaultValuesÄB~:|
z
	eventgridTopicInputMappingDefaultValuesMazure:eventgrid/TopicInputMappingDefaultValues:TopicInputMappingDefaultValueslA `input_mapping_default_values` block as defined below. Changing this forces a new resource to be created.
Á
inputMappingFieldskBi:g
e
	eventgridTopicInputMappingFields?azure:eventgrid/TopicInputMappingFields:TopicInputMappingFieldsdA `input_mapping_fields` block as defined below. Changing this forces a new resource to be created.
é
inputSchemaB" ¯Specifies the schema in which incoming events will be published to this domain. Allowed values are `CloudEventSchemaV1_0`, `CustomEventSchema`, or `EventGridSchema`. Defaults to `EventGridSchema`. Changing this forces a new resource to be created.
w
localAuthEnabledB
 ]Whether local authentication methods is enabled for the EventGrid Topic. Defaults to `true`.
á
locationB" uSpecifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
u
nameB" gSpecifies the name of the EventGrid Topic resource. Changing this forces a new resource to be created.
y
publicNetworkAccessEnabledB
 UWhether or not public network access is allowed for this server. Defaults to `true`.
ê
resourceGroupName" wThe name of the resource group in which the EventGrid Topic exists. Changing this forces a new resource to be created.
=
tagsB2" -A mapping of tags to assign to the resource.
"B
endpoint" 2The Endpoint associated with the EventGrid Topic.
"Å
identityMBK:I
G
	eventgridTopicIdentity+azure:eventgrid/TopicIdentity:TopicIdentity&An `identity` block as defined below.
"©
inboundIpRules^B\*Z:X
V
	eventgridTopicInboundIpRule5azure:eventgrid/TopicInboundIpRule:TopicInboundIpRule7One or more `inbound_ip_rule` blocks as defined below.
"å
inputMappingDefaultValuesÄB~:|
z
	eventgridTopicInputMappingDefaultValuesMazure:eventgrid/TopicInputMappingDefaultValues:TopicInputMappingDefaultValueslA `input_mapping_default_values` block as defined below. Changing this forces a new resource to be created.
"Á
inputMappingFieldskBi:g
e
	eventgridTopicInputMappingFields?azure:eventgrid/TopicInputMappingFields:TopicInputMappingFieldsdA `input_mapping_fields` block as defined below. Changing this forces a new resource to be created.
"é
inputSchemaB" ¯Specifies the schema in which incoming events will be published to this domain. Allowed values are `CloudEventSchemaV1_0`, `CustomEventSchema`, or `EventGridSchema`. Defaults to `EventGridSchema`. Changing this forces a new resource to be created.
"w
localAuthEnabledB
 ]Whether local authentication methods is enabled for the EventGrid Topic. Defaults to `true`.
"Ö
location" uSpecifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
"s
name" gSpecifies the name of the EventGrid Topic resource. Changing this forces a new resource to be created.
"[
primaryAccessKey" CThe Primary Shared Access Key associated with the EventGrid Topic.
"y
publicNetworkAccessEnabledB
 UWhether or not public network access is allowed for this server. Defaults to `true`.
"ê
resourceGroupName" wThe name of the resource group in which the EventGrid Topic exists. Changing this forces a new resource to be created.
"_
secondaryAccessKey" EThe Secondary Shared Access Key associated with the EventGrid Topic.
"=
tagsB2" -A mapping of tags to assign to the resource.
*øY
Q
eventhubAuthorizationRule2azure:eventhub/authorizationRule:AuthorizationRuleŒCManages a Event Hubs authorization Rule within an Event Hub.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "example-resources",
    location: "West Europe",
});
const exampleEventHubNamespace = new azure.eventhub.EventHubNamespace("example", {
    name: "acceptanceTestEventHubNamespace",
    location: example.location,
    resourceGroupName: example.name,
    sku: "Basic",
    capacity: 2,
    tags: {
        environment: "Production",
    },
});
const exampleEventHub = new azure.eventhub.EventHub("example", {
    name: "acceptanceTestEventHub",
    namespaceName: exampleEventHubNamespace.name,
    resourceGroupName: example.name,
    partitionCount: 2,
    messageRetention: 2,
});
const exampleAuthorizationRule = new azure.eventhub.AuthorizationRule("example", {
    name: "navi",
    namespaceName: exampleEventHubNamespace.name,
    eventhubName: exampleEventHub.name,
    resourceGroupName: example.name,
    listen: true,
    send: false,
    manage: false,
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="example-resources",
    location="West Europe")
example_event_hub_namespace = azure.eventhub.EventHubNamespace("example",
    name="acceptanceTestEventHubNamespace",
    location=example.location,
    resource_group_name=example.name,
    sku="Basic",
    capacity=2,
    tags={
        "environment": "Production",
    })
example_event_hub = azure.eventhub.EventHub("example",
    name="acceptanceTestEventHub",
    namespace_name=example_event_hub_namespace.name,
    resource_group_name=example.name,
    partition_count=2,
    message_retention=2)
example_authorization_rule = azure.eventhub.AuthorizationRule("example",
    name="navi",
    namespace_name=example_event_hub_namespace.name,
    eventhub_name=example_event_hub.name,
    resource_group_name=example.name,
    listen=True,
    send=False,
    manage=False)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "example-resources",
        Location = "West Europe",
    });

    var exampleEventHubNamespace = new Azure.EventHub.EventHubNamespace("example", new()
    {
        Name = "acceptanceTestEventHubNamespace",
        Location = example.Location,
        ResourceGroupName = example.Name,
        Sku = "Basic",
        Capacity = 2,
        Tags = 
        {
            { "environment", "Production" },
        },
    });

    var exampleEventHub = new Azure.EventHub.EventHub("example", new()
    {
        Name = "acceptanceTestEventHub",
        NamespaceName = exampleEventHubNamespace.Name,
        ResourceGroupName = example.Name,
        PartitionCount = 2,
        MessageRetention = 2,
    });

    var exampleAuthorizationRule = new Azure.EventHub.AuthorizationRule("example", new()
    {
        Name = "navi",
        NamespaceName = exampleEventHubNamespace.Name,
        EventhubName = exampleEventHub.Name,
        ResourceGroupName = example.Name,
        Listen = true,
        Send = false,
        Manage = false,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("example-resources"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		exampleEventHubNamespace, err := eventhub.NewEventHubNamespace(ctx, "example", &eventhub.EventHubNamespaceArgs{
			Name:              pulumi.String("acceptanceTestEventHubNamespace"),
			Location:          example.Location,
			ResourceGroupName: example.Name,
			Sku:               pulumi.String("Basic"),
			Capacity:          pulumi.Int(2),
			Tags: pulumi.StringMap{
				"environment": pulumi.String("Production"),
			},
		})
		if err != nil {
			return err
		}
		exampleEventHub, err := eventhub.NewEventHub(ctx, "example", &eventhub.EventHubArgs{
			Name:              pulumi.String("acceptanceTestEventHub"),
			NamespaceName:     exampleEventHubNamespace.Name,
			ResourceGroupName: example.Name,
			PartitionCount:    pulumi.Int(2),
			MessageRetention:  pulumi.Int(2),
		})
		if err != nil {
			return err
		}
		_, err = eventhub.NewAuthorizationRule(ctx, "example", &eventhub.AuthorizationRuleArgs{
			Name:              pulumi.String("navi"),
			NamespaceName:     exampleEventHubNamespace.Name,
			EventhubName:      exampleEventHub.Name,
			ResourceGroupName: example.Name,
			Listen:            pulumi.Bool(true),
			Send:              pulumi.Bool(false),
			Manage:            pulumi.Bool(false),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.eventhub.EventHubNamespace;
import com.pulumi.azure.eventhub.EventHubNamespaceArgs;
import com.pulumi.azure.eventhub.EventHub;
import com.pulumi.azure.eventhub.EventHubArgs;
import com.pulumi.azure.eventhub.AuthorizationRule;
import com.pulumi.azure.eventhub.AuthorizationRuleArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("example-resources")
            .location("West Europe")
            .build());

        var exampleEventHubNamespace = new EventHubNamespace("exampleEventHubNamespace", EventHubNamespaceArgs.builder()
            .name("acceptanceTestEventHubNamespace")
            .location(example.location())
            .resourceGroupName(example.name())
            .sku("Basic")
            .capacity(2)
            .tags(Map.of("environment", "Production"))
            .build());

        var exampleEventHub = new EventHub("exampleEventHub", EventHubArgs.builder()
            .name("acceptanceTestEventHub")
            .namespaceName(exampleEventHubNamespace.name())
            .resourceGroupName(example.name())
            .partitionCount(2)
            .messageRetention(2)
            .build());

        var exampleAuthorizationRule = new AuthorizationRule("exampleAuthorizationRule", AuthorizationRuleArgs.builder()
            .name("navi")
            .namespaceName(exampleEventHubNamespace.name())
            .eventhubName(exampleEventHub.name())
            .resourceGroupName(example.name())
            .listen(true)
            .send(false)
            .manage(false)
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: example-resources
      location: West Europe
  exampleEventHubNamespace:
    type: azure:eventhub:EventHubNamespace
    name: example
    properties:
      name: acceptanceTestEventHubNamespace
      location: ${example.location}
      resourceGroupName: ${example.name}
      sku: Basic
      capacity: 2
      tags:
        environment: Production
  exampleEventHub:
    type: azure:eventhub:EventHub
    name: example
    properties:
      name: acceptanceTestEventHub
      namespaceName: ${exampleEventHubNamespace.name}
      resourceGroupName: ${example.name}
      partitionCount: 2
      messageRetention: 2
  exampleAuthorizationRule:
    type: azure:eventhub:AuthorizationRule
    name: example
    properties:
      name: navi
      namespaceName: ${exampleEventHubNamespace.name}
      eventhubName: ${exampleEventHub.name}
      resourceGroupName: ${example.name}
      listen: true
      send: false
      manage: false
```
<!--End PulumiCodeChooser -->

## Import

EventHub Authorization Rules can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:eventhub/authorizationRule:AuthorizationRule rule1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventHub/namespaces/namespace1/eventhubs/eventhub1/authorizationRules/rule1
```

k
eventhubName" WSpecifies the name of the EventHub. Changing this forces a new resource to be created.
o
listenB
 _Does this Authorization Rule have permissions to Listen to the Event Hub? Defaults to `false`.
µ
manageB
 §Does this Authorization Rule have permissions to Manage to the Event Hub? When this property is `true` - both `listen` and `send` must be too. Defaults to `false`.
Å
nameB" sSpecifies the name of the EventHub Authorization Rule resource. Changing this forces a new resource to be created.
Ç
namespaceName" mSpecifies the name of the grandparent EventHub Namespace. Changing this forces a new resource to be created.
Ÿ
resourceGroupName" øThe name of the resource group in which the EventHub Namespace exists. Changing this forces a new resource to be created.

> **NOTE** At least one of the 3 permissions below needs to be set.
k
sendB
 ]Does this Authorization Rule have permissions to Send to the Event Hub? Defaults to `false`.
"k
eventhubName" WSpecifies the name of the EventHub. Changing this forces a new resource to be created.
"o
listenB
 _Does this Authorization Rule have permissions to Listen to the Event Hub? Defaults to `false`.
"µ
manageB
 §Does this Authorization Rule have permissions to Manage to the Event Hub? When this property is `true` - both `listen` and `send` must be too. Defaults to `false`.
"
name" sSpecifies the name of the EventHub Authorization Rule resource. Changing this forces a new resource to be created.
"Ç
namespaceName" mSpecifies the name of the grandparent EventHub Namespace. Changing this forces a new resource to be created.
"d
primaryConnectionString" EThe Primary Connection String for the Event Hubs authorization Rule.
"≠
primaryConnectionStringAlias" àThe alias of the Primary Connection String for the Event Hubs authorization Rule, which is generated when disaster recovery is enabled.
"I

primaryKey" 7The Primary Key for the Event Hubs authorization Rule.
"Ÿ
resourceGroupName" øThe name of the resource group in which the EventHub Namespace exists. Changing this forces a new resource to be created.

> **NOTE** At least one of the 3 permissions below needs to be set.
"h
secondaryConnectionString" GThe Secondary Connection String for the Event Hubs Authorization Rule.
"±
secondaryConnectionStringAlias" äThe alias of the Secondary Connection String for the Event Hubs Authorization Rule, which is generated when disaster recovery is enabled.
"M
secondaryKey" 9The Secondary Key for the Event Hubs Authorization Rule.
"k
sendB
 ]Does this Authorization Rule have permissions to Send to the Event Hub? Defaults to `false`.
*…&
3
eventhubClusterazure:eventhub/cluster:ClusterçManages an EventHub Cluster

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "example-resources",
    location: "West Europe",
});
const test = new azure.eventhub.Cluster("test", {
    name: "example",
    resourceGroupName: example.name,
    location: example.location,
    skuName: "Dedicated_1",
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="example-resources",
    location="West Europe")
test = azure.eventhub.Cluster("test",
    name="example",
    resource_group_name=example.name,
    location=example.location,
    sku_name="Dedicated_1")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "example-resources",
        Location = "West Europe",
    });

    var test = new Azure.EventHub.Cluster("test", new()
    {
        Name = "example",
        ResourceGroupName = example.Name,
        Location = example.Location,
        SkuName = "Dedicated_1",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("example-resources"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		_, err = eventhub.NewCluster(ctx, "test", &eventhub.ClusterArgs{
			Name:              pulumi.String("example"),
			ResourceGroupName: example.Name,
			Location:          example.Location,
			SkuName:           pulumi.String("Dedicated_1"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.eventhub.Cluster;
import com.pulumi.azure.eventhub.ClusterArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("example-resources")
            .location("West Europe")
            .build());

        var test = new Cluster("test", ClusterArgs.builder()
            .name("example")
            .resourceGroupName(example.name())
            .location(example.location())
            .skuName("Dedicated_1")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: example-resources
      location: West Europe
  test:
    type: azure:eventhub:Cluster
    properties:
      name: example
      resourceGroupName: ${example.name}
      location: ${example.location}
      skuName: Dedicated_1
```
<!--End PulumiCodeChooser -->

## Import

EventHub Cluster's can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:eventhub/cluster:Cluster cluster1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventHub/clusters/cluster1
```

á
locationB" uSpecifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
v
nameB" hSpecifies the name of the EventHub Cluster resource. Changing this forces a new resource to be created.
ë
resourceGroupName" xThe name of the resource group in which the EventHub Cluster exists. Changing this forces a new resource to be created.
m
skuName" ^The SKU name of the EventHub Cluster. The only supported value at this time is `Dedicated_1`.
=
tagsB2" -A mapping of tags to assign to the resource.
"Ö
location" uSpecifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
"t
name" hSpecifies the name of the EventHub Cluster resource. Changing this forces a new resource to be created.
"ë
resourceGroupName" xThe name of the resource group in which the EventHub Cluster exists. Changing this forces a new resource to be created.
"m
skuName" ^The SKU name of the EventHub Cluster. The only supported value at this time is `Dedicated_1`.
"=
tagsB2" -A mapping of tags to assign to the resource.
*úM
E
eventhubConsumerGroup*azure:eventhub/consumerGroup:ConsumerGroup†CManages a Event Hubs Consumer Group as a nested resource within an Event Hub.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "example-resources",
    location: "West Europe",
});
const exampleEventHubNamespace = new azure.eventhub.EventHubNamespace("example", {
    name: "acceptanceTestEventHubNamespace",
    location: example.location,
    resourceGroupName: example.name,
    sku: "Basic",
    capacity: 2,
    tags: {
        environment: "Production",
    },
});
const exampleEventHub = new azure.eventhub.EventHub("example", {
    name: "acceptanceTestEventHub",
    namespaceName: exampleEventHubNamespace.name,
    resourceGroupName: example.name,
    partitionCount: 2,
    messageRetention: 2,
});
const exampleConsumerGroup = new azure.eventhub.ConsumerGroup("example", {
    name: "acceptanceTestEventHubConsumerGroup",
    namespaceName: exampleEventHubNamespace.name,
    eventhubName: exampleEventHub.name,
    resourceGroupName: example.name,
    userMetadata: "some-meta-data",
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="example-resources",
    location="West Europe")
example_event_hub_namespace = azure.eventhub.EventHubNamespace("example",
    name="acceptanceTestEventHubNamespace",
    location=example.location,
    resource_group_name=example.name,
    sku="Basic",
    capacity=2,
    tags={
        "environment": "Production",
    })
example_event_hub = azure.eventhub.EventHub("example",
    name="acceptanceTestEventHub",
    namespace_name=example_event_hub_namespace.name,
    resource_group_name=example.name,
    partition_count=2,
    message_retention=2)
example_consumer_group = azure.eventhub.ConsumerGroup("example",
    name="acceptanceTestEventHubConsumerGroup",
    namespace_name=example_event_hub_namespace.name,
    eventhub_name=example_event_hub.name,
    resource_group_name=example.name,
    user_metadata="some-meta-data")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "example-resources",
        Location = "West Europe",
    });

    var exampleEventHubNamespace = new Azure.EventHub.EventHubNamespace("example", new()
    {
        Name = "acceptanceTestEventHubNamespace",
        Location = example.Location,
        ResourceGroupName = example.Name,
        Sku = "Basic",
        Capacity = 2,
        Tags = 
        {
            { "environment", "Production" },
        },
    });

    var exampleEventHub = new Azure.EventHub.EventHub("example", new()
    {
        Name = "acceptanceTestEventHub",
        NamespaceName = exampleEventHubNamespace.Name,
        ResourceGroupName = example.Name,
        PartitionCount = 2,
        MessageRetention = 2,
    });

    var exampleConsumerGroup = new Azure.EventHub.ConsumerGroup("example", new()
    {
        Name = "acceptanceTestEventHubConsumerGroup",
        NamespaceName = exampleEventHubNamespace.Name,
        EventhubName = exampleEventHub.Name,
        ResourceGroupName = example.Name,
        UserMetadata = "some-meta-data",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("example-resources"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		exampleEventHubNamespace, err := eventhub.NewEventHubNamespace(ctx, "example", &eventhub.EventHubNamespaceArgs{
			Name:              pulumi.String("acceptanceTestEventHubNamespace"),
			Location:          example.Location,
			ResourceGroupName: example.Name,
			Sku:               pulumi.String("Basic"),
			Capacity:          pulumi.Int(2),
			Tags: pulumi.StringMap{
				"environment": pulumi.String("Production"),
			},
		})
		if err != nil {
			return err
		}
		exampleEventHub, err := eventhub.NewEventHub(ctx, "example", &eventhub.EventHubArgs{
			Name:              pulumi.String("acceptanceTestEventHub"),
			NamespaceName:     exampleEventHubNamespace.Name,
			ResourceGroupName: example.Name,
			PartitionCount:    pulumi.Int(2),
			MessageRetention:  pulumi.Int(2),
		})
		if err != nil {
			return err
		}
		_, err = eventhub.NewConsumerGroup(ctx, "example", &eventhub.ConsumerGroupArgs{
			Name:              pulumi.String("acceptanceTestEventHubConsumerGroup"),
			NamespaceName:     exampleEventHubNamespace.Name,
			EventhubName:      exampleEventHub.Name,
			ResourceGroupName: example.Name,
			UserMetadata:      pulumi.String("some-meta-data"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.eventhub.EventHubNamespace;
import com.pulumi.azure.eventhub.EventHubNamespaceArgs;
import com.pulumi.azure.eventhub.EventHub;
import com.pulumi.azure.eventhub.EventHubArgs;
import com.pulumi.azure.eventhub.ConsumerGroup;
import com.pulumi.azure.eventhub.ConsumerGroupArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("example-resources")
            .location("West Europe")
            .build());

        var exampleEventHubNamespace = new EventHubNamespace("exampleEventHubNamespace", EventHubNamespaceArgs.builder()
            .name("acceptanceTestEventHubNamespace")
            .location(example.location())
            .resourceGroupName(example.name())
            .sku("Basic")
            .capacity(2)
            .tags(Map.of("environment", "Production"))
            .build());

        var exampleEventHub = new EventHub("exampleEventHub", EventHubArgs.builder()
            .name("acceptanceTestEventHub")
            .namespaceName(exampleEventHubNamespace.name())
            .resourceGroupName(example.name())
            .partitionCount(2)
            .messageRetention(2)
            .build());

        var exampleConsumerGroup = new ConsumerGroup("exampleConsumerGroup", ConsumerGroupArgs.builder()
            .name("acceptanceTestEventHubConsumerGroup")
            .namespaceName(exampleEventHubNamespace.name())
            .eventhubName(exampleEventHub.name())
            .resourceGroupName(example.name())
            .userMetadata("some-meta-data")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: example-resources
      location: West Europe
  exampleEventHubNamespace:
    type: azure:eventhub:EventHubNamespace
    name: example
    properties:
      name: acceptanceTestEventHubNamespace
      location: ${example.location}
      resourceGroupName: ${example.name}
      sku: Basic
      capacity: 2
      tags:
        environment: Production
  exampleEventHub:
    type: azure:eventhub:EventHub
    name: example
    properties:
      name: acceptanceTestEventHub
      namespaceName: ${exampleEventHubNamespace.name}
      resourceGroupName: ${example.name}
      partitionCount: 2
      messageRetention: 2
  exampleConsumerGroup:
    type: azure:eventhub:ConsumerGroup
    name: example
    properties:
      name: acceptanceTestEventHubConsumerGroup
      namespaceName: ${exampleEventHubNamespace.name}
      eventhubName: ${exampleEventHub.name}
      resourceGroupName: ${example.name}
      userMetadata: some-meta-data
```
<!--End PulumiCodeChooser -->

## Import

EventHub Consumer Groups can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:eventhub/consumerGroup:ConsumerGroup consumerGroup1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventHub/namespaces/namespace1/eventhubs/eventhub1/consumerGroups/consumerGroup1
```

k
eventhubName" WSpecifies the name of the EventHub. Changing this forces a new resource to be created.
}
nameB" oSpecifies the name of the EventHub Consumer Group resource. Changing this forces a new resource to be created.
Ç
namespaceName" mSpecifies the name of the grandparent EventHub Namespace. Changing this forces a new resource to be created.
±
resourceGroupName" óThe name of the resource group in which the EventHub Consumer Group's grandparent Namespace exists. Changing this forces a new resource to be created.
3
userMetadataB" Specifies the user metadata.
"k
eventhubName" WSpecifies the name of the EventHub. Changing this forces a new resource to be created.
"{
name" oSpecifies the name of the EventHub Consumer Group resource. Changing this forces a new resource to be created.
"Ç
namespaceName" mSpecifies the name of the grandparent EventHub Namespace. Changing this forces a new resource to be created.
"±
resourceGroupName" óThe name of the resource group in which the EventHub Consumer Group's grandparent Namespace exists. Changing this forces a new resource to be created.
"3
userMetadataB" Specifies the user metadata.
*ÌC
0
eventhubDomainazure:eventhub/domain:DomainÀManages an EventGrid Domain

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "example-resources",
    location: "West Europe",
});
const exampleDomain = new azure.eventgrid.Domain("example", {
    name: "my-eventgrid-domain",
    location: example.location,
    resourceGroupName: example.name,
    tags: {
        environment: "Production",
    },
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="example-resources",
    location="West Europe")
example_domain = azure.eventgrid.Domain("example",
    name="my-eventgrid-domain",
    location=example.location,
    resource_group_name=example.name,
    tags={
        "environment": "Production",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "example-resources",
        Location = "West Europe",
    });

    var exampleDomain = new Azure.EventGrid.Domain("example", new()
    {
        Name = "my-eventgrid-domain",
        Location = example.Location,
        ResourceGroupName = example.Name,
        Tags = 
        {
            { "environment", "Production" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventgrid"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("example-resources"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		_, err = eventgrid.NewDomain(ctx, "example", &eventgrid.DomainArgs{
			Name:              pulumi.String("my-eventgrid-domain"),
			Location:          example.Location,
			ResourceGroupName: example.Name,
			Tags: pulumi.StringMap{
				"environment": pulumi.String("Production"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.eventgrid.Domain;
import com.pulumi.azure.eventgrid.DomainArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("example-resources")
            .location("West Europe")
            .build());

        var exampleDomain = new Domain("exampleDomain", DomainArgs.builder()
            .name("my-eventgrid-domain")
            .location(example.location())
            .resourceGroupName(example.name())
            .tags(Map.of("environment", "Production"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: example-resources
      location: West Europe
  exampleDomain:
    type: azure:eventgrid:Domain
    name: example
    properties:
      name: my-eventgrid-domain
      location: ${example.location}
      resourceGroupName: ${example.name}
      tags:
        environment: Production
```
<!--End PulumiCodeChooser -->

## Import

EventGrid Domains can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:eventhub/domain:Domain domain1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventGrid/domains/domain1
```

µ
$autoCreateTopicWithFirstSubscriptionB
 ÜWhether to create the domain topic when the first event subscription at the scope of the domain topic is created. Defaults to `true`.
≥
#autoDeleteTopicWithLastSubscriptionB
 ÖWhether to delete the domain topic when the last event subscription at the scope of the domain topic is deleted. Defaults to `true`.
Ç
identityNBL:J
H
eventhubDomainIdentity,azure:eventhub/DomainIdentity:DomainIdentity&An `identity` block as defined below.
™
inboundIpRules_B]*[:Y
W
eventhubDomainInboundIpRule6azure:eventhub/DomainInboundIpRule:DomainInboundIpRule7One or more `inbound_ip_rule` blocks as defined below.
ç
inputMappingDefaultValuesÅB:}
{
eventhubDomainInputMappingDefaultValuesNazure:eventhub/DomainInputMappingDefaultValues:DomainInputMappingDefaultValueslA `input_mapping_default_values` block as defined below. Changing this forces a new resource to be created.
Ë
inputMappingFieldslBj:h
f
eventhubDomainInputMappingFields@azure:eventhub/DomainInputMappingFields:DomainInputMappingFieldsdA `input_mapping_fields` block as defined below. Changing this forces a new resource to be created.
é
inputSchemaB" ¯Specifies the schema in which incoming events will be published to this domain. Allowed values are `CloudEventSchemaV1_0`, `CustomEventSchema`, or `EventGridSchema`. Defaults to `EventGridSchema`. Changing this forces a new resource to be created.
x
localAuthEnabledB
 ^Whether local authentication methods is enabled for the EventGrid Domain. Defaults to `true`.
á
locationB" uSpecifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
v
nameB" hSpecifies the name of the EventGrid Domain resource. Changing this forces a new resource to be created.
y
publicNetworkAccessEnabledB
 UWhether or not public network access is allowed for this server. Defaults to `true`.
ë
resourceGroupName" xThe name of the resource group in which the EventGrid Domain exists. Changing this forces a new resource to be created.
=
tagsB2" -A mapping of tags to assign to the resource.
"µ
$autoCreateTopicWithFirstSubscriptionB
 ÜWhether to create the domain topic when the first event subscription at the scope of the domain topic is created. Defaults to `true`.
"≥
#autoDeleteTopicWithLastSubscriptionB
 ÖWhether to delete the domain topic when the last event subscription at the scope of the domain topic is deleted. Defaults to `true`.
"C
endpoint" 3The Endpoint associated with the EventGrid Domain.
"Ç
identityNBL:J
H
eventhubDomainIdentity,azure:eventhub/DomainIdentity:DomainIdentity&An `identity` block as defined below.
"™
inboundIpRules_B]*[:Y
W
eventhubDomainInboundIpRule6azure:eventhub/DomainInboundIpRule:DomainInboundIpRule7One or more `inbound_ip_rule` blocks as defined below.
"ç
inputMappingDefaultValuesÅB:}
{
eventhubDomainInputMappingDefaultValuesNazure:eventhub/DomainInputMappingDefaultValues:DomainInputMappingDefaultValueslA `input_mapping_default_values` block as defined below. Changing this forces a new resource to be created.
"Ë
inputMappingFieldslBj:h
f
eventhubDomainInputMappingFields@azure:eventhub/DomainInputMappingFields:DomainInputMappingFieldsdA `input_mapping_fields` block as defined below. Changing this forces a new resource to be created.
"é
inputSchemaB" ¯Specifies the schema in which incoming events will be published to this domain. Allowed values are `CloudEventSchemaV1_0`, `CustomEventSchema`, or `EventGridSchema`. Defaults to `EventGridSchema`. Changing this forces a new resource to be created.
"x
localAuthEnabledB
 ^Whether local authentication methods is enabled for the EventGrid Domain. Defaults to `true`.
"Ö
location" uSpecifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
"t
name" hSpecifies the name of the EventGrid Domain resource. Changing this forces a new resource to be created.
"\
primaryAccessKey" DThe Primary Shared Access Key associated with the EventGrid Domain.
"y
publicNetworkAccessEnabledB
 UWhether or not public network access is allowed for this server. Defaults to `true`.
"ë
resourceGroupName" xThe name of the resource group in which the EventGrid Domain exists. Changing this forces a new resource to be created.
"`
secondaryAccessKey" FThe Secondary Shared Access Key associated with the EventGrid Domain.
"=
tagsB2" -A mapping of tags to assign to the resource.
*¡@
H
eventhubEventGridTopic,azure:eventhub/eventGridTopic:EventGridTopic† Manages an EventGrid Topic

> **Note:** at this time EventGrid Topic's are only available in a limited number of regions.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "example-resources",
    location: "West Europe",
});
const exampleTopic = new azure.eventgrid.Topic("example", {
    name: "my-eventgrid-topic",
    location: example.location,
    resourceGroupName: example.name,
    tags: {
        environment: "Production",
    },
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="example-resources",
    location="West Europe")
example_topic = azure.eventgrid.Topic("example",
    name="my-eventgrid-topic",
    location=example.location,
    resource_group_name=example.name,
    tags={
        "environment": "Production",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "example-resources",
        Location = "West Europe",
    });

    var exampleTopic = new Azure.EventGrid.Topic("example", new()
    {
        Name = "my-eventgrid-topic",
        Location = example.Location,
        ResourceGroupName = example.Name,
        Tags = 
        {
            { "environment", "Production" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventgrid"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("example-resources"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		_, err = eventgrid.NewTopic(ctx, "example", &eventgrid.TopicArgs{
			Name:              pulumi.String("my-eventgrid-topic"),
			Location:          example.Location,
			ResourceGroupName: example.Name,
			Tags: pulumi.StringMap{
				"environment": pulumi.String("Production"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.eventgrid.Topic;
import com.pulumi.azure.eventgrid.TopicArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("example-resources")
            .location("West Europe")
            .build());

        var exampleTopic = new Topic("exampleTopic", TopicArgs.builder()
            .name("my-eventgrid-topic")
            .location(example.location())
            .resourceGroupName(example.name())
            .tags(Map.of("environment", "Production"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: example-resources
      location: West Europe
  exampleTopic:
    type: azure:eventgrid:Topic
    name: example
    properties:
      name: my-eventgrid-topic
      location: ${example.location}
      resourceGroupName: ${example.name}
      tags:
        environment: Production
```
<!--End PulumiCodeChooser -->

## Import

EventGrid Topic's can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:eventhub/eventGridTopic:EventGridTopic topic1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventGrid/topics/topic1
```

ö
identityfBd:b
`
eventhubEventGridTopicIdentity<azure:eventhub/EventGridTopicIdentity:EventGridTopicIdentity&An `identity` block as defined below.
¬
inboundIpRuleswBu*s:q
o
eventhubEventGridTopicInboundIpRuleFazure:eventhub/EventGridTopicInboundIpRule:EventGridTopicInboundIpRule7One or more `inbound_ip_rule` blocks as defined below.
®
inputMappingDefaultValuesúBô:ñ
ì
eventhub'EventGridTopicInputMappingDefaultValues^azure:eventhub/EventGridTopicInputMappingDefaultValues:EventGridTopicInputMappingDefaultValueslA `input_mapping_default_values` block as defined below. Changing this forces a new resource to be created.
É
inputMappingFieldsÜBÉ:Ä
~
eventhub EventGridTopicInputMappingFieldsPazure:eventhub/EventGridTopicInputMappingFields:EventGridTopicInputMappingFieldsdA `input_mapping_fields` block as defined below. Changing this forces a new resource to be created.
é
inputSchemaB" ¯Specifies the schema in which incoming events will be published to this domain. Allowed values are `CloudEventSchemaV1_0`, `CustomEventSchema`, or `EventGridSchema`. Defaults to `EventGridSchema`. Changing this forces a new resource to be created.
w
localAuthEnabledB
 ]Whether local authentication methods is enabled for the EventGrid Topic. Defaults to `true`.
á
locationB" uSpecifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
u
nameB" gSpecifies the name of the EventGrid Topic resource. Changing this forces a new resource to be created.
y
publicNetworkAccessEnabledB
 UWhether or not public network access is allowed for this server. Defaults to `true`.
ê
resourceGroupName" wThe name of the resource group in which the EventGrid Topic exists. Changing this forces a new resource to be created.
=
tagsB2" -A mapping of tags to assign to the resource.
"B
endpoint" 2The Endpoint associated with the EventGrid Topic.
"ö
identityfBd:b
`
eventhubEventGridTopicIdentity<azure:eventhub/EventGridTopicIdentity:EventGridTopicIdentity&An `identity` block as defined below.
"¬
inboundIpRuleswBu*s:q
o
eventhubEventGridTopicInboundIpRuleFazure:eventhub/EventGridTopicInboundIpRule:EventGridTopicInboundIpRule7One or more `inbound_ip_rule` blocks as defined below.
"®
inputMappingDefaultValuesúBô:ñ
ì
eventhub'EventGridTopicInputMappingDefaultValues^azure:eventhub/EventGridTopicInputMappingDefaultValues:EventGridTopicInputMappingDefaultValueslA `input_mapping_default_values` block as defined below. Changing this forces a new resource to be created.
"É
inputMappingFieldsÜBÉ:Ä
~
eventhub EventGridTopicInputMappingFieldsPazure:eventhub/EventGridTopicInputMappingFields:EventGridTopicInputMappingFieldsdA `input_mapping_fields` block as defined below. Changing this forces a new resource to be created.
"é
inputSchemaB" ¯Specifies the schema in which incoming events will be published to this domain. Allowed values are `CloudEventSchemaV1_0`, `CustomEventSchema`, or `EventGridSchema`. Defaults to `EventGridSchema`. Changing this forces a new resource to be created.
"w
localAuthEnabledB
 ]Whether local authentication methods is enabled for the EventGrid Topic. Defaults to `true`.
"Ö
location" uSpecifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
"s
name" gSpecifies the name of the EventGrid Topic resource. Changing this forces a new resource to be created.
"[
primaryAccessKey" CThe Primary Shared Access Key associated with the EventGrid Topic.
"y
publicNetworkAccessEnabledB
 UWhether or not public network access is allowed for this server. Defaults to `true`.
"ê
resourceGroupName" wThe name of the resource group in which the EventGrid Topic exists. Changing this forces a new resource to be created.
"_
secondaryAccessKey" EThe Secondary Shared Access Key associated with the EventGrid Topic.
"=
tagsB2" -A mapping of tags to assign to the resource.
*öF
6
eventhubEventHub azure:eventhub/eventHub:EventHub¶0Manages a Event Hubs as a nested resource within a Event Hubs namespace.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "example-resources",
    location: "West Europe",
});
const exampleEventHubNamespace = new azure.eventhub.EventHubNamespace("example", {
    name: "acceptanceTestEventHubNamespace",
    location: example.location,
    resourceGroupName: example.name,
    sku: "Standard",
    capacity: 1,
    tags: {
        environment: "Production",
    },
});
const exampleEventHub = new azure.eventhub.EventHub("example", {
    name: "acceptanceTestEventHub",
    namespaceId: exampleEventHubNamespace.id,
    partitionCount: 2,
    messageRetention: 1,
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="example-resources",
    location="West Europe")
example_event_hub_namespace = azure.eventhub.EventHubNamespace("example",
    name="acceptanceTestEventHubNamespace",
    location=example.location,
    resource_group_name=example.name,
    sku="Standard",
    capacity=1,
    tags={
        "environment": "Production",
    })
example_event_hub = azure.eventhub.EventHub("example",
    name="acceptanceTestEventHub",
    namespace_id=example_event_hub_namespace.id,
    partition_count=2,
    message_retention=1)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "example-resources",
        Location = "West Europe",
    });

    var exampleEventHubNamespace = new Azure.EventHub.EventHubNamespace("example", new()
    {
        Name = "acceptanceTestEventHubNamespace",
        Location = example.Location,
        ResourceGroupName = example.Name,
        Sku = "Standard",
        Capacity = 1,
        Tags = 
        {
            { "environment", "Production" },
        },
    });

    var exampleEventHub = new Azure.EventHub.EventHub("example", new()
    {
        Name = "acceptanceTestEventHub",
        NamespaceId = exampleEventHubNamespace.Id,
        PartitionCount = 2,
        MessageRetention = 1,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("example-resources"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		exampleEventHubNamespace, err := eventhub.NewEventHubNamespace(ctx, "example", &eventhub.EventHubNamespaceArgs{
			Name:              pulumi.String("acceptanceTestEventHubNamespace"),
			Location:          example.Location,
			ResourceGroupName: example.Name,
			Sku:               pulumi.String("Standard"),
			Capacity:          pulumi.Int(1),
			Tags: pulumi.StringMap{
				"environment": pulumi.String("Production"),
			},
		})
		if err != nil {
			return err
		}
		_, err = eventhub.NewEventHub(ctx, "example", &eventhub.EventHubArgs{
			Name:             pulumi.String("acceptanceTestEventHub"),
			NamespaceId:      exampleEventHubNamespace.ID(),
			PartitionCount:   pulumi.Int(2),
			MessageRetention: pulumi.Int(1),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.eventhub.EventHubNamespace;
import com.pulumi.azure.eventhub.EventHubNamespaceArgs;
import com.pulumi.azure.eventhub.EventHub;
import com.pulumi.azure.eventhub.EventHubArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("example-resources")
            .location("West Europe")
            .build());

        var exampleEventHubNamespace = new EventHubNamespace("exampleEventHubNamespace", EventHubNamespaceArgs.builder()
            .name("acceptanceTestEventHubNamespace")
            .location(example.location())
            .resourceGroupName(example.name())
            .sku("Standard")
            .capacity(1)
            .tags(Map.of("environment", "Production"))
            .build());

        var exampleEventHub = new EventHub("exampleEventHub", EventHubArgs.builder()
            .name("acceptanceTestEventHub")
            .namespaceId(exampleEventHubNamespace.id())
            .partitionCount(2)
            .messageRetention(1)
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: example-resources
      location: West Europe
  exampleEventHubNamespace:
    type: azure:eventhub:EventHubNamespace
    name: example
    properties:
      name: acceptanceTestEventHubNamespace
      location: ${example.location}
      resourceGroupName: ${example.name}
      sku: Standard
      capacity: 1
      tags:
        environment: Production
  exampleEventHub:
    type: azure:eventhub:EventHub
    name: example
    properties:
      name: acceptanceTestEventHub
      namespaceId: ${exampleEventHubNamespace.id}
      partitionCount: 2
      messageRetention: 1
```
<!--End PulumiCodeChooser -->

## Import

EventHubs can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:eventhub/eventHub:EventHub eventhub1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventHub/namespaces/namespace1/eventhubs/eventhub1
```

∫
captureDescriptionrBp:n
l
eventhubEventHubCaptureDescriptionDazure:eventhub/EventHubCaptureDescription:EventHubCaptureDescription0A `capture_description` block as defined below.
⁄
messageRetention ¡Specifies the number of days to retain the events for this Event Hub.

> **Note:** When using a dedicated Event Hubs cluster, maximum value of `message_retention` is 90 days. When using a shared parent EventHub Namespace, maximum value is 7 days; or 1 day when using a Basic SKU for the shared parent EventHub Namespace.
n
nameB" `Specifies the name of the EventHub resource. Changing this forces a new resource to be created.
t
namespaceIdB" _Specifies the ID of the EventHub Namespace. Changing this forces a new resource to be created.

namespaceNameB" Ì
partitionCount ÷Specifies the current number of shards on the Event Hub.

> **Note:** `partition_count` cannot be changed unless Eventhub Namespace SKU is `Premium` and cannot be decreased.

> **Note:** When using a dedicated Event Hubs cluster, maximum value of `partition_count` is 1024. When using a shared parent EventHub Namespace, maximum value is 32.

resourceGroupNameB" î
statusB" ÉSpecifies the status of the Event Hub resource. Possible values are `Active`, `Disabled` and `SendDisabled`. Defaults to `Active`.
"∫
captureDescriptionrBp:n
l
eventhubEventHubCaptureDescriptionDazure:eventhub/EventHubCaptureDescription:EventHubCaptureDescription0A `capture_description` block as defined below.
"⁄
messageRetention ¡Specifies the number of days to retain the events for this Event Hub.

> **Note:** When using a dedicated Event Hubs cluster, maximum value of `message_retention` is 90 days. When using a shared parent EventHub Namespace, maximum value is 7 days; or 1 day when using a Basic SKU for the shared parent EventHub Namespace.
"l
name" `Specifies the name of the EventHub resource. Changing this forces a new resource to be created.
"r
namespaceId" _Specifies the ID of the EventHub Namespace. Changing this forces a new resource to be created.
"
namespaceName" "Ì
partitionCount ÷Specifies the current number of shards on the Event Hub.

> **Note:** `partition_count` cannot be changed unless Eventhub Namespace SKU is `Premium` and cannot be decreased.

> **Note:** When using a dedicated Event Hubs cluster, maximum value of `partition_count` is 1024. When using a shared parent EventHub Namespace, maximum value is 32.
"M
partitionIds*" 7The identifiers for partitions created for Event Hubs.
"
resourceGroupName" "î
statusB" ÉSpecifies the status of the Event Hub resource. Possible values are `Active`, `Disabled` and `SendDisabled`. Defaults to `Active`.
*ÁY
i
eventhubEventHubAuthorizationRuleBazure:eventhub/eventHubAuthorizationRule:EventHubAuthorizationRuleﬁCManages a Event Hubs authorization Rule within an Event Hub.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "example-resources",
    location: "West Europe",
});
const exampleEventHubNamespace = new azure.eventhub.EventHubNamespace("example", {
    name: "acceptanceTestEventHubNamespace",
    location: example.location,
    resourceGroupName: example.name,
    sku: "Basic",
    capacity: 2,
    tags: {
        environment: "Production",
    },
});
const exampleEventHub = new azure.eventhub.EventHub("example", {
    name: "acceptanceTestEventHub",
    namespaceName: exampleEventHubNamespace.name,
    resourceGroupName: example.name,
    partitionCount: 2,
    messageRetention: 2,
});
const exampleAuthorizationRule = new azure.eventhub.AuthorizationRule("example", {
    name: "navi",
    namespaceName: exampleEventHubNamespace.name,
    eventhubName: exampleEventHub.name,
    resourceGroupName: example.name,
    listen: true,
    send: false,
    manage: false,
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="example-resources",
    location="West Europe")
example_event_hub_namespace = azure.eventhub.EventHubNamespace("example",
    name="acceptanceTestEventHubNamespace",
    location=example.location,
    resource_group_name=example.name,
    sku="Basic",
    capacity=2,
    tags={
        "environment": "Production",
    })
example_event_hub = azure.eventhub.EventHub("example",
    name="acceptanceTestEventHub",
    namespace_name=example_event_hub_namespace.name,
    resource_group_name=example.name,
    partition_count=2,
    message_retention=2)
example_authorization_rule = azure.eventhub.AuthorizationRule("example",
    name="navi",
    namespace_name=example_event_hub_namespace.name,
    eventhub_name=example_event_hub.name,
    resource_group_name=example.name,
    listen=True,
    send=False,
    manage=False)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "example-resources",
        Location = "West Europe",
    });

    var exampleEventHubNamespace = new Azure.EventHub.EventHubNamespace("example", new()
    {
        Name = "acceptanceTestEventHubNamespace",
        Location = example.Location,
        ResourceGroupName = example.Name,
        Sku = "Basic",
        Capacity = 2,
        Tags = 
        {
            { "environment", "Production" },
        },
    });

    var exampleEventHub = new Azure.EventHub.EventHub("example", new()
    {
        Name = "acceptanceTestEventHub",
        NamespaceName = exampleEventHubNamespace.Name,
        ResourceGroupName = example.Name,
        PartitionCount = 2,
        MessageRetention = 2,
    });

    var exampleAuthorizationRule = new Azure.EventHub.AuthorizationRule("example", new()
    {
        Name = "navi",
        NamespaceName = exampleEventHubNamespace.Name,
        EventhubName = exampleEventHub.Name,
        ResourceGroupName = example.Name,
        Listen = true,
        Send = false,
        Manage = false,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("example-resources"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		exampleEventHubNamespace, err := eventhub.NewEventHubNamespace(ctx, "example", &eventhub.EventHubNamespaceArgs{
			Name:              pulumi.String("acceptanceTestEventHubNamespace"),
			Location:          example.Location,
			ResourceGroupName: example.Name,
			Sku:               pulumi.String("Basic"),
			Capacity:          pulumi.Int(2),
			Tags: pulumi.StringMap{
				"environment": pulumi.String("Production"),
			},
		})
		if err != nil {
			return err
		}
		exampleEventHub, err := eventhub.NewEventHub(ctx, "example", &eventhub.EventHubArgs{
			Name:              pulumi.String("acceptanceTestEventHub"),
			NamespaceName:     exampleEventHubNamespace.Name,
			ResourceGroupName: example.Name,
			PartitionCount:    pulumi.Int(2),
			MessageRetention:  pulumi.Int(2),
		})
		if err != nil {
			return err
		}
		_, err = eventhub.NewAuthorizationRule(ctx, "example", &eventhub.AuthorizationRuleArgs{
			Name:              pulumi.String("navi"),
			NamespaceName:     exampleEventHubNamespace.Name,
			EventhubName:      exampleEventHub.Name,
			ResourceGroupName: example.Name,
			Listen:            pulumi.Bool(true),
			Send:              pulumi.Bool(false),
			Manage:            pulumi.Bool(false),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.eventhub.EventHubNamespace;
import com.pulumi.azure.eventhub.EventHubNamespaceArgs;
import com.pulumi.azure.eventhub.EventHub;
import com.pulumi.azure.eventhub.EventHubArgs;
import com.pulumi.azure.eventhub.AuthorizationRule;
import com.pulumi.azure.eventhub.AuthorizationRuleArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("example-resources")
            .location("West Europe")
            .build());

        var exampleEventHubNamespace = new EventHubNamespace("exampleEventHubNamespace", EventHubNamespaceArgs.builder()
            .name("acceptanceTestEventHubNamespace")
            .location(example.location())
            .resourceGroupName(example.name())
            .sku("Basic")
            .capacity(2)
            .tags(Map.of("environment", "Production"))
            .build());

        var exampleEventHub = new EventHub("exampleEventHub", EventHubArgs.builder()
            .name("acceptanceTestEventHub")
            .namespaceName(exampleEventHubNamespace.name())
            .resourceGroupName(example.name())
            .partitionCount(2)
            .messageRetention(2)
            .build());

        var exampleAuthorizationRule = new AuthorizationRule("exampleAuthorizationRule", AuthorizationRuleArgs.builder()
            .name("navi")
            .namespaceName(exampleEventHubNamespace.name())
            .eventhubName(exampleEventHub.name())
            .resourceGroupName(example.name())
            .listen(true)
            .send(false)
            .manage(false)
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: example-resources
      location: West Europe
  exampleEventHubNamespace:
    type: azure:eventhub:EventHubNamespace
    name: example
    properties:
      name: acceptanceTestEventHubNamespace
      location: ${example.location}
      resourceGroupName: ${example.name}
      sku: Basic
      capacity: 2
      tags:
        environment: Production
  exampleEventHub:
    type: azure:eventhub:EventHub
    name: example
    properties:
      name: acceptanceTestEventHub
      namespaceName: ${exampleEventHubNamespace.name}
      resourceGroupName: ${example.name}
      partitionCount: 2
      messageRetention: 2
  exampleAuthorizationRule:
    type: azure:eventhub:AuthorizationRule
    name: example
    properties:
      name: navi
      namespaceName: ${exampleEventHubNamespace.name}
      eventhubName: ${exampleEventHub.name}
      resourceGroupName: ${example.name}
      listen: true
      send: false
      manage: false
```
<!--End PulumiCodeChooser -->

## Import

EventHub Authorization Rules can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:eventhub/eventHubAuthorizationRule:EventHubAuthorizationRule rule1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventHub/namespaces/namespace1/eventhubs/eventhub1/authorizationRules/rule1
```

k
eventhubName" WSpecifies the name of the EventHub. Changing this forces a new resource to be created.
o
listenB
 _Does this Authorization Rule have permissions to Listen to the Event Hub? Defaults to `false`.
µ
manageB
 §Does this Authorization Rule have permissions to Manage to the Event Hub? When this property is `true` - both `listen` and `send` must be too. Defaults to `false`.
Å
nameB" sSpecifies the name of the EventHub Authorization Rule resource. Changing this forces a new resource to be created.
Ç
namespaceName" mSpecifies the name of the grandparent EventHub Namespace. Changing this forces a new resource to be created.
Ÿ
resourceGroupName" øThe name of the resource group in which the EventHub Namespace exists. Changing this forces a new resource to be created.

> **NOTE** At least one of the 3 permissions below needs to be set.
k
sendB
 ]Does this Authorization Rule have permissions to Send to the Event Hub? Defaults to `false`.
"k
eventhubName" WSpecifies the name of the EventHub. Changing this forces a new resource to be created.
"o
listenB
 _Does this Authorization Rule have permissions to Listen to the Event Hub? Defaults to `false`.
"µ
manageB
 §Does this Authorization Rule have permissions to Manage to the Event Hub? When this property is `true` - both `listen` and `send` must be too. Defaults to `false`.
"
name" sSpecifies the name of the EventHub Authorization Rule resource. Changing this forces a new resource to be created.
"Ç
namespaceName" mSpecifies the name of the grandparent EventHub Namespace. Changing this forces a new resource to be created.
"d
primaryConnectionString" EThe Primary Connection String for the Event Hubs authorization Rule.
"≠
primaryConnectionStringAlias" àThe alias of the Primary Connection String for the Event Hubs authorization Rule, which is generated when disaster recovery is enabled.
"I

primaryKey" 7The Primary Key for the Event Hubs authorization Rule.
"Ÿ
resourceGroupName" øThe name of the resource group in which the EventHub Namespace exists. Changing this forces a new resource to be created.

> **NOTE** At least one of the 3 permissions below needs to be set.
"h
secondaryConnectionString" GThe Secondary Connection String for the Event Hubs Authorization Rule.
"±
secondaryConnectionStringAlias" äThe alias of the Secondary Connection String for the Event Hubs Authorization Rule, which is generated when disaster recovery is enabled.
"M
secondaryKey" 9The Secondary Key for the Event Hubs Authorization Rule.
"k
sendB
 ]Does this Authorization Rule have permissions to Send to the Event Hub? Defaults to `false`.
*ƒM
]
eventhubEventHubConsumerGroup:azure:eventhub/eventHubConsumerGroup:EventHubConsumerGroup∞CManages a Event Hubs Consumer Group as a nested resource within an Event Hub.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "example-resources",
    location: "West Europe",
});
const exampleEventHubNamespace = new azure.eventhub.EventHubNamespace("example", {
    name: "acceptanceTestEventHubNamespace",
    location: example.location,
    resourceGroupName: example.name,
    sku: "Basic",
    capacity: 2,
    tags: {
        environment: "Production",
    },
});
const exampleEventHub = new azure.eventhub.EventHub("example", {
    name: "acceptanceTestEventHub",
    namespaceName: exampleEventHubNamespace.name,
    resourceGroupName: example.name,
    partitionCount: 2,
    messageRetention: 2,
});
const exampleConsumerGroup = new azure.eventhub.ConsumerGroup("example", {
    name: "acceptanceTestEventHubConsumerGroup",
    namespaceName: exampleEventHubNamespace.name,
    eventhubName: exampleEventHub.name,
    resourceGroupName: example.name,
    userMetadata: "some-meta-data",
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="example-resources",
    location="West Europe")
example_event_hub_namespace = azure.eventhub.EventHubNamespace("example",
    name="acceptanceTestEventHubNamespace",
    location=example.location,
    resource_group_name=example.name,
    sku="Basic",
    capacity=2,
    tags={
        "environment": "Production",
    })
example_event_hub = azure.eventhub.EventHub("example",
    name="acceptanceTestEventHub",
    namespace_name=example_event_hub_namespace.name,
    resource_group_name=example.name,
    partition_count=2,
    message_retention=2)
example_consumer_group = azure.eventhub.ConsumerGroup("example",
    name="acceptanceTestEventHubConsumerGroup",
    namespace_name=example_event_hub_namespace.name,
    eventhub_name=example_event_hub.name,
    resource_group_name=example.name,
    user_metadata="some-meta-data")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "example-resources",
        Location = "West Europe",
    });

    var exampleEventHubNamespace = new Azure.EventHub.EventHubNamespace("example", new()
    {
        Name = "acceptanceTestEventHubNamespace",
        Location = example.Location,
        ResourceGroupName = example.Name,
        Sku = "Basic",
        Capacity = 2,
        Tags = 
        {
            { "environment", "Production" },
        },
    });

    var exampleEventHub = new Azure.EventHub.EventHub("example", new()
    {
        Name = "acceptanceTestEventHub",
        NamespaceName = exampleEventHubNamespace.Name,
        ResourceGroupName = example.Name,
        PartitionCount = 2,
        MessageRetention = 2,
    });

    var exampleConsumerGroup = new Azure.EventHub.ConsumerGroup("example", new()
    {
        Name = "acceptanceTestEventHubConsumerGroup",
        NamespaceName = exampleEventHubNamespace.Name,
        EventhubName = exampleEventHub.Name,
        ResourceGroupName = example.Name,
        UserMetadata = "some-meta-data",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("example-resources"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		exampleEventHubNamespace, err := eventhub.NewEventHubNamespace(ctx, "example", &eventhub.EventHubNamespaceArgs{
			Name:              pulumi.String("acceptanceTestEventHubNamespace"),
			Location:          example.Location,
			ResourceGroupName: example.Name,
			Sku:               pulumi.String("Basic"),
			Capacity:          pulumi.Int(2),
			Tags: pulumi.StringMap{
				"environment": pulumi.String("Production"),
			},
		})
		if err != nil {
			return err
		}
		exampleEventHub, err := eventhub.NewEventHub(ctx, "example", &eventhub.EventHubArgs{
			Name:              pulumi.String("acceptanceTestEventHub"),
			NamespaceName:     exampleEventHubNamespace.Name,
			ResourceGroupName: example.Name,
			PartitionCount:    pulumi.Int(2),
			MessageRetention:  pulumi.Int(2),
		})
		if err != nil {
			return err
		}
		_, err = eventhub.NewConsumerGroup(ctx, "example", &eventhub.ConsumerGroupArgs{
			Name:              pulumi.String("acceptanceTestEventHubConsumerGroup"),
			NamespaceName:     exampleEventHubNamespace.Name,
			EventhubName:      exampleEventHub.Name,
			ResourceGroupName: example.Name,
			UserMetadata:      pulumi.String("some-meta-data"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.eventhub.EventHubNamespace;
import com.pulumi.azure.eventhub.EventHubNamespaceArgs;
import com.pulumi.azure.eventhub.EventHub;
import com.pulumi.azure.eventhub.EventHubArgs;
import com.pulumi.azure.eventhub.ConsumerGroup;
import com.pulumi.azure.eventhub.ConsumerGroupArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("example-resources")
            .location("West Europe")
            .build());

        var exampleEventHubNamespace = new EventHubNamespace("exampleEventHubNamespace", EventHubNamespaceArgs.builder()
            .name("acceptanceTestEventHubNamespace")
            .location(example.location())
            .resourceGroupName(example.name())
            .sku("Basic")
            .capacity(2)
            .tags(Map.of("environment", "Production"))
            .build());

        var exampleEventHub = new EventHub("exampleEventHub", EventHubArgs.builder()
            .name("acceptanceTestEventHub")
            .namespaceName(exampleEventHubNamespace.name())
            .resourceGroupName(example.name())
            .partitionCount(2)
            .messageRetention(2)
            .build());

        var exampleConsumerGroup = new ConsumerGroup("exampleConsumerGroup", ConsumerGroupArgs.builder()
            .name("acceptanceTestEventHubConsumerGroup")
            .namespaceName(exampleEventHubNamespace.name())
            .eventhubName(exampleEventHub.name())
            .resourceGroupName(example.name())
            .userMetadata("some-meta-data")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: example-resources
      location: West Europe
  exampleEventHubNamespace:
    type: azure:eventhub:EventHubNamespace
    name: example
    properties:
      name: acceptanceTestEventHubNamespace
      location: ${example.location}
      resourceGroupName: ${example.name}
      sku: Basic
      capacity: 2
      tags:
        environment: Production
  exampleEventHub:
    type: azure:eventhub:EventHub
    name: example
    properties:
      name: acceptanceTestEventHub
      namespaceName: ${exampleEventHubNamespace.name}
      resourceGroupName: ${example.name}
      partitionCount: 2
      messageRetention: 2
  exampleConsumerGroup:
    type: azure:eventhub:ConsumerGroup
    name: example
    properties:
      name: acceptanceTestEventHubConsumerGroup
      namespaceName: ${exampleEventHubNamespace.name}
      eventhubName: ${exampleEventHub.name}
      resourceGroupName: ${example.name}
      userMetadata: some-meta-data
```
<!--End PulumiCodeChooser -->

## Import

EventHub Consumer Groups can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:eventhub/eventHubConsumerGroup:EventHubConsumerGroup consumerGroup1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventHub/namespaces/namespace1/eventhubs/eventhub1/consumerGroups/consumerGroup1
```

k
eventhubName" WSpecifies the name of the EventHub. Changing this forces a new resource to be created.
}
nameB" oSpecifies the name of the EventHub Consumer Group resource. Changing this forces a new resource to be created.
Ç
namespaceName" mSpecifies the name of the grandparent EventHub Namespace. Changing this forces a new resource to be created.
±
resourceGroupName" óThe name of the resource group in which the EventHub Consumer Group's grandparent Namespace exists. Changing this forces a new resource to be created.
3
userMetadataB" Specifies the user metadata.
"k
eventhubName" WSpecifies the name of the EventHub. Changing this forces a new resource to be created.
"{
name" oSpecifies the name of the EventHub Consumer Group resource. Changing this forces a new resource to be created.
"Ç
namespaceName" mSpecifies the name of the grandparent EventHub Namespace. Changing this forces a new resource to be created.
"±
resourceGroupName" óThe name of the resource group in which the EventHub Consumer Group's grandparent Namespace exists. Changing this forces a new resource to be created.
"3
userMetadataB" Specifies the user metadata.
*ˇM
Q
eventhubEventHubNamespace2azure:eventhub/eventHubNamespace:EventHubNamespace∑#Manages an EventHub Namespace.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "example-resources",
    location: "West Europe",
});
const exampleEventHubNamespace = new azure.eventhub.EventHubNamespace("example", {
    name: "example-namespace",
    location: example.location,
    resourceGroupName: example.name,
    sku: "Standard",
    capacity: 2,
    tags: {
        environment: "Production",
    },
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="example-resources",
    location="West Europe")
example_event_hub_namespace = azure.eventhub.EventHubNamespace("example",
    name="example-namespace",
    location=example.location,
    resource_group_name=example.name,
    sku="Standard",
    capacity=2,
    tags={
        "environment": "Production",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "example-resources",
        Location = "West Europe",
    });

    var exampleEventHubNamespace = new Azure.EventHub.EventHubNamespace("example", new()
    {
        Name = "example-namespace",
        Location = example.Location,
        ResourceGroupName = example.Name,
        Sku = "Standard",
        Capacity = 2,
        Tags = 
        {
            { "environment", "Production" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("example-resources"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		_, err = eventhub.NewEventHubNamespace(ctx, "example", &eventhub.EventHubNamespaceArgs{
			Name:              pulumi.String("example-namespace"),
			Location:          example.Location,
			ResourceGroupName: example.Name,
			Sku:               pulumi.String("Standard"),
			Capacity:          pulumi.Int(2),
			Tags: pulumi.StringMap{
				"environment": pulumi.String("Production"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.eventhub.EventHubNamespace;
import com.pulumi.azure.eventhub.EventHubNamespaceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("example-resources")
            .location("West Europe")
            .build());

        var exampleEventHubNamespace = new EventHubNamespace("exampleEventHubNamespace", EventHubNamespaceArgs.builder()
            .name("example-namespace")
            .location(example.location())
            .resourceGroupName(example.name())
            .sku("Standard")
            .capacity(2)
            .tags(Map.of("environment", "Production"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: example-resources
      location: West Europe
  exampleEventHubNamespace:
    type: azure:eventhub:EventHubNamespace
    name: example
    properties:
      name: example-namespace
      location: ${example.location}
      resourceGroupName: ${example.name}
      sku: Standard
      capacity: 2
      tags:
        environment: Production
```
<!--End PulumiCodeChooser -->

## Import

EventHub Namespaces can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:eventhub/eventHubNamespace:EventHubNamespace namespace1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventHub/namespaces/namespace1
```

P
autoInflateEnabledB
 4Is Auto Inflate enabled for the EventHub Namespace?
ÿ
capacityB ≈Specifies the Capacity / Throughput Units for a `Standard` SKU namespace. Default capacity has a maximum of `2`, but can be increased in blocks of 2 on a committed purchase basis. Defaults to `1`.
®
dedicatedClusterIdB" ãSpecifies the ID of the EventHub Dedicated Cluster where this Namespace should created. Changing this forces a new resource to be created.
£
identityoBm:k
i
eventhubEventHubNamespaceIdentityBazure:eventhub/EventHubNamespaceIdentity:EventHubNamespaceIdentity&An `identity` block as defined below.
r
localAuthenticationEnabledB
 NIs SAS authentication enabled for the EventHub Namespace? Defaults to `true`.
á
locationB" uSpecifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
ì
maximumThroughputUnitsB sSpecifies the maximum number of throughput units when Auto Inflate is Enabled. Valid values range from `1` - `20`.
È
minimumTlsVersionB" ÕThe minimum supported TLS version for this EventHub Namespace. Valid values are: `1.0`, `1.1` and `1.2`. Defaults to `1.2`.

> **Note** Azure Services will require TLS 1.2+ by August 2025, please see this [announcement](https://azure.microsoft.com/en-us/updates/v2/update-retirement-tls1-0-tls1-1-versions-azure-services/) for more.
x
nameB" jSpecifies the name of the EventHub Namespace resource. Changing this forces a new resource to be created.
…
networkRulesetsÜBÉ:Ä
~
eventhub EventHubNamespaceNetworkRulesetsPazure:eventhub/EventHubNamespaceNetworkRulesets:EventHubNamespaceNetworkRulesets-A `network_rulesets` block as defined below.
u
publicNetworkAccessEnabledB
 QIs public network access enabled for the EventHub Namespace? Defaults to `true`.
ç
resourceGroupName" tThe name of the resource group in which to create the namespace. Changing this forces a new resource to be created.
π
sku" ≠Defines which tier to use. Valid options are `Basic`, `Standard`, and `Premium`. Please note that setting this field to `Premium` will force the creation of a new resource.
=
tagsB2" -A mapping of tags to assign to the resource.
"P
autoInflateEnabledB
 4Is Auto Inflate enabled for the EventHub Namespace?
"ÿ
capacityB ≈Specifies the Capacity / Throughput Units for a `Standard` SKU namespace. Default capacity has a maximum of `2`, but can be increased in blocks of 2 on a committed purchase basis. Defaults to `1`.
"®
dedicatedClusterIdB" ãSpecifies the ID of the EventHub Dedicated Cluster where this Namespace should created. Changing this forces a new resource to be created.
"|
defaultPrimaryConnectionString" VThe primary connection string for the authorization rule `RootManageSharedAccessKey`.
"≈
#defaultPrimaryConnectionStringAlias" ôThe alias of the primary connection string for the authorization rule `RootManageSharedAccessKey`, which is generated when disaster recovery is enabled.
"h
defaultPrimaryKey" OThe primary access key for the authorization rule `RootManageSharedAccessKey`.
"Ä
 defaultSecondaryConnectionString" XThe secondary connection string for the authorization rule `RootManageSharedAccessKey`.
"…
%defaultSecondaryConnectionStringAlias" õThe alias of the secondary connection string for the authorization rule `RootManageSharedAccessKey`, which is generated when disaster recovery is enabled.
"l
defaultSecondaryKey" QThe secondary access key for the authorization rule `RootManageSharedAccessKey`.
"£
identityoBm:k
i
eventhubEventHubNamespaceIdentityBazure:eventhub/EventHubNamespaceIdentity:EventHubNamespaceIdentity&An `identity` block as defined below.
"r
localAuthenticationEnabledB
 NIs SAS authentication enabled for the EventHub Namespace? Defaults to `true`.
"Ö
location" uSpecifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
"ì
maximumThroughputUnitsB sSpecifies the maximum number of throughput units when Auto Inflate is Enabled. Valid values range from `1` - `20`.
"È
minimumTlsVersionB" ÕThe minimum supported TLS version for this EventHub Namespace. Valid values are: `1.0`, `1.1` and `1.2`. Defaults to `1.2`.

> **Note** Azure Services will require TLS 1.2+ by August 2025, please see this [announcement](https://azure.microsoft.com/en-us/updates/v2/update-retirement-tls1-0-tls1-1-versions-azure-services/) for more.
"v
name" jSpecifies the name of the EventHub Namespace resource. Changing this forces a new resource to be created.
"∆
networkRulesetsÉ:Ä
~
eventhub EventHubNamespaceNetworkRulesetsPazure:eventhub/EventHubNamespaceNetworkRulesets:EventHubNamespaceNetworkRulesets-A `network_rulesets` block as defined below.
"u
publicNetworkAccessEnabledB
 QIs public network access enabled for the EventHub Namespace? Defaults to `true`.
"ç
resourceGroupName" tThe name of the resource group in which to create the namespace. Changing this forces a new resource to be created.
"π
sku" ≠Defines which tier to use. Valid options are `Basic`, `Standard`, and `Premium`. Please note that setting this field to `Premium` will force the creation of a new resource.
"=
tagsB2" -A mapping of tags to assign to the resource.
*≠H
Ñ
eventhub"EventHubNamespaceAuthorizationRuleTazure:eventhub/eventHubNamespaceAuthorizationRule:EventHubNamespaceAuthorizationRule›5Manages an Authorization Rule for an Event Hub Namespace.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "resourcegroup",
    location: "West Europe",
});
const exampleEventHubNamespace = new azure.eventhub.EventHubNamespace("example", {
    name: "acceptanceTestEventHubNamespace",
    location: example.location,
    resourceGroupName: example.name,
    sku: "Basic",
    capacity: 2,
    tags: {
        environment: "Production",
    },
});
const exampleEventHubNamespaceAuthorizationRule = new azure.eventhub.EventHubNamespaceAuthorizationRule("example", {
    name: "navi",
    namespaceName: exampleEventHubNamespace.name,
    resourceGroupName: example.name,
    listen: true,
    send: false,
    manage: false,
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="resourcegroup",
    location="West Europe")
example_event_hub_namespace = azure.eventhub.EventHubNamespace("example",
    name="acceptanceTestEventHubNamespace",
    location=example.location,
    resource_group_name=example.name,
    sku="Basic",
    capacity=2,
    tags={
        "environment": "Production",
    })
example_event_hub_namespace_authorization_rule = azure.eventhub.EventHubNamespaceAuthorizationRule("example",
    name="navi",
    namespace_name=example_event_hub_namespace.name,
    resource_group_name=example.name,
    listen=True,
    send=False,
    manage=False)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "resourcegroup",
        Location = "West Europe",
    });

    var exampleEventHubNamespace = new Azure.EventHub.EventHubNamespace("example", new()
    {
        Name = "acceptanceTestEventHubNamespace",
        Location = example.Location,
        ResourceGroupName = example.Name,
        Sku = "Basic",
        Capacity = 2,
        Tags = 
        {
            { "environment", "Production" },
        },
    });

    var exampleEventHubNamespaceAuthorizationRule = new Azure.EventHub.EventHubNamespaceAuthorizationRule("example", new()
    {
        Name = "navi",
        NamespaceName = exampleEventHubNamespace.Name,
        ResourceGroupName = example.Name,
        Listen = true,
        Send = false,
        Manage = false,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("resourcegroup"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		exampleEventHubNamespace, err := eventhub.NewEventHubNamespace(ctx, "example", &eventhub.EventHubNamespaceArgs{
			Name:              pulumi.String("acceptanceTestEventHubNamespace"),
			Location:          example.Location,
			ResourceGroupName: example.Name,
			Sku:               pulumi.String("Basic"),
			Capacity:          pulumi.Int(2),
			Tags: pulumi.StringMap{
				"environment": pulumi.String("Production"),
			},
		})
		if err != nil {
			return err
		}
		_, err = eventhub.NewEventHubNamespaceAuthorizationRule(ctx, "example", &eventhub.EventHubNamespaceAuthorizationRuleArgs{
			Name:              pulumi.String("navi"),
			NamespaceName:     exampleEventHubNamespace.Name,
			ResourceGroupName: example.Name,
			Listen:            pulumi.Bool(true),
			Send:              pulumi.Bool(false),
			Manage:            pulumi.Bool(false),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.eventhub.EventHubNamespace;
import com.pulumi.azure.eventhub.EventHubNamespaceArgs;
import com.pulumi.azure.eventhub.EventHubNamespaceAuthorizationRule;
import com.pulumi.azure.eventhub.EventHubNamespaceAuthorizationRuleArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("resourcegroup")
            .location("West Europe")
            .build());

        var exampleEventHubNamespace = new EventHubNamespace("exampleEventHubNamespace", EventHubNamespaceArgs.builder()
            .name("acceptanceTestEventHubNamespace")
            .location(example.location())
            .resourceGroupName(example.name())
            .sku("Basic")
            .capacity(2)
            .tags(Map.of("environment", "Production"))
            .build());

        var exampleEventHubNamespaceAuthorizationRule = new EventHubNamespaceAuthorizationRule("exampleEventHubNamespaceAuthorizationRule", EventHubNamespaceAuthorizationRuleArgs.builder()
            .name("navi")
            .namespaceName(exampleEventHubNamespace.name())
            .resourceGroupName(example.name())
            .listen(true)
            .send(false)
            .manage(false)
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: resourcegroup
      location: West Europe
  exampleEventHubNamespace:
    type: azure:eventhub:EventHubNamespace
    name: example
    properties:
      name: acceptanceTestEventHubNamespace
      location: ${example.location}
      resourceGroupName: ${example.name}
      sku: Basic
      capacity: 2
      tags:
        environment: Production
  exampleEventHubNamespaceAuthorizationRule:
    type: azure:eventhub:EventHubNamespaceAuthorizationRule
    name: example
    properties:
      name: navi
      namespaceName: ${exampleEventHubNamespace.name}
      resourceGroupName: ${example.name}
      listen: true
      send: false
      manage: false
```
<!--End PulumiCodeChooser -->

## Import

EventHub Namespace Authorization Rules can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:eventhub/eventHubNamespaceAuthorizationRule:EventHubNamespaceAuthorizationRule example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventHub/namespaces/namespace1/authorizationRules/rule1
```

[
listenB
 KGrants listen access to this this Authorization Rule. Defaults to `false`.
°
manageB
 êGrants manage access to this this Authorization Rule. When this property is `true` - both `listen` and `send` must be too. Defaults to `false`.
o
nameB" aSpecifies the name of the Authorization Rule. Changing this forces a new resource to be created.
v
namespaceName" aSpecifies the name of the EventHub Namespace. Changing this forces a new resource to be created.
Ÿ
resourceGroupName" øThe name of the resource group in which the EventHub Namespace exists. Changing this forces a new resource to be created.

> **NOTE** At least one of the 3 permissions below needs to be set.
W
sendB
 IGrants send access to this this Authorization Rule. Defaults to `false`.
"[
listenB
 KGrants listen access to this this Authorization Rule. Defaults to `false`.
"°
manageB
 êGrants manage access to this this Authorization Rule. When this property is `true` - both `listen` and `send` must be too. Defaults to `false`.
"m
name" aSpecifies the name of the Authorization Rule. Changing this forces a new resource to be created.
"v
namespaceName" aSpecifies the name of the EventHub Namespace. Changing this forces a new resource to be created.
"Y
primaryConnectionString" :The Primary Connection String for the Authorization Rule.
"°
primaryConnectionStringAlias" }The alias of the Primary Connection String for the Authorization Rule, which is generated when disaster recovery is enabled.
">

primaryKey" ,The Primary Key for the Authorization Rule.
"Ÿ
resourceGroupName" øThe name of the resource group in which the EventHub Namespace exists. Changing this forces a new resource to be created.

> **NOTE** At least one of the 3 permissions below needs to be set.
"]
secondaryConnectionString" <The Secondary Connection String for the Authorization Rule.
"•
secondaryConnectionStringAlias" The alias of the Secondary Connection String for the Authorization Rule, which is generated when disaster recovery is enabled.
"B
secondaryKey" .The Secondary Key for the Authorization Rule.
"W
sendB
 IGrants send access to this this Authorization Rule. Defaults to `false`.
*™{
Q
eventhubEventSubscription2azure:eventhub/eventSubscription:EventSubscriptioné=Manages an EventGrid Event Subscription

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "example-resources",
    location: "West Europe",
});
const exampleAccount = new azure.storage.Account("example", {
    name: "exampleasa",
    resourceGroupName: example.name,
    location: example.location,
    accountTier: "Standard",
    accountReplicationType: "LRS",
    tags: {
        environment: "staging",
    },
});
const exampleQueue = new azure.storage.Queue("example", {
    name: "example-astq",
    storageAccountName: exampleAccount.name,
});
const exampleEventSubscription = new azure.eventgrid.EventSubscription("example", {
    name: "example-aees",
    scope: example.id,
    storageQueueEndpoint: {
        storageAccountId: exampleAccount.id,
        queueName: exampleQueue.name,
    },
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="example-resources",
    location="West Europe")
example_account = azure.storage.Account("example",
    name="exampleasa",
    resource_group_name=example.name,
    location=example.location,
    account_tier="Standard",
    account_replication_type="LRS",
    tags={
        "environment": "staging",
    })
example_queue = azure.storage.Queue("example",
    name="example-astq",
    storage_account_name=example_account.name)
example_event_subscription = azure.eventgrid.EventSubscription("example",
    name="example-aees",
    scope=example.id,
    storage_queue_endpoint={
        "storage_account_id": example_account.id,
        "queue_name": example_queue.name,
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "example-resources",
        Location = "West Europe",
    });

    var exampleAccount = new Azure.Storage.Account("example", new()
    {
        Name = "exampleasa",
        ResourceGroupName = example.Name,
        Location = example.Location,
        AccountTier = "Standard",
        AccountReplicationType = "LRS",
        Tags = 
        {
            { "environment", "staging" },
        },
    });

    var exampleQueue = new Azure.Storage.Queue("example", new()
    {
        Name = "example-astq",
        StorageAccountName = exampleAccount.Name,
    });

    var exampleEventSubscription = new Azure.EventGrid.EventSubscription("example", new()
    {
        Name = "example-aees",
        Scope = example.Id,
        StorageQueueEndpoint = new Azure.EventGrid.Inputs.EventSubscriptionStorageQueueEndpointArgs
        {
            StorageAccountId = exampleAccount.Id,
            QueueName = exampleQueue.Name,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventgrid"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/storage"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("example-resources"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		exampleAccount, err := storage.NewAccount(ctx, "example", &storage.AccountArgs{
			Name:                   pulumi.String("exampleasa"),
			ResourceGroupName:      example.Name,
			Location:               example.Location,
			AccountTier:            pulumi.String("Standard"),
			AccountReplicationType: pulumi.String("LRS"),
			Tags: pulumi.StringMap{
				"environment": pulumi.String("staging"),
			},
		})
		if err != nil {
			return err
		}
		exampleQueue, err := storage.NewQueue(ctx, "example", &storage.QueueArgs{
			Name:               pulumi.String("example-astq"),
			StorageAccountName: exampleAccount.Name,
		})
		if err != nil {
			return err
		}
		_, err = eventgrid.NewEventSubscription(ctx, "example", &eventgrid.EventSubscriptionArgs{
			Name:  pulumi.String("example-aees"),
			Scope: example.ID(),
			StorageQueueEndpoint: &eventgrid.EventSubscriptionStorageQueueEndpointArgs{
				StorageAccountId: exampleAccount.ID(),
				QueueName:        exampleQueue.Name,
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.storage.Account;
import com.pulumi.azure.storage.AccountArgs;
import com.pulumi.azure.storage.Queue;
import com.pulumi.azure.storage.QueueArgs;
import com.pulumi.azure.eventgrid.EventSubscription;
import com.pulumi.azure.eventgrid.EventSubscriptionArgs;
import com.pulumi.azure.eventgrid.inputs.EventSubscriptionStorageQueueEndpointArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("example-resources")
            .location("West Europe")
            .build());

        var exampleAccount = new Account("exampleAccount", AccountArgs.builder()
            .name("exampleasa")
            .resourceGroupName(example.name())
            .location(example.location())
            .accountTier("Standard")
            .accountReplicationType("LRS")
            .tags(Map.of("environment", "staging"))
            .build());

        var exampleQueue = new Queue("exampleQueue", QueueArgs.builder()
            .name("example-astq")
            .storageAccountName(exampleAccount.name())
            .build());

        var exampleEventSubscription = new EventSubscription("exampleEventSubscription", EventSubscriptionArgs.builder()
            .name("example-aees")
            .scope(example.id())
            .storageQueueEndpoint(EventSubscriptionStorageQueueEndpointArgs.builder()
                .storageAccountId(exampleAccount.id())
                .queueName(exampleQueue.name())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: example-resources
      location: West Europe
  exampleAccount:
    type: azure:storage:Account
    name: example
    properties:
      name: exampleasa
      resourceGroupName: ${example.name}
      location: ${example.location}
      accountTier: Standard
      accountReplicationType: LRS
      tags:
        environment: staging
  exampleQueue:
    type: azure:storage:Queue
    name: example
    properties:
      name: example-astq
      storageAccountName: ${exampleAccount.name}
  exampleEventSubscription:
    type: azure:eventgrid:EventSubscription
    name: example
    properties:
      name: example-aees
      scope: ${example.id}
      storageQueueEndpoint:
        storageAccountId: ${exampleAccount.id}
        queueName: ${exampleQueue.name}
```
<!--End PulumiCodeChooser -->

## Import

EventGrid Event Subscription's can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:eventhub/eventSubscription:EventSubscription eventSubscription1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventGrid/topics/topic1/providers/Microsoft.EventGrid/eventSubscriptions/eventSubscription1
```

¬
advancedFilterÅB:}
{
eventhubEventSubscriptionAdvancedFilterNazure:eventhub/EventSubscriptionAdvancedFilter:EventSubscriptionAdvancedFilter,A `advanced_filter` block as defined below.
π
 advancedFilteringOnArraysEnabledB
 éSpecifies whether advanced filters should be evaluated against an array of values instead of expecting a singular value. Defaults to `false`.
Í
azureFunctionEndpointôBñ:ì
ê
eventhub&EventSubscriptionAzureFunctionEndpoint\azure:eventhub/EventSubscriptionAzureFunctionEndpoint:EventSubscriptionAzureFunctionEndpoint5An `azure_function_endpoint` block as defined below.
Ã
deadLetterIdentityêBç:ä
á
eventhub#EventSubscriptionDeadLetterIdentityVazure:eventhub/EventSubscriptionDeadLetterIdentity:EventSubscriptionDeadLetterIdentity¢A `dead_letter_identity` block as defined below.

> **Note:** `storage_blob_dead_letter_destination` must be specified when a `dead_letter_identity` is specified
œ
deliveryIdentityäBá:Ñ
Å
eventhub!EventSubscriptionDeliveryIdentityRazure:eventhub/EventSubscriptionDeliveryIdentity:EventSubscriptionDeliveryIdentity.A `delivery_identity` block as defined below.
ﬂ
deliveryPropertiesçBä*á:Ñ
Å
eventhub!EventSubscriptionDeliveryPropertyRazure:eventhub/EventSubscriptionDeliveryProperty:EventSubscriptionDeliveryProperty9One or more `delivery_property` blocks as defined below.
â
eventDeliverySchemaB" ÎSpecifies the event delivery schema for the event subscription. Possible values include: `EventGridSchema`, `CloudEventSchemaV1_0`, `CustomInputSchema`. Defaults to `EventGridSchema`. Changing this forces a new resource to be created.
M
eventhubEndpointIdB" 1Specifies the id where the Event Hub is located.
q
expirationTimeUtcB" VSpecifies the expiration time of the event subscription (Datetime Format `RFC 3339`).
]
hybridConnectionEndpointIdB" 9Specifies the id where the Hybrid Connection is located.
o
includedEventTypesB*" QA list of applicable event types that need to be part of the event subscription.
H
labelsB*" 6A list of labels to assign to the event subscription.
Ç
nameB" tSpecifies the name of the EventGrid Event Subscription resource. Changing this forces a new resource to be created.
≤
retryPolicyxBv:t
r
eventhubEventSubscriptionRetryPolicyHazure:eventhub/EventSubscriptionRetryPolicy:EventSubscriptionRetryPolicy)A `retry_policy` block as defined below.
í
scope" ÑSpecifies the scope at which the EventGrid Event Subscription should be created. Changing this forces a new resource to be created.
\
serviceBusQueueEndpointIdB" 9Specifies the id where the Service Bus Queue is located.
\
serviceBusTopicEndpointIdB" 9Specifies the id where the Service Bus Topic is located.
¢
 storageBlobDeadLetterDestination∫B∑:¥
±
eventhub1EventSubscriptionStorageBlobDeadLetterDestinationrazure:eventhub/EventSubscriptionStorageBlobDeadLetterDestination:EventSubscriptionStorageBlobDeadLetterDestinationAA `storage_blob_dead_letter_destination` block as defined below.
‰
storageQueueEndpointñBì:ê
ç
eventhub%EventSubscriptionStorageQueueEndpointZazure:eventhub/EventSubscriptionStorageQueueEndpoint:EventSubscriptionStorageQueueEndpoint3A `storage_queue_endpoint` block as defined below.
º
subjectFilter~B|:z
x
eventhubEventSubscriptionSubjectFilterLazure:eventhub/EventSubscriptionSubjectFilter:EventSubscriptionSubjectFilter+A `subject_filter` block as defined below.
∂
webhookEndpointÜBÉ:Ä
~
eventhub EventSubscriptionWebhookEndpointPazure:eventhub/EventSubscriptionWebhookEndpoint:EventSubscriptionWebhookEndpointôA `webhook_endpoint` block as defined below.

> **NOTE:** One of `eventhub_endpoint_id`, `hybrid_connection_endpoint_id`, `service_bus_queue_endpoint_id`, `service_bus_topic_endpoint_id`, `storage_queue_endpoint`, `webhook_endpoint` or `azure_function_endpoint` must be specified.
"¬
advancedFilterÅB:}
{
eventhubEventSubscriptionAdvancedFilterNazure:eventhub/EventSubscriptionAdvancedFilter:EventSubscriptionAdvancedFilter,A `advanced_filter` block as defined below.
"π
 advancedFilteringOnArraysEnabledB
 éSpecifies whether advanced filters should be evaluated against an array of values instead of expecting a singular value. Defaults to `false`.
"Í
azureFunctionEndpointôBñ:ì
ê
eventhub&EventSubscriptionAzureFunctionEndpoint\azure:eventhub/EventSubscriptionAzureFunctionEndpoint:EventSubscriptionAzureFunctionEndpoint5An `azure_function_endpoint` block as defined below.
"Ã
deadLetterIdentityêBç:ä
á
eventhub#EventSubscriptionDeadLetterIdentityVazure:eventhub/EventSubscriptionDeadLetterIdentity:EventSubscriptionDeadLetterIdentity¢A `dead_letter_identity` block as defined below.

> **Note:** `storage_blob_dead_letter_destination` must be specified when a `dead_letter_identity` is specified
"œ
deliveryIdentityäBá:Ñ
Å
eventhub!EventSubscriptionDeliveryIdentityRazure:eventhub/EventSubscriptionDeliveryIdentity:EventSubscriptionDeliveryIdentity.A `delivery_identity` block as defined below.
"ﬂ
deliveryPropertiesçBä*á:Ñ
Å
eventhub!EventSubscriptionDeliveryPropertyRazure:eventhub/EventSubscriptionDeliveryProperty:EventSubscriptionDeliveryProperty9One or more `delivery_property` blocks as defined below.
"â
eventDeliverySchemaB" ÎSpecifies the event delivery schema for the event subscription. Possible values include: `EventGridSchema`, `CloudEventSchemaV1_0`, `CustomInputSchema`. Defaults to `EventGridSchema`. Changing this forces a new resource to be created.
"K
eventhubEndpointId" 1Specifies the id where the Event Hub is located.
"q
expirationTimeUtcB" VSpecifies the expiration time of the event subscription (Datetime Format `RFC 3339`).
"[
hybridConnectionEndpointId" 9Specifies the id where the Hybrid Connection is located.
"m
includedEventTypes*" QA list of applicable event types that need to be part of the event subscription.
"H
labelsB*" 6A list of labels to assign to the event subscription.
"Ä
name" tSpecifies the name of the EventGrid Event Subscription resource. Changing this forces a new resource to be created.
"∞
retryPolicyv:t
r
eventhubEventSubscriptionRetryPolicyHazure:eventhub/EventSubscriptionRetryPolicy:EventSubscriptionRetryPolicy)A `retry_policy` block as defined below.
"í
scope" ÑSpecifies the scope at which the EventGrid Event Subscription should be created. Changing this forces a new resource to be created.
"\
serviceBusQueueEndpointIdB" 9Specifies the id where the Service Bus Queue is located.
"\
serviceBusTopicEndpointIdB" 9Specifies the id where the Service Bus Topic is located.
"¢
 storageBlobDeadLetterDestination∫B∑:¥
±
eventhub1EventSubscriptionStorageBlobDeadLetterDestinationrazure:eventhub/EventSubscriptionStorageBlobDeadLetterDestination:EventSubscriptionStorageBlobDeadLetterDestinationAA `storage_blob_dead_letter_destination` block as defined below.
"‰
storageQueueEndpointñBì:ê
ç
eventhub%EventSubscriptionStorageQueueEndpointZazure:eventhub/EventSubscriptionStorageQueueEndpoint:EventSubscriptionStorageQueueEndpoint3A `storage_queue_endpoint` block as defined below.
"º
subjectFilter~B|:z
x
eventhubEventSubscriptionSubjectFilterLazure:eventhub/EventSubscriptionSubjectFilter:EventSubscriptionSubjectFilter+A `subject_filter` block as defined below.
"∂
webhookEndpointÜBÉ:Ä
~
eventhub EventSubscriptionWebhookEndpointPazure:eventhub/EventSubscriptionWebhookEndpoint:EventSubscriptionWebhookEndpointôA `webhook_endpoint` block as defined below.

> **NOTE:** One of `eventhub_endpoint_id`, `hybrid_connection_endpoint_id`, `service_bus_queue_endpoint_id`, `service_bus_topic_endpoint_id`, `storage_queue_endpoint`, `webhook_endpoint` or `azure_function_endpoint` must be specified.
*≠C
ì
eventhub'EventhubNamespaceDisasterRecoveryConfig^azure:eventhub/eventhubNamespaceDisasterRecoveryConfig:EventhubNamespaceDisasterRecoveryConfig∂:Manages an Disaster Recovery Config for an Event Hub Namespace.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "eventhub-replication",
    location: "West Europe",
});
const primary = new azure.eventhub.EventHubNamespace("primary", {
    name: "eventhub-primary",
    location: example.location,
    resourceGroupName: example.name,
    sku: "Standard",
});
const secondary = new azure.eventhub.EventHubNamespace("secondary", {
    name: "eventhub-secondary",
    location: example.location,
    resourceGroupName: example.name,
    sku: "Standard",
});
const exampleEventhubNamespaceDisasterRecoveryConfig = new azure.eventhub.EventhubNamespaceDisasterRecoveryConfig("example", {
    name: "replicate-eventhub",
    resourceGroupName: example.name,
    namespaceName: primary.name,
    partnerNamespaceId: secondary.id,
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="eventhub-replication",
    location="West Europe")
primary = azure.eventhub.EventHubNamespace("primary",
    name="eventhub-primary",
    location=example.location,
    resource_group_name=example.name,
    sku="Standard")
secondary = azure.eventhub.EventHubNamespace("secondary",
    name="eventhub-secondary",
    location=example.location,
    resource_group_name=example.name,
    sku="Standard")
example_eventhub_namespace_disaster_recovery_config = azure.eventhub.EventhubNamespaceDisasterRecoveryConfig("example",
    name="replicate-eventhub",
    resource_group_name=example.name,
    namespace_name=primary.name,
    partner_namespace_id=secondary.id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "eventhub-replication",
        Location = "West Europe",
    });

    var primary = new Azure.EventHub.EventHubNamespace("primary", new()
    {
        Name = "eventhub-primary",
        Location = example.Location,
        ResourceGroupName = example.Name,
        Sku = "Standard",
    });

    var secondary = new Azure.EventHub.EventHubNamespace("secondary", new()
    {
        Name = "eventhub-secondary",
        Location = example.Location,
        ResourceGroupName = example.Name,
        Sku = "Standard",
    });

    var exampleEventhubNamespaceDisasterRecoveryConfig = new Azure.EventHub.EventhubNamespaceDisasterRecoveryConfig("example", new()
    {
        Name = "replicate-eventhub",
        ResourceGroupName = example.Name,
        NamespaceName = primary.Name,
        PartnerNamespaceId = secondary.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("eventhub-replication"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		primary, err := eventhub.NewEventHubNamespace(ctx, "primary", &eventhub.EventHubNamespaceArgs{
			Name:              pulumi.String("eventhub-primary"),
			Location:          example.Location,
			ResourceGroupName: example.Name,
			Sku:               pulumi.String("Standard"),
		})
		if err != nil {
			return err
		}
		secondary, err := eventhub.NewEventHubNamespace(ctx, "secondary", &eventhub.EventHubNamespaceArgs{
			Name:              pulumi.String("eventhub-secondary"),
			Location:          example.Location,
			ResourceGroupName: example.Name,
			Sku:               pulumi.String("Standard"),
		})
		if err != nil {
			return err
		}
		_, err = eventhub.NewEventhubNamespaceDisasterRecoveryConfig(ctx, "example", &eventhub.EventhubNamespaceDisasterRecoveryConfigArgs{
			Name:               pulumi.String("replicate-eventhub"),
			ResourceGroupName:  example.Name,
			NamespaceName:      primary.Name,
			PartnerNamespaceId: secondary.ID(),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.eventhub.EventHubNamespace;
import com.pulumi.azure.eventhub.EventHubNamespaceArgs;
import com.pulumi.azure.eventhub.EventhubNamespaceDisasterRecoveryConfig;
import com.pulumi.azure.eventhub.EventhubNamespaceDisasterRecoveryConfigArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("eventhub-replication")
            .location("West Europe")
            .build());

        var primary = new EventHubNamespace("primary", EventHubNamespaceArgs.builder()
            .name("eventhub-primary")
            .location(example.location())
            .resourceGroupName(example.name())
            .sku("Standard")
            .build());

        var secondary = new EventHubNamespace("secondary", EventHubNamespaceArgs.builder()
            .name("eventhub-secondary")
            .location(example.location())
            .resourceGroupName(example.name())
            .sku("Standard")
            .build());

        var exampleEventhubNamespaceDisasterRecoveryConfig = new EventhubNamespaceDisasterRecoveryConfig("exampleEventhubNamespaceDisasterRecoveryConfig", EventhubNamespaceDisasterRecoveryConfigArgs.builder()
            .name("replicate-eventhub")
            .resourceGroupName(example.name())
            .namespaceName(primary.name())
            .partnerNamespaceId(secondary.id())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: eventhub-replication
      location: West Europe
  primary:
    type: azure:eventhub:EventHubNamespace
    properties:
      name: eventhub-primary
      location: ${example.location}
      resourceGroupName: ${example.name}
      sku: Standard
  secondary:
    type: azure:eventhub:EventHubNamespace
    properties:
      name: eventhub-secondary
      location: ${example.location}
      resourceGroupName: ${example.name}
      sku: Standard
  exampleEventhubNamespaceDisasterRecoveryConfig:
    type: azure:eventhub:EventhubNamespaceDisasterRecoveryConfig
    name: example
    properties:
      name: replicate-eventhub
      resourceGroupName: ${example.name}
      namespaceName: ${primary.name}
      partnerNamespaceId: ${secondary.id}
```
<!--End PulumiCodeChooser -->

## Import

EventHubs can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:eventhub/eventhubNamespaceDisasterRecoveryConfig:EventhubNamespaceDisasterRecoveryConfig config1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventHub/namespaces/namespace1/disasterRecoveryConfigs/config1
```

u
nameB" gSpecifies the name of the Disaster Recovery Config. Changing this forces a new resource to be created.
ã
namespaceName" vSpecifies the name of the primary EventHub Namespace to replicate. Changing this forces a new resource to be created.
L
partnerNamespaceId" 2The ID of the EventHub Namespace to replicate to.
ö
resourceGroupName" ÄThe name of the resource group in which the Disaster Recovery Config exists. Changing this forces a new resource to be created.
"s
name" gSpecifies the name of the Disaster Recovery Config. Changing this forces a new resource to be created.
"ã
namespaceName" vSpecifies the name of the primary EventHub Namespace to replicate. Changing this forces a new resource to be created.
"L
partnerNamespaceId" 2The ID of the EventHub Namespace to replicate to.
"ö
resourceGroupName" ÄThe name of the resource group in which the Disaster Recovery Config exists. Changing this forces a new resource to be created.
*¶J
9
eventhub	Namespace"azure:eventhub/namespace:Namespaceó!Manages a ServiceBus Namespace.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "my-servicebus",
    location: "West Europe",
});
const exampleNamespace = new azure.servicebus.Namespace("example", {
    name: "tfex-servicebus-namespace",
    location: example.location,
    resourceGroupName: example.name,
    sku: "Standard",
    tags: {
        source: "example",
    },
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="my-servicebus",
    location="West Europe")
example_namespace = azure.servicebus.Namespace("example",
    name="tfex-servicebus-namespace",
    location=example.location,
    resource_group_name=example.name,
    sku="Standard",
    tags={
        "source": "example",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "my-servicebus",
        Location = "West Europe",
    });

    var exampleNamespace = new Azure.ServiceBus.Namespace("example", new()
    {
        Name = "tfex-servicebus-namespace",
        Location = example.Location,
        ResourceGroupName = example.Name,
        Sku = "Standard",
        Tags = 
        {
            { "source", "example" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/servicebus"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("my-servicebus"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		_, err = servicebus.NewNamespace(ctx, "example", &servicebus.NamespaceArgs{
			Name:              pulumi.String("tfex-servicebus-namespace"),
			Location:          example.Location,
			ResourceGroupName: example.Name,
			Sku:               pulumi.String("Standard"),
			Tags: pulumi.StringMap{
				"source": pulumi.String("example"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.servicebus.Namespace;
import com.pulumi.azure.servicebus.NamespaceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("my-servicebus")
            .location("West Europe")
            .build());

        var exampleNamespace = new Namespace("exampleNamespace", NamespaceArgs.builder()
            .name("tfex-servicebus-namespace")
            .location(example.location())
            .resourceGroupName(example.name())
            .sku("Standard")
            .tags(Map.of("source", "example"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: my-servicebus
      location: West Europe
  exampleNamespace:
    type: azure:servicebus:Namespace
    name: example
    properties:
      name: tfex-servicebus-namespace
      location: ${example.location}
      resourceGroupName: ${example.name}
      sku: Standard
      tags:
        source: example
```
<!--End PulumiCodeChooser -->

## Import

Service Bus Namespace can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:eventhub/namespace:Namespace example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ServiceBus/namespaces/sbns1
```

Ø
capacityB úSpecifies the capacity. When `sku` is `Premium`, capacity can be `1`, `2`, `4`, `8` or `16`. When `sku` is `Basic` or `Standard`, capacity can be `0` only.
ø
customerManagedKeyuBs:q
o
eventhubNamespaceCustomerManagedKeyFazure:eventhub/NamespaceCustomerManagedKey:NamespaceCustomerManagedKey2An `customer_managed_key` block as defined below.
ã
identityWBU:S
Q
eventhubNamespaceIdentity2azure:eventhub/NamespaceIdentity:NamespaceIdentity&An `identity` block as defined below.
z
localAuthEnabledB
 `Whether or not SAS authentication is enabled for the Service Bus namespace. Defaults to `true`.
á
locationB" uSpecifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
ö
minimumTlsVersionB" The minimum supported TLS version for this Service Bus Namespace. Valid values are: `1.0`, `1.1` and `1.2`. Defaults to `1.2`.
{
nameB" mSpecifies the name of the ServiceBus Namespace resource . Changing this forces a new resource to be created.
´
networkRuleSetiBg:e
c
eventhubNamespaceNetworkRuleSet>azure:eventhub/NamespaceNetworkRuleSet:NamespaceNetworkRuleSet.An `network_rule_set` block as defined below.
–
premiumMessagingPartitionsB ´Specifies the number messaging partitions. Only valid when `sku` is `Premium` and the minimum number is `1`. Possible values include `0`, `1`, `2`, and `4`. Defaults to `0` for Standard, Basic namespace. Changing this forces a new resource to be created.

> **Note:** It's not possible to change the partitioning option on any existing namespace. The number of partitions can only be set during namespace creation. Please check the doc https://learn.microsoft.com/en-us/azure/service-bus-messaging/enable-partitions-premium for more feature restrictions.
x
publicNetworkAccessEnabledB
 TIs public network access enabled for the Service Bus Namespace? Defaults to `true`.
ç
resourceGroupName" tThe name of the resource group in which to Changing this forces a new resource to be created.
create the namespace.
±
sku" •Defines which tier to use. Options are `Basic`, `Standard` or `Premium`. Please note that setting this field to `Premium` will force the creation of a new resource.
=
tagsB2" -A mapping of tags to assign to the resource.
"Ø
capacityB úSpecifies the capacity. When `sku` is `Premium`, capacity can be `1`, `2`, `4`, `8` or `16`. When `sku` is `Basic` or `Standard`, capacity can be `0` only.
"ø
customerManagedKeyuBs:q
o
eventhubNamespaceCustomerManagedKeyFazure:eventhub/NamespaceCustomerManagedKey:NamespaceCustomerManagedKey2An `customer_managed_key` block as defined below.
"|
defaultPrimaryConnectionString" VThe primary connection string for the authorization rule `RootManageSharedAccessKey`.
"h
defaultPrimaryKey" OThe primary access key for the authorization rule `RootManageSharedAccessKey`.
"Ä
 defaultSecondaryConnectionString" XThe secondary connection string for the authorization rule `RootManageSharedAccessKey`.
"l
defaultSecondaryKey" QThe secondary access key for the authorization rule `RootManageSharedAccessKey`.
"<
endpoint" ,The URL to access the ServiceBus Namespace.
"ã
identityWBU:S
Q
eventhubNamespaceIdentity2azure:eventhub/NamespaceIdentity:NamespaceIdentity&An `identity` block as defined below.
"z
localAuthEnabledB
 `Whether or not SAS authentication is enabled for the Service Bus namespace. Defaults to `true`.
"Ö
location" uSpecifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
"ö
minimumTlsVersionB" The minimum supported TLS version for this Service Bus Namespace. Valid values are: `1.0`, `1.1` and `1.2`. Defaults to `1.2`.
"y
name" mSpecifies the name of the ServiceBus Namespace resource . Changing this forces a new resource to be created.
"©
networkRuleSetg:e
c
eventhubNamespaceNetworkRuleSet>azure:eventhub/NamespaceNetworkRuleSet:NamespaceNetworkRuleSet.An `network_rule_set` block as defined below.
"–
premiumMessagingPartitionsB ´Specifies the number messaging partitions. Only valid when `sku` is `Premium` and the minimum number is `1`. Possible values include `0`, `1`, `2`, and `4`. Defaults to `0` for Standard, Basic namespace. Changing this forces a new resource to be created.

> **Note:** It's not possible to change the partitioning option on any existing namespace. The number of partitions can only be set during namespace creation. Please check the doc https://learn.microsoft.com/en-us/azure/service-bus-messaging/enable-partitions-premium for more feature restrictions.
"x
publicNetworkAccessEnabledB
 TIs public network access enabled for the Service Bus Namespace? Defaults to `true`.
"ç
resourceGroupName" tThe name of the resource group in which to Changing this forces a new resource to be created.
create the namespace.
"±
sku" •Defines which tier to use. Options are `Basic`, `Standard` or `Premium`. Please note that setting this field to `Premium` will force the creation of a new resource.
"=
tagsB2" -A mapping of tags to assign to the resource.
*¶@
l
eventhubNamespaceAuthorizationRuleDazure:eventhub/namespaceAuthorizationRule:NamespaceAuthorizationRule‡/Manages a ServiceBus Namespace authorization Rule within a ServiceBus.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "my-servicebus",
    location: "West US",
});
const exampleNamespace = new azure.servicebus.Namespace("example", {
    name: "tfex-servicebus-namespace",
    location: example.location,
    resourceGroupName: example.name,
    sku: "Standard",
    tags: {
        source: "example",
    },
});
const exampleNamespaceAuthorizationRule = new azure.servicebus.NamespaceAuthorizationRule("example", {
    name: "examplerule",
    namespaceId: exampleNamespace.id,
    listen: true,
    send: true,
    manage: false,
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="my-servicebus",
    location="West US")
example_namespace = azure.servicebus.Namespace("example",
    name="tfex-servicebus-namespace",
    location=example.location,
    resource_group_name=example.name,
    sku="Standard",
    tags={
        "source": "example",
    })
example_namespace_authorization_rule = azure.servicebus.NamespaceAuthorizationRule("example",
    name="examplerule",
    namespace_id=example_namespace.id,
    listen=True,
    send=True,
    manage=False)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "my-servicebus",
        Location = "West US",
    });

    var exampleNamespace = new Azure.ServiceBus.Namespace("example", new()
    {
        Name = "tfex-servicebus-namespace",
        Location = example.Location,
        ResourceGroupName = example.Name,
        Sku = "Standard",
        Tags = 
        {
            { "source", "example" },
        },
    });

    var exampleNamespaceAuthorizationRule = new Azure.ServiceBus.NamespaceAuthorizationRule("example", new()
    {
        Name = "examplerule",
        NamespaceId = exampleNamespace.Id,
        Listen = true,
        Send = true,
        Manage = false,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/servicebus"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("my-servicebus"),
			Location: pulumi.String("West US"),
		})
		if err != nil {
			return err
		}
		exampleNamespace, err := servicebus.NewNamespace(ctx, "example", &servicebus.NamespaceArgs{
			Name:              pulumi.String("tfex-servicebus-namespace"),
			Location:          example.Location,
			ResourceGroupName: example.Name,
			Sku:               pulumi.String("Standard"),
			Tags: pulumi.StringMap{
				"source": pulumi.String("example"),
			},
		})
		if err != nil {
			return err
		}
		_, err = servicebus.NewNamespaceAuthorizationRule(ctx, "example", &servicebus.NamespaceAuthorizationRuleArgs{
			Name:        pulumi.String("examplerule"),
			NamespaceId: exampleNamespace.ID(),
			Listen:      pulumi.Bool(true),
			Send:        pulumi.Bool(true),
			Manage:      pulumi.Bool(false),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.servicebus.Namespace;
import com.pulumi.azure.servicebus.NamespaceArgs;
import com.pulumi.azure.servicebus.NamespaceAuthorizationRule;
import com.pulumi.azure.servicebus.NamespaceAuthorizationRuleArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("my-servicebus")
            .location("West US")
            .build());

        var exampleNamespace = new Namespace("exampleNamespace", NamespaceArgs.builder()
            .name("tfex-servicebus-namespace")
            .location(example.location())
            .resourceGroupName(example.name())
            .sku("Standard")
            .tags(Map.of("source", "example"))
            .build());

        var exampleNamespaceAuthorizationRule = new NamespaceAuthorizationRule("exampleNamespaceAuthorizationRule", NamespaceAuthorizationRuleArgs.builder()
            .name("examplerule")
            .namespaceId(exampleNamespace.id())
            .listen(true)
            .send(true)
            .manage(false)
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: my-servicebus
      location: West US
  exampleNamespace:
    type: azure:servicebus:Namespace
    name: example
    properties:
      name: tfex-servicebus-namespace
      location: ${example.location}
      resourceGroupName: ${example.name}
      sku: Standard
      tags:
        source: example
  exampleNamespaceAuthorizationRule:
    type: azure:servicebus:NamespaceAuthorizationRule
    name: example
    properties:
      name: examplerule
      namespaceId: ${exampleNamespace.id}
      listen: true
      send: true
      manage: false
```
<!--End PulumiCodeChooser -->

## Import

ServiceBus Namespace authorization rules can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:eventhub/namespaceAuthorizationRule:NamespaceAuthorizationRule rule1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ServiceBus/namespaces/namespace1/authorizationRules/rule1
```

[
listenB
 KGrants listen access to this this Authorization Rule. Defaults to `false`.
°
manageB
 êGrants manage access to this this Authorization Rule. When this property is `true` - both `listen` and `send` must be too. Defaults to `false`.
ç
nameB" Specifies the name of the ServiceBus Namespace Authorization Rule resource. Changing this forces a new resource to be created.
∫
namespaceId" ¶Specifies the ID of the ServiceBus Namespace. Changing this forces a new resource to be created.

> **NOTE** At least one of the 3 permissions below needs to be set.
W
sendB
 IGrants send access to this this Authorization Rule. Defaults to `false`.
"[
listenB
 KGrants listen access to this this Authorization Rule. Defaults to `false`.
"°
manageB
 êGrants manage access to this this Authorization Rule. When this property is `true` - both `listen` and `send` must be too. Defaults to `false`.
"ã
name" Specifies the name of the ServiceBus Namespace Authorization Rule resource. Changing this forces a new resource to be created.
"∫
namespaceId" ¶Specifies the ID of the ServiceBus Namespace. Changing this forces a new resource to be created.

> **NOTE** At least one of the 3 permissions below needs to be set.
"n
primaryConnectionString" OThe Primary Connection String for the ServiceBus Namespace authorization Rule.
"â
primaryConnectionStringAlias" eThe alias Primary Connection String for the ServiceBus Namespace, if the namespace is Geo DR paired.
"S

primaryKey" AThe Primary Key for the ServiceBus Namespace authorization Rule.
"r
secondaryConnectionString" QThe Secondary Connection String for the ServiceBus Namespace authorization Rule.
"i
secondaryConnectionStringAlias" CThe alias Secondary Connection String for the ServiceBus Namespace
"W
secondaryKey" CThe Secondary Key for the ServiceBus Namespace authorization Rule.
"W
sendB
 IGrants send access to this this Authorization Rule. Defaults to `false`.
*™ƒ
o
eventhubNamespaceCustomerManagedKeyFazure:eventhub/namespaceCustomerManagedKey:NamespaceCustomerManagedKeyµ∂Manages a Customer Managed Key for a EventHub Namespace.

!> **Note:** In 2.x versions of the Azure Provider during deletion this resource will **delete and recreate the parent EventHub Namespace which may involve data loss** as it's not possible to remove the Customer Managed Key from the EventHub Namespace once it's been added. Version 3.0 of the Azure Provider will change this so that the Delete operation is a noop, requiring the parent EventHub Namespace is deleted/recreated to remove the Customer Managed Key.

## Example Usage

### With System Assigned Identity

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "example-resources",
    location: "West Europe",
});
const exampleCluster = new azure.eventhub.Cluster("example", {
    name: "example-cluster",
    resourceGroupName: example.name,
    location: example.location,
    skuName: "Dedicated_1",
});
const exampleEventHubNamespace = new azure.eventhub.EventHubNamespace("example", {
    name: "example-namespace",
    location: example.location,
    resourceGroupName: example.name,
    sku: "Standard",
    dedicatedClusterId: exampleCluster.id,
    identity: {
        type: "SystemAssigned",
    },
});
const current = azure.core.getClientConfig({});
const exampleKeyVault = new azure.keyvault.KeyVault("example", {
    name: "examplekv",
    location: example.location,
    resourceGroupName: example.name,
    tenantId: current.then(current => current.tenantId),
    skuName: "standard",
    purgeProtectionEnabled: true,
});
const exampleAccessPolicy = new azure.keyvault.AccessPolicy("example", {
    keyVaultId: exampleKeyVault.id,
    tenantId: exampleEventHubNamespace.identity.apply(identity => identity?.tenantId),
    objectId: exampleEventHubNamespace.identity.apply(identity => identity?.principalId),
    keyPermissions: [
        "Get",
        "UnwrapKey",
        "WrapKey",
    ],
});
const example2 = new azure.keyvault.AccessPolicy("example2", {
    keyVaultId: exampleKeyVault.id,
    tenantId: current.then(current => current.tenantId),
    objectId: current.then(current => current.objectId),
    keyPermissions: [
        "Create",
        "Delete",
        "Get",
        "List",
        "Purge",
        "Recover",
        "GetRotationPolicy",
    ],
});
const exampleKey = new azure.keyvault.Key("example", {
    name: "examplekvkey",
    keyVaultId: exampleKeyVault.id,
    keyType: "RSA",
    keySize: 2048,
    keyOpts: [
        "decrypt",
        "encrypt",
        "sign",
        "unwrapKey",
        "verify",
        "wrapKey",
    ],
}, {
    dependsOn: [
        exampleAccessPolicy,
        example2,
    ],
});
const exampleNamespaceCustomerManagedKey = new azure.eventhub.NamespaceCustomerManagedKey("example", {
    eventhubNamespaceId: exampleEventHubNamespace.id,
    keyVaultKeyIds: [exampleKey.id],
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="example-resources",
    location="West Europe")
example_cluster = azure.eventhub.Cluster("example",
    name="example-cluster",
    resource_group_name=example.name,
    location=example.location,
    sku_name="Dedicated_1")
example_event_hub_namespace = azure.eventhub.EventHubNamespace("example",
    name="example-namespace",
    location=example.location,
    resource_group_name=example.name,
    sku="Standard",
    dedicated_cluster_id=example_cluster.id,
    identity={
        "type": "SystemAssigned",
    })
current = azure.core.get_client_config()
example_key_vault = azure.keyvault.KeyVault("example",
    name="examplekv",
    location=example.location,
    resource_group_name=example.name,
    tenant_id=current.tenant_id,
    sku_name="standard",
    purge_protection_enabled=True)
example_access_policy = azure.keyvault.AccessPolicy("example",
    key_vault_id=example_key_vault.id,
    tenant_id=example_event_hub_namespace.identity.tenant_id,
    object_id=example_event_hub_namespace.identity.principal_id,
    key_permissions=[
        "Get",
        "UnwrapKey",
        "WrapKey",
    ])
example2 = azure.keyvault.AccessPolicy("example2",
    key_vault_id=example_key_vault.id,
    tenant_id=current.tenant_id,
    object_id=current.object_id,
    key_permissions=[
        "Create",
        "Delete",
        "Get",
        "List",
        "Purge",
        "Recover",
        "GetRotationPolicy",
    ])
example_key = azure.keyvault.Key("example",
    name="examplekvkey",
    key_vault_id=example_key_vault.id,
    key_type="RSA",
    key_size=2048,
    key_opts=[
        "decrypt",
        "encrypt",
        "sign",
        "unwrapKey",
        "verify",
        "wrapKey",
    ],
    opts = pulumi.ResourceOptions(depends_on=[
            example_access_policy,
            example2,
        ]))
example_namespace_customer_managed_key = azure.eventhub.NamespaceCustomerManagedKey("example",
    eventhub_namespace_id=example_event_hub_namespace.id,
    key_vault_key_ids=[example_key.id])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "example-resources",
        Location = "West Europe",
    });

    var exampleCluster = new Azure.EventHub.Cluster("example", new()
    {
        Name = "example-cluster",
        ResourceGroupName = example.Name,
        Location = example.Location,
        SkuName = "Dedicated_1",
    });

    var exampleEventHubNamespace = new Azure.EventHub.EventHubNamespace("example", new()
    {
        Name = "example-namespace",
        Location = example.Location,
        ResourceGroupName = example.Name,
        Sku = "Standard",
        DedicatedClusterId = exampleCluster.Id,
        Identity = new Azure.EventHub.Inputs.EventHubNamespaceIdentityArgs
        {
            Type = "SystemAssigned",
        },
    });

    var current = Azure.Core.GetClientConfig.Invoke();

    var exampleKeyVault = new Azure.KeyVault.KeyVault("example", new()
    {
        Name = "examplekv",
        Location = example.Location,
        ResourceGroupName = example.Name,
        TenantId = current.Apply(getClientConfigResult => getClientConfigResult.TenantId),
        SkuName = "standard",
        PurgeProtectionEnabled = true,
    });

    var exampleAccessPolicy = new Azure.KeyVault.AccessPolicy("example", new()
    {
        KeyVaultId = exampleKeyVault.Id,
        TenantId = exampleEventHubNamespace.Identity.Apply(identity => identity?.TenantId),
        ObjectId = exampleEventHubNamespace.Identity.Apply(identity => identity?.PrincipalId),
        KeyPermissions = new[]
        {
            "Get",
            "UnwrapKey",
            "WrapKey",
        },
    });

    var example2 = new Azure.KeyVault.AccessPolicy("example2", new()
    {
        KeyVaultId = exampleKeyVault.Id,
        TenantId = current.Apply(getClientConfigResult => getClientConfigResult.TenantId),
        ObjectId = current.Apply(getClientConfigResult => getClientConfigResult.ObjectId),
        KeyPermissions = new[]
        {
            "Create",
            "Delete",
            "Get",
            "List",
            "Purge",
            "Recover",
            "GetRotationPolicy",
        },
    });

    var exampleKey = new Azure.KeyVault.Key("example", new()
    {
        Name = "examplekvkey",
        KeyVaultId = exampleKeyVault.Id,
        KeyType = "RSA",
        KeySize = 2048,
        KeyOpts = new[]
        {
            "decrypt",
            "encrypt",
            "sign",
            "unwrapKey",
            "verify",
            "wrapKey",
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleAccessPolicy,
            example2,
        },
    });

    var exampleNamespaceCustomerManagedKey = new Azure.EventHub.NamespaceCustomerManagedKey("example", new()
    {
        EventhubNamespaceId = exampleEventHubNamespace.Id,
        KeyVaultKeyIds = new[]
        {
            exampleKey.Id,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventhub"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/keyvault"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("example-resources"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		exampleCluster, err := eventhub.NewCluster(ctx, "example", &eventhub.ClusterArgs{
			Name:              pulumi.String("example-cluster"),
			ResourceGroupName: example.Name,
			Location:          example.Location,
			SkuName:           pulumi.String("Dedicated_1"),
		})
		if err != nil {
			return err
		}
		exampleEventHubNamespace, err := eventhub.NewEventHubNamespace(ctx, "example", &eventhub.EventHubNamespaceArgs{
			Name:               pulumi.String("example-namespace"),
			Location:           example.Location,
			ResourceGroupName:  example.Name,
			Sku:                pulumi.String("Standard"),
			DedicatedClusterId: exampleCluster.ID(),
			Identity: &eventhub.EventHubNamespaceIdentityArgs{
				Type: pulumi.String("SystemAssigned"),
			},
		})
		if err != nil {
			return err
		}
		current, err := core.GetClientConfig(ctx, map[string]interface{}{}, nil)
		if err != nil {
			return err
		}
		exampleKeyVault, err := keyvault.NewKeyVault(ctx, "example", &keyvault.KeyVaultArgs{
			Name:                   pulumi.String("examplekv"),
			Location:               example.Location,
			ResourceGroupName:      example.Name,
			TenantId:               pulumi.String(current.TenantId),
			SkuName:                pulumi.String("standard"),
			PurgeProtectionEnabled: pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		exampleAccessPolicy, err := keyvault.NewAccessPolicy(ctx, "example", &keyvault.AccessPolicyArgs{
			KeyVaultId: exampleKeyVault.ID(),
			TenantId: pulumi.String(exampleEventHubNamespace.Identity.ApplyT(func(identity eventhub.EventHubNamespaceIdentity) (*string, error) {
				return &identity.TenantId, nil
			}).(pulumi.StringPtrOutput)),
			ObjectId: pulumi.String(exampleEventHubNamespace.Identity.ApplyT(func(identity eventhub.EventHubNamespaceIdentity) (*string, error) {
				return &identity.PrincipalId, nil
			}).(pulumi.StringPtrOutput)),
			KeyPermissions: pulumi.StringArray{
				pulumi.String("Get"),
				pulumi.String("UnwrapKey"),
				pulumi.String("WrapKey"),
			},
		})
		if err != nil {
			return err
		}
		example2, err := keyvault.NewAccessPolicy(ctx, "example2", &keyvault.AccessPolicyArgs{
			KeyVaultId: exampleKeyVault.ID(),
			TenantId:   pulumi.String(current.TenantId),
			ObjectId:   pulumi.String(current.ObjectId),
			KeyPermissions: pulumi.StringArray{
				pulumi.String("Create"),
				pulumi.String("Delete"),
				pulumi.String("Get"),
				pulumi.String("List"),
				pulumi.String("Purge"),
				pulumi.String("Recover"),
				pulumi.String("GetRotationPolicy"),
			},
		})
		if err != nil {
			return err
		}
		exampleKey, err := keyvault.NewKey(ctx, "example", &keyvault.KeyArgs{
			Name:       pulumi.String("examplekvkey"),
			KeyVaultId: exampleKeyVault.ID(),
			KeyType:    pulumi.String("RSA"),
			KeySize:    pulumi.Int(2048),
			KeyOpts: pulumi.StringArray{
				pulumi.String("decrypt"),
				pulumi.String("encrypt"),
				pulumi.String("sign"),
				pulumi.String("unwrapKey"),
				pulumi.String("verify"),
				pulumi.String("wrapKey"),
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			exampleAccessPolicy,
			example2,
		}))
		if err != nil {
			return err
		}
		_, err = eventhub.NewNamespaceCustomerManagedKey(ctx, "example", &eventhub.NamespaceCustomerManagedKeyArgs{
			EventhubNamespaceId: exampleEventHubNamespace.ID(),
			KeyVaultKeyIds: pulumi.StringArray{
				exampleKey.ID(),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.eventhub.Cluster;
import com.pulumi.azure.eventhub.ClusterArgs;
import com.pulumi.azure.eventhub.EventHubNamespace;
import com.pulumi.azure.eventhub.EventHubNamespaceArgs;
import com.pulumi.azure.eventhub.inputs.EventHubNamespaceIdentityArgs;
import com.pulumi.azure.core.CoreFunctions;
import com.pulumi.azure.keyvault.KeyVault;
import com.pulumi.azure.keyvault.KeyVaultArgs;
import com.pulumi.azure.keyvault.AccessPolicy;
import com.pulumi.azure.keyvault.AccessPolicyArgs;
import com.pulumi.azure.keyvault.Key;
import com.pulumi.azure.keyvault.KeyArgs;
import com.pulumi.azure.eventhub.NamespaceCustomerManagedKey;
import com.pulumi.azure.eventhub.NamespaceCustomerManagedKeyArgs;
import com.pulumi.resources.CustomResourceOptions;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("example-resources")
            .location("West Europe")
            .build());

        var exampleCluster = new Cluster("exampleCluster", ClusterArgs.builder()
            .name("example-cluster")
            .resourceGroupName(example.name())
            .location(example.location())
            .skuName("Dedicated_1")
            .build());

        var exampleEventHubNamespace = new EventHubNamespace("exampleEventHubNamespace", EventHubNamespaceArgs.builder()
            .name("example-namespace")
            .location(example.location())
            .resourceGroupName(example.name())
            .sku("Standard")
            .dedicatedClusterId(exampleCluster.id())
            .identity(EventHubNamespaceIdentityArgs.builder()
                .type("SystemAssigned")
                .build())
            .build());

        final var current = CoreFunctions.getClientConfig();

        var exampleKeyVault = new KeyVault("exampleKeyVault", KeyVaultArgs.builder()
            .name("examplekv")
            .location(example.location())
            .resourceGroupName(example.name())
            .tenantId(current.applyValue(getClientConfigResult -> getClientConfigResult.tenantId()))
            .skuName("standard")
            .purgeProtectionEnabled(true)
            .build());

        var exampleAccessPolicy = new AccessPolicy("exampleAccessPolicy", AccessPolicyArgs.builder()
            .keyVaultId(exampleKeyVault.id())
            .tenantId(exampleEventHubNamespace.identity().applyValue(identity -> identity.tenantId()))
            .objectId(exampleEventHubNamespace.identity().applyValue(identity -> identity.principalId()))
            .keyPermissions(            
                "Get",
                "UnwrapKey",
                "WrapKey")
            .build());

        var example2 = new AccessPolicy("example2", AccessPolicyArgs.builder()
            .keyVaultId(exampleKeyVault.id())
            .tenantId(current.applyValue(getClientConfigResult -> getClientConfigResult.tenantId()))
            .objectId(current.applyValue(getClientConfigResult -> getClientConfigResult.objectId()))
            .keyPermissions(            
                "Create",
                "Delete",
                "Get",
                "List",
                "Purge",
                "Recover",
                "GetRotationPolicy")
            .build());

        var exampleKey = new Key("exampleKey", KeyArgs.builder()
            .name("examplekvkey")
            .keyVaultId(exampleKeyVault.id())
            .keyType("RSA")
            .keySize(2048)
            .keyOpts(            
                "decrypt",
                "encrypt",
                "sign",
                "unwrapKey",
                "verify",
                "wrapKey")
            .build(), CustomResourceOptions.builder()
                .dependsOn(                
                    exampleAccessPolicy,
                    example2)
                .build());

        var exampleNamespaceCustomerManagedKey = new NamespaceCustomerManagedKey("exampleNamespaceCustomerManagedKey", NamespaceCustomerManagedKeyArgs.builder()
            .eventhubNamespaceId(exampleEventHubNamespace.id())
            .keyVaultKeyIds(exampleKey.id())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: example-resources
      location: West Europe
  exampleCluster:
    type: azure:eventhub:Cluster
    name: example
    properties:
      name: example-cluster
      resourceGroupName: ${example.name}
      location: ${example.location}
      skuName: Dedicated_1
  exampleEventHubNamespace:
    type: azure:eventhub:EventHubNamespace
    name: example
    properties:
      name: example-namespace
      location: ${example.location}
      resourceGroupName: ${example.name}
      sku: Standard
      dedicatedClusterId: ${exampleCluster.id}
      identity:
        type: SystemAssigned
  exampleKeyVault:
    type: azure:keyvault:KeyVault
    name: example
    properties:
      name: examplekv
      location: ${example.location}
      resourceGroupName: ${example.name}
      tenantId: ${current.tenantId}
      skuName: standard
      purgeProtectionEnabled: true
  exampleAccessPolicy:
    type: azure:keyvault:AccessPolicy
    name: example
    properties:
      keyVaultId: ${exampleKeyVault.id}
      tenantId: ${exampleEventHubNamespace.identity.tenantId}
      objectId: ${exampleEventHubNamespace.identity.principalId}
      keyPermissions:
        - Get
        - UnwrapKey
        - WrapKey
  example2:
    type: azure:keyvault:AccessPolicy
    properties:
      keyVaultId: ${exampleKeyVault.id}
      tenantId: ${current.tenantId}
      objectId: ${current.objectId}
      keyPermissions:
        - Create
        - Delete
        - Get
        - List
        - Purge
        - Recover
        - GetRotationPolicy
  exampleKey:
    type: azure:keyvault:Key
    name: example
    properties:
      name: examplekvkey
      keyVaultId: ${exampleKeyVault.id}
      keyType: RSA
      keySize: 2048
      keyOpts:
        - decrypt
        - encrypt
        - sign
        - unwrapKey
        - verify
        - wrapKey
    options:
      dependsOn:
        - ${exampleAccessPolicy}
        - ${example2}
  exampleNamespaceCustomerManagedKey:
    type: azure:eventhub:NamespaceCustomerManagedKey
    name: example
    properties:
      eventhubNamespaceId: ${exampleEventHubNamespace.id}
      keyVaultKeyIds:
        - ${exampleKey.id}
variables:
  current:
    fn::invoke:
      function: azure:core:getClientConfig
      arguments: {}
```
<!--End PulumiCodeChooser -->


### With User Assigned Identity

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "example-resources",
    location: "West Europe",
});
const exampleCluster = new azure.eventhub.Cluster("example", {
    name: "example-cluster",
    resourceGroupName: example.name,
    location: example.location,
    skuName: "Dedicated_1",
});
const exampleUserAssignedIdentity = new azure.authorization.UserAssignedIdentity("example", {
    location: example.location,
    name: "example",
    resourceGroupName: example.name,
});
const exampleEventHubNamespace = new azure.eventhub.EventHubNamespace("example", {
    name: "example-namespace",
    location: example.location,
    resourceGroupName: example.name,
    sku: "Standard",
    dedicatedClusterId: exampleCluster.id,
    identity: {
        type: "UserAssigned",
        identityIds: [exampleUserAssignedIdentity.id],
    },
});
const current = azure.core.getClientConfig({});
const exampleKeyVault = new azure.keyvault.KeyVault("example", {
    name: "examplekv",
    location: example.location,
    resourceGroupName: example.name,
    tenantId: current.then(current => current.tenantId),
    skuName: "standard",
    purgeProtectionEnabled: true,
});
const exampleAccessPolicy = new azure.keyvault.AccessPolicy("example", {
    keyVaultId: exampleKeyVault.id,
    tenantId: test.tenantId,
    objectId: test.principalId,
    keyPermissions: [
        "Get",
        "UnwrapKey",
        "WrapKey",
    ],
});
const example2 = new azure.keyvault.AccessPolicy("example2", {
    keyVaultId: exampleKeyVault.id,
    tenantId: current.then(current => current.tenantId),
    objectId: current.then(current => current.objectId),
    keyPermissions: [
        "Create",
        "Delete",
        "Get",
        "List",
        "Purge",
        "Recover",
        "GetRotationPolicy",
    ],
});
const exampleKey = new azure.keyvault.Key("example", {
    name: "examplekvkey",
    keyVaultId: exampleKeyVault.id,
    keyType: "RSA",
    keySize: 2048,
    keyOpts: [
        "decrypt",
        "encrypt",
        "sign",
        "unwrapKey",
        "verify",
        "wrapKey",
    ],
}, {
    dependsOn: [
        exampleAccessPolicy,
        example2,
    ],
});
const exampleNamespaceCustomerManagedKey = new azure.eventhub.NamespaceCustomerManagedKey("example", {
    eventhubNamespaceId: exampleEventHubNamespace.id,
    keyVaultKeyIds: [exampleKey.id],
    userAssignedIdentityId: exampleUserAssignedIdentity.id,
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="example-resources",
    location="West Europe")
example_cluster = azure.eventhub.Cluster("example",
    name="example-cluster",
    resource_group_name=example.name,
    location=example.location,
    sku_name="Dedicated_1")
example_user_assigned_identity = azure.authorization.UserAssignedIdentity("example",
    location=example.location,
    name="example",
    resource_group_name=example.name)
example_event_hub_namespace = azure.eventhub.EventHubNamespace("example",
    name="example-namespace",
    location=example.location,
    resource_group_name=example.name,
    sku="Standard",
    dedicated_cluster_id=example_cluster.id,
    identity={
        "type": "UserAssigned",
        "identity_ids": [example_user_assigned_identity.id],
    })
current = azure.core.get_client_config()
example_key_vault = azure.keyvault.KeyVault("example",
    name="examplekv",
    location=example.location,
    resource_group_name=example.name,
    tenant_id=current.tenant_id,
    sku_name="standard",
    purge_protection_enabled=True)
example_access_policy = azure.keyvault.AccessPolicy("example",
    key_vault_id=example_key_vault.id,
    tenant_id=test["tenantId"],
    object_id=test["principalId"],
    key_permissions=[
        "Get",
        "UnwrapKey",
        "WrapKey",
    ])
example2 = azure.keyvault.AccessPolicy("example2",
    key_vault_id=example_key_vault.id,
    tenant_id=current.tenant_id,
    object_id=current.object_id,
    key_permissions=[
        "Create",
        "Delete",
        "Get",
        "List",
        "Purge",
        "Recover",
        "GetRotationPolicy",
    ])
example_key = azure.keyvault.Key("example",
    name="examplekvkey",
    key_vault_id=example_key_vault.id,
    key_type="RSA",
    key_size=2048,
    key_opts=[
        "decrypt",
        "encrypt",
        "sign",
        "unwrapKey",
        "verify",
        "wrapKey",
    ],
    opts = pulumi.ResourceOptions(depends_on=[
            example_access_policy,
            example2,
        ]))
example_namespace_customer_managed_key = azure.eventhub.NamespaceCustomerManagedKey("example",
    eventhub_namespace_id=example_event_hub_namespace.id,
    key_vault_key_ids=[example_key.id],
    user_assigned_identity_id=example_user_assigned_identity.id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "example-resources",
        Location = "West Europe",
    });

    var exampleCluster = new Azure.EventHub.Cluster("example", new()
    {
        Name = "example-cluster",
        ResourceGroupName = example.Name,
        Location = example.Location,
        SkuName = "Dedicated_1",
    });

    var exampleUserAssignedIdentity = new Azure.Authorization.UserAssignedIdentity("example", new()
    {
        Location = example.Location,
        Name = "example",
        ResourceGroupName = example.Name,
    });

    var exampleEventHubNamespace = new Azure.EventHub.EventHubNamespace("example", new()
    {
        Name = "example-namespace",
        Location = example.Location,
        ResourceGroupName = example.Name,
        Sku = "Standard",
        DedicatedClusterId = exampleCluster.Id,
        Identity = new Azure.EventHub.Inputs.EventHubNamespaceIdentityArgs
        {
            Type = "UserAssigned",
            IdentityIds = new[]
            {
                exampleUserAssignedIdentity.Id,
            },
        },
    });

    var current = Azure.Core.GetClientConfig.Invoke();

    var exampleKeyVault = new Azure.KeyVault.KeyVault("example", new()
    {
        Name = "examplekv",
        Location = example.Location,
        ResourceGroupName = example.Name,
        TenantId = current.Apply(getClientConfigResult => getClientConfigResult.TenantId),
        SkuName = "standard",
        PurgeProtectionEnabled = true,
    });

    var exampleAccessPolicy = new Azure.KeyVault.AccessPolicy("example", new()
    {
        KeyVaultId = exampleKeyVault.Id,
        TenantId = test.TenantId,
        ObjectId = test.PrincipalId,
        KeyPermissions = new[]
        {
            "Get",
            "UnwrapKey",
            "WrapKey",
        },
    });

    var example2 = new Azure.KeyVault.AccessPolicy("example2", new()
    {
        KeyVaultId = exampleKeyVault.Id,
        TenantId = current.Apply(getClientConfigResult => getClientConfigResult.TenantId),
        ObjectId = current.Apply(getClientConfigResult => getClientConfigResult.ObjectId),
        KeyPermissions = new[]
        {
            "Create",
            "Delete",
            "Get",
            "List",
            "Purge",
            "Recover",
            "GetRotationPolicy",
        },
    });

    var exampleKey = new Azure.KeyVault.Key("example", new()
    {
        Name = "examplekvkey",
        KeyVaultId = exampleKeyVault.Id,
        KeyType = "RSA",
        KeySize = 2048,
        KeyOpts = new[]
        {
            "decrypt",
            "encrypt",
            "sign",
            "unwrapKey",
            "verify",
            "wrapKey",
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleAccessPolicy,
            example2,
        },
    });

    var exampleNamespaceCustomerManagedKey = new Azure.EventHub.NamespaceCustomerManagedKey("example", new()
    {
        EventhubNamespaceId = exampleEventHubNamespace.Id,
        KeyVaultKeyIds = new[]
        {
            exampleKey.Id,
        },
        UserAssignedIdentityId = exampleUserAssignedIdentity.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/authorization"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventhub"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/keyvault"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("example-resources"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		exampleCluster, err := eventhub.NewCluster(ctx, "example", &eventhub.ClusterArgs{
			Name:              pulumi.String("example-cluster"),
			ResourceGroupName: example.Name,
			Location:          example.Location,
			SkuName:           pulumi.String("Dedicated_1"),
		})
		if err != nil {
			return err
		}
		exampleUserAssignedIdentity, err := authorization.NewUserAssignedIdentity(ctx, "example", &authorization.UserAssignedIdentityArgs{
			Location:          example.Location,
			Name:              pulumi.String("example"),
			ResourceGroupName: example.Name,
		})
		if err != nil {
			return err
		}
		exampleEventHubNamespace, err := eventhub.NewEventHubNamespace(ctx, "example", &eventhub.EventHubNamespaceArgs{
			Name:               pulumi.String("example-namespace"),
			Location:           example.Location,
			ResourceGroupName:  example.Name,
			Sku:                pulumi.String("Standard"),
			DedicatedClusterId: exampleCluster.ID(),
			Identity: &eventhub.EventHubNamespaceIdentityArgs{
				Type: pulumi.String("UserAssigned"),
				IdentityIds: pulumi.StringArray{
					exampleUserAssignedIdentity.ID(),
				},
			},
		})
		if err != nil {
			return err
		}
		current, err := core.GetClientConfig(ctx, map[string]interface{}{}, nil)
		if err != nil {
			return err
		}
		exampleKeyVault, err := keyvault.NewKeyVault(ctx, "example", &keyvault.KeyVaultArgs{
			Name:                   pulumi.String("examplekv"),
			Location:               example.Location,
			ResourceGroupName:      example.Name,
			TenantId:               pulumi.String(current.TenantId),
			SkuName:                pulumi.String("standard"),
			PurgeProtectionEnabled: pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		exampleAccessPolicy, err := keyvault.NewAccessPolicy(ctx, "example", &keyvault.AccessPolicyArgs{
			KeyVaultId: exampleKeyVault.ID(),
			TenantId:   pulumi.Any(test.TenantId),
			ObjectId:   pulumi.Any(test.PrincipalId),
			KeyPermissions: pulumi.StringArray{
				pulumi.String("Get"),
				pulumi.String("UnwrapKey"),
				pulumi.String("WrapKey"),
			},
		})
		if err != nil {
			return err
		}
		example2, err := keyvault.NewAccessPolicy(ctx, "example2", &keyvault.AccessPolicyArgs{
			KeyVaultId: exampleKeyVault.ID(),
			TenantId:   pulumi.String(current.TenantId),
			ObjectId:   pulumi.String(current.ObjectId),
			KeyPermissions: pulumi.StringArray{
				pulumi.String("Create"),
				pulumi.String("Delete"),
				pulumi.String("Get"),
				pulumi.String("List"),
				pulumi.String("Purge"),
				pulumi.String("Recover"),
				pulumi.String("GetRotationPolicy"),
			},
		})
		if err != nil {
			return err
		}
		exampleKey, err := keyvault.NewKey(ctx, "example", &keyvault.KeyArgs{
			Name:       pulumi.String("examplekvkey"),
			KeyVaultId: exampleKeyVault.ID(),
			KeyType:    pulumi.String("RSA"),
			KeySize:    pulumi.Int(2048),
			KeyOpts: pulumi.StringArray{
				pulumi.String("decrypt"),
				pulumi.String("encrypt"),
				pulumi.String("sign"),
				pulumi.String("unwrapKey"),
				pulumi.String("verify"),
				pulumi.String("wrapKey"),
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			exampleAccessPolicy,
			example2,
		}))
		if err != nil {
			return err
		}
		_, err = eventhub.NewNamespaceCustomerManagedKey(ctx, "example", &eventhub.NamespaceCustomerManagedKeyArgs{
			EventhubNamespaceId: exampleEventHubNamespace.ID(),
			KeyVaultKeyIds: pulumi.StringArray{
				exampleKey.ID(),
			},
			UserAssignedIdentityId: exampleUserAssignedIdentity.ID(),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.eventhub.Cluster;
import com.pulumi.azure.eventhub.ClusterArgs;
import com.pulumi.azure.authorization.UserAssignedIdentity;
import com.pulumi.azure.authorization.UserAssignedIdentityArgs;
import com.pulumi.azure.eventhub.EventHubNamespace;
import com.pulumi.azure.eventhub.EventHubNamespaceArgs;
import com.pulumi.azure.eventhub.inputs.EventHubNamespaceIdentityArgs;
import com.pulumi.azure.core.CoreFunctions;
import com.pulumi.azure.keyvault.KeyVault;
import com.pulumi.azure.keyvault.KeyVaultArgs;
import com.pulumi.azure.keyvault.AccessPolicy;
import com.pulumi.azure.keyvault.AccessPolicyArgs;
import com.pulumi.azure.keyvault.Key;
import com.pulumi.azure.keyvault.KeyArgs;
import com.pulumi.azure.eventhub.NamespaceCustomerManagedKey;
import com.pulumi.azure.eventhub.NamespaceCustomerManagedKeyArgs;
import com.pulumi.resources.CustomResourceOptions;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("example-resources")
            .location("West Europe")
            .build());

        var exampleCluster = new Cluster("exampleCluster", ClusterArgs.builder()
            .name("example-cluster")
            .resourceGroupName(example.name())
            .location(example.location())
            .skuName("Dedicated_1")
            .build());

        var exampleUserAssignedIdentity = new UserAssignedIdentity("exampleUserAssignedIdentity", UserAssignedIdentityArgs.builder()
            .location(example.location())
            .name("example")
            .resourceGroupName(example.name())
            .build());

        var exampleEventHubNamespace = new EventHubNamespace("exampleEventHubNamespace", EventHubNamespaceArgs.builder()
            .name("example-namespace")
            .location(example.location())
            .resourceGroupName(example.name())
            .sku("Standard")
            .dedicatedClusterId(exampleCluster.id())
            .identity(EventHubNamespaceIdentityArgs.builder()
                .type("UserAssigned")
                .identityIds(exampleUserAssignedIdentity.id())
                .build())
            .build());

        final var current = CoreFunctions.getClientConfig();

        var exampleKeyVault = new KeyVault("exampleKeyVault", KeyVaultArgs.builder()
            .name("examplekv")
            .location(example.location())
            .resourceGroupName(example.name())
            .tenantId(current.applyValue(getClientConfigResult -> getClientConfigResult.tenantId()))
            .skuName("standard")
            .purgeProtectionEnabled(true)
            .build());

        var exampleAccessPolicy = new AccessPolicy("exampleAccessPolicy", AccessPolicyArgs.builder()
            .keyVaultId(exampleKeyVault.id())
            .tenantId(test.tenantId())
            .objectId(test.principalId())
            .keyPermissions(            
                "Get",
                "UnwrapKey",
                "WrapKey")
            .build());

        var example2 = new AccessPolicy("example2", AccessPolicyArgs.builder()
            .keyVaultId(exampleKeyVault.id())
            .tenantId(current.applyValue(getClientConfigResult -> getClientConfigResult.tenantId()))
            .objectId(current.applyValue(getClientConfigResult -> getClientConfigResult.objectId()))
            .keyPermissions(            
                "Create",
                "Delete",
                "Get",
                "List",
                "Purge",
                "Recover",
                "GetRotationPolicy")
            .build());

        var exampleKey = new Key("exampleKey", KeyArgs.builder()
            .name("examplekvkey")
            .keyVaultId(exampleKeyVault.id())
            .keyType("RSA")
            .keySize(2048)
            .keyOpts(            
                "decrypt",
                "encrypt",
                "sign",
                "unwrapKey",
                "verify",
                "wrapKey")
            .build(), CustomResourceOptions.builder()
                .dependsOn(                
                    exampleAccessPolicy,
                    example2)
                .build());

        var exampleNamespaceCustomerManagedKey = new NamespaceCustomerManagedKey("exampleNamespaceCustomerManagedKey", NamespaceCustomerManagedKeyArgs.builder()
            .eventhubNamespaceId(exampleEventHubNamespace.id())
            .keyVaultKeyIds(exampleKey.id())
            .userAssignedIdentityId(exampleUserAssignedIdentity.id())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: example-resources
      location: West Europe
  exampleCluster:
    type: azure:eventhub:Cluster
    name: example
    properties:
      name: example-cluster
      resourceGroupName: ${example.name}
      location: ${example.location}
      skuName: Dedicated_1
  exampleUserAssignedIdentity:
    type: azure:authorization:UserAssignedIdentity
    name: example
    properties:
      location: ${example.location}
      name: example
      resourceGroupName: ${example.name}
  exampleEventHubNamespace:
    type: azure:eventhub:EventHubNamespace
    name: example
    properties:
      name: example-namespace
      location: ${example.location}
      resourceGroupName: ${example.name}
      sku: Standard
      dedicatedClusterId: ${exampleCluster.id}
      identity:
        type: UserAssigned
        identityIds:
          - ${exampleUserAssignedIdentity.id}
  exampleKeyVault:
    type: azure:keyvault:KeyVault
    name: example
    properties:
      name: examplekv
      location: ${example.location}
      resourceGroupName: ${example.name}
      tenantId: ${current.tenantId}
      skuName: standard
      purgeProtectionEnabled: true
  exampleAccessPolicy:
    type: azure:keyvault:AccessPolicy
    name: example
    properties:
      keyVaultId: ${exampleKeyVault.id}
      tenantId: ${test.tenantId}
      objectId: ${test.principalId}
      keyPermissions:
        - Get
        - UnwrapKey
        - WrapKey
  example2:
    type: azure:keyvault:AccessPolicy
    properties:
      keyVaultId: ${exampleKeyVault.id}
      tenantId: ${current.tenantId}
      objectId: ${current.objectId}
      keyPermissions:
        - Create
        - Delete
        - Get
        - List
        - Purge
        - Recover
        - GetRotationPolicy
  exampleKey:
    type: azure:keyvault:Key
    name: example
    properties:
      name: examplekvkey
      keyVaultId: ${exampleKeyVault.id}
      keyType: RSA
      keySize: 2048
      keyOpts:
        - decrypt
        - encrypt
        - sign
        - unwrapKey
        - verify
        - wrapKey
    options:
      dependsOn:
        - ${exampleAccessPolicy}
        - ${example2}
  exampleNamespaceCustomerManagedKey:
    type: azure:eventhub:NamespaceCustomerManagedKey
    name: example
    properties:
      eventhubNamespaceId: ${exampleEventHubNamespace.id}
      keyVaultKeyIds:
        - ${exampleKey.id}
      userAssignedIdentityId: ${exampleUserAssignedIdentity.id}
variables:
  current:
    fn::invoke:
      function: azure:core:getClientConfig
      arguments: {}
```
<!--End PulumiCodeChooser -->

## Import

Customer Managed Keys for a EventHub Namespace can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:eventhub/namespaceCustomerManagedKey:NamespaceCustomerManagedKey example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventHub/namespaces/namespace1
```

p
eventhubNamespaceId" UThe ID of the EventHub Namespace. Changing this forces a new resource to be created.
ù
infrastructureEncryptionEnabledB
 tWhether to enable Infrastructure Encryption (Double Encryption). Changing this forces a new resource to be created.
7
keyVaultKeyIds*" The list of keys of Key Vault.
Ú
userAssignedIdentityIdB" —The ID of a User Managed Identity that will be used to access Key Vaults that contain the encryption keys.

> **Note:** If using `user_assigned_identity_id`, ensure the User Assigned Identity is also assigned to the parent Event Hub.

> **Note:** If using `user_assigned_identity_id`, make sure to assign the identity the appropriate permissions to access the Key Vault key. Failure to grant `Get, UnwrapKey, and WrapKey` will cause this resource to fail to apply.
"p
eventhubNamespaceId" UThe ID of the EventHub Namespace. Changing this forces a new resource to be created.
"ù
infrastructureEncryptionEnabledB
 tWhether to enable Infrastructure Encryption (Double Encryption). Changing this forces a new resource to be created.
"7
keyVaultKeyIds*" The list of keys of Key Vault.
"Ú
userAssignedIdentityIdB" —The ID of a User Managed Identity that will be used to access Key Vaults that contain the encryption keys.

> **Note:** If using `user_assigned_identity_id`, ensure the User Assigned Identity is also assigned to the parent Event Hub.

> **Note:** If using `user_assigned_identity_id`, make sure to assign the identity the appropriate permissions to access the Key Vault key. Failure to grant `Get, UnwrapKey, and WrapKey` will cause this resource to fail to apply.
*‡6
Z
eventhubNamespaceSchemaGroup8azure:eventhub/namespaceSchemaGroup:NamespaceSchemaGroup£-## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "exampleRG-ehn-schemaGroup",
    location: "East US",
});
const test = new azure.eventhub.EventHubNamespace("test", {
    name: "example-ehn-schemaGroup",
    location: testAzurermResourceGroup.location,
    resourceGroupName: testAzurermResourceGroup.name,
    sku: "Standard",
});
const testNamespaceSchemaGroup = new azure.eventhub.NamespaceSchemaGroup("test", {
    name: "example-schemaGroup",
    namespaceId: test.id,
    schemaCompatibility: "Forward",
    schemaType: "Avro",
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="exampleRG-ehn-schemaGroup",
    location="East US")
test = azure.eventhub.EventHubNamespace("test",
    name="example-ehn-schemaGroup",
    location=test_azurerm_resource_group["location"],
    resource_group_name=test_azurerm_resource_group["name"],
    sku="Standard")
test_namespace_schema_group = azure.eventhub.NamespaceSchemaGroup("test",
    name="example-schemaGroup",
    namespace_id=test.id,
    schema_compatibility="Forward",
    schema_type="Avro")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "exampleRG-ehn-schemaGroup",
        Location = "East US",
    });

    var test = new Azure.EventHub.EventHubNamespace("test", new()
    {
        Name = "example-ehn-schemaGroup",
        Location = testAzurermResourceGroup.Location,
        ResourceGroupName = testAzurermResourceGroup.Name,
        Sku = "Standard",
    });

    var testNamespaceSchemaGroup = new Azure.EventHub.NamespaceSchemaGroup("test", new()
    {
        Name = "example-schemaGroup",
        NamespaceId = test.Id,
        SchemaCompatibility = "Forward",
        SchemaType = "Avro",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("exampleRG-ehn-schemaGroup"),
			Location: pulumi.String("East US"),
		})
		if err != nil {
			return err
		}
		test, err := eventhub.NewEventHubNamespace(ctx, "test", &eventhub.EventHubNamespaceArgs{
			Name:              pulumi.String("example-ehn-schemaGroup"),
			Location:          pulumi.Any(testAzurermResourceGroup.Location),
			ResourceGroupName: pulumi.Any(testAzurermResourceGroup.Name),
			Sku:               pulumi.String("Standard"),
		})
		if err != nil {
			return err
		}
		_, err = eventhub.NewNamespaceSchemaGroup(ctx, "test", &eventhub.NamespaceSchemaGroupArgs{
			Name:                pulumi.String("example-schemaGroup"),
			NamespaceId:         test.ID(),
			SchemaCompatibility: pulumi.String("Forward"),
			SchemaType:          pulumi.String("Avro"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.eventhub.EventHubNamespace;
import com.pulumi.azure.eventhub.EventHubNamespaceArgs;
import com.pulumi.azure.eventhub.NamespaceSchemaGroup;
import com.pulumi.azure.eventhub.NamespaceSchemaGroupArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("exampleRG-ehn-schemaGroup")
            .location("East US")
            .build());

        var test = new EventHubNamespace("test", EventHubNamespaceArgs.builder()
            .name("example-ehn-schemaGroup")
            .location(testAzurermResourceGroup.location())
            .resourceGroupName(testAzurermResourceGroup.name())
            .sku("Standard")
            .build());

        var testNamespaceSchemaGroup = new NamespaceSchemaGroup("testNamespaceSchemaGroup", NamespaceSchemaGroupArgs.builder()
            .name("example-schemaGroup")
            .namespaceId(test.id())
            .schemaCompatibility("Forward")
            .schemaType("Avro")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: exampleRG-ehn-schemaGroup
      location: East US
  test:
    type: azure:eventhub:EventHubNamespace
    properties:
      name: example-ehn-schemaGroup
      location: ${testAzurermResourceGroup.location}
      resourceGroupName: ${testAzurermResourceGroup.name}
      sku: Standard
  testNamespaceSchemaGroup:
    type: azure:eventhub:NamespaceSchemaGroup
    name: test
    properties:
      name: example-schemaGroup
      namespaceId: ${test.id}
      schemaCompatibility: Forward
      schemaType: Avro
```
<!--End PulumiCodeChooser -->

## Import

Schema Group for a EventHub Namespace can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:eventhub/namespaceSchemaGroup:NamespaceSchemaGroup example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.EventHub/namespaces/namespace1/schemaGroups/group1
```

j
nameB" \Specifies the name of this schema group. Changing this forces a new resource to be created.
r
namespaceId" _Specifies the ID of the EventHub Namespace. Changing this forces a new resource to be created.
¥
schemaCompatibility" òSpecifies the compatibility of this schema group. Possible values are `None`, `Backward`, `Forward`. Changing this forces a new resource to be created.
ñ

schemaType" ÉSpecifies the Type of this schema group. Possible values are `Avro`, `Unknown`. Changing this forces a new resource to be created.
"h
name" \Specifies the name of this schema group. Changing this forces a new resource to be created.
"r
namespaceId" _Specifies the ID of the EventHub Namespace. Changing this forces a new resource to be created.
"¥
schemaCompatibility" òSpecifies the compatibility of this schema group. Possible values are `None`, `Backward`, `Forward`. Changing this forces a new resource to be created.
"ñ

schemaType" ÉSpecifies the Type of this schema group. Possible values are `Avro`, `Unknown`. Changing this forces a new resource to be created.
*æk
-
eventhubQueueazure:eventhub/queue:Queue∂+Manages a ServiceBus Queue.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "my-servicebus",
    location: "West Europe",
});
const exampleNamespace = new azure.servicebus.Namespace("example", {
    name: "tfex-servicebus-namespace",
    location: example.location,
    resourceGroupName: example.name,
    sku: "Standard",
    tags: {
        source: "example",
    },
});
const exampleQueue = new azure.servicebus.Queue("example", {
    name: "tfex_servicebus_queue",
    namespaceId: exampleNamespace.id,
    partitioningEnabled: true,
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="my-servicebus",
    location="West Europe")
example_namespace = azure.servicebus.Namespace("example",
    name="tfex-servicebus-namespace",
    location=example.location,
    resource_group_name=example.name,
    sku="Standard",
    tags={
        "source": "example",
    })
example_queue = azure.servicebus.Queue("example",
    name="tfex_servicebus_queue",
    namespace_id=example_namespace.id,
    partitioning_enabled=True)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "my-servicebus",
        Location = "West Europe",
    });

    var exampleNamespace = new Azure.ServiceBus.Namespace("example", new()
    {
        Name = "tfex-servicebus-namespace",
        Location = example.Location,
        ResourceGroupName = example.Name,
        Sku = "Standard",
        Tags = 
        {
            { "source", "example" },
        },
    });

    var exampleQueue = new Azure.ServiceBus.Queue("example", new()
    {
        Name = "tfex_servicebus_queue",
        NamespaceId = exampleNamespace.Id,
        PartitioningEnabled = true,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/servicebus"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("my-servicebus"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		exampleNamespace, err := servicebus.NewNamespace(ctx, "example", &servicebus.NamespaceArgs{
			Name:              pulumi.String("tfex-servicebus-namespace"),
			Location:          example.Location,
			ResourceGroupName: example.Name,
			Sku:               pulumi.String("Standard"),
			Tags: pulumi.StringMap{
				"source": pulumi.String("example"),
			},
		})
		if err != nil {
			return err
		}
		_, err = servicebus.NewQueue(ctx, "example", &servicebus.QueueArgs{
			Name:                pulumi.String("tfex_servicebus_queue"),
			NamespaceId:         exampleNamespace.ID(),
			PartitioningEnabled: pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.servicebus.Namespace;
import com.pulumi.azure.servicebus.NamespaceArgs;
import com.pulumi.azure.servicebus.Queue;
import com.pulumi.azure.servicebus.QueueArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("my-servicebus")
            .location("West Europe")
            .build());

        var exampleNamespace = new Namespace("exampleNamespace", NamespaceArgs.builder()
            .name("tfex-servicebus-namespace")
            .location(example.location())
            .resourceGroupName(example.name())
            .sku("Standard")
            .tags(Map.of("source", "example"))
            .build());

        var exampleQueue = new Queue("exampleQueue", QueueArgs.builder()
            .name("tfex_servicebus_queue")
            .namespaceId(exampleNamespace.id())
            .partitioningEnabled(true)
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: my-servicebus
      location: West Europe
  exampleNamespace:
    type: azure:servicebus:Namespace
    name: example
    properties:
      name: tfex-servicebus-namespace
      location: ${example.location}
      resourceGroupName: ${example.name}
      sku: Standard
      tags:
        source: example
  exampleQueue:
    type: azure:servicebus:Queue
    name: example
    properties:
      name: tfex_servicebus_queue
      namespaceId: ${exampleNamespace.id}
      partitioningEnabled: true
```
<!--End PulumiCodeChooser -->

## Import

Service Bus Queue can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:eventhub/queue:Queue example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ServiceBus/namespaces/sbns1/queues/snqueue1
```

î
autoDeleteOnIdleB" zThe ISO 8601 timespan duration of the idle interval after which the Queue is automatically deleted, minimum of 5 minutes.
Ü
batchedOperationsEnabledB
 dBoolean flag which controls whether server-side batched operations are enabled. Defaults to `true`.
ù
 deadLetteringOnMessageExpirationB
 sBoolean flag which controls whether the Queue has dead letter support when a message expires. Defaults to `false`.
¨
defaultMessageTtlB" êThe ISO 8601 timespan duration of the TTL of messages sent to this queue. This is the default value used when TTL is not set on message itself.
ó
#duplicateDetectionHistoryTimeWindowB" jThe ISO 8601 timespan duration during which duplicates can be detected. Defaults to `PT10M` (10 Minutes).
â
expressEnabledB
 Boolean flag which controls whether Express Entities are enabled. An express queue holds a message in memory temporarily before writing it to persistent storage. Defaults to `false` for Basic and Standard. For Premium, it MUST be set to `false`.

> **NOTE:** Service Bus Premium namespaces do not support Express Entities, so `express_enabled` MUST be set to `false`.
x
forwardDeadLetteredMessagesToB" QThe name of a Queue or Topic to automatically forward dead lettered messages to.
‡
	forwardToB" ÃThe name of a Queue or Topic to automatically forward messages to. Please [see the documentation](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-auto-forwarding) for more information.
Õ
lockDurationB" ∂The ISO 8601 timespan duration of a peek-lock; that is, the amount of time that the message is locked for other receivers. Maximum value is 5 minutes. Defaults to `PT1M` (1 Minute).
x
maxDeliveryCountB ^Integer value which controls when a message is automatically dead lettered. Defaults to `10`.
Ã
maxMessageSizeInKilobytesB ®Integer value which controls the maximum size of a message allowed on the queue for Premium SKU. For supported values see the "Large messages support" section of [this document](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-premium-messaging#large-messages-support-preview).
Ö
maxSizeInMegabytesB ËInteger value which controls the size of memory allocated for the queue. For supported values see the "Queue or topic size" section of [Service Bus Quotas](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-quotas).
v
nameB" hSpecifies the name of the ServiceBus Queue resource. Changing this forces a new resource to be created.
Ç
namespaceId" oThe ID of the ServiceBus Namespace to create this queue in. Changing this forces a new resource to be created.
‹
partitioningEnabledB
 æBoolean flag which controls whether to enable the queue to be partitioned across multiple message brokers. Changing this forces a new resource to be created. Defaults to `false` for Basic and Standard.

> **NOTE:** Partitioning is available at entity creation for all queues and topics in Basic or Standard SKUs. For premium namespace, partitioning is available at namespace creation, and all queues and topics in the partitioned namespace will be partitioned, for the premium namespace that has `premium_messaging_partitions` sets to `1`, the namespace is not partitioned.
π
requiresDuplicateDetectionB
 îBoolean flag which controls whether the Queue requires duplicate detection. Changing this forces a new resource to be created. Defaults to `false`.
≈
requiresSessionB
 ´Boolean flag which controls whether the Queue requires sessions. This will allow ordered handling of unbounded sequences of related messages. With sessions enabled a queue can guarantee first-in-first-out delivery of messages. Changing this forces a new resource to be created. Defaults to `false`.
„
statusB" “The status of the Queue. Possible values are `Active`, `Creating`, `Deleting`, `Disabled`, `ReceiveDisabled`, `Renaming`, `SendDisabled`, `Unknown`. Note that `Restoring` is not accepted. Defaults to `Active`.
"í
autoDeleteOnIdle" zThe ISO 8601 timespan duration of the idle interval after which the Queue is automatically deleted, minimum of 5 minutes.
"Ü
batchedOperationsEnabledB
 dBoolean flag which controls whether server-side batched operations are enabled. Defaults to `true`.
"ù
 deadLetteringOnMessageExpirationB
 sBoolean flag which controls whether the Queue has dead letter support when a message expires. Defaults to `false`.
"™
defaultMessageTtl" êThe ISO 8601 timespan duration of the TTL of messages sent to this queue. This is the default value used when TTL is not set on message itself.
"ó
#duplicateDetectionHistoryTimeWindowB" jThe ISO 8601 timespan duration during which duplicates can be detected. Defaults to `PT10M` (10 Minutes).
"â
expressEnabledB
 Boolean flag which controls whether Express Entities are enabled. An express queue holds a message in memory temporarily before writing it to persistent storage. Defaults to `false` for Basic and Standard. For Premium, it MUST be set to `false`.

> **NOTE:** Service Bus Premium namespaces do not support Express Entities, so `express_enabled` MUST be set to `false`.
"x
forwardDeadLetteredMessagesToB" QThe name of a Queue or Topic to automatically forward dead lettered messages to.
"‡
	forwardToB" ÃThe name of a Queue or Topic to automatically forward messages to. Please [see the documentation](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-auto-forwarding) for more information.
"Õ
lockDurationB" ∂The ISO 8601 timespan duration of a peek-lock; that is, the amount of time that the message is locked for other receivers. Maximum value is 5 minutes. Defaults to `PT1M` (1 Minute).
"x
maxDeliveryCountB ^Integer value which controls when a message is automatically dead lettered. Defaults to `10`.
" 
maxMessageSizeInKilobytes ®Integer value which controls the maximum size of a message allowed on the queue for Premium SKU. For supported values see the "Large messages support" section of [this document](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-premium-messaging#large-messages-support-preview).
"É
maxSizeInMegabytes ËInteger value which controls the size of memory allocated for the queue. For supported values see the "Queue or topic size" section of [Service Bus Quotas](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-quotas).
"t
name" hSpecifies the name of the ServiceBus Queue resource. Changing this forces a new resource to be created.
"Ç
namespaceId" oThe ID of the ServiceBus Namespace to create this queue in. Changing this forces a new resource to be created.
"
namespaceName" "‹
partitioningEnabledB
 æBoolean flag which controls whether to enable the queue to be partitioned across multiple message brokers. Changing this forces a new resource to be created. Defaults to `false` for Basic and Standard.

> **NOTE:** Partitioning is available at entity creation for all queues and topics in Basic or Standard SKUs. For premium namespace, partitioning is available at namespace creation, and all queues and topics in the partitioned namespace will be partitioned, for the premium namespace that has `premium_messaging_partitions` sets to `1`, the namespace is not partitioned.
"π
requiresDuplicateDetectionB
 îBoolean flag which controls whether the Queue requires duplicate detection. Changing this forces a new resource to be created. Defaults to `false`.
"≈
requiresSessionB
 ´Boolean flag which controls whether the Queue requires sessions. This will allow ordered handling of unbounded sequences of related messages. With sessions enabled a queue can guarantee first-in-first-out delivery of messages. Changing this forces a new resource to be created. Defaults to `false`.
"
resourceGroupName" "„
statusB" “The status of the Queue. Possible values are `Active`, `Creating`, `Deleting`, `Disabled`, `ReceiveDisabled`, `Renaming`, `SendDisabled`, `Unknown`. Note that `Restoring` is not accepted. Defaults to `Active`.
*≈
`
eventhubQueueAuthorizationRule<azure:eventhub/queueAuthorizationRule:QueueAuthorizationRuleù
Manages an Authorization Rule for a ServiceBus Queue.

## Example Usage

<!--Start PulumiCodeChooser -->
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: my-servicebus
      location: West US
  exampleNamespace:
    type: azure:servicebus:Namespace
    name: example
    properties:
      name: tfex-servicebus-namespace
      location: ${example.location}
      resourceGroupName: ${example.name}
      sku: Standard
      tags:
        source: example
  exampleQueue:
    type: azure:servicebus:Queue
    name: example
    properties:
      name: tfex_servicebus_queue
      namespaceId: ${exampleNamespace.id}
      enablePartitioning: true
  exampleQueueAuthorizationRule:
    type: azure:servicebus:QueueAuthorizationRule
    name: example
    properties:
      name: examplerule
      queueId: ${exampleQueue.id}
      listen: true
      send: true
      manage: false
```
<!--End PulumiCodeChooser -->

## Import

ServiceBus Queue Authorization Rules can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:eventhub/queueAuthorizationRule:QueueAuthorizationRule rule1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ServiceBus/namespaces/namespace1/queues/queue1/authorizationRules/rule1
```

s
listenB
 cDoes this Authorization Rule have Listen permissions to the ServiceBus Queue? Defaults to `false`.
π
manageB
 ®Does this Authorization Rule have Manage permissions to the ServiceBus Queue? When this property is `true` - both `listen` and `send` must be too. Defaults to `false`.
o
nameB" aSpecifies the name of the Authorization Rule. Changing this forces a new resource to be created.
≤
queueId" ¢Specifies the ID of the ServiceBus Queue. Changing this forces a new resource to be created.

> **NOTE** At least one of the 3 permissions below needs to be set.
o
sendB
 aDoes this Authorization Rule have Send permissions to the ServiceBus Queue? Defaults to `false`.
"s
listenB
 cDoes this Authorization Rule have Listen permissions to the ServiceBus Queue? Defaults to `false`.
"π
manageB
 ®Does this Authorization Rule have Manage permissions to the ServiceBus Queue? When this property is `true` - both `listen` and `send` must be too. Defaults to `false`.
"m
name" aSpecifies the name of the Authorization Rule. Changing this forces a new resource to be created.
"Y
primaryConnectionString" :The Primary Connection String for the Authorization Rule.
"â
primaryConnectionStringAlias" eThe alias Primary Connection String for the ServiceBus Namespace, if the namespace is Geo DR paired.
">

primaryKey" ,The Primary Key for the Authorization Rule.
"≤
queueId" ¢Specifies the ID of the ServiceBus Queue. Changing this forces a new resource to be created.

> **NOTE** At least one of the 3 permissions below needs to be set.
"]
secondaryConnectionString" <The Secondary Connection String for the Authorization Rule.
"i
secondaryConnectionStringAlias" CThe alias Secondary Connection String for the ServiceBus Namespace
"B
secondaryKey" .The Secondary Key for the Authorization Rule.
"o
sendB
 aDoes this Authorization Rule have Send permissions to the ServiceBus Queue? Defaults to `false`.
*‚`
B
eventhubSubscription(azure:eventhub/subscription:Subscription°7Manages a ServiceBus Subscription.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "tfex-servicebus-subscription",
    location: "West Europe",
});
const exampleNamespace = new azure.servicebus.Namespace("example", {
    name: "tfex-servicebus-namespace",
    location: example.location,
    resourceGroupName: example.name,
    sku: "Standard",
    tags: {
        source: "example",
    },
});
const exampleTopic = new azure.servicebus.Topic("example", {
    name: "tfex_servicebus_topic",
    namespaceId: exampleNamespace.id,
    partitioningEnabled: true,
});
const exampleSubscription = new azure.servicebus.Subscription("example", {
    name: "tfex_servicebus_subscription",
    topicId: exampleTopic.id,
    maxDeliveryCount: 1,
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="tfex-servicebus-subscription",
    location="West Europe")
example_namespace = azure.servicebus.Namespace("example",
    name="tfex-servicebus-namespace",
    location=example.location,
    resource_group_name=example.name,
    sku="Standard",
    tags={
        "source": "example",
    })
example_topic = azure.servicebus.Topic("example",
    name="tfex_servicebus_topic",
    namespace_id=example_namespace.id,
    partitioning_enabled=True)
example_subscription = azure.servicebus.Subscription("example",
    name="tfex_servicebus_subscription",
    topic_id=example_topic.id,
    max_delivery_count=1)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "tfex-servicebus-subscription",
        Location = "West Europe",
    });

    var exampleNamespace = new Azure.ServiceBus.Namespace("example", new()
    {
        Name = "tfex-servicebus-namespace",
        Location = example.Location,
        ResourceGroupName = example.Name,
        Sku = "Standard",
        Tags = 
        {
            { "source", "example" },
        },
    });

    var exampleTopic = new Azure.ServiceBus.Topic("example", new()
    {
        Name = "tfex_servicebus_topic",
        NamespaceId = exampleNamespace.Id,
        PartitioningEnabled = true,
    });

    var exampleSubscription = new Azure.ServiceBus.Subscription("example", new()
    {
        Name = "tfex_servicebus_subscription",
        TopicId = exampleTopic.Id,
        MaxDeliveryCount = 1,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/servicebus"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("tfex-servicebus-subscription"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		exampleNamespace, err := servicebus.NewNamespace(ctx, "example", &servicebus.NamespaceArgs{
			Name:              pulumi.String("tfex-servicebus-namespace"),
			Location:          example.Location,
			ResourceGroupName: example.Name,
			Sku:               pulumi.String("Standard"),
			Tags: pulumi.StringMap{
				"source": pulumi.String("example"),
			},
		})
		if err != nil {
			return err
		}
		exampleTopic, err := servicebus.NewTopic(ctx, "example", &servicebus.TopicArgs{
			Name:                pulumi.String("tfex_servicebus_topic"),
			NamespaceId:         exampleNamespace.ID(),
			PartitioningEnabled: pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		_, err = servicebus.NewSubscription(ctx, "example", &servicebus.SubscriptionArgs{
			Name:             pulumi.String("tfex_servicebus_subscription"),
			TopicId:          exampleTopic.ID(),
			MaxDeliveryCount: pulumi.Int(1),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.servicebus.Namespace;
import com.pulumi.azure.servicebus.NamespaceArgs;
import com.pulumi.azure.servicebus.Topic;
import com.pulumi.azure.servicebus.TopicArgs;
import com.pulumi.azure.servicebus.Subscription;
import com.pulumi.azure.servicebus.SubscriptionArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("tfex-servicebus-subscription")
            .location("West Europe")
            .build());

        var exampleNamespace = new Namespace("exampleNamespace", NamespaceArgs.builder()
            .name("tfex-servicebus-namespace")
            .location(example.location())
            .resourceGroupName(example.name())
            .sku("Standard")
            .tags(Map.of("source", "example"))
            .build());

        var exampleTopic = new Topic("exampleTopic", TopicArgs.builder()
            .name("tfex_servicebus_topic")
            .namespaceId(exampleNamespace.id())
            .partitioningEnabled(true)
            .build());

        var exampleSubscription = new Subscription("exampleSubscription", SubscriptionArgs.builder()
            .name("tfex_servicebus_subscription")
            .topicId(exampleTopic.id())
            .maxDeliveryCount(1)
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: tfex-servicebus-subscription
      location: West Europe
  exampleNamespace:
    type: azure:servicebus:Namespace
    name: example
    properties:
      name: tfex-servicebus-namespace
      location: ${example.location}
      resourceGroupName: ${example.name}
      sku: Standard
      tags:
        source: example
  exampleTopic:
    type: azure:servicebus:Topic
    name: example
    properties:
      name: tfex_servicebus_topic
      namespaceId: ${exampleNamespace.id}
      partitioningEnabled: true
  exampleSubscription:
    type: azure:servicebus:Subscription
    name: example
    properties:
      name: tfex_servicebus_subscription
      topicId: ${exampleTopic.id}
      maxDeliveryCount: 1
```
<!--End PulumiCodeChooser -->

## Import

Service Bus Subscriptions can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:eventhub/subscription:Subscription example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ServiceBus/namespaces/sbns1/topics/sntopic1/subscriptions/sbsub1
```

Ç
autoDeleteOnIdleB" ÁThe idle interval after which the topic is automatically deleted as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). The minimum duration is `5` minutes or `PT5M`. Defaults to `P10675199DT2H48M5.4775807S`.
t
batchedOperationsEnabledB
 RBoolean flag which controls whether the Subscription supports batched operations.
È
clientScopedSubscriptionìBê:ç
ä
eventhub$SubscriptionClientScopedSubscriptionXazure:eventhub/SubscriptionClientScopedSubscription:SubscriptionClientScopedSubscription7A `client_scoped_subscription` block as defined below.
ÿ
clientScopedSubscriptionEnabledB
 Æwhether the subscription is scoped to a client id. Defaults to `false`.

> **NOTE:** Client Scoped Subscription can only be used for JMS subscription (Java Message Service).
±
$deadLetteringOnFilterEvaluationErrorB
 ÇBoolean flag which controls whether the Subscription has dead letter support on filter evaluation exceptions. Defaults to `true`.
è
 deadLetteringOnMessageExpirationB
 eBoolean flag which controls whether the Subscription has dead letter support when a message expires.
Ï
defaultMessageTtlB" –The Default message timespan to live as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). This is the duration after which the message expires, starting from when the message is sent to Service Bus. This is the value used when TimeToLive is not set on a message itself. Defaults to `P10675199DT2H48M5.4775807S`.
v
forwardDeadLetteredMessagesToB" OThe name of a Queue or Topic to automatically forward Dead Letter messages to.
V
	forwardToB" CThe name of a Queue or Topic to automatically forward messages to.
Ñ
lockDurationB" ÌThe lock duration for the subscription as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). The default value is `1` minute or `P0DT0H1M0S` . The maximum value is `5` minutes or `P0DT0H5M0S` . Defaults to `PT1M`.
:
maxDeliveryCount "The maximum number of deliveries.
}
nameB" oSpecifies the name of the ServiceBus Subscription resource. Changing this forces a new resource to be created.
¶
requiresSessionB
 åBoolean flag which controls whether this Subscription supports the concept of a session. Changing this forces a new resource to be created.
Ö
statusB" uThe status of the Subscription. Possible values are `Active`,`ReceiveDisabled`, or `Disabled`. Defaults to `Active`.
Å
topicId" rThe ID of the ServiceBus Topic to create this Subscription in. Changing this forces a new resource to be created.
"Ç
autoDeleteOnIdleB" ÁThe idle interval after which the topic is automatically deleted as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). The minimum duration is `5` minutes or `PT5M`. Defaults to `P10675199DT2H48M5.4775807S`.
"t
batchedOperationsEnabledB
 RBoolean flag which controls whether the Subscription supports batched operations.
"È
clientScopedSubscriptionìBê:ç
ä
eventhub$SubscriptionClientScopedSubscriptionXazure:eventhub/SubscriptionClientScopedSubscription:SubscriptionClientScopedSubscription7A `client_scoped_subscription` block as defined below.
"ÿ
clientScopedSubscriptionEnabledB
 Æwhether the subscription is scoped to a client id. Defaults to `false`.

> **NOTE:** Client Scoped Subscription can only be used for JMS subscription (Java Message Service).
"±
$deadLetteringOnFilterEvaluationErrorB
 ÇBoolean flag which controls whether the Subscription has dead letter support on filter evaluation exceptions. Defaults to `true`.
"è
 deadLetteringOnMessageExpirationB
 eBoolean flag which controls whether the Subscription has dead letter support when a message expires.
"Ï
defaultMessageTtlB" –The Default message timespan to live as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). This is the duration after which the message expires, starting from when the message is sent to Service Bus. This is the value used when TimeToLive is not set on a message itself. Defaults to `P10675199DT2H48M5.4775807S`.
"v
forwardDeadLetteredMessagesToB" OThe name of a Queue or Topic to automatically forward Dead Letter messages to.
"V
	forwardToB" CThe name of a Queue or Topic to automatically forward messages to.
"Ñ
lockDurationB" ÌThe lock duration for the subscription as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations). The default value is `1` minute or `P0DT0H1M0S` . The maximum value is `5` minutes or `P0DT0H5M0S` . Defaults to `PT1M`.
":
maxDeliveryCount "The maximum number of deliveries.
"{
name" oSpecifies the name of the ServiceBus Subscription resource. Changing this forces a new resource to be created.
"¶
requiresSessionB
 åBoolean flag which controls whether this Subscription supports the concept of a session. Changing this forces a new resource to be created.
"Ö
statusB" uThe status of the Subscription. Possible values are `Active`,`ReceiveDisabled`, or `Disabled`. Defaults to `Active`.
"Å
topicId" rThe ID of the ServiceBus Topic to create this Subscription in. Changing this forces a new resource to be created.
*ﬂ&
N
eventhubSubscriptionRule0azure:eventhub/subscriptionRule:SubscriptionRuleÈManages a ServiceBus Subscription Rule.

## Example Usage

### SQL Filter)

<!--Start PulumiCodeChooser -->
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: tfex-servicebus-subscription-rule-sql
      location: West Europe
  exampleNamespace:
    type: azure:servicebus:Namespace
    name: example
    properties:
      name: tfex-servicebus-namespace
      location: ${example.location}
      resourceGroupName: ${example.name}
      sku: Standard
      tags:
        source: example
  exampleTopic:
    type: azure:servicebus:Topic
    name: example
    properties:
      name: tfex_servicebus_topic
      namespaceId: ${exampleNamespace.id}
      enablePartitioning: true
  exampleSubscription:
    type: azure:servicebus:Subscription
    name: example
    properties:
      name: tfex_servicebus_subscription
      topicId: ${exampleTopic.id}
      maxDeliveryCount: 1
  exampleSubscriptionRule:
    type: azure:servicebus:SubscriptionRule
    name: example
    properties:
      name: tfex_servicebus_rule
      subscriptionId: ${exampleSubscription.id}
      filterType: SqlFilter
      sqlFilter: colour = 'red'
```
<!--End PulumiCodeChooser -->


### Correlation Filter)

<!--Start PulumiCodeChooser -->
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: tfex-servicebus-subscription-rule-cor
      location: West Europe
  exampleNamespace:
    type: azure:servicebus:Namespace
    name: example
    properties:
      name: tfex-servicebus-namespace
      location: ${example.location}
      resourceGroupName: ${example.name}
      sku: Standard
      tags:
        source: example
  exampleTopic:
    type: azure:servicebus:Topic
    name: example
    properties:
      name: tfex_servicebus_topic
      namespaceId: ${exampleNamespace.id}
      enablePartitioning: true
  exampleSubscription:
    type: azure:servicebus:Subscription
    name: example
    properties:
      name: tfex_servicebus_subscription
      topicId: ${exampleTopic.id}
      maxDeliveryCount: 1
  exampleSubscriptionRule:
    type: azure:servicebus:SubscriptionRule
    name: example
    properties:
      name: tfex_servicebus_rule
      subscriptionId: ${exampleSubscription.id}
      filterType: CorrelationFilter
      correlationFilter:
        correlationId: high
        label: red
        properties:
          customProperty: value
```
<!--End PulumiCodeChooser -->

## Import

Service Bus Subscription Rule can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:eventhub/subscriptionRule:SubscriptionRule example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ServiceBus/namespaces/sbns1/topics/sntopic1/subscriptions/sbsub1/rules/sbrule1
```

|
actionB" lRepresents set of actions written in SQL language-based syntax that is performed against a BrokeredMessage.
∫
correlationFilteräBá:Ñ
Å
eventhub!SubscriptionRuleCorrelationFilterRazure:eventhub/SubscriptionRuleCorrelationFilter:SubscriptionRuleCorrelationFilteróA `correlation_filter` block as documented below to be evaluated against a BrokeredMessage. Required when `filter_type` is set to `CorrelationFilter`.
~

filterType" lType of filter to be applied to a BrokeredMessage. Possible values are `SqlFilter` and `CorrelationFilter`.
y
nameB" kSpecifies the name of the ServiceBus Subscription Rule. Changing this forces a new resource to be created.
∞
	sqlFilterB" úRepresents a filter written in SQL language-based syntax that to be evaluated against a BrokeredMessage. Required when `filter_type` is set to `SqlFilter`.
ï
subscriptionId" The ID of the ServiceBus Subscription in which this Rule should be created. Changing this forces a new resource to be created.
"|
actionB" lRepresents set of actions written in SQL language-based syntax that is performed against a BrokeredMessage.
"∫
correlationFilteräBá:Ñ
Å
eventhub!SubscriptionRuleCorrelationFilterRazure:eventhub/SubscriptionRuleCorrelationFilter:SubscriptionRuleCorrelationFilteróA `correlation_filter` block as documented below to be evaluated against a BrokeredMessage. Required when `filter_type` is set to `CorrelationFilter`.
"~

filterType" lType of filter to be applied to a BrokeredMessage. Possible values are `SqlFilter` and `CorrelationFilter`.
"w
name" kSpecifies the name of the ServiceBus Subscription Rule. Changing this forces a new resource to be created.
"∞
	sqlFilterB" úRepresents a filter written in SQL language-based syntax that to be evaluated against a BrokeredMessage. Required when `filter_type` is set to `SqlFilter`.
"!
sqlFilterCompatibilityLevel "ï
subscriptionId" The ID of the ServiceBus Subscription in which this Rule should be created. Changing this forces a new resource to be created.
*«W
-
eventhubTopicazure:eventhub/topic:Topicø,Manages a ServiceBus Topic.

**Note** Topics can only be created in Namespaces with an SKU of `standard` or higher.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "tfex-servicebus-topic",
    location: "West Europe",
});
const exampleNamespace = new azure.servicebus.Namespace("example", {
    name: "tfex-servicebus-namespace",
    location: example.location,
    resourceGroupName: example.name,
    sku: "Standard",
    tags: {
        source: "example",
    },
});
const exampleTopic = new azure.servicebus.Topic("example", {
    name: "tfex_servicebus_topic",
    namespaceId: exampleNamespace.id,
    partitioningEnabled: true,
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="tfex-servicebus-topic",
    location="West Europe")
example_namespace = azure.servicebus.Namespace("example",
    name="tfex-servicebus-namespace",
    location=example.location,
    resource_group_name=example.name,
    sku="Standard",
    tags={
        "source": "example",
    })
example_topic = azure.servicebus.Topic("example",
    name="tfex_servicebus_topic",
    namespace_id=example_namespace.id,
    partitioning_enabled=True)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "tfex-servicebus-topic",
        Location = "West Europe",
    });

    var exampleNamespace = new Azure.ServiceBus.Namespace("example", new()
    {
        Name = "tfex-servicebus-namespace",
        Location = example.Location,
        ResourceGroupName = example.Name,
        Sku = "Standard",
        Tags = 
        {
            { "source", "example" },
        },
    });

    var exampleTopic = new Azure.ServiceBus.Topic("example", new()
    {
        Name = "tfex_servicebus_topic",
        NamespaceId = exampleNamespace.Id,
        PartitioningEnabled = true,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/servicebus"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("tfex-servicebus-topic"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		exampleNamespace, err := servicebus.NewNamespace(ctx, "example", &servicebus.NamespaceArgs{
			Name:              pulumi.String("tfex-servicebus-namespace"),
			Location:          example.Location,
			ResourceGroupName: example.Name,
			Sku:               pulumi.String("Standard"),
			Tags: pulumi.StringMap{
				"source": pulumi.String("example"),
			},
		})
		if err != nil {
			return err
		}
		_, err = servicebus.NewTopic(ctx, "example", &servicebus.TopicArgs{
			Name:                pulumi.String("tfex_servicebus_topic"),
			NamespaceId:         exampleNamespace.ID(),
			PartitioningEnabled: pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.servicebus.Namespace;
import com.pulumi.azure.servicebus.NamespaceArgs;
import com.pulumi.azure.servicebus.Topic;
import com.pulumi.azure.servicebus.TopicArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("tfex-servicebus-topic")
            .location("West Europe")
            .build());

        var exampleNamespace = new Namespace("exampleNamespace", NamespaceArgs.builder()
            .name("tfex-servicebus-namespace")
            .location(example.location())
            .resourceGroupName(example.name())
            .sku("Standard")
            .tags(Map.of("source", "example"))
            .build());

        var exampleTopic = new Topic("exampleTopic", TopicArgs.builder()
            .name("tfex_servicebus_topic")
            .namespaceId(exampleNamespace.id())
            .partitioningEnabled(true)
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: tfex-servicebus-topic
      location: West Europe
  exampleNamespace:
    type: azure:servicebus:Namespace
    name: example
    properties:
      name: tfex-servicebus-namespace
      location: ${example.location}
      resourceGroupName: ${example.name}
      sku: Standard
      tags:
        source: example
  exampleTopic:
    type: azure:servicebus:Topic
    name: example
    properties:
      name: tfex_servicebus_topic
      namespaceId: ${exampleNamespace.id}
      partitioningEnabled: true
```
<!--End PulumiCodeChooser -->

## Import

Service Bus Topics can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:eventhub/topic:Topic example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.ServiceBus/namespaces/sbns1/topics/sntopic1
```

ø
autoDeleteOnIdleB" §The ISO 8601 timespan duration of the idle interval after which the Topic is automatically deleted, minimum of 5 minutes. Defaults to `P10675199DT2H48M5.4775807S`.
m
batchedOperationsEnabledB
 KBoolean flag which controls if server-side batched operations are enabled.
π
defaultMessageTtlB" ùThe ISO 8601 timespan duration of TTL of messages sent to this topic if no TTL value is set on the message itself. Defaults to `P10675199DT2H48M5.4775807S`.
ó
#duplicateDetectionHistoryTimeWindowB" jThe ISO 8601 timespan duration during which duplicates can be detected. Defaults to `PT10M` (10 Minutes).
ª
expressEnabledB
 ¢Boolean flag which controls whether Express Entities are enabled. An express topic holds a message in memory temporarily before writing it to persistent storage.
ﬂ
maxMessageSizeInKilobytesB ªInteger value which controls the maximum size of a message allowed on the topic for Premium SKU. For supported values see the "Large messages support" section of [this document](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-premium-messaging#large-messages-support-preview). Defaults to `256`.
ë
maxSizeInMegabytesB ÙInteger value which controls the size of memory allocated for the topic. For supported values see the "Queue/topic size" section of [this document](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-quotas). Defaults to `5120`.
v
nameB" hSpecifies the name of the ServiceBus Topic resource. Changing this forces a new resource to be created.
Ç
namespaceId" oThe ID of the ServiceBus Namespace to create this topic in. Changing this forces a new resource to be created.
…
partitioningEnabledB
 ´Boolean flag which controls whether to enable the topic to be partitioned across multiple message brokers. Changing this forces a new resource to be created.

> **NOTE:** Partitioning is available at entity creation for all queues and topics in Basic or Standard SKUs. It is not available for the Premium messaging SKU, but any previously existing partitioned entities in Premium namespaces continue to work as expected. Please [see the documentation](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-partitioning) for more information.
π
requiresDuplicateDetectionB
 îBoolean flag which controls whether the Topic requires duplicate detection. Defaults to `false`. Changing this forces a new resource to be created.
y
statusB" iThe Status of the Service Bus Topic. Acceptable values are `Active` or `Disabled`. Defaults to `Active`.
Z
supportOrderingB
 ABoolean flag which controls whether the Topic supports ordering.
"ø
autoDeleteOnIdleB" §The ISO 8601 timespan duration of the idle interval after which the Topic is automatically deleted, minimum of 5 minutes. Defaults to `P10675199DT2H48M5.4775807S`.
"m
batchedOperationsEnabledB
 KBoolean flag which controls if server-side batched operations are enabled.
"π
defaultMessageTtlB" ùThe ISO 8601 timespan duration of TTL of messages sent to this topic if no TTL value is set on the message itself. Defaults to `P10675199DT2H48M5.4775807S`.
"ó
#duplicateDetectionHistoryTimeWindowB" jThe ISO 8601 timespan duration during which duplicates can be detected. Defaults to `PT10M` (10 Minutes).
"ª
expressEnabledB
 ¢Boolean flag which controls whether Express Entities are enabled. An express topic holds a message in memory temporarily before writing it to persistent storage.
"›
maxMessageSizeInKilobytes ªInteger value which controls the maximum size of a message allowed on the topic for Premium SKU. For supported values see the "Large messages support" section of [this document](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-premium-messaging#large-messages-support-preview). Defaults to `256`.
"è
maxSizeInMegabytes ÙInteger value which controls the size of memory allocated for the topic. For supported values see the "Queue/topic size" section of [this document](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-quotas). Defaults to `5120`.
"t
name" hSpecifies the name of the ServiceBus Topic resource. Changing this forces a new resource to be created.
"Ç
namespaceId" oThe ID of the ServiceBus Namespace to create this topic in. Changing this forces a new resource to be created.
"
namespaceName" "…
partitioningEnabledB
 ´Boolean flag which controls whether to enable the topic to be partitioned across multiple message brokers. Changing this forces a new resource to be created.

> **NOTE:** Partitioning is available at entity creation for all queues and topics in Basic or Standard SKUs. It is not available for the Premium messaging SKU, but any previously existing partitioned entities in Premium namespaces continue to work as expected. Please [see the documentation](https://docs.microsoft.com/azure/service-bus-messaging/service-bus-partitioning) for more information.
"π
requiresDuplicateDetectionB
 îBoolean flag which controls whether the Topic requires duplicate detection. Defaults to `false`. Changing this forces a new resource to be created.
"
resourceGroupName" "y
statusB" iThe Status of the Service Bus Topic. Acceptable values are `Active` or `Disabled`. Defaults to `Active`.
"Z
supportOrderingB
 ABoolean flag which controls whether the Topic supports ordering.
*œH
`
eventhubTopicAuthorizationRule<azure:eventhub/topicAuthorizationRule:TopicAuthorizationRuleΩ8Manages a ServiceBus Topic authorization Rule within a ServiceBus Topic.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "tfex-servicebus",
    location: "West Europe",
});
const exampleNamespace = new azure.servicebus.Namespace("example", {
    name: "tfex-servicebus-namespace",
    location: example.location,
    resourceGroupName: example.name,
    sku: "Standard",
    tags: {
        source: "example",
    },
});
const exampleTopic = new azure.servicebus.Topic("example", {
    name: "tfex_servicebus_topic",
    namespaceId: exampleNamespace.id,
});
const exampleTopicAuthorizationRule = new azure.servicebus.TopicAuthorizationRule("example", {
    name: "tfex_servicebus_topic_sasPolicy",
    topicId: exampleTopic.id,
    listen: true,
    send: false,
    manage: false,
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="tfex-servicebus",
    location="West Europe")
example_namespace = azure.servicebus.Namespace("example",
    name="tfex-servicebus-namespace",
    location=example.location,
    resource_group_name=example.name,
    sku="Standard",
    tags={
        "source": "example",
    })
example_topic = azure.servicebus.Topic("example",
    name="tfex_servicebus_topic",
    namespace_id=example_namespace.id)
example_topic_authorization_rule = azure.servicebus.TopicAuthorizationRule("example",
    name="tfex_servicebus_topic_sasPolicy",
    topic_id=example_topic.id,
    listen=True,
    send=False,
    manage=False)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "tfex-servicebus",
        Location = "West Europe",
    });

    var exampleNamespace = new Azure.ServiceBus.Namespace("example", new()
    {
        Name = "tfex-servicebus-namespace",
        Location = example.Location,
        ResourceGroupName = example.Name,
        Sku = "Standard",
        Tags = 
        {
            { "source", "example" },
        },
    });

    var exampleTopic = new Azure.ServiceBus.Topic("example", new()
    {
        Name = "tfex_servicebus_topic",
        NamespaceId = exampleNamespace.Id,
    });

    var exampleTopicAuthorizationRule = new Azure.ServiceBus.TopicAuthorizationRule("example", new()
    {
        Name = "tfex_servicebus_topic_sasPolicy",
        TopicId = exampleTopic.Id,
        Listen = true,
        Send = false,
        Manage = false,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/servicebus"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("tfex-servicebus"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		exampleNamespace, err := servicebus.NewNamespace(ctx, "example", &servicebus.NamespaceArgs{
			Name:              pulumi.String("tfex-servicebus-namespace"),
			Location:          example.Location,
			ResourceGroupName: example.Name,
			Sku:               pulumi.String("Standard"),
			Tags: pulumi.StringMap{
				"source": pulumi.String("example"),
			},
		})
		if err != nil {
			return err
		}
		exampleTopic, err := servicebus.NewTopic(ctx, "example", &servicebus.TopicArgs{
			Name:        pulumi.String("tfex_servicebus_topic"),
			NamespaceId: exampleNamespace.ID(),
		})
		if err != nil {
			return err
		}
		_, err = servicebus.NewTopicAuthorizationRule(ctx, "example", &servicebus.TopicAuthorizationRuleArgs{
			Name:    pulumi.String("tfex_servicebus_topic_sasPolicy"),
			TopicId: exampleTopic.ID(),
			Listen:  pulumi.Bool(true),
			Send:    pulumi.Bool(false),
			Manage:  pulumi.Bool(false),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.servicebus.Namespace;
import com.pulumi.azure.servicebus.NamespaceArgs;
import com.pulumi.azure.servicebus.Topic;
import com.pulumi.azure.servicebus.TopicArgs;
import com.pulumi.azure.servicebus.TopicAuthorizationRule;
import com.pulumi.azure.servicebus.TopicAuthorizationRuleArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("tfex-servicebus")
            .location("West Europe")
            .build());

        var exampleNamespace = new Namespace("exampleNamespace", NamespaceArgs.builder()
            .name("tfex-servicebus-namespace")
            .location(example.location())
            .resourceGroupName(example.name())
            .sku("Standard")
            .tags(Map.of("source", "example"))
            .build());

        var exampleTopic = new Topic("exampleTopic", TopicArgs.builder()
            .name("tfex_servicebus_topic")
            .namespaceId(exampleNamespace.id())
            .build());

        var exampleTopicAuthorizationRule = new TopicAuthorizationRule("exampleTopicAuthorizationRule", TopicAuthorizationRuleArgs.builder()
            .name("tfex_servicebus_topic_sasPolicy")
            .topicId(exampleTopic.id())
            .listen(true)
            .send(false)
            .manage(false)
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: tfex-servicebus
      location: West Europe
  exampleNamespace:
    type: azure:servicebus:Namespace
    name: example
    properties:
      name: tfex-servicebus-namespace
      location: ${example.location}
      resourceGroupName: ${example.name}
      sku: Standard
      tags:
        source: example
  exampleTopic:
    type: azure:servicebus:Topic
    name: example
    properties:
      name: tfex_servicebus_topic
      namespaceId: ${exampleNamespace.id}
  exampleTopicAuthorizationRule:
    type: azure:servicebus:TopicAuthorizationRule
    name: example
    properties:
      name: tfex_servicebus_topic_sasPolicy
      topicId: ${exampleTopic.id}
      listen: true
      send: false
      manage: false
```
<!--End PulumiCodeChooser -->

## Import

ServiceBus Topic authorization rules can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:eventhub/topicAuthorizationRule:TopicAuthorizationRule rule1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.ServiceBus/namespaces/namespace1/topics/topic1/authorizationRules/rule1
```

[
listenB
 KGrants listen access to this this Authorization Rule. Defaults to `false`.
°
manageB
 êGrants manage access to this this Authorization Rule. When this property is `true` - both `listen` and `send` must be too. Defaults to `false`.
â
nameB" {Specifies the name of the ServiceBus Topic Authorization Rule resource. Changing this forces a new resource to be created.
W
sendB
 IGrants send access to this this Authorization Rule. Defaults to `false`.
≤
topicId" ¢Specifies the ID of the ServiceBus Topic. Changing this forces a new resource to be created.

> **NOTE** At least one of the 3 permissions below needs to be set.
"[
listenB
 KGrants listen access to this this Authorization Rule. Defaults to `false`.
"°
manageB
 êGrants manage access to this this Authorization Rule. When this property is `true` - both `listen` and `send` must be too. Defaults to `false`.
"á
name" {Specifies the name of the ServiceBus Topic Authorization Rule resource. Changing this forces a new resource to be created.
"j
primaryConnectionString" KThe Primary Connection String for the ServiceBus Topic authorization Rule.
"â
primaryConnectionStringAlias" eThe alias Primary Connection String for the ServiceBus Namespace, if the namespace is Geo DR paired.
"O

primaryKey" =The Primary Key for the ServiceBus Topic authorization Rule.
"n
secondaryConnectionString" MThe Secondary Connection String for the ServiceBus Topic authorization Rule.
"i
secondaryConnectionStringAlias" CThe alias Secondary Connection String for the ServiceBus Namespace
"S
secondaryKey" ?The Secondary Key for the ServiceBus Topic authorization Rule.
"W
sendB
 IGrants send access to this this Authorization Rule. Defaults to `false`.
"≤
topicId" ¢Specifies the ID of the ServiceBus Topic. Changing this forces a new resource to be created.

> **NOTE** At least one of the 3 permissions below needs to be set.
*ä
X
extendedlocationCustomLocation4azure:extendedlocation/customLocation:CustomLocationÅManages a Custom Location within an Extended Location.

> **Note:** Installing and configuring the Azure Arc Agent on your Kubernetes Cluster to establish connectivity is outside the scope of this document. For more details refer to [Deploy agents to your cluster](https://learn.microsoft.com/en-us/azure/azure-arc/kubernetes/conceptual-agent-overview#deploy-agents-to-your-cluster) and [Connect an existing Kubernetes Cluster](https://learn.microsoft.com/en-us/azure/azure-arc/kubernetes/quickstart-connect-cluster?tabs=azure-cli#connect-an-existing-kubernetes-cluster). If you encounter issues connecting your Kubernetes Cluster to Azure Arc, we'd recommend opening a ticket with Microsoft Support.

## Import

Custom Locations can be imported using the resource id, e.g.

```sh
$ pulumi import azure:extendedlocation/customLocation:CustomLocation example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/example-resources/providers/Microsoft.ExtendedLocation/customLocations/example-custom-location
```

Ã
authenticationãBà:Ö
Ç
extendedlocationCustomLocationAuthenticationPazure:extendedlocation/CustomLocationAuthentication:CustomLocationAuthentication,An `authentication` block as defined below.
J
clusterExtensionIds*" -Specifies the list of Cluster Extension IDs.
H
displayNameB" 3Specifies the display name of the Custom Location.
6
hostResourceId"  Specifies the host resource ID.
s
hostTypeB" aSpecifies the host type of the Custom Location. The only possible values is `KubernetesCluster`.
ë
locationB" Specifies the Azure location where the Custom Location should exist. Changing this forces a new Custom Location to be created.
ä
nameB" |Specifies the name which should be used for this Custom Location. Changing this forces a new Custom Location to be created.
{
	namespace" jSpecifies the namespace of the Custom Location. Changing this forces a new Custom Location to be created.
•
resourceGroupName" ãSpecifies the name of the Resource Group where the Custom Location should exist. Changing this forces a new Custom Location to be created.
"Ã
authenticationãBà:Ö
Ç
extendedlocationCustomLocationAuthenticationPazure:extendedlocation/CustomLocationAuthentication:CustomLocationAuthentication,An `authentication` block as defined below.
"J
clusterExtensionIds*" -Specifies the list of Cluster Extension IDs.
"H
displayNameB" 3Specifies the display name of the Custom Location.
"6
hostResourceId"  Specifies the host resource ID.
"s
hostTypeB" aSpecifies the host type of the Custom Location. The only possible values is `KubernetesCluster`.
"è
location" Specifies the Azure location where the Custom Location should exist. Changing this forces a new Custom Location to be created.
"à
name" |Specifies the name which should be used for this Custom Location. Changing this forces a new Custom Location to be created.
"{
	namespace" jSpecifies the namespace of the Custom Location. Changing this forces a new Custom Location to be created.
"•
resourceGroupName" ãSpecifies the name of the Resource Group where the Custom Location should exist. Changing this forces a new Custom Location to be created.
*ê7
2
fabricCapacityazure:fabric/capacity:Capacity±+Manages a Fabric Capacity.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const current = azure.core.getClientConfig({});
const example = new azure.core.ResourceGroup("example", {
    name: "example-resources",
    location: "West Europe",
});
const exampleCapacity = new azure.fabric.Capacity("example", {
    name: "example-ffc",
    resourceGroupName: example.name,
    location: "West Europe",
    administrationMembers: [current.then(current => current.objectId)],
    sku: {
        name: "F32",
        tier: "Fabric",
    },
    tags: {
        environment: "test",
    },
});
```
```python
import pulumi
import pulumi_azure as azure

current = azure.core.get_client_config()
example = azure.core.ResourceGroup("example",
    name="example-resources",
    location="West Europe")
example_capacity = azure.fabric.Capacity("example",
    name="example-ffc",
    resource_group_name=example.name,
    location="West Europe",
    administration_members=[current.object_id],
    sku={
        "name": "F32",
        "tier": "Fabric",
    },
    tags={
        "environment": "test",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var current = Azure.Core.GetClientConfig.Invoke();

    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "example-resources",
        Location = "West Europe",
    });

    var exampleCapacity = new Azure.Fabric.Capacity("example", new()
    {
        Name = "example-ffc",
        ResourceGroupName = example.Name,
        Location = "West Europe",
        AdministrationMembers = new[]
        {
            current.Apply(getClientConfigResult => getClientConfigResult.ObjectId),
        },
        Sku = new Azure.Fabric.Inputs.CapacitySkuArgs
        {
            Name = "F32",
            Tier = "Fabric",
        },
        Tags = 
        {
            { "environment", "test" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/fabric"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		current, err := core.GetClientConfig(ctx, map[string]interface{}{}, nil)
		if err != nil {
			return err
		}
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("example-resources"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		_, err = fabric.NewCapacity(ctx, "example", &fabric.CapacityArgs{
			Name:              pulumi.String("example-ffc"),
			ResourceGroupName: example.Name,
			Location:          pulumi.String("West Europe"),
			AdministrationMembers: pulumi.StringArray{
				pulumi.String(current.ObjectId),
			},
			Sku: &fabric.CapacitySkuArgs{
				Name: pulumi.String("F32"),
				Tier: pulumi.String("Fabric"),
			},
			Tags: pulumi.StringMap{
				"environment": pulumi.String("test"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.CoreFunctions;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.fabric.Capacity;
import com.pulumi.azure.fabric.CapacityArgs;
import com.pulumi.azure.fabric.inputs.CapacitySkuArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = CoreFunctions.getClientConfig();

        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("example-resources")
            .location("West Europe")
            .build());

        var exampleCapacity = new Capacity("exampleCapacity", CapacityArgs.builder()
            .name("example-ffc")
            .resourceGroupName(example.name())
            .location("West Europe")
            .administrationMembers(current.applyValue(getClientConfigResult -> getClientConfigResult.objectId()))
            .sku(CapacitySkuArgs.builder()
                .name("F32")
                .tier("Fabric")
                .build())
            .tags(Map.of("environment", "test"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: example-resources
      location: West Europe
  exampleCapacity:
    type: azure:fabric:Capacity
    name: example
    properties:
      name: example-ffc
      resourceGroupName: ${example.name}
      location: West Europe
      administrationMembers:
        - ${current.objectId}
      sku:
        name: F32
        tier: Fabric
      tags:
        environment: test
variables:
  current:
    fn::invoke:
      function: azure:core:getClientConfig
      arguments: {}
```
<!--End PulumiCodeChooser -->

## Import

Fabric Capacities can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:fabric/capacity:Capacity example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Fabric/capacities/capacity1
```

å
administrationMembersB*" kAn array of administrator user identities. The member must be an Entra member user or a service principal.
Ñ
locationB" rThe supported Azure location where the Fabric Capacity exists. Changing this forces a new resource to be created.
x
nameB" jThe name which should be used for the Fabric Capacity. Changing this forces a new resource to be created.
ì
resourceGroupName" zThe name of the Resource Group in which to create the Fabric Capacity. Changing this forces a new resource to be created.
h
sku?:=
;
fabricCapacitySku$azure:fabric/CapacitySku:CapacitySku A `sku` block as defined below.
D
tagsB2" 4A mapping of tags to assign to the Fabric Capacity.
"å
administrationMembersB*" kAn array of administrator user identities. The member must be an Entra member user or a service principal.
"Ç
location" rThe supported Azure location where the Fabric Capacity exists. Changing this forces a new resource to be created.
"v
name" jThe name which should be used for the Fabric Capacity. Changing this forces a new resource to be created.
"ì
resourceGroupName" zThe name of the Resource Group in which to create the Fabric Capacity. Changing this forces a new resource to be created.
"h
sku?:=
;
fabricCapacitySku$azure:fabric/CapacitySku:CapacitySku A `sku` block as defined below.
"D
tagsB2" 4A mapping of tags to assign to the Fabric Capacity.
*Ä4
4

fluidrelayServerazure:fluidrelay/server:ServerßManages a Fluid Relay Server.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = new azure.core.ResourceGroup("example", {
    name: "example-resources",
    location: "West Europe",
});
const exampleServer = new azure.fluidrelay.Server("example", {
    name: "example",
    resourceGroupName: example.name,
    location: example.location,
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.core.ResourceGroup("example",
    name="example-resources",
    location="West Europe")
example_server = azure.fluidrelay.Server("example",
    name="example",
    resource_group_name=example.name,
    location=example.location)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "example-resources",
        Location = "West Europe",
    });

    var exampleServer = new Azure.FluidRelay.Server("example", new()
    {
        Name = "example",
        ResourceGroupName = example.Name,
        Location = example.Location,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/fluidrelay"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("example-resources"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		_, err = fluidrelay.NewServer(ctx, "example", &fluidrelay.ServerArgs{
			Name:              pulumi.String("example"),
			ResourceGroupName: example.Name,
			Location:          example.Location,
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.fluidrelay.Server;
import com.pulumi.azure.fluidrelay.ServerArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ResourceGroup("example", ResourceGroupArgs.builder()
            .name("example-resources")
            .location("West Europe")
            .build());

        var exampleServer = new Server("exampleServer", ServerArgs.builder()
            .name("example")
            .resourceGroupName(example.name())
            .location(example.location())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: azure:core:ResourceGroup
    properties:
      name: example-resources
      location: West Europe
  exampleServer:
    type: azure:fluidrelay:Server
    name: example
    properties:
      name: example
      resourceGroupName: ${example.name}
      location: ${example.location}
```
<!--End PulumiCodeChooser -->

## Import

Fluid Relay Servers can be imported using the `resource id`, e.g.

```sh
$ pulumi import azure:fluidrelay/server:Server example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/rg1/providers/Microsoft.FluidRelay/fluidRelayServers/server1
```

Ï
customerManagedKeypBn:l
j

fluidrelayServerCustomerManagedKeyBazure:fluidrelay/ServerCustomerManagedKey:ServerCustomerManagedKeydA `customer_managed_key` block as defined below. Changing this forces a new resource to be created.
Ü
identityRBP:N
L

fluidrelayServerIdentity.azure:fluidrelay/ServerIdentity:ServerIdentity&An `identity` block as defined below.
ã
locationB" yThe Azure Region where the Fluid Relay Server should exist. Changing this forces a new Fluid Relay Server to be created.
Ü
nameB" xThe name which should be used for this Fluid Relay Server. Changing this forces a new Fluid Relay Server to be created.
°
resourceGroupName" áThe name of the Resource Group where the Fluid Relay Server should exist. Changing this forces a new Fluid Relay Server to be created.
Ø

storageSkuB" öSku of the storage associated with the resource, Possible values are `standard` and `basic`. Changing this forces a new Fluid Relay Server to be created.
V
tagsB2" FA mapping of tags which should be assigned to the Fluid Relay Server.
"Ï
customerManagedKeypBn:l
j

fluidrelayServerCustomerManagedKeyBazure:fluidrelay/ServerCustomerManagedKey:ServerCustomerManagedKeydA `customer_managed_key` block as defined below. Changing this forces a new resource to be created.
"7
frsTenantId" $The Fluid tenantId for this server.
"Ü
identityRBP:N
L

fluidrelayServerIdentity.azure:fluidrelay/ServerIdentity:ServerIdentity&An `identity` block as defined below.
"â
location" yThe Azure Region where the Fluid Relay Server should exist. Changing this forces a new Fluid Relay Server to be created.
"Ñ
name" xThe name which should be used for this Fluid Relay Server. Changing this forces a new Fluid Relay Server to be created.
"é
ordererEndpoints*" ÛAn array of the Fluid Relay Orderer endpoints. This will be deprecated in future version of fluid relay server and will always be empty, [more details](https://learn.microsoft.com/en-us/azure/azure-fluid-relay/concepts/version-compatibility).
"3

primaryKey" !The primary key for this server.
"°
resourceGroupName" áThe name of the Resource Group where the Fluid Relay Server should exist. Changing this forces a new Fluid Relay Server to be created.
"7
secondaryKey" #The secondary key for this server.
"U
serviceEndpoints*" ;An array of service endpoints for this Fluid Relay Server.
"ö
storageEndpoints*" ˇAn array of storage endpoints for this Fluid Relay Server. This will be deprecated in future version of fluid relay server and will always be empty, [more details](https://learn.microsoft.com/en-us/azure/azure-fluid-relay/concepts/version-compatibility).
"≠

storageSku" öSku of the storage associated with the resource, Possible values are `standard` and `basic`. Changing this forces a new Fluid Relay Server to be created.
"V
tagsB2" FA mapping of tags which should be assigned to the Fluid Relay Server.
2å$
H
domainservices
getService*azure:domainservices/getService:getService≈Gets information about an Active Directory Domain Service.

> **Supported Modes:** At present this data source only supports **User Forest** mode and _not_ **Resource Forest** mode. [Read more](https://docs.microsoft.com/azure/active-directory-domain-services/concepts-resource-forest) about the different operation modes for this service.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = azure.domainservices.getService({
    name: "example-aadds",
    resourceGroupName: "example-aadds-rg",
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.domainservices.get_service(name="example-aadds",
    resource_group_name="example-aadds-rg")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = Azure.DomainServices.GetService.Invoke(new()
    {
        Name = "example-aadds",
        ResourceGroupName = "example-aadds-rg",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/domainservices"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := domainservices.LookupService(ctx, &domainservices.LookupServiceArgs{
			Name:              "example-aadds",
			ResourceGroupName: "example-aadds-rg",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.domainservices.DomainservicesFunctions;
import com.pulumi.azure.domainservices.inputs.GetServiceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = DomainservicesFunctions.getService(GetServiceArgs.builder()
            .name("example-aadds")
            .resourceGroupName("example-aadds-rg")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: azure:domainservices:getService
      arguments:
        name: example-aadds
        resourceGroupName: example-aadds-rg
```
<!--End PulumiCodeChooser -->
ã
name" The display name for your managed Active Directory Domain Service resource. Changing this forces a new resource to be created.
ï
resourceGroupName" |The name of the Resource Group in which the Domain Service should exist. Changing this forces a new resource to be created.
<
tagsB2" ,A mapping of tags assigned to the resource.
"C
deploymentId" /A unique ID for the managed domain deployment.
"û
domainConfigurationType" The forest type used by the managed domain. One of `ResourceTrusting`, for a _Resource Forest_, or blank, for a _User Forest_.
"¸

domainName" ÈThe Active Directory domain of the Domain Service. See [official documentation](https://docs.microsoft.com/azure/active-directory-domain-services/tutorial-create-instance#create-a-managed-domain) for constraints and recommendations.
"n
filteredSyncEnabled
 SWhether group-based filtered sync (also called scoped synchronisation) is enabled.
"E
id" ;The provider-assigned unique ID for this managed resource.
"E
location" 5The Azure location in which the replica set resides.
"

name" "Ø
notificationsr*p:n
l
domainservicesgetServiceNotificationBazure:domainservices/getServiceNotification:getServiceNotification*A `notifications` block as defined below.
"∞
replicaSetsl*j:h
f
domainservicesgetServiceReplicaSet>azure:domainservices/getServiceReplicaSet:getServiceReplicaSet3One or more `replica_set` blocks as defined below.
"
resourceGroupName" "

resourceId" "•
secureLdapsl*j:h
f
domainservicesgetServiceSecureLdap>azure:domainservices/getServiceSecureLdap:getServiceSecureLdap(A `secure_ldap` block as defined below.
"õ

securitiesf*d:b
`
domainservicesgetServiceSecurity:azure:domainservices/getServiceSecurity:getServiceSecurity%A `security` block as defined below.
"a
sku" VThe SKU of the Domain Service resource. One of `Standard`, `Enterprise` or `Premium`.
"
	syncOwner" "<
tagsB2" ,A mapping of tags assigned to the resource.
"
tenantId" "
version 2⁄'
V
elasticcloudgetElasticsearch4azure:elasticcloud/getElasticsearch:getElasticsearch„Use this data source to access information about an existing Elasticsearch resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = azure.elasticcloud.getElasticsearch({
    name: "my-elastic-search",
    resourceGroupName: "example-resources",
});
export const elasticsearchEndpoint = example.then(example => example.elasticsearchServiceUrl);
export const kibanaEndpoint = example.then(example => example.kibanaServiceUrl);
```
```python
import pulumi
import pulumi_azure as azure

example = azure.elasticcloud.get_elasticsearch(name="my-elastic-search",
    resource_group_name="example-resources")
pulumi.export("elasticsearchEndpoint", example.elasticsearch_service_url)
pulumi.export("kibanaEndpoint", example.kibana_service_url)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = Azure.ElasticCloud.GetElasticsearch.Invoke(new()
    {
        Name = "my-elastic-search",
        ResourceGroupName = "example-resources",
    });

    return new Dictionary<string, object?>
    {
        ["elasticsearchEndpoint"] = example.Apply(getElasticsearchResult => getElasticsearchResult.ElasticsearchServiceUrl),
        ["kibanaEndpoint"] = example.Apply(getElasticsearchResult => getElasticsearchResult.KibanaServiceUrl),
    };
});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/elasticcloud"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := elasticcloud.LookupElasticsearch(ctx, &elasticcloud.LookupElasticsearchArgs{
			Name:              "my-elastic-search",
			ResourceGroupName: "example-resources",
		}, nil)
		if err != nil {
			return err
		}
		ctx.Export("elasticsearchEndpoint", example.ElasticsearchServiceUrl)
		ctx.Export("kibanaEndpoint", example.KibanaServiceUrl)
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.elasticcloud.ElasticcloudFunctions;
import com.pulumi.azure.elasticcloud.inputs.GetElasticsearchArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = ElasticcloudFunctions.getElasticsearch(GetElasticsearchArgs.builder()
            .name("my-elastic-search")
            .resourceGroupName("example-resources")
            .build());

        ctx.export("elasticsearchEndpoint", example.applyValue(getElasticsearchResult -> getElasticsearchResult.elasticsearchServiceUrl()));
        ctx.export("kibanaEndpoint", example.applyValue(getElasticsearchResult -> getElasticsearchResult.kibanaServiceUrl()));
    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: azure:elasticcloud:getElasticsearch
      arguments:
        name: my-elastic-search
        resourceGroupName: example-resources
outputs:
  elasticsearchEndpoint: ${example.elasticsearchServiceUrl}
  kibanaEndpoint: ${example.kibanaServiceUrl}
```
<!--End PulumiCodeChooser -->
í
logsgBe*c:a
_
elasticcloudgetElasticsearchLog:azure:elasticcloud/getElasticsearchLog:getElasticsearchLog!A `logs` block as defined below.
4
name" (The name of the Elasticsearch resource.
[
resourceGroupName" BThe name of the resource group in which the Elasticsearch exists.
"O
elasticCloudDeploymentId" /The ID of the Deployment within Elastic Cloud.
"g
elasticCloudEmailAddress" GThe Email Address which is associated with this Elasticsearch account.
"a
elasticCloudSsoDefaultUrl" @The Default URL used for Single Sign On (SSO) to Elastic Cloud.
"K
elasticCloudUserId" 1The ID of the User Account within Elastic Cloud.
"h
elasticsearchServiceUrl" IThe URL to the Elasticsearch Service associated with this Elasticsearch.
"E
id" ;The provider-assigned unique ID for this managed resource.
"\
kibanaServiceUrl" DThe URL to the Kibana Dashboard associated with this Elasticsearch.
"e
kibanaSsoUri" QThe URI used for SSO to the Kibana Dashboard associated with this Elasticsearch.
"E
location" 5The Azure Region in which this Elasticsearch exists.
"ê
logse*c:a
_
elasticcloudgetElasticsearchLog:azure:elasticcloud/getElasticsearchLog:getElasticsearchLog!A `logs` block as defined below.
"Z
monitoringEnabled
 ASpecifies if monitoring is enabled on this Elasticsearch or not.
"@
name" 4The name (key) of the Tag which should be filtered.
"
resourceGroupName" "@
skuName" 1The name of the SKU used for this Elasticsearch.
"?
tags2" 1A mapping of tags assigned to the Elasticsearch.
2¨
+

elasticsangetazure:elasticsan/get:get“Use this data source to access information about an existing Elastic SAN.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = azure.elasticsan.get({
    name: "existing",
    resourceGroupName: "existing",
});
export const id = example.then(example => example.id);
```
```python
import pulumi
import pulumi_azure as azure

example = azure.elasticsan.get(name="existing",
    resource_group_name="existing")
pulumi.export("id", example.id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = Azure.ElasticSan.Get.Invoke(new()
    {
        Name = "existing",
        ResourceGroupName = "existing",
    });

    return new Dictionary<string, object?>
    {
        ["id"] = example.Apply(getResult => getResult.Id),
    };
});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/elasticsan"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := elasticsan.Get(ctx, &elasticsan.GetArgs{
			Name:              "existing",
			ResourceGroupName: "existing",
		}, nil)
		if err != nil {
			return err
		}
		ctx.Export("id", example.Id)
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.elasticsan.ElasticsanFunctions;
import com.pulumi.azure.elasticsan.inputs.GetArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = ElasticsanFunctions.get(GetArgs.builder()
            .name("existing")
            .resourceGroupName("existing")
            .build());

        ctx.export("id", example.applyValue(getResult -> getResult.id()));
    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: azure:elasticsan:get
      arguments:
        name: existing
        resourceGroupName: existing
outputs:
  id: ${example.id}
```
<!--End PulumiCodeChooser -->
*
name" The name of this Elastic SAN.
V
resourceGroupName" =The name of the Resource Group where the Elastic SAN exists.
"G
baseSizeInTib 2The base size of the Elastic SAN resource in TiB.
"K
extendedSizeInTib 2The base size of the Elastic SAN resource in TiB.
"E
id" ;The provider-assigned unique ID for this managed resource.
"?
location" /The Azure Region where the Elastic SAN exists.
"
name" The SKU name.
"
resourceGroupName" "g
skus=*;:9
7

elasticsangetSkus azure:elasticsan/getSkus:getSkus A `sku` block as defined below.
"=
tags2" /A mapping of tags assigned to the Elastic SAN.
"E
	totalIops 4Total Provisioned IOps of the Elastic SAN resource.
">
	totalMbps -Total Provisioned MBps Elastic SAN resource.
"D
totalSizeInTib .Total size of the Elastic SAN resource in TB.
"J
totalVolumeSizeInGib .Total size of the provisioned Volumes in GiB.
"T
volumeGroupCount <Total number of volume groups in this Elastic SAN resource.
":
zones*" +Logical zone for the Elastic SAN resource.
2≠$
L

elasticsangetVolumeGroup.azure:elasticsan/getVolumeGroup:getVolumeGroupœUse this data source to access information about an existing Elastic SAN Volume Group.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = azure.elasticsan.get({
    name: "existing",
    resourceGroupName: "existing",
});
const exampleGetVolumeGroup = example.then(example => azure.elasticsan.getVolumeGroup({
    name: "existing",
    elasticSanId: example.id,
}));
export const id = exampleGetVolumeGroup.then(exampleGetVolumeGroup => exampleGetVolumeGroup.id);
```
```python
import pulumi
import pulumi_azure as azure

example = azure.elasticsan.get(name="existing",
    resource_group_name="existing")
example_get_volume_group = azure.elasticsan.get_volume_group(name="existing",
    elastic_san_id=example.id)
pulumi.export("id", example_get_volume_group.id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = Azure.ElasticSan.Get.Invoke(new()
    {
        Name = "existing",
        ResourceGroupName = "existing",
    });

    var exampleGetVolumeGroup = Azure.ElasticSan.GetVolumeGroup.Invoke(new()
    {
        Name = "existing",
        ElasticSanId = example.Apply(getResult => getResult.Id),
    });

    return new Dictionary<string, object?>
    {
        ["id"] = exampleGetVolumeGroup.Apply(getVolumeGroupResult => getVolumeGroupResult.Id),
    };
});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/elasticsan"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := elasticsan.Get(ctx, &elasticsan.GetArgs{
			Name:              "existing",
			ResourceGroupName: "existing",
		}, nil)
		if err != nil {
			return err
		}
		exampleGetVolumeGroup, err := elasticsan.LookupVolumeGroup(ctx, &elasticsan.LookupVolumeGroupArgs{
			Name:         "existing",
			ElasticSanId: example.Id,
		}, nil)
		if err != nil {
			return err
		}
		ctx.Export("id", exampleGetVolumeGroup.Id)
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.elasticsan.ElasticsanFunctions;
import com.pulumi.azure.elasticsan.inputs.GetArgs;
import com.pulumi.azure.elasticsan.inputs.GetVolumeGroupArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = ElasticsanFunctions.get(GetArgs.builder()
            .name("existing")
            .resourceGroupName("existing")
            .build());

        final var exampleGetVolumeGroup = ElasticsanFunctions.getVolumeGroup(GetVolumeGroupArgs.builder()
            .name("existing")
            .elasticSanId(example.applyValue(getResult -> getResult.id()))
            .build());

        ctx.export("id", exampleGetVolumeGroup.applyValue(getVolumeGroupResult -> getVolumeGroupResult.id()));
    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: azure:elasticsan:get
      arguments:
        name: existing
        resourceGroupName: existing
  exampleGetVolumeGroup:
    fn::invoke:
      function: azure:elasticsan:getVolumeGroup
      arguments:
        name: existing
        elasticSanId: ${example.id}
outputs:
  id: ${exampleGetVolumeGroup.id}
```
<!--End PulumiCodeChooser -->
Y
elasticSanId" EThe Elastic SAN ID within which the Elastic SAN Volume Group exists.
6
name" *The name of the Elastic SAN Volume Group.
"
elasticSanId" "P
encryptionType" :The type of the key used to encrypt the data of the disk.
"©
encryptionsp*n:l
j

elasticsangetVolumeGroupEncryptionBazure:elasticsan/getVolumeGroupEncryption:getVolumeGroupEncryption(An `encryption` block as defined below.
"E
id" ;The provider-assigned unique ID for this managed resource.
"†

identitiesj*h:f
d

elasticsangetVolumeGroupIdentity>azure:elasticsan/getVolumeGroupIdentity:getVolumeGroupIdentity&An `identity` block as defined below.
"

name" "π
networkRuless*q:o
m

elasticsangetVolumeGroupNetworkRuleDazure:elasticsan/getVolumeGroupNetworkRule:getVolumeGroupNetworkRule4One or more `network_rule` blocks as defined below.
"4
protocolType"  The type of the storage target.
2π+
U

elasticsangetVolumeSnapshot4azure:elasticsan/getVolumeSnapshot:getVolumeSnapshotî'Use this data source to access information about an existing Elastic SAN Volume Snapshot.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = azure.elasticsan.get({
    name: "existing",
    resourceGroupName: "existing",
});
const exampleGetVolumeGroup = example.then(example => azure.elasticsan.getVolumeGroup({
    name: "existing",
    elasticSanId: example.id,
}));
const exampleGetVolumeSnapshot = exampleGetVolumeGroup.then(exampleGetVolumeGroup => azure.elasticsan.getVolumeSnapshot({
    name: "existing",
    volumeGroupId: exampleGetVolumeGroup.id,
}));
export const id = exampleGetVolumeSnapshot.then(exampleGetVolumeSnapshot => exampleGetVolumeSnapshot.id);
```
```python
import pulumi
import pulumi_azure as azure

example = azure.elasticsan.get(name="existing",
    resource_group_name="existing")
example_get_volume_group = azure.elasticsan.get_volume_group(name="existing",
    elastic_san_id=example.id)
example_get_volume_snapshot = azure.elasticsan.get_volume_snapshot(name="existing",
    volume_group_id=example_get_volume_group.id)
pulumi.export("id", example_get_volume_snapshot.id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = Azure.ElasticSan.Get.Invoke(new()
    {
        Name = "existing",
        ResourceGroupName = "existing",
    });

    var exampleGetVolumeGroup = Azure.ElasticSan.GetVolumeGroup.Invoke(new()
    {
        Name = "existing",
        ElasticSanId = example.Apply(getResult => getResult.Id),
    });

    var exampleGetVolumeSnapshot = Azure.ElasticSan.GetVolumeSnapshot.Invoke(new()
    {
        Name = "existing",
        VolumeGroupId = exampleGetVolumeGroup.Apply(getVolumeGroupResult => getVolumeGroupResult.Id),
    });

    return new Dictionary<string, object?>
    {
        ["id"] = exampleGetVolumeSnapshot.Apply(getVolumeSnapshotResult => getVolumeSnapshotResult.Id),
    };
});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/elasticsan"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := elasticsan.Get(ctx, &elasticsan.GetArgs{
			Name:              "existing",
			ResourceGroupName: "existing",
		}, nil)
		if err != nil {
			return err
		}
		exampleGetVolumeGroup, err := elasticsan.LookupVolumeGroup(ctx, &elasticsan.LookupVolumeGroupArgs{
			Name:         "existing",
			ElasticSanId: example.Id,
		}, nil)
		if err != nil {
			return err
		}
		exampleGetVolumeSnapshot, err := elasticsan.GetVolumeSnapshot(ctx, &elasticsan.GetVolumeSnapshotArgs{
			Name:          "existing",
			VolumeGroupId: exampleGetVolumeGroup.Id,
		}, nil)
		if err != nil {
			return err
		}
		ctx.Export("id", exampleGetVolumeSnapshot.Id)
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.elasticsan.ElasticsanFunctions;
import com.pulumi.azure.elasticsan.inputs.GetArgs;
import com.pulumi.azure.elasticsan.inputs.GetVolumeGroupArgs;
import com.pulumi.azure.elasticsan.inputs.GetVolumeSnapshotArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = ElasticsanFunctions.get(GetArgs.builder()
            .name("existing")
            .resourceGroupName("existing")
            .build());

        final var exampleGetVolumeGroup = ElasticsanFunctions.getVolumeGroup(GetVolumeGroupArgs.builder()
            .name("existing")
            .elasticSanId(example.applyValue(getResult -> getResult.id()))
            .build());

        final var exampleGetVolumeSnapshot = ElasticsanFunctions.getVolumeSnapshot(GetVolumeSnapshotArgs.builder()
            .name("existing")
            .volumeGroupId(exampleGetVolumeGroup.applyValue(getVolumeGroupResult -> getVolumeGroupResult.id()))
            .build());

        ctx.export("id", exampleGetVolumeSnapshot.applyValue(getVolumeSnapshotResult -> getVolumeSnapshotResult.id()));
    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: azure:elasticsan:get
      arguments:
        name: existing
        resourceGroupName: existing
  exampleGetVolumeGroup:
    fn::invoke:
      function: azure:elasticsan:getVolumeGroup
      arguments:
        name: existing
        elasticSanId: ${example.id}
  exampleGetVolumeSnapshot:
    fn::invoke:
      function: azure:elasticsan:getVolumeSnapshot
      arguments:
        name: existing
        volumeGroupId: ${exampleGetVolumeGroup.id}
outputs:
  id: ${exampleGetVolumeSnapshot.id}
```
<!--End PulumiCodeChooser -->
9
name" -The name of the Elastic SAN Volume Snapshot.
j
volumeGroupId" UThe Elastic SAN Volume Group ID within which the Elastic SAN Volume Snapshot exists.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

name" "D
sourceId" 4The resource ID from which the Snapshot is created.
"8
sourceVolumeSizeInGib The size of source volume.
"
volumeGroupId" ":

volumeName" (The source volume name of the Snapshot.
2¢$
;
	eventgrid	getDomain#azure:eventgrid/getDomain:getDomainÃUse this data source to access information about an existing EventGrid Domain

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = azure.eventgrid.getDomain({
    name: "my-eventgrid-domain",
    resourceGroupName: "example-resources",
});
export const eventgridDomainMappingTopic = example.then(example => example.inputMappingFields?.[0]?.topic);
```
```python
import pulumi
import pulumi_azure as azure

example = azure.eventgrid.get_domain(name="my-eventgrid-domain",
    resource_group_name="example-resources")
pulumi.export("eventgridDomainMappingTopic", example.input_mapping_fields[0].topic)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = Azure.EventGrid.GetDomain.Invoke(new()
    {
        Name = "my-eventgrid-domain",
        ResourceGroupName = "example-resources",
    });

    return new Dictionary<string, object?>
    {
        ["eventgridDomainMappingTopic"] = example.Apply(getDomainResult => getDomainResult.InputMappingFields[0]?.Topic),
    };
});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventgrid"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := eventgrid.LookupDomain(ctx, &eventgrid.LookupDomainArgs{
			Name:              "my-eventgrid-domain",
			ResourceGroupName: "example-resources",
		}, nil)
		if err != nil {
			return err
		}
		ctx.Export("eventgridDomainMappingTopic", example.InputMappingFields[0].Topic)
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.eventgrid.EventgridFunctions;
import com.pulumi.azure.eventgrid.inputs.GetDomainArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = EventgridFunctions.getDomain(GetDomainArgs.builder()
            .name("my-eventgrid-domain")
            .resourceGroupName("example-resources")
            .build());

        ctx.export("eventgridDomainMappingTopic", example.applyValue(getDomainResult -> getDomainResult.inputMappingFields()[0].topic()));
    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: azure:eventgrid:getDomain
      arguments:
        name: my-eventgrid-domain
        resourceGroupName: example-resources
outputs:
  eventgridDomainMappingTopic: ${example.inputMappingFields[0].topic}
```
<!--End PulumiCodeChooser -->
7
name" +The name of the EventGrid Domain resource.
^
resourceGroupName" EThe name of the resource group in which the EventGrid Domain exists.
"C
endpoint" 3The Endpoint associated with the EventGrid Domain.
"E
id" ;The provider-assigned unique ID for this managed resource.
"í

identitiesY*W:U
S
	eventgridgetDomainIdentity3azure:eventgrid/getDomainIdentity:getDomainIdentity)An `identity` block as documented below.
"≥
inboundIpRulesh*f:d
b
	eventgridgetDomainInboundIpRule=azure:eventgrid/getDomainInboundIpRule:getDomainInboundIpRule7One or more `inbound_ip_rule` blocks as defined below.
"Â
inputMappingDefaultValueså*â:Ü
É
	eventgrid!getDomainInputMappingDefaultValueSazure:eventgrid/getDomainInputMappingDefaultValue:getDomainInputMappingDefaultValue9A `input_mapping_default_values` block as defined below.
"Ω
inputMappingFieldst*r:p
n
	eventgridgetDomainInputMappingFieldEazure:eventgrid/getDomainInputMappingField:getDomainInputMappingField1A `input_mapping_fields` block as defined below.
"±
inputSchema" ùThe schema in which incoming events will be published to this domain. Possible values are `CloudEventSchemaV1_0`, `CustomEventSchema`, or `EventGridSchema`.
"H
location" 8The Azure Region in which this EventGrid Domain exists.
"

name" "U
primaryAccessKey" =The primary access key associated with the EventGrid Domain.
"c
publicNetworkAccessEnabled
 AWhether or not public network access is allowed for this server.
"
resourceGroupName" "Y
secondaryAccessKey" ?The secondary access key associated with the EventGrid Domain.
"B
tags2" 4A mapping of tags assigned to the EventGrid Domain.
2‡
J
	eventgridgetDomainTopic-azure:eventgrid/getDomainTopic:getDomainTopicÅUse this data source to access information about an existing EventGrid Domain Topic

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = azure.eventgrid.getDomainTopic({
    name: "my-eventgrid-domain-topic",
    resourceGroupName: "example-resources",
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.eventgrid.get_domain_topic(name="my-eventgrid-domain-topic",
    resource_group_name="example-resources")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = Azure.EventGrid.GetDomainTopic.Invoke(new()
    {
        Name = "my-eventgrid-domain-topic",
        ResourceGroupName = "example-resources",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventgrid"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := eventgrid.LookupDomainTopic(ctx, &eventgrid.LookupDomainTopicArgs{
			Name:              "my-eventgrid-domain-topic",
			ResourceGroupName: "example-resources",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.eventgrid.EventgridFunctions;
import com.pulumi.azure.eventgrid.inputs.GetDomainTopicArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = EventgridFunctions.getDomainTopic(GetDomainTopicArgs.builder()
            .name("my-eventgrid-domain-topic")
            .resourceGroupName("example-resources")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: azure:eventgrid:getDomainTopic
      arguments:
        name: my-eventgrid-domain-topic
        resourceGroupName: example-resources
```
<!--End PulumiCodeChooser -->
A

domainName" /The name of the EventGrid Domain Topic domain.
=
name" 1The name of the EventGrid Domain Topic resource.
d
resourceGroupName" KThe name of the resource group in which the EventGrid Domain Topic exists.
":

domainName" (The EventGrid Domain Topic Domain name.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

name" "
resourceGroupName" 2ô
J
	eventgridgetSystemTopic-azure:eventgrid/getSystemTopic:getSystemTopicÔUse this data source to access information about an existing EventGrid System Topic

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = azure.eventgrid.getSystemTopic({
    name: "eventgrid-system-topic",
    resourceGroupName: "example-resources",
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.eventgrid.get_system_topic(name="eventgrid-system-topic",
    resource_group_name="example-resources")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = Azure.EventGrid.GetSystemTopic.Invoke(new()
    {
        Name = "eventgrid-system-topic",
        ResourceGroupName = "example-resources",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventgrid"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := eventgrid.LookupSystemTopic(ctx, &eventgrid.LookupSystemTopicArgs{
			Name:              "eventgrid-system-topic",
			ResourceGroupName: "example-resources",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.eventgrid.EventgridFunctions;
import com.pulumi.azure.eventgrid.inputs.GetSystemTopicArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = EventgridFunctions.getSystemTopic(GetSystemTopicArgs.builder()
            .name("eventgrid-system-topic")
            .resourceGroupName("example-resources")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: azure:eventgrid:getSystemTopic
      arguments:
        name: eventgrid-system-topic
        resourceGroupName: example-resources
```
<!--End PulumiCodeChooser -->
=
name" 1The name of the EventGrid System Topic resource.
d
resourceGroupName" KThe name of the resource group in which the EventGrid System Topic exists.
"E
id" ;The provider-assigned unique ID for this managed resource.
"˘

identitiesh*f:d
b
	eventgridgetSystemTopicIdentity=azure:eventgrid/getSystemTopicIdentity:getSystemTopicIdentityÄAn `identity` block as defined below, which contains the Managed Service Identity information for this Event Grid System Topic.
"
location" "V
metricArmResourceId" ;The Metric ARM Resource ID of the Event Grid System Topic.
"

name" "
resourceGroupName" "M
sourceArmResourceId" 2The ID of the Event Grid System Topic ARM Source.
"S
tags2" EA mapping of tags which are assigned to the Event Grid System Topic.
"@
	topicType" /The Topic Type of the Event Grid System Topic.
2˘
8
	eventgridgetTopic!azure:eventgrid/getTopic:getTopicôUse this data source to access information about an existing EventGrid Topic

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = azure.eventgrid.getTopic({
    name: "my-eventgrid-topic",
    resourceGroupName: "example-resources",
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.eventgrid.get_topic(name="my-eventgrid-topic",
    resource_group_name="example-resources")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = Azure.EventGrid.GetTopic.Invoke(new()
    {
        Name = "my-eventgrid-topic",
        ResourceGroupName = "example-resources",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventgrid"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := eventgrid.LookupTopic(ctx, &eventgrid.LookupTopicArgs{
			Name:              "my-eventgrid-topic",
			ResourceGroupName: "example-resources",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.eventgrid.EventgridFunctions;
import com.pulumi.azure.eventgrid.inputs.GetTopicArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = EventgridFunctions.getTopic(GetTopicArgs.builder()
            .name("my-eventgrid-topic")
            .resourceGroupName("example-resources")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: azure:eventgrid:getTopic
      arguments:
        name: my-eventgrid-topic
        resourceGroupName: example-resources
```
<!--End PulumiCodeChooser -->
6
name" *The name of the EventGrid Topic resource.
]
resourceGroupName" DThe name of the resource group in which the EventGrid Topic exists.
"B
endpoint" 2The Endpoint associated with the EventGrid Topic.
"E
id" ;The provider-assigned unique ID for this managed resource.
"
location" "

name" "[
primaryAccessKey" CThe Primary Shared Access Key associated with the EventGrid Topic.
"
resourceGroupName" "_
secondaryAccessKey" EThe Secondary Shared Access Key associated with the EventGrid Topic.
"
tags2" 2£!
Z
eventhubgetAuthorizationRule8azure:eventhub/getAuthorizationRule:getAuthorizationRuleåUse this data source to access information about an existing Event Hubs Authorization Rule within an Event Hub.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const test = azure.eventhub.getAuthorizationRule({
    name: "test",
    namespaceName: testAzurermEventhubNamespace.name,
    eventhubName: testAzurermEventhub.name,
    resourceGroupName: testAzurermResourceGroup.name,
});
```
```python
import pulumi
import pulumi_azure as azure

test = azure.eventhub.get_authorization_rule(name="test",
    namespace_name=test_azurerm_eventhub_namespace["name"],
    eventhub_name=test_azurerm_eventhub["name"],
    resource_group_name=test_azurerm_resource_group["name"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var test = Azure.EventHub.GetAuthorizationRule.Invoke(new()
    {
        Name = "test",
        NamespaceName = testAzurermEventhubNamespace.Name,
        EventhubName = testAzurermEventhub.Name,
        ResourceGroupName = testAzurermResourceGroup.Name,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := eventhub.LookupAuthorizationRule(ctx, &eventhub.LookupAuthorizationRuleArgs{
			Name:              "test",
			NamespaceName:     testAzurermEventhubNamespace.Name,
			EventhubName:      testAzurermEventhub.Name,
			ResourceGroupName: testAzurermResourceGroup.Name,
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.eventhub.EventhubFunctions;
import com.pulumi.azure.eventhub.inputs.GetAuthorizationRuleArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var test = EventhubFunctions.getAuthorizationRule(GetAuthorizationRuleArgs.builder()
            .name("test")
            .namespaceName(testAzurermEventhubNamespace.name())
            .eventhubName(testAzurermEventhub.name())
            .resourceGroupName(testAzurermResourceGroup.name())
            .build());

    }
}
```
```yaml
variables:
  test:
    fn::invoke:
      function: azure:eventhub:getAuthorizationRule
      arguments:
        name: test
        namespaceName: ${testAzurermEventhubNamespace.name}
        eventhubName: ${testAzurermEventhub.name}
        resourceGroupName: ${testAzurermResourceGroup.name}
```
<!--End PulumiCodeChooser -->
8
eventhubName" $Specifies the name of the EventHub.

listenB
 
manageB
 X
name" LSpecifies the name of the EventHub Authorization Rule resource. be created.
O
namespaceName" :Specifies the name of the grandparent EventHub Namespace.
Å
resourceGroupName" hThe name of the resource group in which the EventHub Authorization Rule's grandparent Namespace exists.

sendB
 "
eventhubName" "E
id" ;The provider-assigned unique ID for this managed resource.
"
listenB
 "
manageB
 "

name" "
namespaceName" "d
primaryConnectionString" EThe Primary Connection String for the Event Hubs Authorization Rule.
"v
primaryConnectionStringAlias" RThe alias of the Primary Connection String for the Event Hubs Authorization Rule.
"I

primaryKey" 7The Primary Key for the Event Hubs Authorization Rule.
"
resourceGroupName" "h
secondaryConnectionString" GThe Secondary Connection String for the Event Hubs Authorization Rule.
"z
secondaryConnectionStringAlias" TThe alias of the Secondary Connection String for the Event Hubs Authorization Rule.
"M
secondaryKey" 9The Secondary Key for the Event Hubs Authorization Rule.
"
sendB
 2ª
<
eventhub
getCluster$azure:eventhub/getCluster:getClusterôUse this data source to access information about an existing EventHub.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = azure.eventhub.getCluster({
    name: "search-eventhub",
    resourceGroupName: "search-service",
});
export const eventhubId = example.then(example => example.id);
```
```python
import pulumi
import pulumi_azure as azure

example = azure.eventhub.get_cluster(name="search-eventhub",
    resource_group_name="search-service")
pulumi.export("eventhubId", example.id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = Azure.EventHub.GetCluster.Invoke(new()
    {
        Name = "search-eventhub",
        ResourceGroupName = "search-service",
    });

    return new Dictionary<string, object?>
    {
        ["eventhubId"] = example.Apply(getClusterResult => getClusterResult.Id),
    };
});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := eventhub.LookupCluster(ctx, &eventhub.LookupClusterArgs{
			Name:              "search-eventhub",
			ResourceGroupName: "search-service",
		}, nil)
		if err != nil {
			return err
		}
		ctx.Export("eventhubId", example.Id)
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.eventhub.EventhubFunctions;
import com.pulumi.azure.eventhub.inputs.GetClusterArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = EventhubFunctions.getCluster(GetClusterArgs.builder()
            .name("search-eventhub")
            .resourceGroupName("search-service")
            .build());

        ctx.export("eventhubId", example.applyValue(getClusterResult -> getClusterResult.id()));
    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: azure:eventhub:getCluster
      arguments:
        name: search-eventhub
        resourceGroupName: search-service
outputs:
  eventhubId: ${example.id}
```
<!--End PulumiCodeChooser -->
/
name" #The name of this EventHub Cluster.
[
resourceGroupName" BThe name of the Resource Group where the EventHub Cluster exists.
"E
id" ;The provider-assigned unique ID for this managed resource.
"2
location" "Location of the EventHub Cluster.
"

name" "
resourceGroupName" "1
skuName" "SKU name of the EventHub Cluster.
2â
K
eventhubgetConsumeGroup.azure:eventhub/getConsumeGroup:getConsumeGroupùUse this data source to access information about an existing Event Hubs Consumer Group within an Event Hub.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const test = azure.eventhub.getConsumeGroup({
    name: testAzurermEventhubConsumerGroup.name,
    namespaceName: testAzurermEventhubNamespace.name,
    eventhubName: testAzurermEventhub.name,
    resourceGroupName: testAzurermResourceGroup.name,
});
```
```python
import pulumi
import pulumi_azure as azure

test = azure.eventhub.get_consume_group(name=test_azurerm_eventhub_consumer_group["name"],
    namespace_name=test_azurerm_eventhub_namespace["name"],
    eventhub_name=test_azurerm_eventhub["name"],
    resource_group_name=test_azurerm_resource_group["name"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var test = Azure.EventHub.GetConsumeGroup.Invoke(new()
    {
        Name = testAzurermEventhubConsumerGroup.Name,
        NamespaceName = testAzurermEventhubNamespace.Name,
        EventhubName = testAzurermEventhub.Name,
        ResourceGroupName = testAzurermResourceGroup.Name,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := eventhub.GetConsumeGroup(ctx, &eventhub.GetConsumeGroupArgs{
			Name:              testAzurermEventhubConsumerGroup.Name,
			NamespaceName:     testAzurermEventhubNamespace.Name,
			EventhubName:      testAzurermEventhub.Name,
			ResourceGroupName: testAzurermResourceGroup.Name,
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.eventhub.EventhubFunctions;
import com.pulumi.azure.eventhub.inputs.GetConsumeGroupArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var test = EventhubFunctions.getConsumeGroup(GetConsumeGroupArgs.builder()
            .name(testAzurermEventhubConsumerGroup.name())
            .namespaceName(testAzurermEventhubNamespace.name())
            .eventhubName(testAzurermEventhub.name())
            .resourceGroupName(testAzurermResourceGroup.name())
            .build());

    }
}
```
```yaml
variables:
  test:
    fn::invoke:
      function: azure:eventhub:getConsumeGroup
      arguments:
        name: ${testAzurermEventhubConsumerGroup.name}
        namespaceName: ${testAzurermEventhubNamespace.name}
        eventhubName: ${testAzurermEventhub.name}
        resourceGroupName: ${testAzurermResourceGroup.name}
```
<!--End PulumiCodeChooser -->
8
eventhubName" $Specifies the name of the EventHub.
H
name" <Specifies the name of the EventHub Consumer Group resource.
O
namespaceName" :Specifies the name of the grandparent EventHub Namespace.
}
resourceGroupName" dThe name of the resource group in which the EventHub Consumer Group's grandparent Namespace exists.
"
eventhubName" "E
id" ;The provider-assigned unique ID for this managed resource.
"

name" "
namespaceName" "
resourceGroupName" "1
userMetadata" Specifies the user metadata.
2œ
?
eventhubgetEventHub&azure:eventhub/getEventHub:getEventHub®Use this data source to access information about an existing EventHub.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = azure.eventhub.getEventHub({
    name: "search-eventhub",
    resourceGroupName: "search-service",
    namespaceName: "search-eventhubns",
});
export const eventhubId = example.then(example => example.id);
```
```python
import pulumi
import pulumi_azure as azure

example = azure.eventhub.get_event_hub(name="search-eventhub",
    resource_group_name="search-service",
    namespace_name="search-eventhubns")
pulumi.export("eventhubId", example.id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = Azure.EventHub.GetEventHub.Invoke(new()
    {
        Name = "search-eventhub",
        ResourceGroupName = "search-service",
        NamespaceName = "search-eventhubns",
    });

    return new Dictionary<string, object?>
    {
        ["eventhubId"] = example.Apply(getEventHubResult => getEventHubResult.Id),
    };
});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := eventhub.LookupEventHub(ctx, &eventhub.LookupEventHubArgs{
			Name:              "search-eventhub",
			ResourceGroupName: "search-service",
			NamespaceName:     "search-eventhubns",
		}, nil)
		if err != nil {
			return err
		}
		ctx.Export("eventhubId", example.Id)
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.eventhub.EventhubFunctions;
import com.pulumi.azure.eventhub.inputs.GetEventHubArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = EventhubFunctions.getEventHub(GetEventHubArgs.builder()
            .name("search-eventhub")
            .resourceGroupName("search-service")
            .namespaceName("search-eventhubns")
            .build());

        ctx.export("eventhubId", example.applyValue(getEventHubResult -> getEventHubResult.id()));
    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: azure:eventhub:getEventHub
      arguments:
        name: search-eventhub
        resourceGroupName: search-service
        namespaceName: search-eventhubns
outputs:
  eventhubId: ${example.id}
```
<!--End PulumiCodeChooser -->
'
name" The name of this EventHub.
S
namespaceName" >The name of the EventHub Namespace where the EventHub exists.
S
resourceGroupName" :The name of the Resource Group where the EventHub exists.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

name" "
namespaceName" "@
partitionCount *The number of partitions in the EventHub.
"K
partitionIds*" 5The identifiers for the partitions of this EventHub.
"
resourceGroupName" 2ô"
Z
eventhubgetEventhubNamespace8azure:eventhub/getEventhubNamespace:getEventhubNamespaceˇUse this data source to access information about an existing EventHub Namespace.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = azure.eventhub.getNamespace({
    name: "search-eventhubns",
    resourceGroupName: "search-service",
});
export const eventhubNamespaceId = example.then(example => example.id);
```
```python
import pulumi
import pulumi_azure as azure

example = azure.eventhub.get_namespace(name="search-eventhubns",
    resource_group_name="search-service")
pulumi.export("eventhubNamespaceId", example.id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = Azure.EventHub.GetNamespace.Invoke(new()
    {
        Name = "search-eventhubns",
        ResourceGroupName = "search-service",
    });

    return new Dictionary<string, object?>
    {
        ["eventhubNamespaceId"] = example.Apply(getNamespaceResult => getNamespaceResult.Id),
    };
});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := eventhub.LookupNamespace(ctx, &eventhub.LookupNamespaceArgs{
			Name:              "search-eventhubns",
			ResourceGroupName: "search-service",
		}, nil)
		if err != nil {
			return err
		}
		ctx.Export("eventhubNamespaceId", example.Id)
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.eventhub.EventhubFunctions;
import com.pulumi.azure.eventhub.inputs.GetNamespaceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = EventhubFunctions.getNamespace(GetNamespaceArgs.builder()
            .name("search-eventhubns")
            .resourceGroupName("search-service")
            .build());

        ctx.export("eventhubNamespaceId", example.applyValue(getNamespaceResult -> getNamespaceResult.id()));
    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: azure:eventhub:getNamespace
      arguments:
        name: search-eventhubns
        resourceGroupName: search-service
outputs:
  eventhubNamespaceId: ${example.id}
```
<!--End PulumiCodeChooser -->
0
name" $The name of the EventHub Namespace.
]
resourceGroupName" DThe Name of the Resource Group where the EventHub Namespace exists.
"N
autoInflateEnabled
 4Is Auto Inflate enabled for the EventHub Namespace?
"P
capacity @The Capacity / Throughput Units for a `Standard` SKU namespace.
"`
dedicatedClusterId" FThe ID of the EventHub Dedicated Cluster where this Namespace exists.
"|
defaultPrimaryConnectionString" VThe primary connection string for the authorization
rule `RootManageSharedAccessKey`.
"é
#defaultPrimaryConnectionStringAlias" cThe alias of the primary connection string for the authorization
rule `RootManageSharedAccessKey`.
"h
defaultPrimaryKey" OThe primary access key for the authorization rule `RootManageSharedAccessKey`.
"Ä
 defaultSecondaryConnectionString" XThe secondary connection string for the
authorization rule `RootManageSharedAccessKey`.
"í
%defaultSecondaryConnectionStringAlias" eThe alias of the secondary connection string for the
authorization rule `RootManageSharedAccessKey`.
"l
defaultSecondaryKey" QThe secondary access key for the authorization rule `RootManageSharedAccessKey`.
"E
id" ;The provider-assigned unique ID for this managed resource.
"
kafkaEnabled
 "G
location" 7The Azure location where the EventHub Namespace exists
"m
maximumThroughputUnits OSpecifies the maximum number of throughput units when Auto Inflate is Enabled.
"

name" "
resourceGroupName" "&
sku" Defines which tier to use.
"E
tags2" 7A mapping of tags to assign to the EventHub Namespace.
2Å"
B
eventhubgetNamespace(azure:eventhub/getNamespace:getNamespaceˇUse this data source to access information about an existing EventHub Namespace.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = azure.eventhub.getNamespace({
    name: "search-eventhubns",
    resourceGroupName: "search-service",
});
export const eventhubNamespaceId = example.then(example => example.id);
```
```python
import pulumi
import pulumi_azure as azure

example = azure.eventhub.get_namespace(name="search-eventhubns",
    resource_group_name="search-service")
pulumi.export("eventhubNamespaceId", example.id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = Azure.EventHub.GetNamespace.Invoke(new()
    {
        Name = "search-eventhubns",
        ResourceGroupName = "search-service",
    });

    return new Dictionary<string, object?>
    {
        ["eventhubNamespaceId"] = example.Apply(getNamespaceResult => getNamespaceResult.Id),
    };
});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := eventhub.LookupNamespace(ctx, &eventhub.LookupNamespaceArgs{
			Name:              "search-eventhubns",
			ResourceGroupName: "search-service",
		}, nil)
		if err != nil {
			return err
		}
		ctx.Export("eventhubNamespaceId", example.Id)
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.eventhub.EventhubFunctions;
import com.pulumi.azure.eventhub.inputs.GetNamespaceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = EventhubFunctions.getNamespace(GetNamespaceArgs.builder()
            .name("search-eventhubns")
            .resourceGroupName("search-service")
            .build());

        ctx.export("eventhubNamespaceId", example.applyValue(getNamespaceResult -> getNamespaceResult.id()));
    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: azure:eventhub:getNamespace
      arguments:
        name: search-eventhubns
        resourceGroupName: search-service
outputs:
  eventhubNamespaceId: ${example.id}
```
<!--End PulumiCodeChooser -->
0
name" $The name of the EventHub Namespace.
]
resourceGroupName" DThe Name of the Resource Group where the EventHub Namespace exists.
"N
autoInflateEnabled
 4Is Auto Inflate enabled for the EventHub Namespace?
"P
capacity @The Capacity / Throughput Units for a `Standard` SKU namespace.
"`
dedicatedClusterId" FThe ID of the EventHub Dedicated Cluster where this Namespace exists.
"|
defaultPrimaryConnectionString" VThe primary connection string for the authorization
rule `RootManageSharedAccessKey`.
"é
#defaultPrimaryConnectionStringAlias" cThe alias of the primary connection string for the authorization
rule `RootManageSharedAccessKey`.
"h
defaultPrimaryKey" OThe primary access key for the authorization rule `RootManageSharedAccessKey`.
"Ä
 defaultSecondaryConnectionString" XThe secondary connection string for the
authorization rule `RootManageSharedAccessKey`.
"í
%defaultSecondaryConnectionStringAlias" eThe alias of the secondary connection string for the
authorization rule `RootManageSharedAccessKey`.
"l
defaultSecondaryKey" QThe secondary access key for the authorization rule `RootManageSharedAccessKey`.
"E
id" ;The provider-assigned unique ID for this managed resource.
"
kafkaEnabled
 "G
location" 7The Azure location where the EventHub Namespace exists
"m
maximumThroughputUnits OSpecifies the maximum number of throughput units when Auto Inflate is Enabled.
"

name" "
resourceGroupName" "&
sku" Defines which tier to use.
"E
tags2" 7A mapping of tags to assign to the EventHub Namespace.
2â#
u
eventhubgetNamespaceAuthorizationRuleJazure:eventhub/getNamespaceAuthorizationRule:getNamespaceAuthorizationRuleªUse this data source to access information about an Authorization Rule for an Event Hub Namespace.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = azure.eventhub.getNamespaceAuthorizationRule({
    name: "navi",
    resourceGroupName: "example-resources",
    namespaceName: "example-ns",
});
export const eventhubAuthorizationRuleId = example.then(example => example.id);
```
```python
import pulumi
import pulumi_azure as azure

example = azure.eventhub.get_namespace_authorization_rule(name="navi",
    resource_group_name="example-resources",
    namespace_name="example-ns")
pulumi.export("eventhubAuthorizationRuleId", example.id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = Azure.EventHub.GetNamespaceAuthorizationRule.Invoke(new()
    {
        Name = "navi",
        ResourceGroupName = "example-resources",
        NamespaceName = "example-ns",
    });

    return new Dictionary<string, object?>
    {
        ["eventhubAuthorizationRuleId"] = example.Apply(getNamespaceAuthorizationRuleResult => getNamespaceAuthorizationRuleResult.Id),
    };
});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := eventhub.LookupNamespaceAuthorizationRule(ctx, &eventhub.LookupNamespaceAuthorizationRuleArgs{
			Name:              "navi",
			ResourceGroupName: "example-resources",
			NamespaceName:     "example-ns",
		}, nil)
		if err != nil {
			return err
		}
		ctx.Export("eventhubAuthorizationRuleId", example.Id)
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.eventhub.EventhubFunctions;
import com.pulumi.azure.eventhub.inputs.GetNamespaceAuthorizationRuleArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = EventhubFunctions.getNamespaceAuthorizationRule(GetNamespaceAuthorizationRuleArgs.builder()
            .name("navi")
            .resourceGroupName("example-resources")
            .namespaceName("example-ns")
            .build());

        ctx.export("eventhubAuthorizationRuleId", example.applyValue(getNamespaceAuthorizationRuleResult -> getNamespaceAuthorizationRuleResult.id()));
    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: azure:eventhub:getNamespaceAuthorizationRule
      arguments:
        name: navi
        resourceGroupName: example-resources
        namespaceName: example-ns
outputs:
  eventhubAuthorizationRuleId: ${example.id}
```
<!--End PulumiCodeChooser -->
B
name" 6The name of the EventHub Authorization Rule resource.
C
namespaceName" .Specifies the name of the EventHub Namespace.
`
resourceGroupName" GThe name of the resource group in which the EventHub Namespace exists.
"E
id" ;The provider-assigned unique ID for this managed resource.
"X
listen
 JDoes this Authorization Rule have permissions to Listen to the Event Hub?
"X
manage
 JDoes this Authorization Rule have permissions to Manage to the Event Hub?
"

name" "
namespaceName" "d
primaryConnectionString" EThe Primary Connection String for the Event Hubs authorization Rule.
"v
primaryConnectionStringAlias" RThe alias of the Primary Connection String for the Event Hubs authorization Rule.
"I

primaryKey" 7The Primary Key for the Event Hubs authorization Rule.
"
resourceGroupName" "h
secondaryConnectionString" GThe Secondary Connection String for the Event Hubs authorization Rule.
"z
secondaryConnectionStringAlias" TThe alias of the Secondary Connection String for the Event Hubs authorization Rule.
"M
secondaryKey" 9The Secondary Key for the Event Hubs authorization Rule.
"T
send
 HDoes this Authorization Rule have permissions to Send to the Event Hub?
2 ]
0
eventhubgetSasazure:eventhub/getSas:getSasúZUse this data source to obtain a Shared Access Signature (SAS Token) for an existing Event Hub.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const exampleResourceGroup = new azure.core.ResourceGroup("example", {
    name: "example-resources",
    location: "West Europe",
});
const exampleEventHubNamespace = new azure.eventhub.EventHubNamespace("example", {
    name: "example-ehn",
    location: exampleResourceGroup.location,
    resourceGroupName: exampleResourceGroup.name,
    sku: "Basic",
});
const exampleEventHub = new azure.eventhub.EventHub("example", {
    name: "example-eh",
    namespaceName: exampleEventHubNamespace.name,
    resourceGroupName: exampleResourceGroup.name,
    partitionCount: 1,
    messageRetention: 1,
});
const exampleAuthorizationRule = new azure.eventhub.AuthorizationRule("example", {
    name: "example-ehar",
    namespaceName: exampleEventHubNamespace.name,
    eventhubName: exampleEventHub.name,
    resourceGroupName: exampleResourceGroup.name,
    listen: true,
    send: true,
    manage: true,
});
const example = azure.eventhub.getAuthorizationRuleOutput({
    name: exampleAuthorizationRule.name,
    namespaceName: exampleEventHubNamespace.name,
    eventhubName: exampleEventHub.name,
    resourceGroupName: exampleResourceGroup.name,
});
const exampleGetSas = example.apply(example => azure.eventhub.getSasOutput({
    connectionString: example.primaryConnectionString,
    expiry: "2023-06-23T00:00:00Z",
}));
```
```python
import pulumi
import pulumi_azure as azure

example_resource_group = azure.core.ResourceGroup("example",
    name="example-resources",
    location="West Europe")
example_event_hub_namespace = azure.eventhub.EventHubNamespace("example",
    name="example-ehn",
    location=example_resource_group.location,
    resource_group_name=example_resource_group.name,
    sku="Basic")
example_event_hub = azure.eventhub.EventHub("example",
    name="example-eh",
    namespace_name=example_event_hub_namespace.name,
    resource_group_name=example_resource_group.name,
    partition_count=1,
    message_retention=1)
example_authorization_rule = azure.eventhub.AuthorizationRule("example",
    name="example-ehar",
    namespace_name=example_event_hub_namespace.name,
    eventhub_name=example_event_hub.name,
    resource_group_name=example_resource_group.name,
    listen=True,
    send=True,
    manage=True)
example = azure.eventhub.get_authorization_rule_output(name=example_authorization_rule.name,
    namespace_name=example_event_hub_namespace.name,
    eventhub_name=example_event_hub.name,
    resource_group_name=example_resource_group.name)
example_get_sas = example.apply(lambda example: azure.eventhub.get_sas_output(connection_string=example.primary_connection_string,
    expiry="2023-06-23T00:00:00Z"))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var exampleResourceGroup = new Azure.Core.ResourceGroup("example", new()
    {
        Name = "example-resources",
        Location = "West Europe",
    });

    var exampleEventHubNamespace = new Azure.EventHub.EventHubNamespace("example", new()
    {
        Name = "example-ehn",
        Location = exampleResourceGroup.Location,
        ResourceGroupName = exampleResourceGroup.Name,
        Sku = "Basic",
    });

    var exampleEventHub = new Azure.EventHub.EventHub("example", new()
    {
        Name = "example-eh",
        NamespaceName = exampleEventHubNamespace.Name,
        ResourceGroupName = exampleResourceGroup.Name,
        PartitionCount = 1,
        MessageRetention = 1,
    });

    var exampleAuthorizationRule = new Azure.EventHub.AuthorizationRule("example", new()
    {
        Name = "example-ehar",
        NamespaceName = exampleEventHubNamespace.Name,
        EventhubName = exampleEventHub.Name,
        ResourceGroupName = exampleResourceGroup.Name,
        Listen = true,
        Send = true,
        Manage = true,
    });

    var example = Azure.EventHub.GetAuthorizationRule.Invoke(new()
    {
        Name = exampleAuthorizationRule.Name,
        NamespaceName = exampleEventHubNamespace.Name,
        EventhubName = exampleEventHub.Name,
        ResourceGroupName = exampleResourceGroup.Name,
    });

    var exampleGetSas = Azure.EventHub.GetSas.Invoke(new()
    {
        ConnectionString = example.Apply(getAuthorizationRuleResult => getAuthorizationRuleResult.PrimaryConnectionString),
        Expiry = "2023-06-23T00:00:00Z",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/core"
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/eventhub"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		exampleResourceGroup, err := core.NewResourceGroup(ctx, "example", &core.ResourceGroupArgs{
			Name:     pulumi.String("example-resources"),
			Location: pulumi.String("West Europe"),
		})
		if err != nil {
			return err
		}
		exampleEventHubNamespace, err := eventhub.NewEventHubNamespace(ctx, "example", &eventhub.EventHubNamespaceArgs{
			Name:              pulumi.String("example-ehn"),
			Location:          exampleResourceGroup.Location,
			ResourceGroupName: exampleResourceGroup.Name,
			Sku:               pulumi.String("Basic"),
		})
		if err != nil {
			return err
		}
		exampleEventHub, err := eventhub.NewEventHub(ctx, "example", &eventhub.EventHubArgs{
			Name:              pulumi.String("example-eh"),
			NamespaceName:     exampleEventHubNamespace.Name,
			ResourceGroupName: exampleResourceGroup.Name,
			PartitionCount:    pulumi.Int(1),
			MessageRetention:  pulumi.Int(1),
		})
		if err != nil {
			return err
		}
		exampleAuthorizationRule, err := eventhub.NewAuthorizationRule(ctx, "example", &eventhub.AuthorizationRuleArgs{
			Name:              pulumi.String("example-ehar"),
			NamespaceName:     exampleEventHubNamespace.Name,
			EventhubName:      exampleEventHub.Name,
			ResourceGroupName: exampleResourceGroup.Name,
			Listen:            pulumi.Bool(true),
			Send:              pulumi.Bool(true),
			Manage:            pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		example := eventhub.LookupAuthorizationRuleOutput(ctx, eventhub.GetAuthorizationRuleOutputArgs{
			Name:              exampleAuthorizationRule.Name,
			NamespaceName:     exampleEventHubNamespace.Name,
			EventhubName:      exampleEventHub.Name,
			ResourceGroupName: exampleResourceGroup.Name,
		}, nil)
		_ = example.ApplyT(func(example eventhub.GetAuthorizationRuleResult) (eventhub.GetSasResult, error) {
			return eventhub.GetSasResult(interface{}(eventhub.GetSasOutput(ctx, eventhub.GetSasOutputArgs{
				ConnectionString: example.PrimaryConnectionString,
				Expiry:           "2023-06-23T00:00:00Z",
			}, nil))), nil
		}).(eventhub.GetSasResultOutput)
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.core.ResourceGroup;
import com.pulumi.azure.core.ResourceGroupArgs;
import com.pulumi.azure.eventhub.EventHubNamespace;
import com.pulumi.azure.eventhub.EventHubNamespaceArgs;
import com.pulumi.azure.eventhub.EventHub;
import com.pulumi.azure.eventhub.EventHubArgs;
import com.pulumi.azure.eventhub.AuthorizationRule;
import com.pulumi.azure.eventhub.AuthorizationRuleArgs;
import com.pulumi.azure.eventhub.EventhubFunctions;
import com.pulumi.azure.eventhub.inputs.GetAuthorizationRuleArgs;
import com.pulumi.azure.eventhub.inputs.GetSasArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var exampleResourceGroup = new ResourceGroup("exampleResourceGroup", ResourceGroupArgs.builder()
            .name("example-resources")
            .location("West Europe")
            .build());

        var exampleEventHubNamespace = new EventHubNamespace("exampleEventHubNamespace", EventHubNamespaceArgs.builder()
            .name("example-ehn")
            .location(exampleResourceGroup.location())
            .resourceGroupName(exampleResourceGroup.name())
            .sku("Basic")
            .build());

        var exampleEventHub = new EventHub("exampleEventHub", EventHubArgs.builder()
            .name("example-eh")
            .namespaceName(exampleEventHubNamespace.name())
            .resourceGroupName(exampleResourceGroup.name())
            .partitionCount(1)
            .messageRetention(1)
            .build());

        var exampleAuthorizationRule = new AuthorizationRule("exampleAuthorizationRule", AuthorizationRuleArgs.builder()
            .name("example-ehar")
            .namespaceName(exampleEventHubNamespace.name())
            .eventhubName(exampleEventHub.name())
            .resourceGroupName(exampleResourceGroup.name())
            .listen(true)
            .send(true)
            .manage(true)
            .build());

        final var example = EventhubFunctions.getAuthorizationRule(GetAuthorizationRuleArgs.builder()
            .name(exampleAuthorizationRule.name())
            .namespaceName(exampleEventHubNamespace.name())
            .eventhubName(exampleEventHub.name())
            .resourceGroupName(exampleResourceGroup.name())
            .build());

        final var exampleGetSas = EventhubFunctions.getSas(GetSasArgs.builder()
            .connectionString(example.applyValue(getAuthorizationRuleResult -> getAuthorizationRuleResult).applyValue(example -> example.applyValue(getAuthorizationRuleResult -> getAuthorizationRuleResult.primaryConnectionString())))
            .expiry("2023-06-23T00:00:00Z")
            .build());

    }
}
```
```yaml
resources:
  exampleResourceGroup:
    type: azure:core:ResourceGroup
    name: example
    properties:
      name: example-resources
      location: West Europe
  exampleEventHubNamespace:
    type: azure:eventhub:EventHubNamespace
    name: example
    properties:
      name: example-ehn
      location: ${exampleResourceGroup.location}
      resourceGroupName: ${exampleResourceGroup.name}
      sku: Basic
  exampleEventHub:
    type: azure:eventhub:EventHub
    name: example
    properties:
      name: example-eh
      namespaceName: ${exampleEventHubNamespace.name}
      resourceGroupName: ${exampleResourceGroup.name}
      partitionCount: 1
      messageRetention: 1
  exampleAuthorizationRule:
    type: azure:eventhub:AuthorizationRule
    name: example
    properties:
      name: example-ehar
      namespaceName: ${exampleEventHubNamespace.name}
      eventhubName: ${exampleEventHub.name}
      resourceGroupName: ${exampleResourceGroup.name}
      listen: true
      send: true
      manage: true
variables:
  example:
    fn::invoke:
      function: azure:eventhub:getAuthorizationRule
      arguments:
        name: ${exampleAuthorizationRule.name}
        namespaceName: ${exampleEventHubNamespace.name}
        eventhubName: ${exampleEventHub.name}
        resourceGroupName: ${exampleResourceGroup.name}
  exampleGetSas:
    fn::invoke:
      function: azure:eventhub:getSas
      arguments:
        connectionString: ${example.primaryConnectionString}
        expiry: 2023-06-23T00:00:00Z
```
<!--End PulumiCodeChooser -->
[
connectionString" CThe connection string for the Event Hub to which this SAS applies.
j
expiry" \The expiration time and date of this SAS. Must be a valid ISO-8601 format time/date string.
"
connectionString" "
expiry" "E
id" ;The provider-assigned unique ID for this managed resource.
"A
sas" 6The computed Event Hub Shared Access Signature (SAS).
2Ä
`
eventhubgetServiceBusNamespace<azure:eventhub/getServiceBusNamespace:getServiceBusNamespaceÖUse this data source to access information about an existing ServiceBus Namespace.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = azure.servicebus.getNamespace({
    name: "examplenamespace",
    resourceGroupName: "example-resources",
});
export const location = example.then(example => example.location);
```
```python
import pulumi
import pulumi_azure as azure

example = azure.servicebus.get_namespace(name="examplenamespace",
    resource_group_name="example-resources")
pulumi.export("location", example.location)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = Azure.ServiceBus.GetNamespace.Invoke(new()
    {
        Name = "examplenamespace",
        ResourceGroupName = "example-resources",
    });

    return new Dictionary<string, object?>
    {
        ["location"] = example.Apply(getNamespaceResult => getNamespaceResult.Location),
    };
});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/servicebus"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := servicebus.LookupNamespace(ctx, &servicebus.LookupNamespaceArgs{
			Name:              "examplenamespace",
			ResourceGroupName: "example-resources",
		}, nil)
		if err != nil {
			return err
		}
		ctx.Export("location", example.Location)
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.servicebus.ServicebusFunctions;
import com.pulumi.azure.servicebus.inputs.GetNamespaceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = ServicebusFunctions.getNamespace(GetNamespaceArgs.builder()
            .name("examplenamespace")
            .resourceGroupName("example-resources")
            .build());

        ctx.export("location", example.applyValue(getNamespaceResult -> getNamespaceResult.location()));
    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: azure:servicebus:getNamespace
      arguments:
        name: examplenamespace
        resourceGroupName: example-resources
outputs:
  location: ${example.location}
```
<!--End PulumiCodeChooser -->
<
name" 0Specifies the name of the ServiceBus Namespace.
i
resourceGroupName" PSpecifies the name of the Resource Group where the ServiceBus Namespace exists.
":
capacity *The capacity of the ServiceBus Namespace.
"|
defaultPrimaryConnectionString" VThe primary connection string for the authorization
rule `RootManageSharedAccessKey`.
"h
defaultPrimaryKey" OThe primary access key for the authorization rule `RootManageSharedAccessKey`.
"Ä
 defaultSecondaryConnectionString" XThe secondary connection string for the
authorization rule `RootManageSharedAccessKey`.
"l
defaultSecondaryKey" QThe secondary access key for the authorization rule `RootManageSharedAccessKey`.
"<
endpoint" ,The URL to access the ServiceBus Namespace.
"E
id" ;The provider-assigned unique ID for this managed resource.
"]
location" MThe location of the Resource Group in which the ServiceBus Namespace exists.
"

name" "X
premiumMessagingPartitions 6The messaging partitions of the ServiceBus Namespace.
"
resourceGroupName" "7
sku" ,The Tier used for the ServiceBus Namespace.
":
tags2" ,A mapping of tags assigned to the resource.
2ì
Y
expressroutegetCircuitPeering6azure:expressroute/getCircuitPeering:getCircuitPeeringˇUse this data source to access information about an existing ExpressRoute Circuit Peering.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as azure from "@pulumi/azure";

const example = azure.expressroute.getCircuitPeering({
    peeringType: "example-peering",
    expressRouteCircuitName: "example-expressroute",
    resourceGroupName: "example-resources",
});
```
```python
import pulumi
import pulumi_azure as azure

example = azure.expressroute.get_circuit_peering(peering_type="example-peering",
    express_route_circuit_name="example-expressroute",
    resource_group_name="example-resources")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Azure = Pulumi.Azure;

return await Deployment.RunAsync(() => 
{
    var example = Azure.ExpressRoute.GetCircuitPeering.Invoke(new()
    {
        PeeringType = "example-peering",
        ExpressRouteCircuitName = "example-expressroute",
        ResourceGroupName = "example-resources",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-azure/sdk/v6/go/azure/expressroute"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := expressroute.GetCircuitPeering(ctx, &expressroute.GetCircuitPeeringArgs{
			PeeringType:             "example-peering",
			ExpressRouteCircuitName: "example-expressroute",
			ResourceGroupName:       "example-resources",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.azure.expressroute.ExpressrouteFunctions;
import com.pulumi.azure.expressroute.inputs.GetCircuitPeeringArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = ExpressrouteFunctions.getCircuitPeering(GetCircuitPeeringArgs.builder()
            .peeringType("example-peering")
            .expressRouteCircuitName("example-expressroute")
            .resourceGroupName("example-resources")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: azure:expressroute:getCircuitPeering
      arguments:
        peeringType: example-peering
        expressRouteCircuitName: example-expressroute
        resourceGroupName: example-resources
```
<!--End PulumiCodeChooser -->
ó
expressRouteCircuitName" xThe name of the ExpressRoute Circuit in which to create the Peering. Changing this forces a new resource to be created.
†
peeringType" åThe type of the ExpressRoute Circuit Peering. Acceptable values include `AzurePrivatePeering`, `AzurePublicPeering` and `MicrosoftPeering`.
¢
resourceGroupName" àThe name of the resource group in which to create the Express Route Circuit Peering. Changing this forces a new resource to be created.
"7
azureAsn 'The ASN used by Azure for the peering.
"
expressRouteCircuitName" "
gatewayManagerEtag" "E
id" ;The provider-assigned unique ID for this managed resource.
"1
ipv4Enabled
 Indicates if IPv4 is enabled.
"
peerAsn "A
peeringType" .The type of the ExpressRoute Circuit Peering.
"I
primaryAzurePort" 1The primary port used by Azure for this peering.
"A
primaryPeerAddressPrefix" !The primary peer address prefix.
"
resourceGroupName" "
routeFilterId" "M
secondaryAzurePort" 3The secondary port used by Azure for this peering.
"E
secondaryPeerAddressPrefix" #The secondary peer address prefix.
"
	sharedKey" "1
vlanId #The VLAN ID used for this peering.
:Ñ
r
domainservicesServiceInitialReplicaSetFazure:domainservices/ServiceInitialReplicaSet:ServiceInitialReplicaSetç
äã
domainControllerIpAddressesB*" dA list of subnet IP addresses for the domain controllers in the initial replica set, typically two.
y
externalAccessIpAddressB" XThe publicly routable IP address for the domain controllers in the initial replica set.
-
idB" !A unique ID for the replica set.
y
locationB" gThe Azure location where the Domain Service exists. Changing this forces a new resource to be created.
O
serviceStatusB" 8The current service status for the initial replica set.
É
subnetId" sThe ID of the subnet in which to place the initial replica set. Changing this forces a new resource to be created.
:Í
f
domainservicesServiceNotifications>azure:domainservices/ServiceNotifications:ServiceNotificationsˇ
¸|
additionalRecipientsB*" \A list of additional email addresses to notify when there are alerts in the managed domain.
Ñ
notifyDcAdminsB
 lWhether to notify members of the _AAD DC Administrators_ group when there are alerts in the managed domain.
u
notifyGlobalAdminsB
 YWhether to notify all Global Administrators when there are alerts in the managed domain.
:à
]
domainservicesServiceSecureLdap8azure:domainservices/ServiceSecureLdap:ServiceSecureLdap¶
£?
certificateExpiryB" $The expiry time of the certificate.
B
certificateThumbprintB" #The thumbprint of the certificate.
Å
enabled
 ÒWhether to enable secure LDAP for the managed domain. For more information, please see [official documentation on enabling LDAPS](https://docs.microsoft.com/azure/active-directory-domain-services/tutorial-configure-ldaps), paying particular attention to the section on network security to avoid unnecessarily exposing your service to Internet-borne bruteforce attacks.
r
externalAccessEnabledB
 SWhether to enable external access to LDAPS over the Internet. Defaults to `false`.
å
pfxCertificate" vThe certificate/private key to use for LDAPS, as a base64-encoded TripleDES-SHA1 encrypted PKCS#12 bundle (PFX file).
`
pfxCertificatePassword" BThe password to use for decrypting the PKCS#12 bundle (PFX file).
3
publicCertificateB" The public certificate.
:¬
W
domainservicesServiceSecurity4azure:domainservices/ServiceSecurity:ServiceSecurityÊ
„[
kerberosArmoringEnabledB
 :Whether to enable Kerberos Armoring. Defaults to `false`.
f
kerberosRc4EncryptionEnabledB
 @Whether to enable Kerberos RC4 Encryption. Defaults to `false`.
V
ntlmV1EnabledB
 ?Whether to enable legacy NTLM v1 support. Defaults to `false`.
{
syncKerberosPasswordsB
 \Whether to synchronize Kerberos password hashes to the managed domain. Defaults to `false`.
s
syncNtlmPasswordsB
 XWhether to synchronize NTLM password hashes to the managed domain. Defaults to `false`.
|
syncOnPremPasswordsB
 _Whether to synchronize on-premises password hashes to the managed domain. Defaults to `false`.
T
tlsV1EnabledB
 >Whether to enable legacy TLS v1 support. Defaults to `false`.
:Ô
l
domainservicesgetServiceNotificationBazure:domainservices/getServiceNotification:getServiceNotification˛
˚z
additionalRecipients*" \A list of additional email addresses to notify when there are alerts in the managed domain.
Ñ
notifyDcAdmins
 nWhethermembers of the _AAD DC Administrators_ group are notified when there are alerts in the managed domain.
v
notifyGlobalAdmins
 \Whether all Global Administrators are notified when there are alerts in the managed domain.
:‰
f
domainservicesgetServiceReplicaSet>azure:domainservices/getServiceReplicaSet:getServiceReplicaSet˘
ˆÅ
domainControllerIpAddresses*" \A list of subnet IP addresses for the domain controllers in the replica set, typically two.
o
externalAccessIpAddress" PThe publicly routable IP address for the domain controllers in the replica set.
(
id" The ID of the Domain Service.
E
location" 5The Azure location in which the replica set resides.
E
serviceStatus" 0The current service status for the replica set.
G
subnetId" 7The ID of the subnet in which the replica set resides.
:‰
f
domainservicesgetServiceSecureLdap>azure:domainservices/getServiceSecureLdap:getServiceSecureLdap˘
ˆ
certificateExpiry" 
certificateThumbprint" F
enabled
 7Whether secure LDAP is enabled for the managed domain.
]
externalAccessEnabled
 @Whether external access to LDAPS over the Internet, is enabled.

publicCertificate" :“
`
domainservicesgetServiceSecurity:azure:domainservices/getServiceSecurity:getServiceSecurityÌ
ÍT
kerberosArmoringEnabled
 5(Optional) Whether the Kerberos Armoring is enabled.
_
kerberosRc4EncryptionEnabled
 ;(Optional) Whether the Kerberos RC4 Encryption is enabled.
@
ntlmV1Enabled
 +Whether legacy NTLM v1 support is enabled.
f
syncKerberosPasswords
 IWhether Kerberos password hashes are synchronized to the managed domain.
^
syncNtlmPasswords
 EWhether NTLM password hashes are synchronized to the managed domain.
g
syncOnPremPasswords
 LWhether on-premises password hashes are synchronized to the managed domain.
>
tlsV1Enabled
 *Whether legacy TLS v1 support is enabled.
:ﬂ
M
	dynatraceMonitorIdentity/azure:dynatrace/MonitorIdentity:MonitorIdentityç
ä
principalIdB" 
tenantIdB" a
type" UThe type of identity used for the resource. Only possible value is `SystemAssigned`.
:‰
A
	dynatraceMonitorPlan'azure:dynatrace/MonitorPlan:MonitorPlanû
õ\
billingCycleB" FDifferent billing cycles. Possible values are `MONTHLY` and `WEEKLY`.
3
effectiveDateB" Date when plan was applied.
/
plan" #Plan id as published by Dynatrace.
U
	usageTypeB" BDifferent usage type. Possible values are `PAYG` and `COMMITTED`.
:Ô
A
	dynatraceMonitorUser'azure:dynatrace/MonitorUser:MonitorUser©
¶$
country" Country of the user.
P
email" CEmail of the user used by Dynatrace for contacting them if needed.
)
	firstName" First name of the user.
'
lastName" Last name of the user.
X
phoneNumber" Ephone number of the user by Dynatrace for contacting them if needed.
:Ω
Y
elasticcloudElasticsearchLogs6azure:elasticcloud/ElasticsearchLogs:ElasticsearchLogsﬂ
‹Œ
filteringTagsáBÑ*Å:
}
elasticcloudElasticsearchLogsFilteringTagNazure:elasticcloud/ElasticsearchLogsFilteringTag:ElasticsearchLogsFilteringTag3A list of `filtering_tag` blocks as defined above.
Å
sendActivityLogsB
 gSpecifies if the Azure Activity Logs should be sent to the Elasticsearch cluster. Defaults to `false`.
y
sendAzureadLogsB
 `Specifies if the AzureAD Logs should be sent to the Elasticsearch cluster. Defaults to `false`.
â
sendSubscriptionLogsB
 kSpecifies if the Azure Subscription Logs should be sent to the Elasticsearch cluster. Defaults to `false`.
:∏
}
elasticcloudElasticsearchLogsFilteringTagNazure:elasticcloud/ElasticsearchLogsFilteringTag:ElasticsearchLogsFilteringTag∂
≥ú
action" çSpecifies the type of action which should be taken when the Tag matches the `name` and `value`. Possible values are `Exclude` and `Include`.
J
name" >Specifies the name (key) of the Tag which should be filtered.
F
value" 9Specifies the value of the Tag which should be filtered.
:Ô
_
elasticcloudgetElasticsearchLog:azure:elasticcloud/getElasticsearchLog:getElasticsearchLogã
à”
filteringTagså*â:Ü
É
elasticcloudgetElasticsearchLogFilteringTagRazure:elasticcloud/getElasticsearchLogFilteringTag:getElasticsearchLogFilteringTag3A list of `filtering_tag` blocks as defined above.
d
sendActivityLogs
 LShould the Azure Activity Logs should be sent to the Elasticsearch cluster?
\
sendAzureadLogs
 EShould the AzureAD Logs should be sent to the Elasticsearch cluster?
l
sendSubscriptionLogs
 PShould the Azure Subscription Logs should be sent to the Elasticsearch cluster?
:ﬂ
É
elasticcloudgetElasticsearchLogFilteringTagRazure:elasticcloud/getElasticsearchLogFilteringTag:getElasticsearchLogFilteringTag÷
”]
action" OThe type of action which is taken when the Tag matches the `name` and `value`.
4
name" (The name of the Elasticsearch resource.
<
value" /The value of the Tag which should be filtered.
:Û
I

elasticsanElasticSanSku,azure:elasticsan/ElasticSanSku:ElasticSanSku•
¢ƒ
name" ∑The SKU name. Possible values are `Premium_LRS` and `Premium_ZRS`. Changing this forces a new resource to be created.

> **NOTE** `Premium_ZRS` SKU is only available in limited Azure regions including `France Central`, `North Europe`, `West Europe`, and `West US 2`. Please refer to this [document](https://azure.microsoft.com/updates/regional-expansion-azure-elastic-san-public-preview-is-now-available-in-more-regions) for more details.
Y
tierB" KThe SKU tier. The only possible value is `Premium`. Defaults to `Premium`.
:“
X

elasticsanVolumeCreateSource6azure:elasticsan/VolumeCreateSource:VolumeCreateSourceı
Úâ
sourceId" ySpecifies the ID of the source to create the Elastic SAN Volume from. Changing this forces a new resource to be created.
„

sourceType" –Specifies the type of the source to create the Elastic SAN Volume from. Possible values are `Disk`, `DiskRestorePoint`, `DiskSnapshot` and `VolumeSnapshot`. Changing this forces a new resource to be created.
:†
a

elasticsanVolumeGroupEncryption<azure:elasticsan/VolumeGroupEncryption:VolumeGroupEncryption∫
∑ä
&currentVersionedKeyExpirationTimestampB" ZThe timestamp of the expiration time for the current version of the customer managed key.
U
currentVersionedKeyIdB" 6The ID of the current versioned Key Vault Key in use.
Ö
keyVaultKeyId" pThe Key Vault key URI for Customer Managed Key encryption, which can be either a full URI or a versionless URI.
[
lastKeyRotationTimestampB" 9The timestamp of the last rotation of the Key Vault Key.
l
userAssignedIdentityIdB" LThe ID of the User Assigned Identity used by this Elastic SAN Volume Group.
:Ñ
[

elasticsanVolumeGroupIdentity8azure:elasticsan/VolumeGroupIdentity:VolumeGroupIdentity§
°z
identityIdsB*" cA list of the User Assigned Identity IDs that should be assigned to this Elastic SAN Volume Group.
~
principalIdB" iThe Principal ID associated with the Managed Service Identity assigned to this Elastic SAN Volume Group.
y
tenantIdB" gThe Tenant ID associated with this Managed Service Identity assigned to this Elastic SAN Volume Group.
ß
type" öSpecifies the type of Managed Identity that should be assigned to this Elastic SAN Volume Group. Possible values are `SystemAssigned` and `UserAssigned`.
:ˆ
d

elasticsanVolumeGroupNetworkRule>azure:elasticsan/VolumeGroupNetworkRule:VolumeGroupNetworkRuleç
äü
actionB" éThe action to take when the Subnet attempts to access this Elastic SAN Volume Group. The only possible value is `Allow`. Defaults to `Allow`.
f
subnetId" VThe ID of the Subnet which should be allowed to access this Elastic SAN Volume Group.
:Ö
7

elasticsangetSkus azure:elasticsan/getSkus:getSkusJ
H*
name" The name of this Elastic SAN.

tier" The SKU tier.
:°
j

elasticsangetVolumeGroupEncryptionBazure:elasticsan/getVolumeGroupEncryption:getVolumeGroupEncryption≤
Øà
&currentVersionedKeyExpirationTimestamp" ZThe timestamp of the expiration time for the current version of the Customer Managed Key.
S
currentVersionedKeyId" 6The ID of the current versioned Key Vault Key in use.
Ö
keyVaultKeyId" pThe Key Vault Key URI for Customer Managed Key encryption, which can be either a full URI or a versionless URI.
Y
lastKeyRotationTimestamp" 9The timestamp of the last rotation of the Key Vault Key.
j
userAssignedIdentityId" LThe ID of the User Assigned Identity used by this Elastic SAN Volume Group.
:§
d

elasticsangetVolumeGroupIdentity>azure:elasticsan/getVolumeGroupIdentity:getVolumeGroupIdentityª
∏i
identityIds*" TA list of the User Assigned Identity IDs assigned to this Elastic SAN Volume Group.
|
principalId" iThe Principal ID associated with the Managed Service Identity assigned to this Elastic SAN Volume Group.
w
tenantId" gThe Tenant ID associated with this Managed Service Identity assigned to this Elastic SAN Volume Group.
T
type" HThe type of Managed Identity assigned to this Elastic SAN Volume Group.
:–
m

elasticsangetVolumeGroupNetworkRuleDazure:elasticsan/getVolumeGroupNetworkRule:getVolumeGroupNetworkRuleﬁ
€s
action" eThe action to take when an access attempt to this Elastic SAN Volume Group from this Subnet is made.
d
subnetId" TThe ID of the Subnet from which access to this Elastic SAN Volume Group is allowed.
:•
J
	eventgridDomainIdentity-azure:eventgrid/DomainIdentity:DomainIdentity÷
”˛
identityIdsB*" ÊSpecifies a list of User Assigned Managed Identity IDs to be assigned to this Event Grid Domain.

> **NOTE:** This is required when `type` is set to `UserAssigned`

> **NOTE:** When `type` is set to `SystemAssigned`, The assigned `principal_id` and `tenant_id` can be retrieved after the Event Grid Domain has been created. More details are available below.
U
principalIdB" @The Principal ID associated with this Managed Service Identity.
O
tenantIdB" =The Tenant ID associated with this Managed Service Identity.
ß
type" öSpecifies the type of Managed Service Identity that should be configured on this Event Grid Domain. Possible values are `SystemAssigned`, `UserAssigned`.
:Ç
Y
	eventgridDomainInboundIpRule7azure:eventgrid/DomainInboundIpRule:DomainInboundIpRule§
°o
actionB" _The action to take when the rule is matched. Possible values are `Allow`. Defaults to `Allow`.
.
ipMask"  The IP mask (CIDR) to match on.
:‰
}
	eventgridDomainInputMappingDefaultValuesOazure:eventgrid/DomainInputMappingDefaultValues:DomainInputMappingDefaultValues‚
ﬂ°
dataVersionB" ãSpecifies the default data version of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
ù
	eventTypeB" âSpecifies the default event type of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
ò
subjectB" ÜSpecifies the default subject of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
:‰
h
	eventgridDomainInputMappingFieldsAazure:eventgrid/DomainInputMappingFields:DomainInputMappingFields˜
Ùô
dataVersionB" ÉSpecifies the data version of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
ï
	eventTimeB" ÅSpecifies the event time of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
ï
	eventTypeB" ÅSpecifies the event type of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
Ö
idB" ySpecifies the id of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
è
subjectB" ~Specifies the subject of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
ã
topicB" |Specifies the topic of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
:Æ)
}
	eventgridEventSubscriptionAdvancedFilterOazure:eventgrid/EventSubscriptionAdvancedFilter:EventSubscriptionAdvancedFilter¨(
©(

boolEquals§B°*û:õ
ò
	eventgrid(EventSubscriptionAdvancedFilterBoolEqualaazure:eventgrid/EventSubscriptionAdvancedFilterBoolEqual:EventSubscriptionAdvancedFilterBoolEqual;Compares a value of an event using a single boolean value.
Ô

isNotNulls§B°*û:õ
ò
	eventgrid(EventSubscriptionAdvancedFilterIsNotNullaazure:eventgrid/EventSubscriptionAdvancedFilterIsNotNull:EventSubscriptionAdvancedFilterIsNotNull:Evaluates if a value of an event isn't NULL or undefined.
…
isNullOrUndefinedsºBπ*∂:≥
∞
	eventgrid0EventSubscriptionAdvancedFilterIsNullOrUndefinedqazure:eventgrid/EventSubscriptionAdvancedFilterIsNullOrUndefined:EventSubscriptionAdvancedFilterIsNullOrUndefinedtEvaluates if a value of an event is NULL or undefined.

Each nested block consists of a key and a value(s) element.
¥
numberGreaterThanOrEquals—BŒ*À:»
≈
	eventgrid7EventSubscriptionAdvancedFilterNumberGreaterThanOrEqualazure:eventgrid/EventSubscriptionAdvancedFilterNumberGreaterThanOrEqual:EventSubscriptionAdvancedFilterNumberGreaterThanOrEqualCCompares a value of an event using a single floating point number.
ò
numberGreaterThansºBπ*∂:≥
∞
	eventgrid0EventSubscriptionAdvancedFilterNumberGreaterThanqazure:eventgrid/EventSubscriptionAdvancedFilterNumberGreaterThan:EventSubscriptionAdvancedFilterNumberGreaterThanCCompares a value of an event using a single floating point number.
è
numberInRanges∞B≠*™:ß
§
	eventgrid,EventSubscriptionAdvancedFilterNumberInRangeiazure:eventgrid/EventSubscriptionAdvancedFilterNumberInRange:EventSubscriptionAdvancedFilterNumberInRangeJCompares a value of an event using multiple floating point number ranges.
ı
	numberIns°Bû*õ:ò
ï
	eventgrid'EventSubscriptionAdvancedFilterNumberIn_azure:eventgrid/EventSubscriptionAdvancedFilterNumberIn:EventSubscriptionAdvancedFilterNumberInDCompares a value of an event using multiple floating point numbers.
®
numberLessThanOrEquals»B≈*¬:ø
º
	eventgrid4EventSubscriptionAdvancedFilterNumberLessThanOrEqualyazure:eventgrid/EventSubscriptionAdvancedFilterNumberLessThanOrEqual:EventSubscriptionAdvancedFilterNumberLessThanOrEqualCCompares a value of an event using a single floating point number.
å
numberLessThans≥B∞*≠:™
ß
	eventgrid-EventSubscriptionAdvancedFilterNumberLessThankazure:eventgrid/EventSubscriptionAdvancedFilterNumberLessThan:EventSubscriptionAdvancedFilterNumberLessThanCCompares a value of an event using a single floating point number.
õ
numberNotInRangesπB∂*≥:∞
≠
	eventgrid/EventSubscriptionAdvancedFilterNumberNotInRangeoazure:eventgrid/EventSubscriptionAdvancedFilterNumberNotInRange:EventSubscriptionAdvancedFilterNumberNotInRangeJCompares a value of an event using multiple floating point number ranges.
Å
numberNotIns™Bß*§:°
û
	eventgrid*EventSubscriptionAdvancedFilterNumberNotIneazure:eventgrid/EventSubscriptionAdvancedFilterNumberNotIn:EventSubscriptionAdvancedFilterNumberNotInDCompares a value of an event using multiple floating point numbers.
å
stringBeginsWithsπB∂*≥:∞
≠
	eventgrid/EventSubscriptionAdvancedFilterStringBeginsWithoazure:eventgrid/EventSubscriptionAdvancedFilterStringBeginsWith:EventSubscriptionAdvancedFilterStringBeginsWith;Compares a value of an event using multiple string values.
Ä
stringContains∞B≠*™:ß
§
	eventgrid,EventSubscriptionAdvancedFilterStringContainiazure:eventgrid/EventSubscriptionAdvancedFilterStringContain:EventSubscriptionAdvancedFilterStringContain;Compares a value of an event using multiple string values.
Ñ
stringEndsWiths≥B∞*≠:™
ß
	eventgrid-EventSubscriptionAdvancedFilterStringEndsWithkazure:eventgrid/EventSubscriptionAdvancedFilterStringEndsWith:EventSubscriptionAdvancedFilterStringEndsWith;Compares a value of an event using multiple string values.
Ï
	stringIns°Bû*õ:ò
ï
	eventgrid'EventSubscriptionAdvancedFilterStringIn_azure:eventgrid/EventSubscriptionAdvancedFilterStringIn:EventSubscriptionAdvancedFilterStringIn;Compares a value of an event using multiple string values.
ò
stringNotBeginsWiths¬Bø*º:π
∂
	eventgrid2EventSubscriptionAdvancedFilterStringNotBeginsWithuazure:eventgrid/EventSubscriptionAdvancedFilterStringNotBeginsWith:EventSubscriptionAdvancedFilterStringNotBeginsWith;Compares a value of an event using multiple string values.
å
stringNotContainsπB∂*≥:∞
≠
	eventgrid/EventSubscriptionAdvancedFilterStringNotContainoazure:eventgrid/EventSubscriptionAdvancedFilterStringNotContain:EventSubscriptionAdvancedFilterStringNotContain;Compares a value of an event using multiple string values.
ê
stringNotEndsWithsºBπ*∂:≥
∞
	eventgrid0EventSubscriptionAdvancedFilterStringNotEndsWithqazure:eventgrid/EventSubscriptionAdvancedFilterStringNotEndsWith:EventSubscriptionAdvancedFilterStringNotEndsWith;Compares a value of an event using multiple string values.
¯
stringNotIns™Bß*§:°
û
	eventgrid*EventSubscriptionAdvancedFilterStringNotIneazure:eventgrid/EventSubscriptionAdvancedFilterStringNotIn:EventSubscriptionAdvancedFilterStringNotIn;Compares a value of an event using multiple string values.
:¬
ò
	eventgrid(EventSubscriptionAdvancedFilterBoolEqualaazure:eventgrid/EventSubscriptionAdvancedFilterBoolEqual:EventSubscriptionAdvancedFilterBoolEqual§
°ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.

value
 :µ
ò
	eventgrid(EventSubscriptionAdvancedFilterIsNotNullaazure:eventgrid/EventSubscriptionAdvancedFilterIsNotNull:EventSubscriptionAdvancedFilterIsNotNulló
îë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
:Õ
∞
	eventgrid0EventSubscriptionAdvancedFilterIsNullOrUndefinedqazure:eventgrid/EventSubscriptionAdvancedFilterIsNullOrUndefined:EventSubscriptionAdvancedFilterIsNullOrUndefinedó
îë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
:⁄
∞
	eventgrid0EventSubscriptionAdvancedFilterNumberGreaterThanqazure:eventgrid/EventSubscriptionAdvancedFilterNumberGreaterThan:EventSubscriptionAdvancedFilterNumberGreaterThan§
°ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.

value :Ô
≈
	eventgrid7EventSubscriptionAdvancedFilterNumberGreaterThanOrEqualazure:eventgrid/EventSubscriptionAdvancedFilterNumberGreaterThanOrEqual:EventSubscriptionAdvancedFilterNumberGreaterThanOrEqual§
°ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.

value :˛
ï
	eventgrid'EventSubscriptionAdvancedFilterNumberIn_azure:eventgrid/EventSubscriptionAdvancedFilterNumberIn:EventSubscriptionAdvancedFilterNumberIn„
‡ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
…
values* ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:è
§
	eventgrid,EventSubscriptionAdvancedFilterNumberInRangeiazure:eventgrid/EventSubscriptionAdvancedFilterNumberInRange:EventSubscriptionAdvancedFilterNumberInRangeÂ
‚ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
À
values** ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:—
ß
	eventgrid-EventSubscriptionAdvancedFilterNumberLessThankazure:eventgrid/EventSubscriptionAdvancedFilterNumberLessThan:EventSubscriptionAdvancedFilterNumberLessThan§
°ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.

value :Ê
º
	eventgrid4EventSubscriptionAdvancedFilterNumberLessThanOrEqualyazure:eventgrid/EventSubscriptionAdvancedFilterNumberLessThanOrEqual:EventSubscriptionAdvancedFilterNumberLessThanOrEqual§
°ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.

value :á
û
	eventgrid*EventSubscriptionAdvancedFilterNumberNotIneazure:eventgrid/EventSubscriptionAdvancedFilterNumberNotIn:EventSubscriptionAdvancedFilterNumberNotIn„
‡ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
…
values* ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:ò
≠
	eventgrid/EventSubscriptionAdvancedFilterNumberNotInRangeoazure:eventgrid/EventSubscriptionAdvancedFilterNumberNotInRange:EventSubscriptionAdvancedFilterNumberNotInRangeÂ
‚ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
À
values** ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:ñ
≠
	eventgrid/EventSubscriptionAdvancedFilterStringBeginsWithoazure:eventgrid/EventSubscriptionAdvancedFilterStringBeginsWith:EventSubscriptionAdvancedFilterStringBeginsWith„
‡ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
…
values*" ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:ç
§
	eventgrid,EventSubscriptionAdvancedFilterStringContainiazure:eventgrid/EventSubscriptionAdvancedFilterStringContain:EventSubscriptionAdvancedFilterStringContain„
‡ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
…
values*" ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:ê
ß
	eventgrid-EventSubscriptionAdvancedFilterStringEndsWithkazure:eventgrid/EventSubscriptionAdvancedFilterStringEndsWith:EventSubscriptionAdvancedFilterStringEndsWith„
‡ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
…
values*" ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:˛
ï
	eventgrid'EventSubscriptionAdvancedFilterStringIn_azure:eventgrid/EventSubscriptionAdvancedFilterStringIn:EventSubscriptionAdvancedFilterStringIn„
‡ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
…
values*" ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:ü
∂
	eventgrid2EventSubscriptionAdvancedFilterStringNotBeginsWithuazure:eventgrid/EventSubscriptionAdvancedFilterStringNotBeginsWith:EventSubscriptionAdvancedFilterStringNotBeginsWith„
‡ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
…
values*" ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:ñ
≠
	eventgrid/EventSubscriptionAdvancedFilterStringNotContainoazure:eventgrid/EventSubscriptionAdvancedFilterStringNotContain:EventSubscriptionAdvancedFilterStringNotContain„
‡ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
…
values*" ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:ô
∞
	eventgrid0EventSubscriptionAdvancedFilterStringNotEndsWithqazure:eventgrid/EventSubscriptionAdvancedFilterStringNotEndsWith:EventSubscriptionAdvancedFilterStringNotEndsWith„
‡ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
…
values*" ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:á
û
	eventgrid*EventSubscriptionAdvancedFilterStringNotIneazure:eventgrid/EventSubscriptionAdvancedFilterStringNotIn:EventSubscriptionAdvancedFilterStringNotIn„
‡ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
…
values*" ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:›
í
	eventgrid&EventSubscriptionAzureFunctionEndpoint]azure:eventgrid/EventSubscriptionAzureFunctionEndpoint:EventSubscriptionAzureFunctionEndpoint≈
¬≤

functionId" üSpecifies the ID of the Function where the Event Subscription will receive events. This must be the functions ID in format {function_app.id}/functions/{name}.
?
maxEventsPerBatchB $Maximum number of events per batch.
J
preferredBatchSizeInKilobytesB #Preferred batch size in Kilobytes.
:ı
â
	eventgrid#EventSubscriptionDeadLetterIdentityWazure:eventgrid/EventSubscriptionDeadLetterIdentity:EventSubscriptionDeadLetterIdentityÊ
„ê
type" ÉSpecifies the type of Managed Service Identity that is used for dead lettering. Allowed value is `SystemAssigned`, `UserAssigned`.
N
userAssignedIdentityB" 0The user identity associated with the resource.
:Ô
É
	eventgrid!EventSubscriptionDeliveryIdentitySazure:eventgrid/EventSubscriptionDeliveryIdentity:EventSubscriptionDeliveryIdentityÊ
„ê
type" ÉSpecifies the type of Managed Service Identity that is used for event delivery. Allowed value is `SystemAssigned`, `UserAssigned`.
N
userAssignedIdentityB" 0The user identity associated with the resource.
:Ü
É
	eventgrid!EventSubscriptionDeliveryPropertySazure:eventgrid/EventSubscriptionDeliveryProperty:EventSubscriptionDeliveryProperty˝
˙G

headerName" 5The name of the header to send on to the destination
õ
secretB
 äTrue if the `value` is a secret and should be protected, otherwise false. If True, then this value won't be returned from Azure API calls
õ
sourceFieldB" ÖIf the `type` is `Dynamic`, then provide the payload field to be used as the value. Valid source fields differ by subscription type.
)
type" Either `Static` or `Dynamic`
H
valueB" 9If the `type` is `Static`, then provide the value to use
:—
t
	eventgridEventSubscriptionRetryPolicyIazure:eventgrid/EventSubscriptionRetryPolicy:EventSubscriptionRetryPolicyÿ
’Ò
eventTimeToLive ŸSpecifies the time to live (in minutes) for events. Supported range is `1` to `1440`. See [official documentation](https://docs.microsoft.com/azure/event-grid/manage-event-delivery#set-retry-policy) for more details.
_
maxDeliveryAttempts DSpecifies the maximum number of delivery retry attempts for events.
:™
≥
	eventgrid1EventSubscriptionStorageBlobDeadLetterDestinationsazure:eventgrid/EventSubscriptionStorageBlobDeadLetterDestination:EventSubscriptionStorageBlobDeadLetterDestinationÒ
Óf
storageAccountId" NSpecifies the id of the storage account id where the storage blob is located.
É
storageBlobContainerName" cSpecifies the name of the Storage blob container that is the destination of the deadletter events.
:»
è
	eventgrid%EventSubscriptionStorageQueueEndpoint[azure:eventgrid/EventSubscriptionStorageQueueEndpoint:EventSubscriptionStorageQueueEndpoint≥
∞X
queueMessageTimeToLiveInSecondsB /Storage queue message time to live in seconds.
k
	queueName" ZSpecifies the name of the storage queue where the Event Subscription will receive events.
g
storageAccountId" OSpecifies the id of the storage account id where the storage queue is located.
:”
z
	eventgridEventSubscriptionSubjectFilterMazure:eventgrid/EventSubscriptionSubjectFilter:EventSubscriptionSubjectFilter‘
—m
caseSensitiveB
 VSpecifies if `subject_begins_with` and `subject_ends_with` case sensitive. This value
p
subjectBeginsWithB" UA string to filter events for an event subscription based on a resource path prefix.
n
subjectEndsWithB" UA string to filter events for an event subscription based on a resource path suffix.
:®
Ä
	eventgrid EventSubscriptionWebhookEndpointQazure:eventgrid/EventSubscriptionWebhookEndpoint:EventSubscriptionWebhookEndpoint¢
ü≠
activeDirectoryAppIdOrUriB" âThe Azure Active Directory Application ID or URI to get the access token that will be included as the bearer token in delivery requests.
û
activeDirectoryTenantIdB" }The Azure Active Directory Tenant ID to get the access token that will be included as the bearer token in delivery requests.
_
baseUrlB" NThe base url of the webhook where the Event Subscription will receive events.
?
maxEventsPerBatchB $Maximum number of events per batch.
J
preferredBatchSizeInKilobytesB #Preferred batch size in Kilobytes.
^
url" SSpecifies the url of the webhook where the Event Subscription will receive events.
:Ò
S
	eventgridNamespaceIdentity3azure:eventgrid/NamespaceIdentity:NamespaceIdentityô
ñø
identityIdsB*" ßSpecifies a list of User Assigned Managed Identity IDs to be assigned to this Event Grid Namespace.

> **NOTE:** This is required when `type` is set to `UserAssigned`

principalIdB" 
tenantIdB" ™
type" ùSpecifies the type of Managed Service Identity that should be configured on this Event Grid Namespace. Possible values are `SystemAssigned`, `UserAssigned`.
:ã
b
	eventgridNamespaceInboundIpRule=azure:eventgrid/NamespaceInboundIpRule:NamespaceInboundIpRule§
°o
actionB" _The action to take when the rule is matched. Possible values are `Allow`. Defaults to `Allow`.
.
ipMask"  The IP mask (CIDR) to match on.
:˙
É
	eventgrid!NamespaceTopicSpacesConfigurationSazure:eventgrid/NamespaceTopicSpacesConfiguration:NamespaceTopicSpacesConfigurationÒ

Ó
≠
$alternativeAuthenticationNameSourcesB*" ¸Specifies a list of alternative sources for the client authentication name from the client certificate. Possible values are `ClientCertificateDns`, `ClientCertificateEmail`, `ClientCertificateIp`, `ClientCertificateSubject` and `ClientCertificateUri`.
∫
dynamicRoutingEnrichmentsÿB’*“:œ
Ã
	eventgrid9NamespaceTopicSpacesConfigurationDynamicRoutingEnrichmentÉazure:eventgrid/NamespaceTopicSpacesConfigurationDynamicRoutingEnrichment:NamespaceTopicSpacesConfigurationDynamicRoutingEnrichmentBOne or more `dynamic_routing_enrichment` blocks as defined below.
®
*maximumClientSessionsPerAuthenticationNameB tSpecifies the maximum number of client sessions per authentication name. Valid values can be between `1` and `100`.
¬
maximumSessionExpiryInHoursB úSpecifies the maximum session expiry interval allowed for all MQTT clients connecting to the Event Grid namespace. Valid values can be between `1` and `8`.
W
routeTopicIdB" ASpecifies the Event Grid topic resource ID to route messages to.
µ
staticRoutingEnrichments’B“*œ:Ã
…
	eventgrid8NamespaceTopicSpacesConfigurationStaticRoutingEnrichmentÅazure:eventgrid/NamespaceTopicSpacesConfigurationStaticRoutingEnrichment:NamespaceTopicSpacesConfigurationStaticRoutingEnrichmentAOne or more `static_routing_enrichment` blocks as defined below.
:ô
Ã
	eventgrid9NamespaceTopicSpacesConfigurationDynamicRoutingEnrichmentÉazure:eventgrid/NamespaceTopicSpacesConfigurationDynamicRoutingEnrichment:NamespaceTopicSpacesConfigurationDynamicRoutingEnrichmentH
F
key" The enrichment key.
#
value" The enrichment value.
:ñ
…
	eventgrid8NamespaceTopicSpacesConfigurationStaticRoutingEnrichmentÅazure:eventgrid/NamespaceTopicSpacesConfigurationStaticRoutingEnrichment:NamespaceTopicSpacesConfigurationStaticRoutingEnrichmentH
F
key" The enrichment key.
#
value" The enrichment value.
:Œ.
û
	eventgrid*SystemTopicEventSubscriptionAdvancedFiltereazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilter:SystemTopicEventSubscriptionAdvancedFilter™-
ß-ë

boolEquals≈B¬*ø:º
π
	eventgrid3SystemTopicEventSubscriptionAdvancedFilterBoolEqualwazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterBoolEqual:SystemTopicEventSubscriptionAdvancedFilterBoolEqual;Compares a value of an event using a single boolean value.
ê

isNotNulls≈B¬*ø:º
π
	eventgrid3SystemTopicEventSubscriptionAdvancedFilterIsNotNullwazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterIsNotNull:SystemTopicEventSubscriptionAdvancedFilterIsNotNull:Evaluates if a value of an event isn't NULL or undefined.
Î
isNullOrUndefinedsﬁB€*ÿ:’
“
	eventgrid;SystemTopicEventSubscriptionAdvancedFilterIsNullOrUndefinedáazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterIsNullOrUndefined:SystemTopicEventSubscriptionAdvancedFilterIsNullOrUndefinedtEvaluates if a value of an event is NULL or undefined.

Each nested block consists of a key and a value(s) element.
÷
numberGreaterThanOrEqualsÛB*Ì:Í
Á
	eventgridBSystemTopicEventSubscriptionAdvancedFilterNumberGreaterThanOrEqualïazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterNumberGreaterThanOrEqual:SystemTopicEventSubscriptionAdvancedFilterNumberGreaterThanOrEqualCCompares a value of an event using a single floating point number.
∫
numberGreaterThansﬁB€*ÿ:’
“
	eventgrid;SystemTopicEventSubscriptionAdvancedFilterNumberGreaterThanáazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterNumberGreaterThan:SystemTopicEventSubscriptionAdvancedFilterNumberGreaterThanCCompares a value of an event using a single floating point number.
∞
numberInRanges—BŒ*À:»
≈
	eventgrid7SystemTopicEventSubscriptionAdvancedFilterNumberInRangeazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterNumberInRange:SystemTopicEventSubscriptionAdvancedFilterNumberInRangeJCompares a value of an event using multiple floating point number ranges.
ñ
	numberIns¬Bø*º:π
∂
	eventgrid2SystemTopicEventSubscriptionAdvancedFilterNumberInuazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterNumberIn:SystemTopicEventSubscriptionAdvancedFilterNumberInDCompares a value of an event using multiple floating point numbers.
 
numberLessThanOrEqualsÍBÁ*‰:·
ﬁ
	eventgrid?SystemTopicEventSubscriptionAdvancedFilterNumberLessThanOrEqualèazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterNumberLessThanOrEqual:SystemTopicEventSubscriptionAdvancedFilterNumberLessThanOrEqualCCompares a value of an event using a single floating point number.
Æ
numberLessThans’B“*œ:Ã
…
	eventgrid8SystemTopicEventSubscriptionAdvancedFilterNumberLessThanÅazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterNumberLessThan:SystemTopicEventSubscriptionAdvancedFilterNumberLessThanCCompares a value of an event using a single floating point number.
Ω
numberNotInRanges€Bÿ*’:“
œ
	eventgrid:SystemTopicEventSubscriptionAdvancedFilterNumberNotInRangeÖazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterNumberNotInRange:SystemTopicEventSubscriptionAdvancedFilterNumberNotInRangeJCompares a value of an event using multiple floating point number ranges.
¢
numberNotInsÀB»*≈:¬
ø
	eventgrid5SystemTopicEventSubscriptionAdvancedFilterNumberNotIn{azure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterNumberNotIn:SystemTopicEventSubscriptionAdvancedFilterNumberNotInDCompares a value of an event using multiple floating point numbers.
Æ
stringBeginsWiths€Bÿ*’:“
œ
	eventgrid:SystemTopicEventSubscriptionAdvancedFilterStringBeginsWithÖazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterStringBeginsWith:SystemTopicEventSubscriptionAdvancedFilterStringBeginsWith;Compares a value of an event using multiple string values.
°
stringContains—BŒ*À:»
≈
	eventgrid7SystemTopicEventSubscriptionAdvancedFilterStringContainazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterStringContain:SystemTopicEventSubscriptionAdvancedFilterStringContain;Compares a value of an event using multiple string values.
¶
stringEndsWiths’B“*œ:Ã
…
	eventgrid8SystemTopicEventSubscriptionAdvancedFilterStringEndsWithÅazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterStringEndsWith:SystemTopicEventSubscriptionAdvancedFilterStringEndsWith;Compares a value of an event using multiple string values.
ç
	stringIns¬Bø*º:π
∂
	eventgrid2SystemTopicEventSubscriptionAdvancedFilterStringInuazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterStringIn:SystemTopicEventSubscriptionAdvancedFilterStringIn;Compares a value of an event using multiple string values.
∫
stringNotBeginsWiths‰B·*ﬁ:€
ÿ
	eventgrid=SystemTopicEventSubscriptionAdvancedFilterStringNotBeginsWithãazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterStringNotBeginsWith:SystemTopicEventSubscriptionAdvancedFilterStringNotBeginsWith;Compares a value of an event using multiple string values.
Æ
stringNotContains€Bÿ*’:“
œ
	eventgrid:SystemTopicEventSubscriptionAdvancedFilterStringNotContainÖazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterStringNotContain:SystemTopicEventSubscriptionAdvancedFilterStringNotContain;Compares a value of an event using multiple string values.
≤
stringNotEndsWithsﬁB€*ÿ:’
“
	eventgrid;SystemTopicEventSubscriptionAdvancedFilterStringNotEndsWitháazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterStringNotEndsWith:SystemTopicEventSubscriptionAdvancedFilterStringNotEndsWith;Compares a value of an event using multiple string values.
ô
stringNotInsÀB»*≈:¬
ø
	eventgrid5SystemTopicEventSubscriptionAdvancedFilterStringNotIn{azure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterStringNotIn:SystemTopicEventSubscriptionAdvancedFilterStringNotIn;Compares a value of an event using multiple string values.
:„
π
	eventgrid3SystemTopicEventSubscriptionAdvancedFilterBoolEqualwazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterBoolEqual:SystemTopicEventSubscriptionAdvancedFilterBoolEqual§
°ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.

value
 :÷
π
	eventgrid3SystemTopicEventSubscriptionAdvancedFilterIsNotNullwazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterIsNotNull:SystemTopicEventSubscriptionAdvancedFilterIsNotNulló
îë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
:Ô
“
	eventgrid;SystemTopicEventSubscriptionAdvancedFilterIsNullOrUndefinedáazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterIsNullOrUndefined:SystemTopicEventSubscriptionAdvancedFilterIsNullOrUndefinedó
îë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
:¸
“
	eventgrid;SystemTopicEventSubscriptionAdvancedFilterNumberGreaterThanáazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterNumberGreaterThan:SystemTopicEventSubscriptionAdvancedFilterNumberGreaterThan§
°ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.

value :ë
Á
	eventgridBSystemTopicEventSubscriptionAdvancedFilterNumberGreaterThanOrEqualïazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterNumberGreaterThanOrEqual:SystemTopicEventSubscriptionAdvancedFilterNumberGreaterThanOrEqual§
°ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.

value :ü
∂
	eventgrid2SystemTopicEventSubscriptionAdvancedFilterNumberInuazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterNumberIn:SystemTopicEventSubscriptionAdvancedFilterNumberIn„
‡ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
…
values* ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:∞
≈
	eventgrid7SystemTopicEventSubscriptionAdvancedFilterNumberInRangeazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterNumberInRange:SystemTopicEventSubscriptionAdvancedFilterNumberInRangeÂ
‚ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
À
values** ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:Û
…
	eventgrid8SystemTopicEventSubscriptionAdvancedFilterNumberLessThanÅazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterNumberLessThan:SystemTopicEventSubscriptionAdvancedFilterNumberLessThan§
°ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.

value :à
ﬁ
	eventgrid?SystemTopicEventSubscriptionAdvancedFilterNumberLessThanOrEqualèazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterNumberLessThanOrEqual:SystemTopicEventSubscriptionAdvancedFilterNumberLessThanOrEqual§
°ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.

value :®
ø
	eventgrid5SystemTopicEventSubscriptionAdvancedFilterNumberNotIn{azure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterNumberNotIn:SystemTopicEventSubscriptionAdvancedFilterNumberNotIn„
‡ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
…
values* ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:∫
œ
	eventgrid:SystemTopicEventSubscriptionAdvancedFilterNumberNotInRangeÖazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterNumberNotInRange:SystemTopicEventSubscriptionAdvancedFilterNumberNotInRangeÂ
‚ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
À
values** ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:∏
œ
	eventgrid:SystemTopicEventSubscriptionAdvancedFilterStringBeginsWithÖazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterStringBeginsWith:SystemTopicEventSubscriptionAdvancedFilterStringBeginsWith„
‡ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
…
values*" ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:Æ
≈
	eventgrid7SystemTopicEventSubscriptionAdvancedFilterStringContainazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterStringContain:SystemTopicEventSubscriptionAdvancedFilterStringContain„
‡ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
…
values*" ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:≤
…
	eventgrid8SystemTopicEventSubscriptionAdvancedFilterStringEndsWithÅazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterStringEndsWith:SystemTopicEventSubscriptionAdvancedFilterStringEndsWith„
‡ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
…
values*" ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:ü
∂
	eventgrid2SystemTopicEventSubscriptionAdvancedFilterStringInuazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterStringIn:SystemTopicEventSubscriptionAdvancedFilterStringIn„
‡ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
…
values*" ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:¡
ÿ
	eventgrid=SystemTopicEventSubscriptionAdvancedFilterStringNotBeginsWithãazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterStringNotBeginsWith:SystemTopicEventSubscriptionAdvancedFilterStringNotBeginsWith„
‡ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
…
values*" ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:∏
œ
	eventgrid:SystemTopicEventSubscriptionAdvancedFilterStringNotContainÖazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterStringNotContain:SystemTopicEventSubscriptionAdvancedFilterStringNotContain„
‡ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
…
values*" ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:ª
“
	eventgrid;SystemTopicEventSubscriptionAdvancedFilterStringNotEndsWitháazure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterStringNotEndsWith:SystemTopicEventSubscriptionAdvancedFilterStringNotEndsWith„
‡ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
…
values*" ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:®
ø
	eventgrid5SystemTopicEventSubscriptionAdvancedFilterStringNotIn{azure:eventgrid/SystemTopicEventSubscriptionAdvancedFilterStringNotIn:SystemTopicEventSubscriptionAdvancedFilterStringNotIn„
‡ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
…
values*" ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:˛
≥
	eventgrid1SystemTopicEventSubscriptionAzureFunctionEndpointsazure:eventgrid/SystemTopicEventSubscriptionAzureFunctionEndpoint:SystemTopicEventSubscriptionAzureFunctionEndpoint≈
¬≤

functionId" üSpecifies the ID of the Function where the Event Subscription will receive events. This must be the functions ID in format {function_app.id}/functions/{name}.
?
maxEventsPerBatchB $Maximum number of events per batch.
J
preferredBatchSizeInKilobytesB #Preferred batch size in Kilobytes.
:ñ
™
	eventgrid.SystemTopicEventSubscriptionDeadLetterIdentitymazure:eventgrid/SystemTopicEventSubscriptionDeadLetterIdentity:SystemTopicEventSubscriptionDeadLetterIdentityÊ
„ê
type" ÉSpecifies the type of Managed Service Identity that is used for dead lettering. Allowed value is `SystemAssigned`, `UserAssigned`.
N
userAssignedIdentityB" 0The user identity associated with the resource.
:ê
§
	eventgrid,SystemTopicEventSubscriptionDeliveryIdentityiazure:eventgrid/SystemTopicEventSubscriptionDeliveryIdentity:SystemTopicEventSubscriptionDeliveryIdentityÊ
„ê
type" ÉSpecifies the type of Managed Service Identity that is used for event delivery. Allowed value is `SystemAssigned`, `UserAssigned`.
N
userAssignedIdentityB" 0The user identity associated with the resource.
:∑
§
	eventgrid,SystemTopicEventSubscriptionDeliveryPropertyiazure:eventgrid/SystemTopicEventSubscriptionDeliveryProperty:SystemTopicEventSubscriptionDeliveryPropertyç
äH

headerName" 6The name of the header to send on to the destination.
®
secretB
 óSet to `true` if the `value` is a secret and should be protected, otherwise `false`. If `true` then this value won't be returned from Azure API calls.
õ
sourceFieldB" ÖIf the `type` is `Dynamic`, then provide the payload field to be used as the value. Valid source fields differ by subscription type.
*
type" Either `Static` or `Dynamic`.
I
valueB" :If the `type` is `Static`, then provide the value to use.
:Û
ï
	eventgrid'SystemTopicEventSubscriptionRetryPolicy_azure:eventgrid/SystemTopicEventSubscriptionRetryPolicy:SystemTopicEventSubscriptionRetryPolicyÿ
’Ò
eventTimeToLive ŸSpecifies the time to live (in minutes) for events. Supported range is `1` to `1440`. See [official documentation](https://docs.microsoft.com/azure/event-grid/manage-event-delivery#set-retry-policy) for more details.
_
maxDeliveryAttempts DSpecifies the maximum number of delivery retry attempts for events.
:Ã
’
	eventgrid<SystemTopicEventSubscriptionStorageBlobDeadLetterDestinationâazure:eventgrid/SystemTopicEventSubscriptionStorageBlobDeadLetterDestination:SystemTopicEventSubscriptionStorageBlobDeadLetterDestinationÒ
Óf
storageAccountId" NSpecifies the id of the storage account id where the storage blob is located.
É
storageBlobContainerName" cSpecifies the name of the Storage blob container that is the destination of the deadletter events.
:È
∞
	eventgrid0SystemTopicEventSubscriptionStorageQueueEndpointqazure:eventgrid/SystemTopicEventSubscriptionStorageQueueEndpoint:SystemTopicEventSubscriptionStorageQueueEndpoint≥
∞X
queueMessageTimeToLiveInSecondsB /Storage queue message time to live in seconds.
k
	queueName" ZSpecifies the name of the storage queue where the Event Subscription will receive events.
g
storageAccountId" OSpecifies the id of the storage account id where the storage queue is located.
:ı
õ
	eventgrid)SystemTopicEventSubscriptionSubjectFiltercazure:eventgrid/SystemTopicEventSubscriptionSubjectFilter:SystemTopicEventSubscriptionSubjectFilter‘
—m
caseSensitiveB
 VSpecifies if `subject_begins_with` and `subject_ends_with` case sensitive. This value
p
subjectBeginsWithB" UA string to filter events for an event subscription based on a resource path prefix.
n
subjectEndsWithB" UA string to filter events for an event subscription based on a resource path suffix.
:…
°
	eventgrid+SystemTopicEventSubscriptionWebhookEndpointgazure:eventgrid/SystemTopicEventSubscriptionWebhookEndpoint:SystemTopicEventSubscriptionWebhookEndpoint¢
ü≠
activeDirectoryAppIdOrUriB" âThe Azure Active Directory Application ID or URI to get the access token that will be included as the bearer token in delivery requests.
û
activeDirectoryTenantIdB" }The Azure Active Directory Tenant ID to get the access token that will be included as the bearer token in delivery requests.
_
baseUrlB" NThe base url of the webhook where the Event Subscription will receive events.
?
maxEventsPerBatchB $Maximum number of events per batch.
J
preferredBatchSizeInKilobytesB #Preferred batch size in Kilobytes.
^
url" SSpecifies the url of the webhook where the Event Subscription will receive events.
:∆
Y
	eventgridSystemTopicIdentity7azure:eventgrid/SystemTopicIdentity:SystemTopicIdentityË
Âä
identityIdsB*" ÚSpecifies a list of User Assigned Managed Identity IDs to be assigned to this Event Grid System Topic.

> **NOTE:** This is required when `type` is set to `UserAssigned`

> **NOTE:** When `type` is set to `SystemAssigned`, The assigned `principal_id` and `tenant_id` can be retrieved after the Event Grid System Topic has been created. More details are available below.
U
principalIdB" @The Principal ID associated with this Managed Service Identity.
O
tenantIdB" =The Tenant ID associated with this Managed Service Identity.
≠
type" †Specifies the type of Managed Service Identity that should be configured on this Event Grid System Topic. Possible values are `SystemAssigned`, `UserAssigned`.
:ü
G
	eventgridTopicIdentity+azure:eventgrid/TopicIdentity:TopicIdentity”
–¸
identityIdsB*" ‰Specifies a list of User Assigned Managed Identity IDs to be assigned to this Event Grid Topic.

> **NOTE:** This is required when `type` is set to `UserAssigned`

> **NOTE:** When `type` is set to `SystemAssigned`, The assigned `principal_id` and `tenant_id` can be retrieved after the Event Grid Topic has been created. More details are available below.
U
principalIdB" @The Principal ID associated with this Managed Service Identity.
O
tenantIdB" =The Tenant ID associated with this Managed Service Identity.
¶
type" ôSpecifies the type of Managed Service Identity that should be configured on this Event Grid Topic. Possible values are `SystemAssigned`, `UserAssigned`.
:ˇ
V
	eventgridTopicInboundIpRule5azure:eventgrid/TopicInboundIpRule:TopicInboundIpRule§
°o
actionB" _The action to take when the rule is matched. Possible values are `Allow`. Defaults to `Allow`.
.
ipMask"  The IP mask (CIDR) to match on.
:·
z
	eventgridTopicInputMappingDefaultValuesMazure:eventgrid/TopicInputMappingDefaultValues:TopicInputMappingDefaultValues‚
ﬂ°
dataVersionB" ãSpecifies the default data version of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
ù
	eventTypeB" âSpecifies the default event type of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
ò
subjectB" ÜSpecifies the default subject of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
:·
e
	eventgridTopicInputMappingFields?azure:eventgrid/TopicInputMappingFields:TopicInputMappingFields˜
Ùô
dataVersionB" ÉSpecifies the data version of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
ï
	eventTimeB" ÅSpecifies the event time of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
ï
	eventTypeB" ÅSpecifies the event type of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
Ö
idB" ySpecifies the id of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
è
subjectB" ~Specifies the subject of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
ã
topicB" |Specifies the topic of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
:Ã
S
	eventgridgetDomainIdentity3azure:eventgrid/getDomainIdentity:getDomainIdentityÙ
Òg
identityIds*" RThe list of User Assigned Managed Identity IDs assigned to this EventGrid Domain.
U
principalId" BThe Principal ID of the System Assigned Managed Service Identity.
O
tenantId" ?The Tenant ID of the System Assigned Managed Service Identity.
^
type" RThe type of Managed Service Identity that is configured on this EventGrid Domain.
:Ù
b
	eventgridgetDomainInboundIpRule=azure:eventgrid/getDomainInboundIpRule:getDomainInboundIpRuleç
äX
action" JThe action to take when the rule is matched. Possible values are `Allow`.
.
ipMask"  The IP mask (CIDR) to match on.
:¿
É
	eventgrid!getDomainInputMappingDefaultValueSazure:eventgrid/getDomainInputMappingDefaultValue:getDomainInputMappingDefaultValue∑
¥i
dataVersion" VSpecifies the default data version of the EventGrid Event associated with the domain.
e
	eventType" TSpecifies the default event type of the EventGrid Event associated with the domain.
`
subject" QSpecifies the default subject of the EventGrid Event associated with the domain.
:Ø
n
	eventgridgetDomainInputMappingFieldEazure:eventgrid/getDomainInputMappingField:getDomainInputMappingFieldº
πi
dataVersion" VSpecifies the default data version of the EventGrid Event associated with the domain.
]
	eventTime" LSpecifies the event time of the EventGrid Event associated with the domain.
e
	eventType" TSpecifies the default event type of the EventGrid Event associated with the domain.
N
id" DSpecifies the id of the EventGrid Event associated with the domain.
`
subject" QSpecifies the default subject of the EventGrid Event associated with the domain.
T
topic" GSpecifies the topic of the EventGrid Event associated with the domain.
:—
b
	eventgridgetSystemTopicIdentity=azure:eventgrid/getSystemTopicIdentity:getSystemTopicIdentityÍ
Án
identityIds*" YThe list of User Assigned Managed Identity IDs assigned to this Event Grid System Topic.
à
principalId" uThe Principal ID of the System Assigned Managed Service Identity that is configured on this Event Grid System Topic.
Ç
tenantId" rThe Tenant ID of the System Assigned Managed Service Identity that is configured on this Event Grid System Topic.
e
type" YThe type of Managed Service Identity that is configured on this Event Grid System Topic.
:£
H
eventhubDomainIdentity,azure:eventhub/DomainIdentity:DomainIdentity÷
”˛
identityIdsB*" ÊSpecifies a list of User Assigned Managed Identity IDs to be assigned to this Event Grid Domain.

> **NOTE:** This is required when `type` is set to `UserAssigned`

> **NOTE:** When `type` is set to `SystemAssigned`, The assigned `principal_id` and `tenant_id` can be retrieved after the Event Grid Domain has been created. More details are available below.
U
principalIdB" @The Principal ID associated with this Managed Service Identity.
O
tenantIdB" =The Tenant ID associated with this Managed Service Identity.
ß
type" öSpecifies the type of Managed Service Identity that should be configured on this Event Grid Domain. Possible values are `SystemAssigned`, `UserAssigned`.
:Ä
W
eventhubDomainInboundIpRule6azure:eventhub/DomainInboundIpRule:DomainInboundIpRule§
°o
actionB" _The action to take when the rule is matched. Possible values are `Allow`. Defaults to `Allow`.
.
ipMask"  The IP mask (CIDR) to match on.
:‚
{
eventhubDomainInputMappingDefaultValuesNazure:eventhub/DomainInputMappingDefaultValues:DomainInputMappingDefaultValues‚
ﬂ°
dataVersionB" ãSpecifies the default data version of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
ù
	eventTypeB" âSpecifies the default event type of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
ò
subjectB" ÜSpecifies the default subject of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
:‚
f
eventhubDomainInputMappingFields@azure:eventhub/DomainInputMappingFields:DomainInputMappingFields˜
Ùô
dataVersionB" ÉSpecifies the data version of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
ï
	eventTimeB" ÅSpecifies the event time of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
ï
	eventTypeB" ÅSpecifies the event type of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
Ö
idB" ySpecifies the id of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
è
subjectB" ~Specifies the subject of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
ã
topicB" |Specifies the topic of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
:∏
`
eventhubEventGridTopicIdentity<azure:eventhub/EventGridTopicIdentity:EventGridTopicIdentity”
–¸
identityIdsB*" ‰Specifies a list of User Assigned Managed Identity IDs to be assigned to this Event Grid Topic.

> **NOTE:** This is required when `type` is set to `UserAssigned`

> **NOTE:** When `type` is set to `SystemAssigned`, The assigned `principal_id` and `tenant_id` can be retrieved after the Event Grid Topic has been created. More details are available below.
U
principalIdB" @The Principal ID associated with this Managed Service Identity.
O
tenantIdB" =The Tenant ID associated with this Managed Service Identity.
¶
type" ôSpecifies the type of Managed Service Identity that should be configured on this Event Grid Topic. Possible values are `SystemAssigned`, `UserAssigned`.
:ò
o
eventhubEventGridTopicInboundIpRuleFazure:eventhub/EventGridTopicInboundIpRule:EventGridTopicInboundIpRule§
°o
actionB" _The action to take when the rule is matched. Possible values are `Allow`. Defaults to `Allow`.
.
ipMask"  The IP mask (CIDR) to match on.
:˚
ì
eventhub'EventGridTopicInputMappingDefaultValues^azure:eventhub/EventGridTopicInputMappingDefaultValues:EventGridTopicInputMappingDefaultValues‚
ﬂ°
dataVersionB" ãSpecifies the default data version of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
ù
	eventTypeB" âSpecifies the default event type of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
ò
subjectB" ÜSpecifies the default subject of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
:˙
~
eventhub EventGridTopicInputMappingFieldsPazure:eventhub/EventGridTopicInputMappingFields:EventGridTopicInputMappingFields˜
Ùô
dataVersionB" ÉSpecifies the data version of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
ï
	eventTimeB" ÅSpecifies the event time of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
ï
	eventTypeB" ÅSpecifies the event type of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
Ö
idB" ySpecifies the id of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
è
subjectB" ~Specifies the subject of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
ã
topicB" |Specifies the topic of the EventGrid Event to associate with the domain. Changing this forces a new resource to be created.
:ñ
l
eventhubEventHubCaptureDescriptionDazure:eventhub/EventHubCaptureDescription:EventHubCaptureDescription•
¢Õ
destinationì:ê
ç
eventhub%EventHubCaptureDescriptionDestinationZazure:eventhub/EventHubCaptureDescriptionDestination:EventHubCaptureDescriptionDestination(A `destination` block as defined below.
@
enabled
 1Specifies if the Capture Description is Enabled.
w
encoding" gSpecifies the Encoding used for the Capture Description. Possible values are `Avro` and `AvroDeflate`.
Æ
intervalInSecondsB íSpecifies the time interval in seconds at which the capture will happen. Values can be between `60` and `900` seconds. Defaults to `300` seconds.
Œ
sizeLimitInBytesB ≥Specifies the amount of data built up in your EventHub before a Capture Operation occurs. Value should be between `10485760` and `524288000` bytes. Defaults to `314572800` bytes.
í
skipEmptyArchivesB
 wSpecifies if empty files should not be emitted if no events occur during the Capture time window. Defaults to `false`.
:˝
ç
eventhub%EventHubCaptureDescriptionDestinationZazure:eventhub/EventHubCaptureDescriptionDestination:EventHubCaptureDescriptionDestinationÍ
Á˜
archiveNameFormat" ›The Blob naming convention for archiving. e.g. `{Namespace}/{EventHub}/{PartitionId}/{Year}/{Month}/{Day}/{Hour}/{Minute}/{Second}`. Here all the parameters (Namespace,EventHub .. etc) are mandatory irrespective of order
v
blobContainerName" ]The name of the Container within the Blob Storage Account where messages should be archived.
í
name" ÖThe Name of the Destination where the capture should take place. At this time the only supported value is `EventHubArchive.AzureBlockBlob`.

> At this time it's only possible to Capture EventHub messages to Blob Storage. There's [a Feature Request for the Azure SDK to add support for Capturing messages to Azure Data Lake here](https://github.com/Azure/azure-rest-api-specs/issues/2255).
^
storageAccountId" FThe ID of the Blob Storage Account where messages should be archived.
:∑
i
eventhubEventHubNamespaceIdentityBazure:eventhub/EventHubNamespaceIdentity:EventHubNamespaceIdentity…
∆Ì
identityIdsB*" ’Specifies a list of User Assigned Managed Identity IDs to be assigned to this EventHub namespace.

> **NOTE:** This is required when `type` is set to `UserAssigned` or `SystemAssigned, UserAssigned`.

> **Note:** Due to the limitation of the current Azure API, once an EventHub Namespace has been assigned an identity, it cannot be removed.
U
principalIdB" @The Principal ID associated with this Managed Service Identity.
O
tenantIdB" =The Tenant ID associated with this Managed Service Identity.
´
type" ûSpecifies the type of Managed Service Identity that should be configured on this Event Hub Namespace. Possible values are `SystemAssigned` or `UserAssigned`.
:’
~
eventhub EventHubNamespaceNetworkRulesetsPazure:eventhub/EventHubNamespaceNetworkRulesets:EventHubNamespaceNetworkRulesets“
œt
defaultAction" _The default action to take when a rule is not matched. Possible values are `Allow` and `Deny`.
Ÿ
ipRulesúBô*ñ:ì
ê
eventhub&EventHubNamespaceNetworkRulesetsIpRule\azure:eventhub/EventHubNamespaceNetworkRulesetsIpRule:EventHubNamespaceNetworkRulesetsIpRule/One or more `ip_rule` blocks as defined below.
˜
publicNetworkAccessEnabledB
 “Is public network access enabled for the EventHub Namespace? Defaults to `true`.

> **Note:** The public network access setting at the network rule sets level should be the same as it's at the namespace level.
h
trustedServiceAccessEnabledB
 CWhether Trusted Microsoft Services are allowed to bypass firewall.
ñ
virtualNetworkRules¿BΩ*∫:∑
¥
eventhub2EventHubNamespaceNetworkRulesetsVirtualNetworkRuletazure:eventhub/EventHubNamespaceNetworkRulesetsVirtualNetworkRule:EventHubNamespaceNetworkRulesetsVirtualNetworkRule<One or more `virtual_network_rule` blocks as defined below.
:≥
ê
eventhub&EventHubNamespaceNetworkRulesetsIpRule\azure:eventhub/EventHubNamespaceNetworkRulesetsIpRule:EventHubNamespaceNetworkRulesetsIpRuleù
öo
actionB" _The action to take when the rule is matched. Possible values are `Allow`. Defaults to `Allow`.
'
ipMask" The IP mask to match on.
:ﬁ
¥
eventhub2EventHubNamespaceNetworkRulesetsVirtualNetworkRuletazure:eventhub/EventHubNamespaceNetworkRulesetsVirtualNetworkRule:EventHubNamespaceNetworkRulesetsVirtualNetworkRule§
°k
*ignoreMissingVirtualNetworkServiceEndpointB
 7Are missing virtual network service endpoints ignored?
2
subnetId" "The id of the subnet to match on.
:Ü)
{
eventhubEventSubscriptionAdvancedFilterNazure:eventhub/EventSubscriptionAdvancedFilter:EventSubscriptionAdvancedFilterÜ(
É(Ó

boolEquals¢Bü*ú:ô
ñ
eventhub(EventSubscriptionAdvancedFilterBoolEqual`azure:eventhub/EventSubscriptionAdvancedFilterBoolEqual:EventSubscriptionAdvancedFilterBoolEqual;Compares a value of an event using a single boolean value.
Ì

isNotNulls¢Bü*ú:ô
ñ
eventhub(EventSubscriptionAdvancedFilterIsNotNull`azure:eventhub/EventSubscriptionAdvancedFilterIsNotNull:EventSubscriptionAdvancedFilterIsNotNull:Evaluates if a value of an event isn't NULL or undefined.
«
isNullOrUndefineds∫B∑*¥:±
Æ
eventhub0EventSubscriptionAdvancedFilterIsNullOrUndefinedpazure:eventhub/EventSubscriptionAdvancedFilterIsNullOrUndefined:EventSubscriptionAdvancedFilterIsNullOrUndefinedtEvaluates if a value of an event is NULL or undefined.

Each nested block consists of a key and a value(s) element.
≤
numberGreaterThanOrEqualsœBÃ*…:∆
√
eventhub7EventSubscriptionAdvancedFilterNumberGreaterThanOrEqual~azure:eventhub/EventSubscriptionAdvancedFilterNumberGreaterThanOrEqual:EventSubscriptionAdvancedFilterNumberGreaterThanOrEqualCCompares a value of an event using a single floating point number.
ñ
numberGreaterThans∫B∑*¥:±
Æ
eventhub0EventSubscriptionAdvancedFilterNumberGreaterThanpazure:eventhub/EventSubscriptionAdvancedFilterNumberGreaterThan:EventSubscriptionAdvancedFilterNumberGreaterThanCCompares a value of an event using a single floating point number.
ç
numberInRangesÆB´*®:•
¢
eventhub,EventSubscriptionAdvancedFilterNumberInRangehazure:eventhub/EventSubscriptionAdvancedFilterNumberInRange:EventSubscriptionAdvancedFilterNumberInRangeJCompares a value of an event using multiple floating point number ranges.
Û
	numberInsüBú*ô:ñ
ì
eventhub'EventSubscriptionAdvancedFilterNumberIn^azure:eventhub/EventSubscriptionAdvancedFilterNumberIn:EventSubscriptionAdvancedFilterNumberInDCompares a value of an event using multiple floating point numbers.
¶
numberLessThanOrEquals∆B√*¿:Ω
∫
eventhub4EventSubscriptionAdvancedFilterNumberLessThanOrEqualxazure:eventhub/EventSubscriptionAdvancedFilterNumberLessThanOrEqual:EventSubscriptionAdvancedFilterNumberLessThanOrEqualCCompares a value of an event using a single floating point number.
ä
numberLessThans±BÆ*´:®
•
eventhub-EventSubscriptionAdvancedFilterNumberLessThanjazure:eventhub/EventSubscriptionAdvancedFilterNumberLessThan:EventSubscriptionAdvancedFilterNumberLessThanCCompares a value of an event using a single floating point number.
ô
numberNotInRanges∑B¥*±:Æ
´
eventhub/EventSubscriptionAdvancedFilterNumberNotInRangenazure:eventhub/EventSubscriptionAdvancedFilterNumberNotInRange:EventSubscriptionAdvancedFilterNumberNotInRangeJCompares a value of an event using multiple floating point number ranges.
ˇ
numberNotIns®B•*¢:ü
ú
eventhub*EventSubscriptionAdvancedFilterNumberNotIndazure:eventhub/EventSubscriptionAdvancedFilterNumberNotIn:EventSubscriptionAdvancedFilterNumberNotInDCompares a value of an event using multiple floating point numbers.
ä
stringBeginsWiths∑B¥*±:Æ
´
eventhub/EventSubscriptionAdvancedFilterStringBeginsWithnazure:eventhub/EventSubscriptionAdvancedFilterStringBeginsWith:EventSubscriptionAdvancedFilterStringBeginsWith;Compares a value of an event using multiple string values.
˛
stringContainsÆB´*®:•
¢
eventhub,EventSubscriptionAdvancedFilterStringContainhazure:eventhub/EventSubscriptionAdvancedFilterStringContain:EventSubscriptionAdvancedFilterStringContain;Compares a value of an event using multiple string values.
Ç
stringEndsWiths±BÆ*´:®
•
eventhub-EventSubscriptionAdvancedFilterStringEndsWithjazure:eventhub/EventSubscriptionAdvancedFilterStringEndsWith:EventSubscriptionAdvancedFilterStringEndsWith;Compares a value of an event using multiple string values.
Í
	stringInsüBú*ô:ñ
ì
eventhub'EventSubscriptionAdvancedFilterStringIn^azure:eventhub/EventSubscriptionAdvancedFilterStringIn:EventSubscriptionAdvancedFilterStringIn;Compares a value of an event using multiple string values.
ñ
stringNotBeginsWiths¿BΩ*∫:∑
¥
eventhub2EventSubscriptionAdvancedFilterStringNotBeginsWithtazure:eventhub/EventSubscriptionAdvancedFilterStringNotBeginsWith:EventSubscriptionAdvancedFilterStringNotBeginsWith;Compares a value of an event using multiple string values.
ä
stringNotContains∑B¥*±:Æ
´
eventhub/EventSubscriptionAdvancedFilterStringNotContainnazure:eventhub/EventSubscriptionAdvancedFilterStringNotContain:EventSubscriptionAdvancedFilterStringNotContain;Compares a value of an event using multiple string values.
é
stringNotEndsWiths∫B∑*¥:±
Æ
eventhub0EventSubscriptionAdvancedFilterStringNotEndsWithpazure:eventhub/EventSubscriptionAdvancedFilterStringNotEndsWith:EventSubscriptionAdvancedFilterStringNotEndsWith;Compares a value of an event using multiple string values.
ˆ
stringNotIns®B•*¢:ü
ú
eventhub*EventSubscriptionAdvancedFilterStringNotIndazure:eventhub/EventSubscriptionAdvancedFilterStringNotIn:EventSubscriptionAdvancedFilterStringNotIn;Compares a value of an event using multiple string values.
:¿
ñ
eventhub(EventSubscriptionAdvancedFilterBoolEqual`azure:eventhub/EventSubscriptionAdvancedFilterBoolEqual:EventSubscriptionAdvancedFilterBoolEqual§
°ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.

value
 :≥
ñ
eventhub(EventSubscriptionAdvancedFilterIsNotNull`azure:eventhub/EventSubscriptionAdvancedFilterIsNotNull:EventSubscriptionAdvancedFilterIsNotNulló
îë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
:À
Æ
eventhub0EventSubscriptionAdvancedFilterIsNullOrUndefinedpazure:eventhub/EventSubscriptionAdvancedFilterIsNullOrUndefined:EventSubscriptionAdvancedFilterIsNullOrUndefinedó
îë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
:ÿ
Æ
eventhub0EventSubscriptionAdvancedFilterNumberGreaterThanpazure:eventhub/EventSubscriptionAdvancedFilterNumberGreaterThan:EventSubscriptionAdvancedFilterNumberGreaterThan§
°ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.

value :Ì
√
eventhub7EventSubscriptionAdvancedFilterNumberGreaterThanOrEqual~azure:eventhub/EventSubscriptionAdvancedFilterNumberGreaterThanOrEqual:EventSubscriptionAdvancedFilterNumberGreaterThanOrEqual§
°ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.

value :¸
ì
eventhub'EventSubscriptionAdvancedFilterNumberIn^azure:eventhub/EventSubscriptionAdvancedFilterNumberIn:EventSubscriptionAdvancedFilterNumberIn„
‡ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
…
values* ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:ç
¢
eventhub,EventSubscriptionAdvancedFilterNumberInRangehazure:eventhub/EventSubscriptionAdvancedFilterNumberInRange:EventSubscriptionAdvancedFilterNumberInRangeÂ
‚ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
À
values** ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:œ
•
eventhub-EventSubscriptionAdvancedFilterNumberLessThanjazure:eventhub/EventSubscriptionAdvancedFilterNumberLessThan:EventSubscriptionAdvancedFilterNumberLessThan§
°ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.

value :‰
∫
eventhub4EventSubscriptionAdvancedFilterNumberLessThanOrEqualxazure:eventhub/EventSubscriptionAdvancedFilterNumberLessThanOrEqual:EventSubscriptionAdvancedFilterNumberLessThanOrEqual§
°ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.

value :Ö
ú
eventhub*EventSubscriptionAdvancedFilterNumberNotIndazure:eventhub/EventSubscriptionAdvancedFilterNumberNotIn:EventSubscriptionAdvancedFilterNumberNotIn„
‡ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
…
values* ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:ñ
´
eventhub/EventSubscriptionAdvancedFilterNumberNotInRangenazure:eventhub/EventSubscriptionAdvancedFilterNumberNotInRange:EventSubscriptionAdvancedFilterNumberNotInRangeÂ
‚ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
À
values** ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:î
´
eventhub/EventSubscriptionAdvancedFilterStringBeginsWithnazure:eventhub/EventSubscriptionAdvancedFilterStringBeginsWith:EventSubscriptionAdvancedFilterStringBeginsWith„
‡ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
…
values*" ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:ã
¢
eventhub,EventSubscriptionAdvancedFilterStringContainhazure:eventhub/EventSubscriptionAdvancedFilterStringContain:EventSubscriptionAdvancedFilterStringContain„
‡ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
…
values*" ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:é
•
eventhub-EventSubscriptionAdvancedFilterStringEndsWithjazure:eventhub/EventSubscriptionAdvancedFilterStringEndsWith:EventSubscriptionAdvancedFilterStringEndsWith„
‡ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
…
values*" ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:¸
ì
eventhub'EventSubscriptionAdvancedFilterStringIn^azure:eventhub/EventSubscriptionAdvancedFilterStringIn:EventSubscriptionAdvancedFilterStringIn„
‡ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
…
values*" ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:ù
¥
eventhub2EventSubscriptionAdvancedFilterStringNotBeginsWithtazure:eventhub/EventSubscriptionAdvancedFilterStringNotBeginsWith:EventSubscriptionAdvancedFilterStringNotBeginsWith„
‡ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
…
values*" ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:î
´
eventhub/EventSubscriptionAdvancedFilterStringNotContainnazure:eventhub/EventSubscriptionAdvancedFilterStringNotContain:EventSubscriptionAdvancedFilterStringNotContain„
‡ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
…
values*" ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:ó
Æ
eventhub0EventSubscriptionAdvancedFilterStringNotEndsWithpazure:eventhub/EventSubscriptionAdvancedFilterStringNotEndsWith:EventSubscriptionAdvancedFilterStringNotEndsWith„
‡ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
…
values*" ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:Ö
ú
eventhub*EventSubscriptionAdvancedFilterStringNotIndazure:eventhub/EventSubscriptionAdvancedFilterStringNotIn:EventSubscriptionAdvancedFilterStringNotIn„
‡ë
key" ÖSpecifies the field within the event data that you want to use for filtering. Type of the field can be a number, boolean, or string.
…
values*" ∏Specifies an array of values to compare to when using a multiple values operator.

> **NOTE:** A maximum of total number of advanced filter values allowed on event subscription is 25.
:€
ê
eventhub&EventSubscriptionAzureFunctionEndpoint\azure:eventhub/EventSubscriptionAzureFunctionEndpoint:EventSubscriptionAzureFunctionEndpoint≈
¬≤

functionId" üSpecifies the ID of the Function where the Event Subscription will receive events. This must be the functions ID in format {function_app.id}/functions/{name}.
?
maxEventsPerBatchB $Maximum number of events per batch.
J
preferredBatchSizeInKilobytesB #Preferred batch size in Kilobytes.
:Û
á
eventhub#EventSubscriptionDeadLetterIdentityVazure:eventhub/EventSubscriptionDeadLetterIdentity:EventSubscriptionDeadLetterIdentityÊ
„ê
type" ÉSpecifies the type of Managed Service Identity that is used for dead lettering. Allowed value is `SystemAssigned`, `UserAssigned`.
N
userAssignedIdentityB" 0The user identity associated with the resource.
:Ì
Å
eventhub!EventSubscriptionDeliveryIdentityRazure:eventhub/EventSubscriptionDeliveryIdentity:EventSubscriptionDeliveryIdentityÊ
„ê
type" ÉSpecifies the type of Managed Service Identity that is used for event delivery. Allowed value is `SystemAssigned`, `UserAssigned`.
N
userAssignedIdentityB" 0The user identity associated with the resource.
:Ñ
Å
eventhub!EventSubscriptionDeliveryPropertyRazure:eventhub/EventSubscriptionDeliveryProperty:EventSubscriptionDeliveryProperty˝
˙G

headerName" 5The name of the header to send on to the destination
õ
secretB
 äTrue if the `value` is a secret and should be protected, otherwise false. If True, then this value won't be returned from Azure API calls
õ
sourceFieldB" ÖIf the `type` is `Dynamic`, then provide the payload field to be used as the value. Valid source fields differ by subscription type.
)
type" Either `Static` or `Dynamic`
H
valueB" 9If the `type` is `Static`, then provide the value to use
:œ
r
eventhubEventSubscriptionRetryPolicyHazure:eventhub/EventSubscriptionRetryPolicy:EventSubscriptionRetryPolicyÿ
’Ò
eventTimeToLive ŸSpecifies the time to live (in minutes) for events. Supported range is `1` to `1440`. See [official documentation](https://docs.microsoft.com/azure/event-grid/manage-event-delivery#set-retry-policy) for more details.
_
maxDeliveryAttempts DSpecifies the maximum number of delivery retry attempts for events.
:®
±
eventhub1EventSubscriptionStorageBlobDeadLetterDestinationrazure:eventhub/EventSubscriptionStorageBlobDeadLetterDestination:EventSubscriptionStorageBlobDeadLetterDestinationÒ
Óf
storageAccountId" NSpecifies the id of the storage account id where the storage blob is located.
É
storageBlobContainerName" cSpecifies the name of the Storage blob container that is the destination of the deadletter events.
:∆
ç
eventhub%EventSubscriptionStorageQueueEndpointZazure:eventhub/EventSubscriptionStorageQueueEndpoint:EventSubscriptionStorageQueueEndpoint≥
∞X
queueMessageTimeToLiveInSecondsB /Storage queue message time to live in seconds.
k
	queueName" ZSpecifies the name of the storage queue where the Event Subscription will receive events.
g
storageAccountId" OSpecifies the id of the storage account id where the storage queue is located.
:—
x
eventhubEventSubscriptionSubjectFilterLazure:eventhub/EventSubscriptionSubjectFilter:EventSubscriptionSubjectFilter‘
—m
caseSensitiveB
 VSpecifies if `subject_begins_with` and `subject_ends_with` case sensitive. This value
p
subjectBeginsWithB" UA string to filter events for an event subscription based on a resource path prefix.
n
subjectEndsWithB" UA string to filter events for an event subscription based on a resource path suffix.
:•
~
eventhub EventSubscriptionWebhookEndpointPazure:eventhub/EventSubscriptionWebhookEndpoint:EventSubscriptionWebhookEndpoint¢
ü≠
activeDirectoryAppIdOrUriB" âThe Azure Active Directory Application ID or URI to get the access token that will be included as the bearer token in delivery requests.
û
activeDirectoryTenantIdB" }The Azure Active Directory Tenant ID to get the access token that will be included as the bearer token in delivery requests.
_
baseUrlB" NThe base url of the webhook where the Event Subscription will receive events.
?
maxEventsPerBatchB $Maximum number of events per batch.
J
preferredBatchSizeInKilobytesB #Preferred batch size in Kilobytes.
^
url" SSpecifies the url of the webhook where the Event Subscription will receive events.
:Ù
o
eventhubNamespaceCustomerManagedKeyFazure:eventhub/NamespaceCustomerManagedKey:NamespaceCustomerManagedKeyÄ
˝S

identityId" AThe ID of the User Assigned Identity that has access to the key.
´
infrastructureEncryptionEnabledB
 ÅUsed to specify whether enable Infrastructure Encryption (Double Encryption). Changing this forces a new resource to be created.
x
keyVaultKeyId" cThe ID of the Key Vault Key which should be used to Encrypt the data in this ServiceBus Namespace.
:≤
Q
eventhubNamespaceIdentity2azure:eventhub/NamespaceIdentity:NamespaceIdentity‹
Ÿ‚
identityIdsB*"  Specifies a list of User Assigned Managed Identity IDs to be assigned to this ServiceBus namespace.

> **NOTE:** This is required when `type` is set to `UserAssigned` or `SystemAssigned, UserAssigned`.
ã
principalIdB" vThe Principal ID for the Service Principal associated with the Managed Service Identity of this ServiceBus Namespace.
Ö
tenantIdB" sThe Tenant ID for the Service Principal associated with the Managed Service Identity of this ServiceBus Namespace.
€
type" ŒSpecifies the type of Managed Service Identity that should be configured on this ServiceBus Namespace. Possible values are `SystemAssigned`, `UserAssigned`, `SystemAssigned, UserAssigned` (to enable both).
:ã	
c
eventhubNamespaceNetworkRuleSet>azure:eventhub/NamespaceNetworkRuleSet:NamespaceNetworkRuleSet£
†ã
defaultActionB" tSpecifies the default action for the Network Rule Set. Possible values are `Allow` and `Deny`. Defaults to `Allow`.
u
ipRulesB*" bOne or more IP Addresses, or CIDR Blocks which should be able to access the ServiceBus Namespace.
ÿ
networkRulesêBç*ä:á
Ñ
eventhub"NamespaceNetworkRuleSetNetworkRuleTazure:eventhub/NamespaceNetworkRuleSetNetworkRule:NamespaceNetworkRuleSetNetworkRule5One or more `network_rules` blocks as defined below.
Ñ
publicNetworkAccessEnabledB
 ﬂWhether to allow traffic over public network. Possible values are `true` and `false`. Defaults to `true`.

> **Note:** To disable public network access, you must also configure the property `public_network_access_enabled`.
∂
trustedServicesAllowedB
 ïAre Azure Services that are known and trusted for this resource type are allowed to bypass firewall configuration? See [Trusted Microsoft Services](https://github.com/MicrosoftDocs/azure-docs/blob/master/articles/service-bus-messaging/includes/service-bus-trusted-services.md)
:°
Ñ
eventhub"NamespaceNetworkRuleSetNetworkRuleTazure:eventhub/NamespaceNetworkRuleSetNetworkRule:NamespaceNetworkRuleSetNetworkRuleó
î∑
 ignoreMissingVnetServiceEndpointB
 åShould the ServiceBus Namespace Network Rule Set ignore missing Virtual Network Service Endpoint option in the Subnet? Defaults to `false`.
X
subnetId" HThe Subnet ID which should be able to access this ServiceBus Namespace.
:›
ä
eventhub$SubscriptionClientScopedSubscriptionXazure:eventhub/SubscriptionClientScopedSubscription:SubscriptionClientScopedSubscriptionÕ
 ˚
clientIdB" ËSpecifies the Client ID of the application that created the client-scoped subscription. Changing this forces a new resource to be created.

> **NOTE:** Client ID can be null or empty, but it must match the client ID set on the JMS client application. From the Azure Service Bus perspective, a null client ID and an empty client id have the same behavior. If the client ID is set to null or empty, it is only accessible to client applications whose client ID is also set to null or empty.
û
!isClientScopedSubscriptionDurableB
 sWhether the client scoped subscription is durable. This property can only be controlled from the application side.
®
#isClientScopedSubscriptionShareableB
 {Whether the client scoped subscription is shareable. Defaults to `true` Changing this forces a new resource to be created.
:°
Å
eventhub!SubscriptionRuleCorrelationFilterRazure:eventhub/SubscriptionRuleCorrelationFilter:SubscriptionRuleCorrelationFilterö
ó2
contentTypeB" Content type of the message.
6
correlationIdB" Identifier of the correlation.
+
labelB" Application specific label.
.
	messageIdB" Identifier of the message.
è

propertiesB2" ¯A list of user defined properties to be included in the filter. Specified as a map of name/value pairs.

> **NOTE:** When creating a subscription rule of type `CorrelationFilter` at least one property must be set in the `correlation_filter` block.
3
replyToB" "Address of the queue to reply to.
:
replyToSessionIdB"  Session identifier to reply to.
'
	sessionIdB" Session identifier.
 
toB" Address to send to.
:ı
Ç
extendedlocationCustomLocationAuthenticationPazure:extendedlocation/CustomLocationAuthentication:CustomLocationAuthenticationn
l4
typeB" &Specifies the type of authentication.
4
value" 'Specifies the value of authentication.
:”
;
fabricCapacitySku$azure:fabric/CapacitySku:CapacitySkuì
ê¶
name" ôThe name of the SKU to use for the Fabric Capacity. Possible values are `F2`, `F4`, `F8`, `F16`, `F32`, `F64`, `F128`, `F256`, `F512`, `F1024`, `F2048`.
e
tier" YThe tier of the SKU to use for the Fabric Capacity. The only possible value is `Fabric`.
:‹
j

fluidrelayServerCustomerManagedKeyBazure:fluidrelay/ServerCustomerManagedKey:ServerCustomerManagedKeyÌ
Í_
keyVaultKeyId" JThe Key Vault Key Id that will be used to encrypt the Fluid Relay Server.
Ü
userAssignedIdentityId" hThe User Assigned Managed Identity ID to be used for accessing the Customer Managed Key for encryption.
:é
L

fluidrelayServerIdentity.azure:fluidrelay/ServerIdentity:ServerIdentityΩ
∫z
identityIdsB*" cSpecifies a list of User Assigned Managed Identity IDs to be assigned to this Fluid Relay Service.
y
principalIdB" dThe Principal ID for the Service Principal associated with the Identity of this Fluid Relay Server.
s
tenantIdB" aThe Tenant ID for the Service Principal associated with the Identity of this Fluid Relay Server.
À
type" æSpecifies the type of Managed Service Identity that should be configured on this Fluid Relay Service. Possible values are `SystemAssigned`,`UserAssigned` and `SystemAssigned, UserAssigned`.
