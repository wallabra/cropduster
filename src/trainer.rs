use crate::crop::{self, Crop, CropBuilderChange};
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Clone)]
pub struct Genome {
    start: Crop,
    changes: Vec<crop::SetCrop>,
    num_croptypes: u8,
}

impl Genome {
    pub fn new(base: &Crop, num_croptypes: u8) -> Self {
        Self {
            start: base.clone(),
            changes: vec![],
            num_croptypes,
        }
    }

    pub fn generate(&self) -> Crop {
        let mut new_crop = self.start.clone();

        for change in &self.changes {
            change.apply(&mut new_crop);
        }

        new_crop
    }

    pub fn score(&self) -> i32 {
        let crop = self.generate();
        let mut score = 0;

        for y in 0..crop.grid.column_len() {
            for x in 0..crop.grid.row_len() {
                if crop.grid[(y, x)] != 0 {
                    let neighbours = crop.crop_neighbours((x as isize, y as isize));

                    if neighbours != 2 {
                        score -= 1;
                    }
                }
            }
        }

        score
    }

    pub fn add_change(&mut self, pos: (usize, usize), new_croptype: u8) {
        self.changes.push(crop::SetCrop {
            xy: pos,
            croptype: new_croptype,
        });
    }

    pub fn mutate(&self) -> Genome {
        let mut positions: Vec<(usize, usize)> = vec![];
        let crop = self.generate();

        for y in 0..crop.grid.column_len() {
            for x in 0..crop.grid.row_len() {
                if crop.grid[(y, x)] != 0 {
                    positions.push((x, y));
                }
            }
        }

        let pos = *positions.choose(&mut rand::thread_rng()).unwrap();
        let new_croptype = rand::thread_rng().gen_range(1..=self.num_croptypes);

        let mut res = self.clone();
        res.add_change(pos, new_croptype);
        res
    }
}

pub struct TrainerParams {
    pub elite: f32,
    pub survivors: f32,
}

pub struct CropTrainer {
    base: Crop,
    population: Vec<Genome>,
    num_croptypes: u8,
    pub params: TrainerParams,
}

impl CropTrainer {
    pub fn new(
        base: &Crop,
        init_pop: usize,
        num_croptypes: u8,
        randomized: f32,
        params: TrainerParams,
    ) -> Self {
        let mut res = Self {
            base: base.clone(),
            population: (0..init_pop)
                .map(|_| Genome::new(base, num_croptypes))
                .collect(),
            num_croptypes,
            params,
        };

        let randoms = &(0..(randomized * init_pop as f32) as usize)
            .map(|_| res.make_random_genome())
            .collect::<Vec<Genome>>();

        res.population[0..(randomized * init_pop as f32) as usize].clone_from_slice(randoms);

        res.sort();

        res
    }

    pub fn make_random_crop(&self) -> Crop {
        let mut crop = self.base.clone();

        for y in 0..crop.grid.column_len() {
            for x in 0..crop.grid.row_len() {
                if crop.grid[(y, x)] != 0 {
                    crop.grid[(y, x)] = rand::thread_rng().gen_range(1..=self.num_croptypes);
                }
            }
        }

        crop
    }

    pub fn make_random_genome(&self) -> Genome {
        let mut genome = Genome::new(&self.base, self.num_croptypes);

        for y in 0..self.base.grid.column_len() {
            for x in 0..self.base.grid.row_len() {
                if self.base.grid[(y, x)] != 0 {
                    genome.add_change((x, y), rand::thread_rng().gen_range(1..=self.num_croptypes));
                }
            }
        }

        genome
    }

    fn sort(&mut self) {
        self.population.sort_by_key(|genome| -genome.score())
    }

    fn cutoff_index(&self, cutoff: f32) -> usize {
        (cutoff * self.population.len() as f32) as usize
    }

    pub fn mutate(&mut self) {
        self.sort();

        // Mutate only non-elite
        let cutoff = self.cutoff_index(self.params.elite);
        let upper_cutoff = self.cutoff_index(self.params.survivors) - cutoff;
        for genome in self.population.iter_mut().skip(cutoff).take(upper_cutoff) {
            genome.mutate();
        }
    }

    pub fn repopulate_lumpen(&mut self) {
        let cutoff = self.cutoff_index(self.params.survivors);
        // let parents = (&self.population)[0..cutoff].to_vec();
        let full_len = self.population.len();

        let randoms = (cutoff..self.population.len())
            .map(|_| self.make_random_genome())
            .collect::<Vec<Genome>>();
        self.population[cutoff..full_len].clone_from_slice(&randoms);
    }

    pub fn step(&mut self) {
        self.sort();

        self.repopulate_lumpen();
        self.mutate();
    }

    pub fn best(&mut self) -> &Genome {
        self.sort();
        &self.population[0]
    }

    pub fn improve(
        from: &Crop,
        population: usize,
        num_croptypes: u8,
        randomized: f32,
        params: TrainerParams,
        iterations: usize,
    ) -> Crop {
        let mut trainer = Self::new(from, population, num_croptypes, randomized, params);

        for i in 1..=iterations {
            trainer.step();
            println!("Generation {} best score {}", i, trainer.best().score())
        }

        trainer.best().generate()
    }
}
