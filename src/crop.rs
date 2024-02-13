use array2d::Array2D;

#[derive(Clone)]
pub struct Crop {
    pub grid: Array2D<u8>,
}

impl Crop {
    pub fn builder(width: usize, height: usize) -> CropBuilder {
        CropBuilder {
            width,
            height,
            changes: vec![],
        }
    }

    pub fn filled(width: usize, height: usize) -> Self {
        Self {
            grid: Array2D::filled_with(1, height, width),
        }
    }

    pub fn has_coord(&self, coord: (isize, isize)) -> bool {
        coord.0 >= 0
            && coord.0 < self.grid.num_columns() as isize
            && coord.1 >= 0
            && coord.1 < self.grid.num_rows() as isize
    }

    pub fn crop_neighbours(&self, xy: (isize, isize)) -> u8 {
        let crop_match = self.grid[(xy.1 as usize, xy.0 as usize)];
        let coords = [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .map(|c| (xy.0 + c.0, xy.1 + c.1));

        coords
            .map(|c| {
                if !self.has_coord(c) {
                    0
                } else {
                    *self.grid.get(c.1 as usize, c.0 as usize).unwrap_or(&0)
                }
            })
            .map(|x| (x == crop_match) as u8)
            .sum()
    }

    pub fn print_rows(&self) {
        for row in self.grid.rows_iter() {
            println!(
                "{}",
                row.map(|&x| if x == 0 {
                    "*"
                } else {
                    unsafe {
                        "abcdefghijklmnopqrstuvwxyz".get_unchecked(x as usize - 1..x as usize)
                    }
                })
                .collect::<String>()
            )
        }
    }
}

pub struct CropBuilder {
    width: usize,
    height: usize,
    changes: Vec<Box<dyn CropBuilderChange>>,
}

impl CropBuilder {
    pub fn empty(mut self, xy: (usize, usize)) -> Self {
        self.changes.push(Box::from(EmptyAt { xy }));
        self
    }

    pub fn set(mut self, xy: (usize, usize), croptype: u8) -> Self {
        self.changes.push(Box::from(SetCrop { xy, croptype }));
        self
    }

    pub fn build(&mut self) -> Crop {
        let mut crop = Crop::filled(self.width, self.height);

        self.changes.iter().for_each(|change| {
            change.apply(&mut crop);
        });

        crop
    }
}

pub trait CropBuilderChange {
    fn apply(&self, crop: &mut Crop);
}

#[derive(Clone, Copy)]
pub struct EmptyAt {
    pub xy: (usize, usize),
}

impl CropBuilderChange for EmptyAt {
    fn apply(&self, crop: &mut Crop) {
        crop.grid[(self.xy.1, self.xy.0)] = 0;
    }
}

#[derive(Clone, Copy)]
pub struct SetCrop {
    pub xy: (usize, usize),
    pub croptype: u8,
}

impl CropBuilderChange for SetCrop {
    fn apply(&self, crop: &mut Crop) {
        crop.grid[(self.xy.1, self.xy.0)] = self.croptype;
    }
}
