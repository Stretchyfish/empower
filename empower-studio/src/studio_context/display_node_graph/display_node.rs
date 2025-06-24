use egui;
use empower_node_graph::EmpowerKey;
use empower_node_graph::Node;

pub struct DisplayNodeReponse
{
    pub key: i32,
    pub kind: DisplayNodeResponseType,
}

pub enum DisplayNodeResponseType
{
    Clicked,
    Hover,
    InsideSelectionArea,
    ClickedInputPort(EmpowerKey),
    ClickedOutputPort(EmpowerKey)
}

pub struct DisplayNode
{
    pub title: String,
    // pub key: EmpowerKey,
    pub position: egui::Pos2,
    pub size: egui::Vec2,
}

impl DisplayNode
{
    pub fn new(title: String,position: egui::Pos2, size: egui::Vec2) -> Self
    {
        Self
        {
            title,
            // key: 0,
            position,
            size, 
        }
    }

    // pub fn new_with_position(position: egui::Pos2) -> Self
    // {
    //     Self
    //     {
    //         title: String::from("test"),
    //         // key: 0,
    //         position,
    //         size: egui::Vec2 { x: 200.0, y: 200.0 }
    //     }
    // }

    // pub fn new_with_key(new_key: EmpowerKey) -> Self
    // {
    //     Self
    //     {
    //         title: String::from("test"),
    //         // key: new_key,
    //         position: egui::Pos2::new(0.0, 0.0),
    //         size: egui::Vec2 { x: 200.0, y: 200.0 }
    //     }
    // }

    // pub fn new_with_key_and_position(new_key: EmpowerKey, new_position: egui::Pos2, new_size: egui::Vec2) -> Self
    // {
    //     Self
    //     {
    //         title: String::from("test"),
    //         // key: new_key,
    //         position: new_position,
    //         size: new_size        
    //     }
    // }

    pub fn show(&mut self, ui: &mut egui::Ui, empower_node: &mut Node)
    {
        let node_position = self.position;
        let node_screen_size= self.size;

        let node_rect= egui::Rect::from_min_size(
            node_position,
            node_screen_size
        );

        let node_body_color = egui::Color32::from_rgb(63, 63, 63);
        let rect_margin = egui::Vec2 { x: 10.0, y: 10.0 };
    
        let node_outline_rect = node_rect.expand2(rect_margin);

        // let node_is_selected = graph_viewport_state.selected_nodes.iter().any(| selected_node_key | *selected_node_key == display_node_key ); // @TODO, find a reduce the computation of this check
        let title_text_font_size = 40.0;
        let node_title = self.title.clone();
        let text_size = ui
            .painter()
            .layout_no_wrap(
                node_title.to_string(),
                egui::FontId::proportional(title_text_font_size),
                egui::Color32::YELLOW,
            )
            .size();

        let title_box_rect = egui::Rect::from_min_size(
            node_rect.min,
            egui::Vec2 {
                x: node_rect.size().x,
                y: text_size.y * 2.0,
            },
        );

        let node_body_title_area_overlap = 12.0;
        let node_body_bottom_area_overlap = 20.0;

        let node_title_pos= egui::Pos2 {
            x: title_box_rect.center().x,
            y: title_box_rect.min.y
                + (title_box_rect.size().y - node_body_title_area_overlap) / 2.0,
        };

        let node_rect_round_bottom = egui::Rect::from_min_size(
            egui::Pos2 {
                x: node_rect.min.x,
                y: node_rect.max.y - 20.0,
            },
            egui::Vec2 {
                x: node_rect.size().x,
                y: 20.0,
            },
        );

        let node_rect_without_title_and_bottom = egui::Rect::from_min_size(
            egui::Pos2 {
                x: node_rect.min.x,
                y: node_rect.min.y + title_box_rect.size().y - node_body_title_area_overlap,
            },
            egui::Vec2 {
                x: node_rect.size().x,
                y: node_rect.size().y - title_box_rect.size().y - node_rect_round_bottom.size().y
                    + node_body_bottom_area_overlap,
            },
        );

        let mut title_rect_color= egui::Color32::from_rgb(50, 50, 50);

        let node_reponse = ui.interact(
            title_box_rect,
            // egui::Id::new( graph_title.clone() + "_node_body_" + display_node_key.to_string().as_str()), // @TODO, find a way to move title out of state
            egui::Id::new( String::from("test") + "_node_body_" + empower_node.key.to_string().as_str()), // @TODO, find a way to move title out of state
            egui::Sense::click_and_drag(),
        );

        
        if node_reponse.hovered() // Important that this is done before clicked
        {
            // view_graph_node_reponse = Some( NodeViewReponse { key: display_node.key, kind: NodeViewResponseType::Hover } );
            title_rect_color= egui::Color32::from_rgb(40, 40, 40);
        }

        if node_reponse.clicked()
        {
            // view_graph_node_reponse = Some( NodeViewReponse { key: display_node_key, kind: NodeViewResponseType::Clicked } );
        }


        // if node_is_selected // @TODO, add this back
        // {
        //     ui.painter().rect(
        //         node_outline_rect,
        //         6.0,
        //         egui::Color32::ORANGE,
        //         egui::Stroke::NONE,
        //         egui::StrokeKind::Inside,
        //     );
        // }


        ui.painter().rect(
            title_box_rect,
            6.0,
            title_rect_color,
            egui::Stroke::NONE,
                egui::StrokeKind::Inside,
        );

        ui.painter().text(
            node_title_pos,
            // egui::Align2::LEFT_TOP,
            egui::Align2::CENTER_CENTER,
            node_title,
            // egui::FontId::monospace(40.0),
            egui::FontId::proportional(title_text_font_size),
            egui::Color32::WHITE,
        );



        // Show node body
        ui.painter().rect(
            node_rect_without_title_and_bottom,
            0.0,
            node_body_color,
            egui::Stroke::NONE,
                egui::StrokeKind::Inside,
        );

        // Show node bottom
        ui.painter().rect(
            node_rect_round_bottom,
            6.0,
            node_body_color,
            egui::Stroke::NONE,
                egui::StrokeKind::Inside,
        );
        
    }
}

