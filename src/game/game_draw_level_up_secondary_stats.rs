use ggez::{
    graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect, Text},
    mint::Point2,
    Context,
};
use crate::player::player::Player;
use super::level_up_panel_config::LevelUpPanelConfig;

pub fn draw_character_secondary_stats(ctx: &mut Context, canvas: &mut Canvas, player: &mut Player) {
    let config: LevelUpPanelConfig = LevelUpPanelConfig::new();
    let right_panel: Rect = config.right_subpanel;

    let line_height: f32 = 60.0;
    let rect_width: f32 = right_panel.w - config.padding * 2.0;
    let rect_height: f32 = 40.0;

    let (health, speed, speed_jump, damage, defense) =
        player.character.stats.get_calculated_stats();

    let stat_labels: Vec<&str> = vec!["Vida", "Velocidad", "Salto", "Daño", "Defensa"];
    let stat_values: Vec<f32> = vec![health, speed, speed_jump, damage, defense];

    for (i, label) in stat_labels.iter().enumerate() {
        let y: f32 = right_panel.y + config.padding + i as f32 * line_height;
        let stat_rect: Mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(2.0),
            Rect::new(right_panel.x + config.padding, y, rect_width, rect_height),
            Color::new(1.0, 1.0, 1.0, 0.5),
        )
        .unwrap();

        canvas.draw(&stat_rect, DrawParam::default());

        let text: Text = Text::new(format!("{}: {:.1}", label, stat_values[i]));
        let params: DrawParam = DrawParam::new().dest(Point2 {
            x: right_panel.x + config.padding + 10.0,
            y: y + 15.0,
        });

        canvas.draw(&text, params);
    }
}
