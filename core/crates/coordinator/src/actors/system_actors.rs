// use std::{path::PathBuf, time::Duration};

// use anything_graph::{Flow, Flowfile};
// use anything_persistence::{flow_repo, FlowRepo, FlowRepoImpl};
// use anything_store::{types::ChangeMessage, FileStore};

// use crate::{CoordinatorError, CoordinatorResult};

// pub struct SystemActor {
//     pub file_store: FileStore,
//     pub flow_repo: FlowRepoImpl,
// }

// // impl Actor for SystemActor {
// //     type Context = Context<Self>;

// //     fn on_start(&mut self, ctx: &mut Context<Self>) {
// //         println!("in started: {:?}", System::current());
// //         // ctx.run_later(Duration::from_millis(200), |act, ctx| {
// //         //     println!("started later ");
// //         // });
// //     }
// // }

// // impl Handler<SystemMessages> for SystemActor {
// //     type Result = crate::error::CoordinatorResult<()>;

// //     fn handle(&mut self, msg: SystemMessages, _ctx: &mut Context<Self>) -> Self::Result {
// //         println!("Handling system message got a message: {:?}", msg);
// //         match msg {
// //             SystemMessages::StoreChanged(_) => {
// //                 println!("Store changed");
// //                 Ok(())
// //             }
// //         }
// //     }
// // }

// impl SystemActor {
//     pub async fn reload_flows(&mut self, context: &mut Context<Self>) -> CoordinatorResult<()> {
//         let root_dir = self.file_store.store_path(&["flows"]);

//         let flow_repo = self.flow_repo.clone();

//         let flow_files: Vec<PathBuf> = anything_common::utils::anythingfs::read_flow_directories(
//             root_dir,
//             vec!["toml".to_string()],
//         )
//         .map_err(|e| {
//             tracing::error!("error when reading flow directories: {:#?}", e);
//             CoordinatorError::AnythingError(e)
//         })?;

//         for flow_file_path in flow_files {
//             let flow = match Flowfile::from_file(flow_file_path) {
//                 Ok(flow) => flow,
//                 Err(e) => {
//                     tracing::error!("error when parsing flow file: {:#?}", e);
//                     continue;
//                 }
//             };
//             let flow: Flow = flow.into();
//             match flow_repo.create_or_update_flow(flow.into()).await {
//                 Ok(_) => {}
//                 Err(e) => {
//                     tracing::error!("error when saving flow: {:#?}", e);
//                     continue;
//                 }
//             };
//             // self.flow_repo.clone().save_flow(flow.into()).await?;
//         }

//         Ok(())
//     }
// }
