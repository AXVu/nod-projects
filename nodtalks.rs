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
    sentence.to_lowercase().split(" ").map(|x| x.to_string()).collect()
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
fn add_vocab(dictionary: &mut HashMap<String, Vec<f32>>, word_idx: &mut HashMap<String, usize>, idx_word: &mut HashMap<usize, String>, word: String, network: &mut Network) {
    // Maintain length of dictionary entries
    for k in dictionary.values_mut() {
        k.push(0.0)
    }
    // Add new word to dictionary
    let mut word_vec: Vec<f32> = (0..dictionary.len()).map(|_| 0.0).collect();
    word_vec.push(1.0);
    dictionary.insert(word.clone(), word_vec);
    word_idx.insert(word.clone(), dictionary.len() - 1);
    idx_word.insert(dictionary.len() - 1, word.clone());
    network.add_input();
    network.add_input();
    network.add_output(AF::Sigmoid)
} // End add_vocab

// get_sentence queries user for a sentence and then splits it, returning Vec<String>
fn get_sentence() -> Vec<String> {
    let input: String = match get_user_instruction() {
        Ok(f) => f,
        Err(f) => {
            println!("{}",&f);
            String::new()
        }
    };
    words_from_sentence(input)
}

// add_specified_word queries user for a new word to add to the dictionary
fn add_specified_word(dictionary: &mut HashMap<String, Vec<f32>>, word_idx: &mut HashMap<String, usize>, idx_word: &mut HashMap<usize, String>, network: &mut Network) {
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
        add_vocab(dictionary, word_idx, idx_word, word, network)
    };
} // End add_specified_word

// zip_words takes the history vec and previous word vec and returns a zipped version
fn zip_words(previous_word:Vec<f32>, history_vec: Vec<f32>) -> Vec<f32> {
    let mut zipped: Vec<f32> = Vec::new();
    for (word, hist) in previous_word.iter().zip(history_vec.iter()) {
        zipped.push(*word);
        zipped.push(*hist);
    }
    zipped
}

// gen_features takes in word history to generate the features to input for generation
fn gen_features(dictionary: &mut HashMap<String, Vec<f32>>, word_idx: &mut HashMap<String, usize>, past_words: Vec<String>) -> Vec<f32> {
    let mut history: Vec<String> = past_words.clone();
    history.reverse();

    let previous_word = dictionary.get(&history.remove(0)).expect("failed to extract previous word").clone();

    let mut history_vec: Vec<f32> = (0..previous_word.len()).map(|_| 0.0).collect();

    let mut iter_count: f32 = 1.0;
    for word in history.iter() {
        history_vec[*word_idx.get(word).expect("word_idx failure")] += 1.0 / iter_count;
        iter_count += 1.0;
    }
    
    zip_words(previous_word, history_vec)
} // End gen_features

// next_word takes in a network and the past words and will run assuming that the model is already in the state that it should be given past words, and that all words are known
fn next_word(dictionary: &mut HashMap<String, Vec<f32>>, word_idx: &mut HashMap<String, usize>, idx_word: &mut HashMap<usize, String>, network: &mut Network, past_words: Vec<String>) -> String {
    
    let features = gen_features(dictionary, word_idx, past_words);

    let prediction = network.process(features);

    let mut max_idx: usize = 0;
    let mut max_num: f32 = 0.0;

    for word in 0..prediction.len() {
        if prediction[word] > max_num {
            max_num = prediction[word];
            max_idx = word;
        }
    }

    idx_word[&max_idx].clone()

} // End next_word

// predict_word predict 
fn predict_word(dictionary: &mut HashMap<String, Vec<f32>>, word_idx: &mut HashMap<String, usize>, idx_word: &mut HashMap<usize, String>, network: &mut Network, past_words: Vec<String>) -> String {
    let mut working_network = network.clone();
    
    let mut working_words: Vec<String> = Vec::new();

    // High-learning rate train network to accustom it to the sentence
    for word in 0..past_words.len() - 1 {
        working_words.push(past_words[word].clone());
        let features: Vec<f32> = gen_features(dictionary, word_idx, working_words.clone());
        let expect_idx: usize = word_idx[&past_words[word + 1]];
        let expected: Vec<f32> = (0..dictionary.len()).map(|x| if x == expect_idx {1.0} else {0.0}).collect();
        working_network.train(features, expected, "mse", 0.1)
    } 

    next_word(dictionary, word_idx, idx_word, &mut working_network, past_words)
}

fn main() {
    let mut instruction: &str = "";
    let mut temp_ins: String;

    let mut network: Network;

    let mut word_idx: HashMap<String, usize> = HashMap::new();
    let mut idx_word: HashMap<usize, String> = HashMap::new();
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
            "add word" => add_specified_word(&mut dictionary, &mut word_idx, &mut idx_word, &mut network),
            "print dict" => println!("{:?}",dictionary.keys()),
            "print network" => println!("{:?}",network.get_weights()),
            "next word" => println!("{:?}", next_word(&mut dictionary, &mut word_idx, &mut idx_word, &mut network, get_sentence())),
            "predict word" => println!("{:?}", predict_word(&mut dictionary, &mut word_idx, &mut idx_word, &mut network, get_sentence())),

            // End and cycle //
            "end" => println!("Ending session!"),
            _ => ()
        }
    }
}