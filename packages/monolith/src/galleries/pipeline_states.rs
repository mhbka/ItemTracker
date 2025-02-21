//! This module holds types related to each stage of the scraping pipeline.
//! We can map each stage's state to the next stage using `map_to_next_stage`.

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use super::{
    domain_types::{GalleryId, ItemId, Marketplace, UnixUtcDateTime, ValidCronString}, eval_criteria::EvaluationCriteria, items::{item_data::MarketplaceItemData, pipeline_items::{MarketplaceAnalyzedItems, MarketplaceEmbeddedAndAnalyzedItems}}, search_criteria::GallerySearchCriteria
};

/// The possible states of a gallery in the scraping pipeline.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GalleryPipelineStates {
    Initialization(GallerySchedulerState),
    SearchScraping(GallerySearchScrapingState),
    ItemScraping(GalleryItemScrapingState),
    ItemAnalysis(GalleryItemAnalysisState),
    ItemEmbedding(GalleryItemEmbedderState),
    Final(GalleryFinalState)
}

impl GalleryPipelineStates {
    /// Returns if the state type matches the state.
    pub fn matches(&self, state_type: &GalleryPipelineStateTypes) -> bool {
        matches!(
            (self, state_type),
            (GalleryPipelineStates::Initialization(_), GalleryPipelineStateTypes::Initialization) |
            (GalleryPipelineStates::SearchScraping(_), GalleryPipelineStateTypes::SearchScraping) |
            (GalleryPipelineStates::ItemScraping(_), GalleryPipelineStateTypes::ItemScraping) |
            (GalleryPipelineStates::ItemAnalysis(_), GalleryPipelineStateTypes::ItemAnalysis) |
            (GalleryPipelineStates::ItemEmbedding(_), GalleryPipelineStateTypes::ItemEmbedding) |
            (GalleryPipelineStates::Final(_), GalleryPipelineStateTypes::Final)
        )
    }

    /// Returns the state's state type.
    pub fn state_type(&self) -> GalleryPipelineStateTypes {
        match self {
            GalleryPipelineStates::Initialization(_) => GalleryPipelineStateTypes::Initialization,
            GalleryPipelineStates::SearchScraping(_) => GalleryPipelineStateTypes::SearchScraping,
            GalleryPipelineStates::ItemScraping(_) => GalleryPipelineStateTypes::ItemScraping,
            GalleryPipelineStates::ItemAnalysis(_) => GalleryPipelineStateTypes::ItemAnalysis,
            GalleryPipelineStates::ItemEmbedding(_) => GalleryPipelineStateTypes::ItemEmbedding,
            GalleryPipelineStates::Final(_) => GalleryPipelineStateTypes::Final,
        }
    }
}

/// A stateless enum of the possible states in the pipeline.
/// 
/// Used for matching on the stateful version using its `matches` function.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GalleryPipelineStateTypes {
    Initialization, 
    SearchScraping, 
    ItemScraping,
    ItemAnalysis,
    ItemEmbedding,
    Final
}

impl GalleryPipelineStateTypes {
    /// Returns if the state type matches the state.
    pub fn matches(&self, state: &GalleryPipelineStates) -> bool {
        state.matches(self)
    }
}

/// This is the state of a gallery in the scheduler.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GallerySchedulerState {
    pub gallery_id: GalleryId,
    pub scraping_periodicity: ValidCronString,
    pub search_criteria: GallerySearchCriteria,
    pub marketplace_previous_scraped_datetimes: HashMap<Marketplace, UnixUtcDateTime>,
    pub evaluation_criteria: EvaluationCriteria,
}

impl GallerySchedulerState {
    /// Convenience function for mapping to the next state.
    pub fn to_next_stage(self) -> GallerySearchScrapingState {
        GallerySearchScrapingState {
            gallery_id: self.gallery_id,
            search_criteria: self.search_criteria,
            marketplace_previous_scraped_datetimes: self.marketplace_previous_scraped_datetimes,
            evaluation_criteria: self.evaluation_criteria,
        }
    }
}

/// This is the initial state that a scraping job starts in.
/// 
/// Initialized in the scraper scheduler module.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GallerySearchScrapingState {
    pub gallery_id: GalleryId,
    pub search_criteria: GallerySearchCriteria,
    pub marketplace_previous_scraped_datetimes: HashMap<Marketplace, UnixUtcDateTime>,
    pub evaluation_criteria: EvaluationCriteria,
}

impl GallerySearchScrapingState {

}

/// This is the state of a gallery after it has been search-scraped.
/// 
/// Initialized in the scraper scheduler module.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GalleryItemScrapingState {
    pub gallery_id: GalleryId,
    pub item_ids: HashMap<Marketplace, Vec<ItemId>>,
    pub marketplace_updated_datetimes: HashMap<Marketplace, UnixUtcDateTime>,
    pub failed_marketplace_reasons: HashMap<Marketplace, String>,
    pub evaluation_criteria: EvaluationCriteria,
}

impl GalleryItemScrapingState {
    /// Convenience function for mapping to the next state.
    pub fn to_next_stage(self, items: HashMap<Marketplace, Vec<MarketplaceItemData>>) -> GalleryItemAnalysisState {
        GalleryItemAnalysisState {
            gallery_id: self.gallery_id,
            items,
            marketplace_updated_datetimes: self.marketplace_updated_datetimes,
            failed_marketplace_reasons: self.failed_marketplace_reasons,
            evaluation_criteria: self.evaluation_criteria,
        }
    }
}

/// This is the state of a scraping job after the items are scraped.
/// 
/// Initialized in the scraper module.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GalleryItemAnalysisState {
    pub gallery_id: GalleryId,
    pub items: HashMap<Marketplace, Vec<MarketplaceItemData>>,
    pub marketplace_updated_datetimes: HashMap<Marketplace, UnixUtcDateTime>,
    pub failed_marketplace_reasons: HashMap<Marketplace, String>,
    pub evaluation_criteria: EvaluationCriteria,
}

impl GalleryItemAnalysisState {
    /// Convenience function for mapping to the next state.
    pub fn to_next_stage(self, items: HashMap<Marketplace, MarketplaceAnalyzedItems>) -> GalleryItemEmbedderState {
        GalleryItemEmbedderState {
            gallery_id: self.gallery_id,
            items,
            marketplace_updated_datetimes: self.marketplace_updated_datetimes,
            failed_marketplace_reasons: self.failed_marketplace_reasons,
        }
    }
}

/// This is the state of a gallery after its items are embedded.
/// 
/// Initialized in the item analysis module.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GalleryItemEmbedderState {
    pub gallery_id: GalleryId,
    pub items: HashMap<Marketplace, MarketplaceAnalyzedItems>,
    pub marketplace_updated_datetimes: HashMap<Marketplace, UnixUtcDateTime>,
    pub failed_marketplace_reasons: HashMap<Marketplace, String>,
}

impl GalleryItemEmbedderState {

}

/// This is the state of a gallery after its items are classified into groups within the gallery.
/// 
/// Initialized in the image classifier module.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GalleryFinalState {
    // TODO: figure out the state here
    pub gallery_id: GalleryId,
    pub items: HashMap<Marketplace, MarketplaceEmbeddedAndAnalyzedItems>,
    pub marketplace_updated_datetimes: HashMap<Marketplace, UnixUtcDateTime>,
    pub failed_marketplace_reasons: HashMap<Marketplace, String>,
}

impl GalleryFinalState {

}