use rand::Rng;
use rand::seq::SliceRandom;
struct Individual(Vec<u8>, u8);
struct Population(Vec<Individual>);
const MY_ENCODING: &[u8] = b"hello world";

fn main() {
    let encoding : Vec<u8> = b"hello world".to_vec(); // encoded bytes we will eventually check distance from

    let my_len = encoding.len(); // length for convenience

    let mut my_pop = Population((0..100).map(|_| generate_individual(&my_len)).collect()); // creates population
    let mut found : bool = false;
    for _ in 0..150 {
        my_pop.0.sort_by(|a, b| a.1.cmp(&b.1)); // sorting population by fitness - best candidates at top
    
        my_pop = generate_new_population(my_pop, 100);
    
        found = print_and_check_population(&my_pop);
        
        if found == true {
            
            println!("CONVERGED!!!!!");
            break;
        }
    }  
}
fn generate_new_population(old_population : Population, size : u8) -> Population{
    let old_population_vector = old_population.0;
    let mut new_population_vec = vec!();
    let mut rng = rand::thread_rng();
    
    let to_choose_from = &old_population_vector[0..old_population_vector.len()/2];
    for _ in 0..size {
        // Attempt to choose two parents from the old_population
        if let (Some(parent1), Some(parent2)) = ( // we do this because "choose" could return None or smth bad, so we have to handle the unpack
            to_choose_from.choose(&mut rng),
            to_choose_from.choose(&mut rng),
        ) { // handling if choose works
            let child = mate_individuals(parent1, parent2);
            new_population_vec.push(child);
        } else {
            // Handle the case where parents could not be chosen, perhaps by breaking or adding a default Individual
            // For simplicity, we're just continuing here, but you might want to handle this more robustly
            continue;
        }
    }
    Population(new_population_vec)
}

fn mate_individuals(ind1 : &Individual, ind2 : &Individual) -> Individual {
    // walks char by char and randomly selects genes from parents or mutates genes
    let mut my_vec = vec!();
    for (individual1_char, individual2_char) in ind1.0.iter().zip(ind2.0.iter()) {
        let mut rng = rand::thread_rng();
        let my_num = rng.gen_range(0..100);
        if my_num >= 0 && my_num < 45 {
            my_vec.push(individual1_char.clone());
        } else if my_num >= 45 && my_num < 90 {
            my_vec.push(individual2_char.clone());
        }
        else {
            my_vec.push(rng.gen_range(0..128));
        }
    }
    let fitness = calculate_individual_fitness(&my_vec);
    let my_individual = Individual(my_vec, fitness);
    my_individual
}

fn print_and_check_population(pop: &Population) -> bool {
    let mut rng = rand::thread_rng();
    let to_choose_from : &Vec<Individual> = &pop.0;

    for _ in 0..10 {
        if let Some(ind) = to_choose_from.choose(&mut rng) { // because random choice might give smth we can't use these method on it we explicitly hande cases
            print_individual(&ind);
            
            if ind.1 == 0 {

                return true;
            }
        } else {
            continue; // handle edge case
        }

        
    }
    return false
}

fn print_individual(ind : &Individual) { // function name 
    match String::from_utf8(ind.0.clone()) {
        Ok(my_str) => {
            println!("{}, fitness: {}", my_str, ind.1);
        },
        Err(e) => println!("Failed to convert individual to string: {:?}", e),
    } 
}

fn generate_individual(n : &usize) -> Individual { // creates individual of 128-byte string
    let encoding = MY_ENCODING.to_vec();

    let mut rng = rand::thread_rng();
    let my_vec : Vec<u8> = (0..*n).map(|_| rng.gen_range(0..128)).collect(); // generate random list of u8s
    let fitness = calculate_individual_fitness(&my_vec);
    Individual(my_vec, fitness)
}

fn calculate_individual_fitness(to_check: &Vec<u8>) -> u8 {
    // calculates fitness by checking individual chars
    let encoding = MY_ENCODING.to_vec();
    let mut fitness = 0;
    for (a, b) in encoding.iter().zip(to_check.iter()) {
        if a != b {
            fitness += 1;
        }

    }
    fitness
}