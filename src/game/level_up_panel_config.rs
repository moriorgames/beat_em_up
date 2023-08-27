use ggez::graphics::Rect;

pub struct LevelUpPanelConfig {
    pub main_panel: Rect,
    pub left_subpanel: Rect,
    pub left_top_subpanel: Rect,
    pub left_bot_subpanel: Rect,
    pub right_subpanel: Rect,
    pub padding: f32,
}

impl LevelUpPanelConfig {
    const PADDING: f32 = 20.0;
    const PANEL_X: f32 = 800.0;
    const PANEL_Y: f32 = 100.0;
    const PANEL_W: f32 = 900.0;
    const PANEL_H: f32 = 800.0;
    const TOP_SUBPANEL_HEIGHT: f32 = 100.0;

    pub fn new() -> Self {
        let padding: f32 = Self::PADDING;
        let subpanel_width: f32 = (Self::PANEL_W - 3.0 * padding) / 2.0;
        let subpanel_height: f32 = Self::PANEL_H - 2.0 * padding;

        let main_panel: Rect =
            Rect::new(Self::PANEL_X, Self::PANEL_Y, Self::PANEL_W, Self::PANEL_H);

        let left_subpanel: Rect = Rect::new(
            main_panel.x + padding,
            main_panel.y + padding,
            subpanel_width,
            subpanel_height,
        );

        let left_top_subpanel: Rect = Rect::new(
            left_subpanel.x,
            left_subpanel.y,
            subpanel_width,
            Self::TOP_SUBPANEL_HEIGHT,
        );

        let left_bot_subpanel: Rect = Rect::new(
            left_subpanel.x,
            left_subpanel.y + Self::TOP_SUBPANEL_HEIGHT + padding,
            subpanel_width,
            subpanel_height - Self::TOP_SUBPANEL_HEIGHT - padding,
        );

        let right_subpanel: Rect = Rect::new(
            main_panel.x + 2.0 * padding + subpanel_width,
            main_panel.y + padding,
            subpanel_width,
            subpanel_height,
        );

        Self {
            main_panel,
            left_subpanel,
            left_top_subpanel,
            left_bot_subpanel,
            right_subpanel,
            padding,
        }
    }
}
