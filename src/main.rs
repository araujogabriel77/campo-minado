use rand::Rng;
// use std::io;

const FIELD_SIZE: usize = 5;
const BOMB_QUANTITY: usize = FIELD_SIZE;
fn main() {
    let mut field = Box::new([[0; FIELD_SIZE]; FIELD_SIZE]);
    let mut bomb_coordinates = Box::new([[0; 2]; BOMB_QUANTITY]);

    let mut bomb_count = 0;
    while bomb_count < bomb_coordinates.len() {
        let bomb_position_x = set_coordinates();
        let bomb_position_y = set_coordinates();

        bomb_coordinates[bomb_count][0] = bomb_position_x;
        bomb_coordinates[bomb_count][1] = bomb_position_y;

        if field[bomb_position_x][bomb_position_y] == 0 {
            field[bomb_position_x][bomb_position_y] = 1;
            bomb_count += 1;
        }
    }

    for n in 0..=FIELD_SIZE - 1 {
        println!("{:?}", field[n]);
    }
}

fn set_coordinates() -> usize {
    return rand::thread_rng().gen_range(0..=FIELD_SIZE - 1);
}
