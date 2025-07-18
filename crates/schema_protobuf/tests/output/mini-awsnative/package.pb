

aws-native"0.0.1:�

(Regionaws-native:index/Region:Region�	�	
RA Region represents any valid Amazon region that may be targeted with deployments.*
AFSouth1
af-south-1Africa (Cape Town).
APEast1	ap-east-1Asia Pacific (Hong Kong)4
APNortheast1ap-northeast-1Asia Pacific (Tokyo)4
APNortheast2ap-northeast-2Asia Pacific (Seoul)4
APNortheast3ap-northeast-3Asia Pacific (Osaka)-
APSouth1
ap-south-1Asia Pacific (Mumbai)8
APSoutheast1ap-southeast-1Asia Pacific (Singapore)5
APSoutheast2ap-southeast-2Asia Pacific (Sydney)+
	CACentralca-central-1Canada (Central)'
CNNorth1
cn-north-1China (Beijing)/
CNNorthwest1cn-northwest-1China (Ningxia).

EUCentral1eu-central-1Europe (Frankfurt)*
EUNorth1
eu-north-1Europe (Stockholm)&
EUWest1	eu-west-1Europe (Ireland)%
EUWest2	eu-west-2Europe (London)$
EUWest3	eu-west-3Europe (Paris)&
EUSouth1
eu-south-1Europe (Milan)-
MESouth1
me-south-1Middle East (Bahrain)0
SAEast1	sa-east-1South America (São Paulo)3

USGovEast1us-gov-east-1AWS GovCloud (US-East)3

USGovWest1us-gov-west-1AWS GovCloud (US-West)+
USEast1	us-east-1US East (N. Virginia)$
USEast2	us-east-2US East (Ohio)-
USWest1	us-west-1US West (N. California)&
USWest2	us-west-2US West (Oregon):�	
2
config
IgnoreTagsaws-native:config:IgnoreTags�	
�	
�The configuration with resource tag settings to ignore across all resources handled by this provider (except any individual service tag resources such as `ec2.Tag`) for situations where external systems are managing certain resource tags.�
keyPrefixesB*" �List of exact resource tag keys to ignore across all resources handled by this provider. This configuration prevents Pulumi from returning the tag in any `tags` attributes and displaying any configuration difference for the tag value. If any resource configuration still has this tag key configured in the `tags` argument, it will display a perpetual difference until the tag is removed from the argument or `ignoreChanges` is also used.�
keysB*" �List of resource tag key prefixes to ignore across all resources handled by this provider. This configuration prevents Pulumi from returning any tag key matching the prefixes in any `tags` attributes and displaying any configuration difference for those tag values. If any resource configuration still has a tag matching one of the prefixes configured in the `tags` argument, it will display a perpetual difference until the tag is removed from the argument or `ignoreChanges` is also used.