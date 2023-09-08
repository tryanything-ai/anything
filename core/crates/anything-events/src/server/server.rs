use std::sync::{atomic::AtomicUsize, Arc, Mutex};

use crossbeam::channel::unbounded;
use crossbeam::channel::{Receiver, Sender};

use sqlx::types::Uuid;

use crate::{
    context::Context,
    executor::spawn_or_crash,
    messages::EventNotification,
    post_office::PostOffice,
    server::{api, heartbeat},
    EvtResult,
};

use super::events::{consumers, controller, incoming};

pub struct CommChannels {
    pub producer_events_tx: Sender<EventNotification>,
    pub producer_events_rx: Receiver<EventNotification>,

    pub consumer_events_tx: Sender<EventNotification>,
    pub consumer_events_rx: Receiver<EventNotification>,

    pub controller_tx: Sender<EventNotification>,
    pub controller_rx: Receiver<EventNotification>,
}

impl Default for CommChannels {
    fn default() -> Self {
        let (producer_events_tx, producer_events_rx) = unbounded::<EventNotification>();
        let (consumer_events_tx, consumer_events_rx) = unbounded::<EventNotification>();
        let (controller_tx, controller_rx) = unbounded::<EventNotification>();

        Self {
            producer_events_tx,
            producer_events_rx,
            consumer_events_tx,
            consumer_events_rx,
            controller_tx,
            controller_rx,
        }
    }
}

pub struct Server {
    pub port: u16,
    pub post_office: PostOffice,
    // pub store: Box<dyn StoreAdapter + Send + Sync>,
    pub context: Context,
    // pub
    pub waiting_for_trigger_id: Mutex<Option<Uuid>>,
    pub scheduler_id: Uuid,
    pub queued_triggers: AtomicUsize,
    pub comm_channels: Mutex<CommChannels>,
}

impl Server {
    pub async fn new(context: Context) -> EvtResult<Arc<Self>> {
        let comm_channels = Mutex::new(CommChannels::default());

        let server = Self {
            port: context.config().server.port,
            post_office: PostOffice::open(),
            waiting_for_trigger_id: Mutex::default(),
            context,
            scheduler_id: Uuid::new_v4(),
            queued_triggers: AtomicUsize::new(0),
            comm_channels,
        };

        Ok(Arc::new(server))
    }

    pub async fn run_server(self: Arc<Self>) -> EvtResult<()> {
        spawn_or_crash("heartbeat", self.clone(), heartbeat::heartbeat);
        spawn_or_crash(
            "incoming_events",
            self.clone(),
            incoming::process_incoming_updates,
        );

        spawn_or_crash(
            "event_consumers",
            self.clone(),
            consumers::process_consumers,
        );

        spawn_or_crash(
            "control_plane",
            self.clone(),
            controller::handle_controller_plane,
        );

        api::serve(self.context.clone()).await?;

        Ok(())
    }
}
