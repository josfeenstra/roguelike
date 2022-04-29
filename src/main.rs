use rltk::{Rltk, GameState};

struct State {}
impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        ctx.print(1, 1, "Hello Rust World hallo oei this looks nice!");
        ctx.print(1, 2, "-------------------------------------------");
    }
}

fn main() -> rltk::BError {

    use rltk::RltkBuilder;
    let context = RltkBuilder::simple(50,30).unwrap()
        .with_title("Roguelike Tutorial")
        .build()?;
    let gs = State{ };
    rltk::main_loop(context, gs)
}