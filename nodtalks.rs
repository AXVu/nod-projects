use std::{io, usize, collections::HashMap};
use vio::{self, Network, build_network, AF};


// str_to_AF converts a given &str to a vio::AF
fn str_to_af(input: &str) -> AF {
    match input {
        "ReLU" => AF::ReLU,
        "Tanh" => AF::Tanh,
        "Sigmoid" => AF::Sigmoid,
        "Linear" => AF::Linear,
        _ => AF::ReLU
    }
} // End str_to_AF

// network_from_shape returns a Network with a user specified shape
fn network_from_shape() -> Network {
    let mut read: String = String::new();
    match io::stdin().read_line(&mut read) {
        Ok(_) => {
            let read = read.trim();

            let mut features: Vec<&str> = read.split(",").collect();
            let num_features: usize = features.len();
            
            // Split off activation functions
            let gen_f_str: &str = features.remove(num_features - 2);
            let out_f_str: &str = features.remove(num_features - 2);
            // Convert str to AF
            let gen_func:AF = str_to_af(gen_f_str);
            let out_func:AF = str_to_af(out_f_str);

            // Get network inputs and shape
            let mut shape: Vec<usize> = features.iter().map(|x| x.parse::<usize>().expect("failed to read shape")).collect();
            let inps: usize = shape.remove(0);

            build_network(inps, shape, gen_func, out_func)
        },
        Err(_) => Network { layers: Vec::new() }
    }
} // End network_from_shape

// words_from_sentence reads in a line and splits it into its respective words
fn words_from_sentence(sentence: String) -> Vec<String> {
    vec![String::new()]
} // End words_from_sentence

// get user instruction reads in an instruction line
fn get_user_instruction() -> Result<String, String> {
    let mut read: String = String::new();
    match io::stdin().read_line(&mut read) {
        Ok(_) => {
            let read = read.trim();
            Ok(read.to_string())
        },
        Err(_) => Err("Failed to read instruction".to_string())
    }
} // End get_user_input

// add_vocab adds given word to vocab
fn add_vocab(dictionary: &mut HashMap<String, Vec<f32>>, word: String, network: &mut Network) {
    // Maintain length of dictionary entries
    for k in dictionary.values_mut() {
        k.push(0.0)
    }
    // Add new word to dictionary
    let mut word_vec: Vec<f32> = (0..dictionary.len()).map(|_| 0.0).collect();
    word_vec.push(1.0);
    dictionary.insert(word, word_vec);
    network.add_input();
    network.add_input();
} // End add_vocab

// add_specified_word queries user for a new word to add to the dictionary
fn add_specified_word(dictionary: &mut HashMap<String, Vec<f32>>, network: &mut Network) {
    // query word
    println!("Input new word:");
    let word: String = match get_user_instruction() {
        Ok(f) => f,
        Err(f) => {
            println!("{}",&f);
            String::new()
        }
    };
    // If the word is not already in the dictionary, add it
    if !dictionary.contains_key(&word){
        add_vocab(dictionary, word, network)
    };
} // End add_specified_word

// predict_word predict 
fn predict_word(dictionary: &mut HashMap<String, Vec<f32>>, network: &mut Network) {
    
}

fn main() {
    let mut instruction: &str = "";
    let mut temp_ins: String;

    let mut network: Network;

    let mut dictionary: HashMap<String, Vec<f32>> = HashMap::new();

    // Construct initial network //
    println!("Input network shape and activation functions");
    network = network_from_shape();

    while instruction != "end" {
        println!("Input instruction");
        // Instruction read
        temp_ins = match get_user_instruction() {
            Ok(f) => f,
            Err(f) => {
                println!("{}",&f);
                String::new()
            }
        };
        instruction = temp_ins.as_str();

        ///// Actions /////
        match instruction {
            "new network" => network = network_from_shape(),
            "add word" => add_specified_word(&mut dictionary, &mut network),
            "print dict" => println!("{:?}",dictionary.keys()),

            // End and cycle //
            "end" => println!("Ending session!"),
            _ => ()
        }
    

    }
}