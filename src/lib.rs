use rand::Rng;

pub fn life_facts() -> String {
    let facts = vec![
        "If you try to walk in a straight line, you will always be at least one degree off from the path you intended to follow.",
        "People fight with you not because they are stupid, but because you are!",
        "our left hand will always be attached to the left side of your body unless you undergo a hand transplant surgery.",
        "Where there is a will there might not be a way",
    ];

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..facts.len());

    String::from(facts[index])
}

