use message_buses::{MessageError, MessageReceiver, MessageSender};
use message_types::{
    item_embedder::ItemEmbedderMessage, item_analysis::ItemAnalysisMessage, item_scraper::ItemScraperMessage, scraper_scheduler::SchedulerMessage, search_scraper::SearchScraperMessage, state_tracker::{AddGalleryMessage, CheckGalleryDoesntExistMessage, CheckGalleryStateMessage, RemoveGalleryMessage, StateTrackerError, StateTrackerMessage, TakeGalleryStateMessage, UpdateGalleryStateMessage}, storage::marketplace_items::MarketplaceItemsStorageMessage, web_backend::WebBackendMessage
};

use crate::galleries::{domain_types::GalleryId, pipeline_states::{GalleryPipelineStateTypes, GalleryPipelineStates}};

pub mod message_buses;
pub mod message_types;

/// Handle for sending the web backend messages.
pub type WebBackendSender = MessageSender<WebBackendMessage>;
/// Handle for the web backend to receive messages.
pub type WebBackendReceiver = MessageReceiver<WebBackendMessage>;

/// Handle for sending the scraper scheduler messages.
pub type ScraperSchedulerSender = MessageSender<SchedulerMessage>;
/// Handle for the scraper scheduler to receive messages.
pub type ScraperSchedulerReceiver = MessageReceiver<SchedulerMessage>;

/// Handle for sending messages to the search scraper.
pub type SearchScraperSender = MessageSender<SearchScraperMessage>;
/// Handle for the search scraper module to receive messages.
pub type SearchScraperReceiver = MessageReceiver<SearchScraperMessage>;

/// Handle for sending messages to the item scraper.
pub type ItemScraperSender = MessageSender<ItemScraperMessage>;
/// Handle for the item scraper module to receive messages.
pub type ItemScraperReceiver = MessageReceiver<ItemScraperMessage>;

/// Handle for sending the item analysis module messages.
pub type ItemAnalysisSender = MessageSender<ItemAnalysisMessage>;
/// Handle for the item analysis module to receive messages.
pub type ItemAnalysisReceiver = MessageReceiver<ItemAnalysisMessage>;

/// Handle for sending the image classifier module messages.
pub type ItemEmbedderSender = MessageSender<ItemEmbedderMessage>;
/// Handle for the image classifier module to receive messages.
pub type ItemEmbedderReceiver = MessageReceiver<ItemEmbedderMessage>;

/// Handle for sending the marketplace items storage module messages.
pub type MarketplaceItemsStorageSender = MessageSender<MarketplaceItemsStorageMessage>;
/// Handle for the marketplace items storage storage module to receive messages.
pub type MarketplaceItemsStorageReceiver = MessageReceiver<MarketplaceItemsStorageMessage>;

/// Handle for the scraper scheduler to receive messages.
pub type StateTrackerReceiver = MessageReceiver<StateTrackerMessage>;

/// Handle for sending the scraper scheduler messages.
/// 
/// Wraps messaging with functions for ease of use.
#[derive(Clone, Debug)]
pub struct StateTrackerSender { sender: MessageSender<StateTrackerMessage> }

impl StateTrackerSender {
    /// Initialize the message sender.
    pub fn new(sender: MessageSender<StateTrackerMessage>) -> Self {
        Self { sender }
    }

    /// Add a gallery to the state.
    /// 
    /// Returns an `Err` if the gallery already exists.
    pub async fn add_gallery(
        &mut self,
        gallery_id: GalleryId, 
        state: GalleryPipelineStates
    ) -> Result<Result<(), StateTrackerError>, MessageError> {
        let (msg, receiver) = AddGalleryMessage::new((gallery_id, state));
        self.sender
            .send(StateTrackerMessage::AddGallery(msg))
            .await?;
        receiver.await
            .map_err(Into::into)
    }

    /// Verify that a gallery doesn't exist.
    /// Useful for modules to check before processing new galleries.
    /// 
    /// Returns an `Err` if it does.
    pub async fn check_gallery_doesnt_exist(
        &mut self,
        gallery_id: GalleryId
    ) -> Result<Result<(), StateTrackerError>, MessageError> {
        let (msg, receiver) = CheckGalleryDoesntExistMessage::new(gallery_id);
        self.sender
            .send(StateTrackerMessage::CheckGalleryDoesntExist(msg))
            .await?;
        receiver.await
            .map_err(Into::into)
    }

    /// Verify if a gallery matches the given state type.
    /// 
    /// Returns an `Err` if it doesn't exist or doesn't match.
    pub async fn check_gallery_state(
        &mut self,
        gallery_id: GalleryId,
        state_type: GalleryPipelineStateTypes
    ) -> Result<Result<(), StateTrackerError>, MessageError> {
        let (msg, receiver) = CheckGalleryStateMessage::new((gallery_id, state_type));
        self.sender
            .send(StateTrackerMessage::CheckGalleryState(msg))
            .await?;
        receiver.await
            .map_err(Into::into)
    }
    
    /// Take a gallery's state, leaving it stored as `None`.
    /// 
    /// Returns an `Err` if it doesn't exist, its state is wrong, or its state is already taken.
    pub async fn take_gallery_state(
        &mut self,
        gallery_id: GalleryId,
        state_type: GalleryPipelineStateTypes
    ) -> Result<Result<GalleryPipelineStates, StateTrackerError>, MessageError> {
        let (msg, receiver) = TakeGalleryStateMessage::new((gallery_id, state_type));
        self.sender
            .send(StateTrackerMessage::TakeGalleryState(msg))
            .await?;
        receiver.await
            .map_err(Into::into)
    }

    /// Update a gallery's state.
    /// 
    /// Returns an `Err` if it doesn't exist, its state is wrong, or its state isn't taken.
    pub async fn update_gallery_state(
        &mut self,
        gallery_id: GalleryId,
        state: GalleryPipelineStates
    ) -> Result<Result<(), StateTrackerError>, MessageError> {
        let (msg, receiver) = UpdateGalleryStateMessage::new((gallery_id, state));
        self.sender
            .send(StateTrackerMessage::UpdateGalleryState(msg))
            .await?;
        receiver.await
            .map_err(Into::into)
    }

    /// Remove a gallery from state.
    /// 
    /// Returns an `Err` if it doesn't exist.
    pub async fn remove_gallery(
        &mut self,
        gallery_id: GalleryId
    ) -> Result<Result<(), StateTrackerError>, MessageError> {
        let (msg, receiver) = RemoveGalleryMessage::new(gallery_id);
        self.sender
            .send(StateTrackerMessage::RemoveGallery(msg))
            .await?;
        receiver.await
            .map_err(Into::into)
    }
}
