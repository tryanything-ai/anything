use crate::Flow;

#[derive(Debug, Clone)]
pub enum SystemChangeEvent {
    Shutdown(ShutdownNotification),
    FlowChange(Flow),
}

#[derive(Debug, Clone)]
pub struct ShutdownNotification;
