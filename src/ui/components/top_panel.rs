use crate::app::{AppAction, DesiredAppAction};
use crate::context::AppContext;
use dash_sdk::dashcore_rpc::dashcore::Network;
use egui::{
    Align, Color32, Context, Frame, Layout, Margin, RichText, Stroke, TextBuffer, TopBottomPanel,
    Ui,
};
use std::sync::Arc;

fn add_location_view(ui: &mut Ui, location: Vec<(&str, AppAction)>) -> AppAction {
    let mut action = AppAction::None;
    let font_id = egui::FontId::proportional(22.0);

    egui::menu::bar(ui, |ui| {
        ui.horizontal(|ui| {
            let len = location.len();
            for (index, (text, location_action)) in location.into_iter().enumerate() {
                if ui
                    .button(
                        RichText::new(text)
                            .font(font_id.clone())
                            .color(Color32::WHITE),
                    )
                    .clicked()
                {
                    action = location_action;
                }

                // Add a separator (e.g., '>' symbol) between buttons, except for the last one
                if index < len - 1 {
                    ui.label(
                        RichText::new(">")
                            .font(font_id.clone())
                            .color(Color32::WHITE),
                    );
                }
            }
        });
    });

    action
}

pub fn add_top_panel(
    ctx: &Context,
    app_context: &Arc<AppContext>,
    location: Vec<(&str, AppAction)>,
    right_button: Option<(&str, DesiredAppAction)>,
) -> AppAction {
    let mut action = AppAction::None;
    let color = match app_context.network {
        Network::Dash => Color32::from_rgb(21, 101, 192), // A blue color for mainnet
        Network::Testnet => Color32::from_rgb(255, 165, 0), // Orange for testnet
        Network::Devnet => Color32::from_rgb(255, 0, 0),  // Red for devnet
        Network::Regtest => Color32::from_rgb(139, 69, 19), // Orange-brown for regtest
        _ => Color32::BLACK,
    };
    TopBottomPanel::top("top_panel")
        .frame(
            Frame::none()
                .fill(color) // Dash blue color
                .inner_margin(Margin::symmetric(10.0, 10.0)),
        ) // Customize inner margin (top/bottom padding)
        .exact_height(50.0) // Set exact height for the panel
        .show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // Left-aligned content with white text
                action = add_location_view(ui, location);

                if let Some((text, right_button_action)) = right_button {
                    // Right-aligned content with white text
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.add_space(8.0);

                        // Define the font and color
                        let font_id = egui::FontId::proportional(16.0); // Adjust the font size as needed
                        let color = Color32::WHITE;

                        // Calculate the text size using the new layout method
                        let button_text = text.to_string();
                        let text_size = ui
                            .fonts(|fonts| {
                                fonts.layout_no_wrap(button_text.clone(), font_id.clone(), color)
                            })
                            .size();

                        let button_width = text_size.x + 16.0; // Add some padding for the button

                        let button = egui::Button::new(RichText::new(text).color(Color32::WHITE))
                            .fill(Color32::from_rgb(0, 128, 255)) // Button background color
                            .frame(true) // Frame to make it look like a button
                            .rounding(3.0) // Rounded corners
                            .stroke(Stroke::new(1.0, Color32::WHITE)) // Border with white stroke
                            .min_size(egui::vec2(button_width, 30.0));

                        if ui.add(button).clicked() {
                            action = right_button_action.create_action(app_context);
                        }
                    });
                }
            });
        });
    action
}
