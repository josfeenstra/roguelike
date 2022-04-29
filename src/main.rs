use rltk::{Rltk, GameState};

const WIDTH: u32 = 50;
const HEIGHT: u32 = 30;
const HH: u32 = HEIGHT / 2;

struct State {}

fn print_menu(ctx : &mut Rltk) {
    ctx.print(4, HH + 0, "Welcome, Dungeoneer!");
    ctx.print(4, HH + 1, "--------------------");
    ctx.print(4, HH + 3, "> Play");
    ctx.print(4, HH + 4, "  Options");
    ctx.print(4, HH + 5, "  Quit");
}

impl GameState for State {

    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        print_menu(ctx);
    }
}

fn main() -> rltk::BError {

    use rltk::RltkBuilder;
    let context = RltkBuilder::simple(WIDTH, HEIGHT).unwrap()
        .with_title("Roguelike Tutorial")
        .build()?;
    let gs = State{ };
    rltk::main_loop(context, gs)
}