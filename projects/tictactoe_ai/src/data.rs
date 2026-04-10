#[derive(Debug, Clone)]
pub struct TrainingExample {
    pub input: [f32; 9],
    pub target: [f32; 9],
    pub mask: [f32; 9],
}

pub fn load_examples() -> Vec<TrainingExample> {
    let mut examples: Vec<TrainingExample> = Vec::new();

    let example_1 = TrainingExample {
        // Tablero:
        // X | O | .
        // . | X | O
        // . | . | .
        input: [1.0, -1.0, 0.0, 0.0, 1.0, -1.0, 0.0, 0.0, 0.0],

        // Mejor jugada: casilla 8
        target: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0],

        // Casillas válidas: las vacías
        mask: [0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0],
    };

    let example_2 = TrainingExample {
        // Tablero:
        // X | . | O
        // . | X | .
        // O | . | .
        input: [1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -1.0, 0.0, 0.0],

        // Mejor jugada: casilla 8
        target: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0],

        mask: [0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0],
    };

    let example_3 = TrainingExample {
        // Tablero:
        // X | X | .
        // O | O | .
        // . | . | .
        input: [1.0, 1.0, 0.0, -1.0, -1.0, 0.0, 0.0, 0.0, 0.0],

        // Mejor jugada: casilla 2
        target: [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],

        mask: [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0],
    };

    examples.push(example_1);
    examples.push(example_2);
    examples.push(example_3);

    examples
}
