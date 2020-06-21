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
const CHROMOSOME_SIZE: usize = 10;
const MAX_GENE: i8 = CHROMOSOME_SIZE as i8;
const MAX_FITNESS: u8 = 100;

const MAX_ITERATIONS: u32 = 5000;
const ELITE: usize = 6;

#[derive(Copy, Clone)]
struct Chromosome {
    genes: [i8; CHROMOSOME_SIZE],
    fitness: u8,
}

impl Default for Chromosome {
    fn default() -> Self {
        let mut rng = rand::thread_rng();

        let mut genes = [0; CHROMOSOME_SIZE];
        for i in 0..CHROMOSOME_SIZE {
            genes[i] = rng.gen_range(0, MAX_GENE);
        }
        let fitness = fitness(genes);
        return Chromosome { genes, fitness };
    }
}

fn fitness(genes: [i8; CHROMOSOME_SIZE]) -> u8 {
    let mut result = 0;
    for i in 0..genes.len() {
        // TODO Can I remove this cast somehow?
        if genes[i] == i as i8 {
            result += 1;
        }
    }
    return result;
}

// Assumes population is sorted
fn find_best(population: [Chromosome; POPULATION_SIZE]) -> Chromosome {
    return population.last().expect("Population was empty, somehow").to_owned();
}

// Assumes population is sorted, returns an unsorted one
fn next_generation(population: [Chromosome; POPULATION_SIZE]) -> [Chromosome; POPULATION_SIZE] {
    const START: usize = POPULATION_SIZE - ELITE - 1;
    const END: usize = POPULATION_SIZE - 1;
    let elite = &population[START..END];
    assert_eq!(elite.len(), ELITE);

    let mut offspring = unsafe { MaybeUninit::<[Chromosome; ELITE]>::uninit().assume_init() };
    for i in (0..elite.len()).step_by(2) {
        let mom = elite[i];
        let dad = elite[i + 1];

        offspring[i] = make_offspring(mom, dad);
        offspring[i + 1] = make_offspring(dad, mom);
    }

    assert_eq!(offspring.len(), ELITE);

    let mut final_population = unsafe { MaybeUninit::<[Chromosome; POPULATION_SIZE]>::uninit().assume_init() };

    for (i, chromosome) in elite.iter().enumerate() { final_population[i] = chromosome.to_owned() }
    for (i, chromosome) in offspring.iter().enumerate() { final_population[i + ELITE] = chromosome.to_owned() };
    for i in (ELITE + offspring.len())..POPULATION_SIZE { final_population[i] = Default::default(); }

    return final_population;
}

fn make_offspring(mom: Chromosome, dad: Chromosome) -> Chromosome {
    const MIDDLE: usize = CHROMOSOME_SIZE / 2;

    let mut child = [0; CHROMOSOME_SIZE];
    mom.genes[0..MIDDLE].iter().enumerate().for_each(|(i, gene)| child[i] = gene.to_owned());
    dad.genes[MIDDLE..CHROMOSOME_SIZE].iter().enumerate().for_each(|(i, gene)| child[i + MIDDLE] = gene.to_owned());
    return Chromosome { genes: child, fitness: fitness(child) };
}

fn run() -> Chromosome {
    let mut population = unsafe { MaybeUninit::<[Chromosome; POPULATION_SIZE]>::uninit().assume_init() };

    for i in 0..population.len() {
        population[i] = Default::default();
    }

    for _ in 0..MAX_ITERATIONS {
        // TODO Create a Population struct instead of assuming it's always sorted in this step
        population.sort_by(|a, b| a.fitness.cmp(&b.fitness));

        let best_individual = find_best(population);
        if best_individual.fitness == MAX_FITNESS { return best_individual; }

        population = next_generation(population);
    }

    population.sort_by(|a, b| a.fitness.cmp(&b.fitness));
    return find_best(population);
}

fn main() {
    assert_eq!((ELITE % 2), 0);
    assert!((ELITE * 2) < POPULATION_SIZE);
    let result = run();
    println!("Best candidate has fitness {}: {:?}", result.fitness, result.genes);
}
