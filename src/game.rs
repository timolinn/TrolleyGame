struct RTrolleyProblem {}
struct Points<T> {
    kantpoints: T,
    utils: T,
}

struct Game {
    points: Points<i32>,
    dilemmas: Vec<RTrolleyProblem>,
    count: i64,
    weights: Vec<i64>,
}

impl Game {
    pub fn new(
        points: Points<i32>,
        dilemmas: Vec<RTrolleyProblem>,
        count: i32,
        weights: Vec<i32>,
    ) -> Game {
        g = Game {
            points: points,
            dilemmas: dilemmas,
            count: count,
            weights: weights,
        };

        g
    }

    pub fn play(&self) {
        self.count += 1;
        self.dilemmas = vec![RTrolleyProblem::new()]
    }
}

pub fn start() {
    println!("This is Trolley Game written in rust.");
}
