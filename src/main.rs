pub mod crop;
pub mod trainer;

use crop::Crop;

fn main() {
    let crop = Crop::builder(9, 16).empty((4, 4)).empty((4, 11)).build();
    //Crop::builder(6, 3)/*.empty((2, 2))*/.build();
    let improved = trainer::CropTrainer::improve(
        &crop,
        3000,
        2,
        1.0,
        trainer::TrainerParams {
            elite: 0.2,
            survivors: 0.55,
        },
        500,
    );

    //improved.print_rows()
}
