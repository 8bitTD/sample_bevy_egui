use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy::prelude::*;

mod ime;

#[derive(Resource)] 
pub struct MyApp{
    pub txt: String,
    pub ime: ime::ImeGroup,
}
impl Default for MyApp{
    fn default() -> Self{
        MyApp{
            txt: String::new(),
            ime: ime::ImeGroup::default(),
        }
    }
}

fn main() {
    App::new()    
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            position: WindowPosition::new(IVec2::new( 400, 200)),
            resolution: (220.0, 100.0).into(),
            ..default()
        }),
        ..default()
    }))

    .add_plugins(EguiPlugin) 
    .insert_resource(MyApp::default())
    .add_systems(Startup, setup_system)
    .add_systems(Update, 
        (
            ui_system,
            ime::listen_ime_events
        ) 
    )      
    .run();
}

pub fn setup_system(
    mut egui_context: EguiContexts,
    mut windows: Query<&mut Window>
) {

    let mut window = windows.single_mut();
    window.ime_enabled = true;
    let mut txt_font = egui::FontDefinitions::default();
    txt_font.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "Meiryo".to_owned());
    let fd = egui::FontData::from_static(include_bytes!("C:/Windows/Fonts/Meiryo.ttc"));
    txt_font.font_data.insert("Meiryo".to_owned(), fd);
    egui_context.ctx_mut().set_fonts(txt_font); 
}

pub fn ui_system(
    mut contexts: EguiContexts, 
    mut app: ResMut<MyApp>, 
) {
    let ctx = contexts.ctx_mut();
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui|{
        let teo = ime::get_texteditoutput(ui, &mut app.as_mut().ime.ime_txt, 200.0);
        teo.state.store(ctx, teo.response.id);
        app.txt = app.ime.ime_txt.text.to_string();
    });
}