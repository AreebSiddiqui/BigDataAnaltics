use std::fs;
use std::io;
use stemmer::Stemmer;
use std::collections::HashMap;
fn main() {
    println!("1. Read file and converted to lower case\n************");
    let mut text = readfile().unwrap();
    println!("{:#?}",text);
    text = text.to_lowercase();  // converted to lower casse
   
    
    
    println!("\n\n2. Tokenization and Stemming\n************");
    let mut words:Vec <String> = Vec::new();
    let mut stemmer = Stemmer::new("english").unwrap();
    for word in text.split_whitespace() {
      let s : String = stemmer.stem_str(word).to_owned();
      words.push(s);
    }
    println!("{:?}",words);

    println!("{:?}",word_count(&mut words));



    
    //println!("{:?}",stop_word(&mut words));

}

// fn stop_word (list : &mut[String]) -> Vec<String> {
//     let stop_words:Vec <String>  = vec!["is".to_string(),"the".to_string(),"are".to_string(),"i".to_string(),"on".to_string(),"it".to_string(),"a".to_string(),"of".to_string(),"in".to_string()];
//     let mut words:Vec <String> = Vec::new();
    
//     for k in list {
//         for l in &stop_words {
//            if k==l {
//             println!("{},{}",k,l);
             
//            }
//            else {
//                words.push(k.to_string());
               
//             }
//         }
        
        
//     }
//     words
    
// }

//Word Counting
fn word_count(list: &mut [String]) -> HashMap <String,u32> {
    println!("\n\nEach Word Count\n********************");
    let mut map: HashMap<String,u32> = HashMap::new();
    for word in list.iter() {
        let count = map.entry(word.to_string()).or_insert(0);
        *count += 1;
    }
    map

}



fn readfile() -> Result<String, io::Error> {
    //let mut reader = String::new(); //created a string that will possess content of file.
     fs::read_to_string("file.txt") //file read here 
}





// fn main () {
//     let mut v = vec![1,2,4];
//     let mut m = vec![1,4,5,6,7,8,9,0,1,2,3];


//     for word in v.iter() {
//         for word1 in m.iter() {
//             if word != word1 {  
//                 println!("{},{}",word,word1);
//             } 
//         }
//     }
// }