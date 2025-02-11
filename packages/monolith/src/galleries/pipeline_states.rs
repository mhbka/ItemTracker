//! This module holds types related to each stage of the scraping pipeline.
//! We can map each stage's state to the next stage using `map_to_next_stage`.

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use super::{
    domain_types::{GalleryId, Marketplace, UnixUtcDateTime, ValidCronString}, eval_criteria::EvaluationCriteria, items::pipeline_items::{
        AnalyzedItems, 
        ClassifiedItems, 
        ScrapedItems
    }, search_criteria::GallerySearchCriteria
};

/// The possible states of a gallery in the scraping pipeline.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GalleryPipelineStates {
    Initialization(GalleryInitializationState),
    SearchScraping(GallerySearchScrapingState),
    ItemScraping(GalleryItemScrapingState),
    ItemAnalysis(GalleryItemAnalysisState),
    Classification(GalleryClassifierState),
    Final(GalleryFinalState)
}

/// This is the initial state that a gallery starts in.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GalleryInitializationState {
    pub gallery_id: GalleryId,
    pub scraping_periodicity: ValidCronString,
    pub search_criteria: GallerySearchCriteria,
    pub marketplace_previous_scraped_datetimes: HashMap<Marketplace, UnixUtcDateTime>,
    pub evaluation_criteria: EvaluationCriteria,
}

impl GalleryInitializationState {
    /// Convenience fn for mapping to the next state.
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
    pub search_criteria: GallerySearchCriteria,
    pub marketplace_updated_datetimes: HashMap<Marketplace, UnixUtcDateTime>,
    pub failed_marketplace_reasons: HashMap<Marketplace, String>,
    pub evaluation_criteria: EvaluationCriteria,
}

impl GalleryItemScrapingState {

}

/// This is the state of a scraping job after the items are scraped.
/// 
/// Initialized in the scraper module.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GalleryItemAnalysisState {
    pub gallery_id: GalleryId,
    pub items: ScrapedItems,
    pub marketplace_updated_datetimes: HashMap<Marketplace, UnixUtcDateTime>,
    pub failed_marketplace_reasons: HashMap<Marketplace, String>,
    pub evaluation_criteria: EvaluationCriteria,
}

impl GalleryItemAnalysisState {

}

/// This is the state of a scraping State after its items are analyzed.
/// 
/// Initialized in the item analysis module.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GalleryClassifierState {
    pub gallery_id: GalleryId,
    pub items: AnalyzedItems,
    pub marketplace_updated_datetimes: HashMap<Marketplace, UnixUtcDateTime>,
    pub failed_marketplace_reasons: HashMap<Marketplace, String>,
}

impl GalleryClassifierState {

}

/// This is the state of a scraping State after its items are classified into groups within the gallery.
/// 
/// Initialized in the image classifier module.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GalleryFinalState {
    // TODO: figure out the state here
}

impl GalleryFinalState {

}