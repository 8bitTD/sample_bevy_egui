use bevy_egui::egui;
use bevy::prelude::*;

#[derive()]
pub struct ImeGroup{
    pub ime_txt: ImeText,
}
impl Default for ImeGroup{
    fn default() -> Self{
        ImeGroup { 
            ime_txt: ImeText::default(),
        }
    }
}
#[derive()]
pub struct ImeText{
    pub text: String,
    pub ime_string: String,
    pub ime_string_index: usize,
    pub cursor_index: usize,
    pub is_ime_input: bool,
    pub is_focus: bool,
    pub is_ime: bool,
    pub is_cursor_move: bool,
}
impl Default for ImeText{
    fn default() -> Self{
        ImeText{
            text: String::from(""),
            ime_string: String::from(""),
            ime_string_index: 0,
            cursor_index: 0,
            is_ime_input: false,
            is_focus: false,
            is_ime: false,
            is_cursor_move: true,
        }
    }
}

pub fn get_texteditoutput(ui: &mut egui::Ui, ime: &mut ImeText, width: f32) -> egui::text_edit::TextEditOutput{
    let mut lyt = |ui: &egui::Ui, string: &str, _wrap_width: f32| {
        let tmp = get_layoutjob(string, ime);
        ui.fonts(|f| f.layout_job(tmp))
    };
    let mut tmp_text = match ime.ime_string.len(){
        0 => {ime.text.to_string()},
        _ => {
            let mut front = String::new();
            let mut back = String::new();
            let mut cnt = 0;
            for c in ime.text.chars(){
                if cnt < ime.cursor_index{ front.push_str(&c.to_string()); } 
                else{ back.push_str(&c.to_string()); }
                cnt += 1;
            }                 
            format!("{}{}{}", front, ime.ime_string, back)
        }
    };
    let mut te1 = egui::TextEdit::singleline(&mut tmp_text).desired_width(width).layouter(&mut lyt).show(ui);
    ime.is_focus = te1.response.has_focus();
    if !ime.is_ime {ime.text = tmp_text.to_string();}
    if te1.cursor_range.is_some(){ 
        ime.cursor_index = te1.cursor_range.unwrap().secondary.rcursor.column; 
    }
    if ime.is_ime {
        
    }
    if ime.is_ime_input{ 
        ime.is_ime_input = false;

        if ime.is_cursor_move{
            let mut res_cursor = te1.cursor_range.unwrap().primary.clone();
            for _ in 0..ime.ime_string_index{
                res_cursor = te1.galley.cursor_right_one_character(&res_cursor);
            }
            let cr = egui::text_edit::CursorRange{
                primary: res_cursor,
                secondary: res_cursor,
            };
            te1.state.set_cursor_range(Some(cr));
        }
    }
    if !ime.is_cursor_move{
        ime.is_cursor_move = true;
    }
    te1
}

fn get_layoutjob(string: &str, ime: &ImeText) -> egui::text::LayoutJob{
    let tmp = match ime.is_ime{
        false => { egui::text::LayoutJob::simple_singleline(string.into(),egui::FontId::default(), egui::Color32::WHITE) },
        _ => {
            let mut front = String::new();
            let mut back = String::new();
            let mut cnt = 0;
            for c in ime.text.chars(){
                if cnt < ime.cursor_index{ front.push_str(&c.to_string()); } 
                else{ back.push_str(&c.to_string()); }
                cnt += 1;
            }

            let mut lss:Vec<egui::text::LayoutSection> = vec![];
            let mut f_cnt = 0;
            let mut b_cnt = 0;
            b_cnt = b_cnt + front.len();
            let ls_front = egui::text::LayoutSection {
                leading_space: 0.0,
                byte_range: f_cnt..b_cnt,
                format: egui::TextFormat {
                    color: egui::Color32::WHITE,
                    ..Default::default()
                },
            };
            lss.push(ls_front);
            f_cnt = b_cnt;

            b_cnt = b_cnt + ime.ime_string.len();
            let ls_text = egui::text::LayoutSection {
                leading_space: 0.0,
                byte_range: f_cnt..b_cnt,
                format: egui::TextFormat {
                    color: egui::Color32::GREEN,
                    background: egui::Color32::from_rgb(0, 128, 64),
                    ..Default::default()
                },
            };
            lss.push(ls_text);
            f_cnt = b_cnt;

            b_cnt = b_cnt + back.len();
            let ls_back = egui::text::LayoutSection {
                leading_space: 0.0,
                byte_range: f_cnt..b_cnt,
                format: egui::TextFormat {
                    color: egui::Color32::WHITE,
                    ..Default::default()
                },
            };
            lss.push(ls_back);

            egui::text::LayoutJob {
                sections: lss,
                text: format!("{}{}{}",front, ime.ime_string, back),        
                ..Default::default()
            }
        }
    };
    tmp
}

pub fn listen_ime_events(
    mut events: EventReader<Ime>,
    mut app: ResMut<super::MyApp>, 
) {
    for event in events.iter() {
        match event {
            Ime::Preedit { value, cursor, .. } if cursor.is_some() => {
                if app.ime.ime_txt.is_focus{ 
                    app.ime.ime_txt.ime_string = value.to_string();
                    app.ime.ime_txt.ime_string_index = app.ime.ime_txt.ime_string.chars().count();
                }
            }
            Ime::Preedit { cursor, .. } if cursor.is_none() => {
                
            }
            Ime::Commit { value,.. } => {
                if value.is_empty(){
                    app.ime.ime_txt.is_cursor_move = false;
                }      
                if app.ime.ime_txt.is_focus{
                    let tmp = value.to_string();
                    if app.ime.ime_txt.text.chars().count() == app.ime.ime_txt.cursor_index{
                        app.ime.ime_txt.text.push_str(&tmp);
                    }else{
                        let mut front = String::new();
                        let mut back = String::new();
                        let mut cnt = 0;
                        for c in app.ime.ime_txt.text.chars(){
                            if cnt < app.ime.ime_txt.cursor_index{ front.push_str(&c.to_string()); } 
                            else{ back.push_str(&c.to_string()); }
                            cnt += 1;
                        }                 
                        app.ime.ime_txt.text = format!("{}{}{}", front, tmp, back);
                    }
                    app.ime.ime_txt.is_ime_input = true;
                    app.ime.ime_txt.ime_string = String::new();
                }                
            }
            Ime::Enabled { .. } => { 
                app.ime.ime_txt.is_ime = true;
            }
            Ime::Disabled { .. } => { 
                app.ime.ime_txt.is_ime = false;
            }
            _ => (),
        }
    }
}