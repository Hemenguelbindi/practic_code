pub fn bio_neo(){
    let user = vec![
        "NICKNAME: Neo",
        "CITY: St-Petersburg",
        "AGE: 35",
        "HEIGHT: 180",
        "WEIGHT: 75",
    ];
    for i in &user{
        println!("{i}")
    }
}