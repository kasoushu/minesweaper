use graphics::types::Color;
use graphics::{Graphics, Context, CharacterCache};
use crate::Controller::MineGridController;


pub struct MineGridViewSetting{
    pub position:[f64;2],
    pub size:f64,
    pub background_color : Color,
    pub border_color:Color,
    pub cell_background:Color,
    pub edge_color:Color,
    pub line_color:Color,
    pub text_color:Color,
    pub line_radius:f64,
    pub border_radius:f64,
}

impl MineGridViewSetting{

    pub fn new() -> MineGridViewSetting{
        MineGridViewSetting{
            position:[10.0,10.0],
            size:600.0,
            background_color:[0.8,0.8,1.0,1.0],
            border_color:[0.8,0.3,0.0,0.99],
            edge_color: [0.0, 0.0, 0.2, 1.0],
            line_color: [0.0, 0.0, 0.2, 1.0],
            cell_background: [0.8, 0.8, 0.8, 0.8],
            line_radius: 1.0,
            text_color:[0.0,0.0,0.1,1.0],
            border_radius:3.0
        }
    }
}

pub struct MineGridView{
    pub settings:MineGridViewSetting,
}
impl MineGridView{
    pub fn new(settings:MineGridViewSetting) -> MineGridView{
        MineGridView{
            settings
        }
    }
    pub fn draw<G:Graphics,C>(
        &self,
        controller:&MineGridController,
        c:&Context,
        g:& mut G,
        glyphs:&mut C
    )
    where C:CharacterCache<Texture = G::Texture>
    {
        use graphics::{Line,Rectangle,Image,Transformed,Text};
        let ref settings = self.settings;
        let board_size = [settings.position[0],settings.position[1],settings.size,settings.size];
        Rectangle::new(settings.background_color).draw(
            board_size,&c.draw_state,c.transform,g
        );

        match controller.check_state() {
            -1 =>{
                Text::new_color(settings.text_color,32).draw(
                    "you are loser",
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(settings.size/2.0-75.0,settings.size/2.0),
                    g
                );
            },
            1 =>{
                Text::new_color(settings.text_color,32).draw(
                    "you win !!!!!",
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(settings.size/2.0-75.0,settings.size/2.0),
                    g
                );
            },
            0 =>{

                //draw characters
                let text_image = Image::new_color(settings.text_color);
                let cell_size = settings.size/10.0;
                for i in 0..10{
                    for j in 0..10{
                        if controller.visit(i,j) {
                            let pos =[
                                settings.position[0]+i as f64 *cell_size +20.0,
                                settings.position[1]+j as f64 *cell_size +0.0
                            ];
                            if let Ok(character) = glyphs.character(34,controller.get_char(i,j)){
                                let ch_x = pos[0] +character.left();
                                let ch_y = pos[1] +character.top();
                                let text_image = text_image.src_rect(
                                  [character.atlas_offset[0],
                                        character.atlas_offset[1],
                                      character.atlas_size[0],
                                      character.atlas_size[1],
                                  ]
                                );
                                text_image.draw(character.texture,&c.draw_state,c.transform.trans(ch_x,ch_y),g);
                            }
                        }
                    }
                }

                // draw '0' rectangle
                for i in 0..10{
                    for j in 0..10{
                        if controller.visit(i,j)&& controller.get_char(i,j)==' '{
                            let pos = [
                                settings.position[0]+ i as f64 *cell_size,
                                settings.position[1]+ j as f64 *cell_size,
                            ];
                            let board = [pos[0],pos[1],cell_size,cell_size];
                            Rectangle::new(settings.cell_background).draw(
                                board,&c.draw_state,c.transform,g
                            );
                        }
                    }
                }


                //draw line and border
                Rectangle::new_border(settings.border_color,settings.border_radius).draw(
                    board_size,&c.draw_state,c.transform,g
                );
                //line
                let (ceiling_x,ceiling_y) = (
                    settings.position[0]+settings.size,
                    settings.position[1]+settings.size,
                    );
                let line = Line::new(settings.line_color,settings.line_radius);
                for i in 0..10 {
                    let x = settings.position[0]+ i as f64 /10.0 *settings.size;
                    let y = settings.position[1]+ i as f64 /10.0 *settings.size;
                    let line_v = [x,settings.position[0],x,ceiling_y];
                    let line_h = [settings.position[0],y,ceiling_x,y];
                    line.draw(line_v,&c.draw_state,c.transform,g);
                    line.draw(line_h,&c.draw_state,c.transform,g);
                }
            },
            _ => {

            },
        }




        }
}


