use async_trait::async_trait;
use crate::remote_access::DnsClient;
use cloudflare::framework::async_api::{Client, ApiClient};
use cloudflare::framework::Environment;
use cloudflare::endpoints::dns::{CreateDnsRecord, CreateDnsRecordParams, DnsContent, DeleteDnsRecord};
use crate::config::CloudflareCredentials;

pub struct CloudflareDns {
    credentials: CloudflareCredentials,
    client: Client,
    zone: String,
    zone_id: String
}

impl Clone for CloudflareDns {
    fn clone(&self) -> Self {
        CloudflareDns::new(self.credentials.clone(), self.zone.clone(), self.zone_id.clone()).unwrap()
    }
}

impl CloudflareDns {
    pub fn new(credentials: CloudflareCredentials, zone: String, zone_id: String) -> Result<CloudflareDns, failure::Error> {
        let client = Client::new(credentials.clone().into(), Default::default(), Environment::Production)?;

        Ok(CloudflareDns {
            credentials,
            client,
            zone,
            zone_id
        })
    }
}

#[async_trait]
impl DnsClient for CloudflareDns {
    async fn add_subdomain(&self, sub_domain: &str) -> Result<(), failure::Error> {
        let request = CreateDnsRecord {
            zone_identifier: &self.zone_id,
            params: CreateDnsRecordParams {
                name: &format!("{}.rustic", sub_domain),
                content: DnsContent::CNAME {
                    content: self.zone.clone()
                },
                priority: None,
                ttl: None,
                proxied: Some(false)
            }
        };
        self.client.request(&request).await?;
        Ok(())
    }

    async fn remove_subdomain(&self, sub_domain: &str) -> Result<(), failure::Error> {
        let request = DeleteDnsRecord {
            zone_identifier: &self.zone_id,
            identifier: sub_domain
        };
        self.client.request(&request).await?;
        Ok(())
    }
}
