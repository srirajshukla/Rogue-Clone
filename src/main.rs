use rltk::GameState;
use rltk::RGB;
use rltk::Rltk;
use specs::prelude::*;
use specs_derive::Component;

mod map;
pub use map::*;
mod player;
pub use player::*;
mod components;
pub use components::*;

pub struct State {
    ecs: World
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk){
        ctx.cls();
        ctx.print(1, 1, "hello world");
        self.run_systems();
        player_input(self, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        
        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);


        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }

        let scores = self.ecs.read_storage::<Score>();
        for(score, pos) in (&scores, &positions).join() {
            ctx.print(pos.x, pos.y, format!("Score: {}", score.score));
        }
    }
}

struct LeftWalker {}
impl<'a> System<'a> for LeftWalker {
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);

    fn run(&mut self, (lefty, mut pos): (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>)){
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 {
                pos.x = 79;
            }
        }
    }
}

impl State {
    fn run_systems(&mut self){
        let mut lw = LeftWalker{};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

#[derive(Component, Debug)]
struct Player {}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    let context = RltkBuilder::simple80x50()
            .with_title("Roguelike Tutorial")
            .build()?;


    let mut gs = State{
        ecs: World::new()
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Score>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();
    gs.ecs.insert(new_map());

    gs.ecs.create_entity()
            .with(Position {x:40, y:25})
            .with(Renderable {
                glyph: rltk::to_cp437('@'),
                fg: RGB::named(rltk::YELLOW),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Player{})
            .build();

    gs.ecs.create_entity()
            .with(Position {x:42, y:25})
            .with(Score{score:32})
            .build();

    for i in 0..10{
        gs.ecs
            .create_entity()
            .with(Position {x: i*7, y:20})
            .with(Renderable {
                glyph: rltk::to_cp437('☺'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(LeftMover{})
            .build();
    }

    for i in 0..5{
        for j in i..5{
            return_glyph(&mut gs, i*5, j);
        }
    }


    rltk::main_loop(context, gs)
}

fn return_glyph(gs:&mut State,  x: i32, y: i32)  {
    gs.ecs
        .create_entity()
        .with(Position {x: x, y: y*5})
        .with(Renderable {
            glyph: rltk::to_cp437('☺'),
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
        })
        .build();
}

