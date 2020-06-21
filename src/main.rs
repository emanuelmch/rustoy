/*
 * Copyright (c) 2020 Emanuel Machado da Silva
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use rand::Rng;

use std::mem::MaybeUninit;

const POPULATION_SIZE: usize = 40;

#[derive(Copy, Clone)]
struct Chromosome {
    genes: [i8; 10],
    fitness: i8,
}

impl Default for Chromosome {
    fn default() -> Self {
        let mut rng = rand::thread_rng();

        let mut genes = [0; 10];
        for i in 0..10 {
            genes[i] = rng.gen_range(0, 10);
        }
        let fitness = fitness(genes);
        return Chromosome { genes, fitness };
    }
}

fn fitness(genes: [i8; 10]) -> i8 {
    let mut result = 0;
    for i in 0..10 {
        // TODO Can I remove this cast somehow?
        if genes[i] == i as i8 {
            result += 1;
        }
    }
    return result;
}

fn run() -> Chromosome {
    let mut population = unsafe {
        let mut arr: [Chromosome; POPULATION_SIZE] = MaybeUninit::uninit().assume_init();
        for i in 0..arr.len() {
            arr[i] = Default::default();
        }
        arr
    };
    population.sort_by(|a, b| a.fitness.cmp(&b.fitness));
    return population[0];
}

fn main() {
    let result = run();
    println!("Best candidate has fitness {}: {:?}", result.fitness, result.genes);
}
