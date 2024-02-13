pub mod crop;
pub mod trainer;

use crop::Crop;

fn main() {
    let crop = Crop::builder(9, 16)
        .empty((4, 4))
        .empty((4, 5))
        .empty((4, 6))
        .empty((4, 7))
        .empty((4, 8))
        .empty((4, 9))
        .empty((4, 10))
        .empty((4, 11))
        .build();
    let improved = trainer::CropTrainer::improve(
        &crop,
        5000,
        2,
        1.0,
        trainer::TrainerParams {
            elite: 0.2,
            survivors: 0.5,
        },
        60,
    );

    //improved.print_rows()
}
