use mongodb::Database;
use prometheus_client::registry::Registry;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::counter::Counter;
use prometheus_client::encoding::EncodeLabelSet;
use std::sync::Arc;

//labels
#[derive(Debug, Clone , Hash, PartialEq , Eq, EncodeLabelSet)]
pub struct MetricLabels{
    pub method : String , 
    pub path : String 
}
// Purpose: Holds shared application state (MongoDB database connection)
// Why Clone: Axum requires state to be cloneable to share across request handlers
// Benefit: Avoids creating new DB connections for each request
#[derive(Clone)]
pub struct AppState {
    pub db: Database,
 // registry wrapped in arc for thread saftey (WHAT???)
    pub registry : Arc<Registry>,
// store metricfamily for easy access in handlers
    pub request_counter: Family<MetricLabels,Counter>, 


}
