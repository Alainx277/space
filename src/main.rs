pub mod core;
pub mod entities;

use self::core::SpacePlugin;

use bevy_app::App;
use bevy_core::DefaultTaskPoolOptions;
fn main() {
    App::new()
        //.insert_resource(ReportExecutionOrderAmbiguities)
        .insert_resource(DefaultTaskPoolOptions::with_num_threads(2))
        .add_plugin(SpacePlugin)
        .run();
}
