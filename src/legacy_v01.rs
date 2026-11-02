use macroquad::prelude::*;
use std::collections::HashMap;

const RADIUS: i32 = 20;
const REACH: f32 = 6.0;
const PLAYER_HEIGHT: f32 = 1.7;
const EYE_HEIGHT: f32 = 1.55;
const PLAYER_RADIUS: f32 = 0.28;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Block { Grass, Dirt, Stone, Wood, Leaves, Sand }
impl Block {
    fn color(self) -> Color { match self {
        Self::Grass => Color::new(0.35,0.65,0.23,1.),
        Self::Dirt => Color::new(0.48,0.30,0.17,1.),
        Self::Stone => GRAY,
        Self::Wood => Color::new(0.47,0.31,0.17,1.),
        Self::Leaves => Color::new(0.20,0.49,0.17,1.),
        Self::Sand => Color::new(0.83,0.77,0.51,1.),
    }}
    fn name(self) -> &'static str { match self { Self::Grass=>"Grass",Self::Dirt=>"Dirt",Self::Stone=>"Stone",Self::Wood=>"Wood",Self::Leaves=>"Leaves",Self::Sand=>"Sand" } }
}
type Pos = (i32,i32,i32);
fn terrain_height(x:i32,z:i32)->i32 {
    let a=(x as f32 * 0.17).sin()*2.4;
    let b=(z as f32 * 0.13).cos()*2.0;
    let c=((x+z) as f32 * 0.065).sin()*3.0;
    (4.0+a+b+c).round() as i32
}
fn generate()->HashMap<Pos,Block> {
    let mut world=HashMap::new();
    for x in -RADIUS..=RADIUS { for z in -RADIUS..=RADIUS {
        let h=terrain_height(x,z);
        for y in -5..=h {
            let block=if y==h {if h<=1 {Block::Sand} else {Block::Grass}}
                else if y>=h-3 {Block::Dirt} else {Block::Stone};
            world.insert((x,y,z),block);
        }
        let seed=(x.wrapping_mul(73856093)^z.wrapping_mul(19349663)) as u32;
        if seed%127==0 && h>2 && x.abs()<RADIUS-3 && z.abs()<RADIUS-3 {
            for dy in 1..=4 {world.insert((x,h+dy,z),Block::Wood);}
            for dx in -2i32..=2 {for dz in -2i32..=2 {for dy in 3i32..=5 {
                if dx.abs()+dz.abs()+(dy-4).abs()<=4 {world.entry((x+dx,h+dy,z+dz)).or_insert(Block::Leaves);}
            }}}
        }
    }}
    world
}
fn solid(world:&HashMap<Pos,Block>,p:Vec3)->bool {
    world.contains_key(&(p.x.floor() as i32,p.y.floor() as i32,p.z.floor() as i32))
}
fn collides(world:&HashMap<Pos,Block>,feet:Vec3)->bool {
    for x in [-PLAYER_RADIUS,PLAYER_RADIUS] {for z in [-PLAYER_RADIUS,PLAYER_RADIUS] {
        for y in [0.05,0.85,PLAYER_HEIGHT-0.05] {
            if solid(world,feet+vec3(x,y,z)) {return true;}
        }
    }} false
}
fn raycast(world:&HashMap<Pos,Block>,origin:Vec3,dir:Vec3)->Option<(Pos,Pos)> {
    let mut previous=(origin.x.floor() as i32,origin.y.floor() as i32,origin.z.floor() as i32);
    let mut t=0.0;
    while t<=REACH {
        let p=origin+dir*t;
        let current=(p.x.floor() as i32,p.y.floor() as i32,p.z.floor() as i32);
        if world.contains_key(&current) {return Some((current,previous));}
        previous=current;
        t+=0.035;
    }
    None
}
fn window_conf()->Conf {Conf {window_title:"RustCraft — Milestone 1".into(),window_width:1100,window_height:700,window_resizable:true,..Default::default()}}

#[macroquad::main(window_conf)]
async fn main() {
    let mut world=generate();
    let mut player=vec3(0.5,terrain_height(0,0) as f32+2.0,0.5);
    let mut yaw=0.0f32;
    let mut pitch=0.0f32;
    let mut velocity_y=0.0f32;
    let mut selected=0usize;
    let palette=[Block::Grass,Block::Dirt,Block::Stone,Block::Wood,Block::Leaves,Block::Sand];
    let mut captured=false;
    let mut fly=false;
    loop {
        let dt=get_frame_time().min(0.05);
        if is_key_pressed(KeyCode::Escape) {captured=false;set_cursor_grab(false);show_mouse(true);}
        if is_mouse_button_pressed(MouseButton::Left) && !captured {captured=true;set_cursor_grab(true);show_mouse(false);}
        if is_key_pressed(KeyCode::F) {fly=!fly;velocity_y=0.;}
        if is_key_pressed(KeyCode::R) {world=generate();player=vec3(0.5,terrain_height(0,0) as f32+2.,0.5);velocity_y=0.;}
        for i in 0..palette.len() {let key=match i {0=>KeyCode::Key1,1=>KeyCode::Key2,2=>KeyCode::Key3,3=>KeyCode::Key4,4=>KeyCode::Key5,_=>KeyCode::Key6};if is_key_pressed(key) {selected=i;}}
        let (_sx,sy)=mouse_wheel();if sy!=0. {selected=((selected as i32-(sy.signum() as i32)).rem_euclid(palette.len() as i32)) as usize;}
        if captured {let delta=mouse_delta_position();yaw+=delta.x*0.0025;pitch=(pitch-delta.y*0.0025).clamp(-1.54,1.54);}
        let direction=vec3(yaw.sin()*pitch.cos(),pitch.sin(),yaw.cos()*pitch.cos());
        let forward=vec3(yaw.sin(),0.,yaw.cos());
        let right=vec3(forward.z,0.,-forward.x);
        let mut motion=Vec3::ZERO;
        if is_key_down(KeyCode::W){motion+=forward;}
        if is_key_down(KeyCode::S){motion-=forward;}
        if is_key_down(KeyCode::D){motion+=right;}
        if is_key_down(KeyCode::A){motion-=right;}
        if motion.length_squared()>0. {motion=motion.normalize();}
        let speed=if is_key_down(KeyCode::LeftShift){8.0}else{4.5};
        let horizontal=motion*speed*dt;
        if fly {
            player+=horizontal;
            if is_key_down(KeyCode::Space){player.y+=speed*dt;}
            if is_key_down(KeyCode::LeftControl){player.y-=speed*dt;}
        } else {
            let grounded=collides(&world,player+vec3(0.,-0.08,0.));
            if grounded && velocity_y<0. {velocity_y=0.;}
            if grounded && is_key_pressed(KeyCode::Space){velocity_y=7.5;}
            velocity_y-=20.0*dt;
            let mut candidate=player;candidate.x+=horizontal.x;
            if !collides(&world,candidate){player.x=candidate.x;}
            candidate=player;candidate.z+=horizontal.z;
            if !collides(&world,candidate){player.z=candidate.z;}
            candidate=player;candidate.y+=velocity_y*dt;
            if !collides(&world,candidate){player.y=candidate.y;}else{velocity_y=0.;}
        }
        if player.y < -20. {player=vec3(0.5,terrain_height(0,0) as f32+3.,0.5);velocity_y=0.;}
        let eye=player+vec3(0.,EYE_HEIGHT,0.);
        let target=raycast(&world,eye,direction);
        if captured {
            if is_mouse_button_pressed(MouseButton::Left){if let Some((hit,_))=target{world.remove(&hit);}}
            if is_mouse_button_pressed(MouseButton::Right){if let Some((_,empty))=target{
                if !world.contains_key(&empty) {world.insert(empty,palette[selected]);
                    if collides(&world,player){world.remove(&empty);}
                }
            }}
        }
        clear_background(Color::new(0.55,0.77,0.96,1.));
        set_camera(&Camera3D {position:eye,target:eye+direction,up:vec3(0.,1.,0.),fovy:75.,..Default::default()});
        for (&(x,y,z),&block) in world.iter() {
            if (x as f32-player.x).abs()>24. || (z as f32-player.z).abs()>24. {continue;}
            let neighbors=[(x+1,y,z),(x-1,y,z),(x,y+1,z),(x,y-1,z),(x,y,z+1),(x,y,z-1)];
            if neighbors.iter().all(|p|world.contains_key(p)){continue;}
            draw_cube(vec3(x as f32+0.5,y as f32+0.5,z as f32+0.5),Vec3::ONE,None,block.color());
        }
        if let Some((hit,_))=target {
            draw_cube_wires(vec3(hit.0 as f32+0.5,hit.1 as f32+0.5,hit.2 as f32+0.5),vec3(1.012,1.012,1.012),BLACK);
        }
        set_default_camera();
        let sw=screen_width();let sh=screen_height();
        draw_line(sw/2.-8.,sh/2.,sw/2.+8.,sh/2.,2.,BLACK);
        draw_line(sw/2.,sh/2.-8.,sw/2.,sh/2.+8.,2.,BLACK);
        draw_rectangle(8.,8.,390.,82.,Color::new(0.,0.,0.,0.55));
        draw_text(&format!("RustCraft v0.1 | FPS {} | Blocks {}",get_fps(),world.len()),16.,30.,22.,WHITE);
        draw_text(&format!("XYZ {:.1} {:.1} {:.1} | {}",player.x,player.y,player.z,if fly{"FLY"}else{"SURVIVAL MOVEMENT"}),16.,54.,18.,WHITE);
        draw_text(if captured{"ESC: release mouse"}else{"Click to capture mouse"},16.,77.,18.,YELLOW);
        let slot_w=70.;let start=sw/2.-slot_w*palette.len() as f32/2.;
        for (i,block) in palette.iter().enumerate(){let x=start+i as f32*slot_w;
            draw_rectangle(x,sh-76.,slot_w-4.,64.,if i==selected{YELLOW}else{DARKGRAY});
            draw_rectangle(x+4.,sh-72.,slot_w-12.,56.,block.color());
            draw_text(&format!("{}",i+1),x+6.,sh-54.,20.,WHITE);
            draw_text(block.name(),x+5.,sh-22.,14.,WHITE);
        }
        draw_text("WASD move | Mouse look | Space jump | Shift sprint | LMB mine | RMB place | 1-6 blocks | F fly | R reset",12.,sh-90.,17.,BLACK);
        next_frame().await;
    }
}
