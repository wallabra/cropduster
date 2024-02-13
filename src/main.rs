pub mod crop;
pub mod trainer;

use crop::Crop;

fn main() {
    let crop = Crop::builder(9, 5).empty((4, 0)).build();
    let improved = trainer::CropTrainer::improve(
        &crop,
        5000,
        2,
        1.0,
        trainer::TrainerParams {
            elite: 0.2,
            survivors: 0.5,
        },
        40,
    );

    //improved.print_rows()
}
