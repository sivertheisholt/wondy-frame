use reqwest::Client;
use super::cache::{ApiCache, ApiResult};
use crate::models::drop_data::{
    blueprint_locations::BlueprintLocations, cetus_bounty_rewards::CetusBountyRewards,
    enemy_blueprint_tables::EnemyBlueprintTables, enemy_mod_tables::EnemyModTables, info::Info,
    misc_items::MiscItems, mission_rewards::MissionRewards, mod_locations::ModLocations,
    relics::Relics, sortie_rewards::SortieRewards, syndicates::Syndicates,
    transient_rewards::TransientRewards, zariman_rewards::ZarimanRewards,
};

pub struct WarframeDropsClient {
    base_url: String,
    client: Client,
    cache: ApiCache,
}

impl WarframeDropsClient {
    pub fn new() -> Self {
        Self {
            base_url: "https://drops.warframestat.us/data".to_string(),
            client: Client::new(),
            cache: ApiCache::new(),
        }
    }

    pub async fn fetch_mission_rewards(&self) -> ApiResult<MissionRewards> {
        self.cache.fetch(&self.client, &format!("{}/missionRewards.json", self.base_url), None).await
    }

    pub async fn fetch_relics(&self) -> ApiResult<Relics> {
        self.cache.fetch(&self.client, &format!("{}/relics.json", self.base_url), None).await
    }

    pub async fn fetch_transient_rewards(&self) -> ApiResult<TransientRewards> {
        self.cache.fetch(&self.client, &format!("{}/transientRewards.json", self.base_url), None).await
    }

    pub async fn sortie_rewards(&self) -> ApiResult<SortieRewards> {
        self.cache.fetch(&self.client, &format!("{}/sortieRewards.json", self.base_url), None).await
    }

    pub async fn fetch_mod_locations(&self) -> ApiResult<ModLocations> {
        self.cache.fetch(&self.client, &format!("{}/modLocations.json", self.base_url), None).await
    }

    pub async fn fetch_enemy_mod_tables(&self) -> ApiResult<EnemyModTables> {
        self.cache.fetch(&self.client, &format!("{}/enemyModTables.json", self.base_url), None).await
    }

    pub async fn fetch_enemy_blueprint_tables(&self) -> ApiResult<EnemyBlueprintTables> {
        self.cache.fetch(&self.client, &format!("{}/enemyBlueprintTables.json", self.base_url), None).await
    }

    pub async fn fetch_blueprint_locations(&self) -> ApiResult<BlueprintLocations> {
        self.cache.fetch(&self.client, &format!("{}/blueprintLocations.json", self.base_url), None).await
    }

    pub async fn fetch_cetus_bounty_rewards(&self) -> ApiResult<CetusBountyRewards> {
        self.cache.fetch(&self.client, &format!("{}/cetusBountyRewards.json", self.base_url), None).await
    }

    pub async fn fetch_zariman_rewards(&self) -> ApiResult<ZarimanRewards> {
        self.cache.fetch(&self.client, &format!("{}/zarimanRewards.json", self.base_url), None).await
    }

    pub async fn fetch_syndicates(&self) -> ApiResult<Syndicates> {
        self.cache.fetch(&self.client, &format!("{}/syndicates.json", self.base_url), None).await
    }

    pub async fn fetch_misc_items(&self) -> ApiResult<MiscItems> {
        self.cache.fetch(&self.client, &format!("{}/miscItems.json", self.base_url), None).await
    }

    pub async fn fetch_info(&self) -> ApiResult<Info> {
        self.cache.fetch(&self.client, &format!("{}/info.json", self.base_url), None).await
    }
}
