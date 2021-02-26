use std::path::MAIN_SEPARATOR;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL,
            TextureSettings,Texture,GlyphCache,Filter
        };
use piston::{EventSettings, WindowSettings, Event, Events, EventLoop, RenderEvent, PressEvent};


mod MineGrid;
mod MineView;
mod Controller;

fn main() {
    let opengl = OpenGL::V3_2;
    let window_settings = WindowSettings::new("minesweep",[620,620])
        .graphics_api(opengl).exit_on_esc(true);
    let mut window:GlutinWindow  = window_settings.build().expect("error,windows do not build");    
    let mut events = Events::new(EventSettings::new()).lazy(true);
    let mut gl = GlGraphics::new(opengl);
    
    let mine_grid = MineGrid::MineGrid::new();
    // test mine
    MineGrid::print_grid(mine_grid.grid, 10);
    MineGrid::print_grid(mine_grid.answer,10);
    let my_grid_settings = MineView::MineGridViewSetting::new();
    let my_grid_view = MineView::MineGridView::new(my_grid_settings);
    let mut my_grid_controller = Controller::MineGridController::new(mine_grid);
    let texture_settings =  TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("src/assets/FiraSans-Regular.ttf",
                                         (),
                                         texture_settings
    ).expect("Could not load fonts");

    while let Some(e) = events.next(& mut window){
        use piston::input::{Button,Key,MouseButton};
            my_grid_controller.solve_event(
            my_grid_view.settings.position,
            my_grid_view.settings.size,
            &e);
        if let Some(args) = e.render_args(){
            gl.draw(args.viewport(),|c,g|{
                use graphics::clear;
                clear([1.0,1.0,1.0,1.0],g);
                my_grid_view.draw(&my_grid_controller,&c,g,glyphs);
            });
        }
    }
}
